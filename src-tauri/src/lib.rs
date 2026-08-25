mod bootstrap;
mod dialog;
mod host_process;
mod materials;
mod native_effects;
mod profile;
mod protocol;
mod recovery;
mod user_data;
mod window_ops;
mod window_spec;
mod zip_store;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use protocol::{decode_message, encode_message, err_result, ok_result, RpcMessage, PROTOCOL_VERSION};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use std::sync::mpsc;
use std::time::Duration;
use tauri::{AppHandle, DragDropEvent, Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_opener::OpenerExt;
use native_effects::NativeSession;
use window_ops::{folder_drop_script, open_external_href, persist_window_bounds, WindowBounds};
use window_spec::{chrome_for_mode, ShellPayload};

struct AppState {
    generation: Mutex<Option<String>>,
    host_stdin: Mutex<Option<std::process::ChildStdin>>,
    payload: Mutex<Option<ShellPayload>>,
    quitting: AtomicBool,
    session: Mutex<NativeSession>,
}

fn current_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "win32"
    } else {
        "linux"
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![open_external])
        .manage(AppState {
            generation: Mutex::new(None),
            host_stdin: Mutex::new(None),
            payload: Mutex::new(None),
            quitting: AtomicBool::new(false),
            session: Mutex::new(NativeSession::new(
                user_data::desktop_user_data_dir(),
                current_platform(),
            )),
        })
        .setup(|app| {
            let handle = app.handle().clone();
            let user_data = user_data::desktop_user_data_dir();
            let _bootstrap = bootstrap::load_native_bootstrap(&user_data);
            let _profile = profile::load_profile_state(&user_data.join("profile-selection").join("state.json"));
            // macOS tao panics in did_finish_launching if the last window is closed during setup.
            // Create main before spawning Host so shell.mount cannot race a missing window.
            let window = create_main_window(app)?;
            bind_main_window_events(&window, handle.clone());
            let show = MenuItemBuilder::with_id("show", "Show DSH Desktop").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;
            let _tray = TrayIconBuilder::with_id("main")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        if let Some(state) = app.try_state::<AppState>() {
                            state.quitting.store(true, Ordering::SeqCst);
                        }
                        app.exit(0);
                    }
                    id => {
                        write_to_host(
                            app,
                            &protocol::RpcMessage::Event {
                                v: protocol::PROTOCOL_VERSION,
                                method: "event.trayInvoke".into(),
                                params: Some(json!({ "id": id })),
                            },
                        );
                    }
                })
                .build(app)?;
            spawn_host_bridge(handle.clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running DSH Desktop");
}

const EXTERNAL_LINK_CLICK_SCRIPT: &str = r#"document.addEventListener('click', (event) => {
  const node = event.target && event.target.closest && event.target.closest('a[href]');
  if (!node) return;
  const href = node.href || '';
  const invoke = window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
  if (typeof invoke !== 'function') return;
  try {
    const target = new URL(href, location.href);
    const loopback = target.hostname === '127.0.0.1' || target.hostname === 'localhost' || target.hostname === '[::1]';
    const external = target.protocol === 'mailto:' || ((target.protocol === 'http:' || target.protocol === 'https:') && !loopback);
    if (!external) return;
    event.preventDefault();
    invoke('open_external', { origin: location.origin, href: target.href });
  } catch (error) {}
}, true);"#;

fn create_main_window(app: &tauri::App) -> tauri::Result<tauri::WebviewWindow> {
    let config = app
        .config()
        .app
        .windows
        .iter()
        .find(|window| window.label == "main")
        .cloned()
        .expect("main window config");
    let nav_handle = app.handle().clone();
    WebviewWindowBuilder::from_config(app.handle(), &config)?
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true)
        .traffic_light_position(tauri::Position::Logical(tauri::LogicalPosition { x: 16.0, y: 12.0 }))
        .on_navigation(move |url| {
            let href = url.as_str().to_string();
            let origin = nav_handle
                .try_state::<AppState>()
                .and_then(|state| {
                    state
                        .payload
                        .lock()
                        .ok()
                        .and_then(|payload| payload.as_ref().map(|item| item.url.clone()))
                })
                .unwrap_or_else(|| href.clone());
            match open_external_href(&origin, &href, |target| {
                nav_handle
                    .opener()
                    .open_url(target, None::<&str>)
                    .map_err(|error| error.to_string())
            }) {
                Ok(true) => false,
                _ => true,
            }
        })
        .on_page_load(|window, payload| {
            if payload.event() != tauri::webview::PageLoadEvent::Finished {
                return;
            }
            let _ = window.eval(EXTERNAL_LINK_CLICK_SCRIPT);
        })
        .build()
}

fn bind_main_window_events(window: &tauri::WebviewWindow, app_handle: AppHandle) {
    window.on_window_event(move |event| match event {
        WindowEvent::CloseRequested { api, .. } => {
            if let Some(state) = app_handle.try_state::<AppState>() {
                if !state.quitting.load(Ordering::SeqCst) {
                    api.prevent_close();
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
            }
        }
        WindowEvent::DragDrop(DragDropEvent::Drop { paths, position }) => {
            let list = paths
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<_>>();
            let script = folder_drop_script(&list, position.x, position.y);
            if let Some(state) = app_handle.try_state::<AppState>() {
                if let Ok(mut session) = state.session.lock() {
                    session.last_drop_script = Some(script.clone());
                }
            }
            let _ = app_handle.emit("dsh-desktop-folder-drop", json!({ "paths": list }));
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.eval(&script);
            }
        }
        WindowEvent::Moved(_) | WindowEvent::Resized(_) => {
            if let Some(window) = app_handle.get_webview_window("main") {
                if let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) {
                    let bounds = WindowBounds {
                        x: pos.x,
                        y: pos.y,
                        width: size.width,
                        height: size.height,
                    };
                    if let Some(state) = app_handle.try_state::<AppState>() {
                        if let Ok(mut session) = state.session.lock() {
                            let _ = persist_window_bounds(&session.user_data, &bounds);
                            session.bounds = Some(bounds);
                        }
                    }
                }
            }
        }
        _ => {}
    });
}

#[tauri::command]
fn open_external(app: AppHandle, origin: String, href: String) -> Result<bool, String> {
    open_external_href(&origin, &href, |url| {
        app.opener()
            .open_url(url, None::<&str>)
            .map_err(|error| error.to_string())
    })
}

fn run_on_main_thread<T: Send + 'static>(
    app: &AppHandle,
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, String> {
    let (tx, rx) = mpsc::channel();
    app.run_on_main_thread(move || {
        let _ = tx.send(work());
    })
    .map_err(|error| error.to_string())?;
    rx.recv_timeout(Duration::from_secs(30))
        .map_err(|error| error.to_string())
}

fn spawn_host_bridge(app: AppHandle) {
    std::thread::spawn(move || {
        let user_data = user_data::desktop_user_data_dir();
        let resource_dir = app.path().resource_dir().ok();
        let entry = host_process::host_entry_path_from(resource_dir.as_deref());
        if !entry.exists() {
            let _ = open_recovery_window(
                &app,
                json!({
                    "failureStage": "runtime-bootstrap",
                    "failureDetail": format!("Host entry is missing: {}", entry.display()),
                }),
            );
            return;
        }
        let mut child = match host_process::spawn_host(&entry, &user_data, false, resource_dir.as_deref()) {
            Ok(child) => child,
            Err(error) => {
                let _ = open_recovery_window(
                    &app,
                    json!({
                        "failureStage": "host-boot",
                        "failureDetail": error.to_string(),
                    }),
                );
                return;
            }
        };
        if let Some(stdin) = host_process::take_stdin(&mut child) {
            if let Some(state) = app.try_state::<AppState>() {
                *state.host_stdin.lock().expect("stdin lock") = Some(stdin);
            }
        }
        let stdout = child.stdout.take().expect("host stdout");
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let Ok(line) = line else { break };
            match decode_message(&line) {
                Ok(RpcMessage::Request { id, method, params, .. }) => {
                    let result = handle_host_request(&app, &method, params.unwrap_or(Value::Null));
                    let reply = match result {
                        Ok(value) => ok_result(id, value),
                        Err(message) => err_result(id, message),
                    };
                    write_to_host(&app, &reply);
                }
                Ok(RpcMessage::Event { method, params, .. }) => {
                    if method == "event.sidecarFailed" {
                        let _ = open_recovery_window(&app, params.unwrap_or(Value::Null));
                    } else if method.starts_with("shell.") {
                        let _ = handle_host_request(&app, &method, params.unwrap_or(Value::Null));
                    }
                }
                Err(error) => eprintln!("dsh-desktop: {error}"),
                _ => {}
            }
        }
    });
}

fn write_to_host(app: &AppHandle, message: &RpcMessage) {
    if let Some(state) = app.try_state::<AppState>() {
        if let Ok(mut stdin) = state.host_stdin.lock() {
            if let Some(stdin) = stdin.as_mut() {
                if let Ok(encoded) = encode_message(message) {
                    let _ = stdin.write_all(encoded.as_bytes());
                    let _ = stdin.flush();
                }
            }
        }
    }
}

fn handle_host_request(app: &AppHandle, method: &str, params: Value) -> Result<Value, String> {
    let state = app.state::<AppState>();
    let result = {
        let mut session = state.session.lock().expect("session");
        session.dispatch(method, params.clone())?
    };
    match method {
        "shell.schedule" => {
            if let Some(id) = result.get("generationId").and_then(Value::as_str) {
                *state.generation.lock().expect("generation") = Some(id.to_string());
            }
            if let Ok(payload) = serde_json::from_value::<ShellPayload>(params.clone()) {
                *state.payload.lock().expect("payload") = Some(payload);
            }
        }
        "shell.mount" => {
            let payload = state
                .payload
                .lock()
                .expect("payload")
                .clone()
                .ok_or_else(|| "no scheduled generation".to_string())?;
            let handle = app.clone();
            run_on_main_thread(app, move || mount_main_window(&handle, &payload))??;
        }
        "shell.release" => {
            *state.generation.lock().expect("generation") = None;
            *state.payload.lock().expect("payload") = None;
        }
        "shell.show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "shell.openRecovery" => {
            if params.get("href").is_none() {
                open_recovery_window(app, params.get("state").cloned().unwrap_or(params))?;
            }
        }
        "shell.openDialog" => {
            if params.get("href").is_none() {
                return open_dialog_window(app, params);
            }
        }
        "shell.openProfileCreate" => {
            if params.get("name").is_none() {
                open_auxiliary(app, "profile-create", "profile-create.html", 480.0, 320.0)?;
            }
        }
        "shell.openDevtools" => {
            if let Some(url) = result.get("url").and_then(Value::as_str) {
                open_devtools_window(app, url)?;
            }
        }
        "shell.reloadRenderer" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("location.reload()");
            }
        }
        "shell.toggleDeveloperTools" => {
            if let Some(window) = app.get_webview_window("main") {
                if window.is_devtools_open() {
                    let _ = window.close_devtools();
                } else {
                    window.open_devtools();
                }
            }
        }
        "shell.pickDirectory" => {
            if params.get("path").is_none() {
                let folder = tauri_plugin_dialog::DialogExt::dialog(app)
                    .file()
                    .blocking_pick_folder();
                return Ok(json!({
                    "path": folder.and_then(|path| path.as_path().map(|value| value.to_string_lossy().into_owned()))
                }));
            }
        }
        "shell.notify" | "shell.notifyAttention" => {
            if let (Some(title), Some(body)) = (
                params.get("title").and_then(Value::as_str),
                params.get("body").and_then(Value::as_str),
            ) {
                let _ = tauri_plugin_notification::NotificationExt::notification(app)
                    .builder()
                    .title(title)
                    .body(body)
                    .show();
            }
            if method == "shell.notifyAttention" {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.request_user_attention(Some(tauri::UserAttentionType::Informational));
                }
            }
        }
        "shell.tray.upsert" | "shell.tray.remove" => rebuild_tray_menu(app),
        "shell.openTerminal" => {
            if let Some(command) = result.get("command").and_then(Value::as_array) {
                if let Some(program) = command.first().and_then(Value::as_str) {
                    let args = command
                        .iter()
                        .skip(1)
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect::<Vec<_>>();
                    let _ = std::process::Command::new(program).args(args).spawn();
                }
            }
        }
        "shell.restart" | "shell.restartRecovery" => {
            app.restart();
        }
        _ => {}
    }
    Ok(result)
}

fn rebuild_tray_menu(app: &AppHandle) {
    let Ok(show) = MenuItemBuilder::with_id("show", "Show DSH Desktop").build(app) else {
        return;
    };
    let Ok(quit) = MenuItemBuilder::with_id("quit", "Quit").build(app) else {
        return;
    };
    let mut extras = Vec::new();
    if let Some(state) = app.try_state::<AppState>() {
        if let Ok(session) = state.session.lock() {
            for tray in &session.tray {
                if let Ok(item) = MenuItemBuilder::with_id(&tray.id, &tray.label).build(app) {
                    extras.push(item);
                }
            }
        }
    }
    let mut refs: Vec<&dyn tauri::menu::IsMenuItem<tauri::Wry>> = vec![&show, &quit];
    for extra in &extras {
        refs.push(extra);
    }
    if let Ok(menu) = MenuBuilder::new(app).items(&refs).build() {
        if let Some(icon) = app.tray_by_id("main") {
            let _ = icon.set_menu(Some(menu));
        }
    }
}

fn mount_main_window(app: &AppHandle, payload: &ShellPayload) -> Result<(), String> {
    let Some(window) = app.get_webview_window("main") else {
        return Err("main window missing".into());
    };
    let plan = {
        let state = app.state::<AppState>();
        let session = state.session.lock().expect("session");
        session.last_plan.clone()
    };
    let material = plan
        .as_ref()
        .map(|plan| plan.material.as_str())
        .unwrap_or(payload.material.as_str());
    apply_material(&window, material);
    let _ = chrome_for_mode(payload.mode.as_str());
    #[cfg(target_os = "macos")]
    {
        let _ = window.set_title_bar_style(tauri::TitleBarStyle::Overlay);
    }
    if let Some(plan) = plan.as_ref() {
        let zoom = 1.0 + f64::from(plan.zoom_level) * 0.1;
        let _ = window.eval(&format!("document.documentElement.style.zoom = '{zoom}'"));
        if let Some(bounds) = &plan.bounds {
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: bounds.x,
                y: bounds.y,
            }));
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: bounds.width,
                height: bounds.height,
            }));
        }
    }
    let parsed = url::Url::parse(&payload.url).map_err(|error: url::ParseError| error.to_string())?;
    window.navigate(parsed).map_err(|error| error.to_string())?;
    let _ = window.set_title(&payload.window_title);
    let _ = window.set_min_size(Some(tauri::Size::Logical(tauri::LogicalSize {
        width: payload.min_width as f64,
        height: payload.min_height as f64,
    })));
    let _ = window.eval(&format!("window.location.replace({})", serde_json::to_string(&payload.url).unwrap()));
    let _ = window.show();
    Ok(())
}

fn apply_material(window: &tauri::WebviewWindow, material: &str) {
    #[cfg(target_os = "macos")]
    if material == "transparent" {
        let _ = window_vibrancy::apply_vibrancy(
            window,
            window_vibrancy::NSVisualEffectMaterial::Sidebar,
            None,
            None,
        );
    }
    #[cfg(target_os = "windows")]
    {
        if material == "acrylic" {
            let _ = window_vibrancy::apply_acrylic(window, Some((32, 32, 32, 204)));
        }
        if material == "mica" {
            let _ = window_vibrancy::apply_mica(window, None);
        }
    }
}

fn open_devtools_window(app: &AppHandle, href: &str) -> Result<(), String> {
    #[cfg(not(dsh_devtools))]
    {
        let _ = (app, href);
        return Ok(());
    }
    #[cfg(dsh_devtools)]
    {
        let parsed = url::Url::parse(href).map_err(|error: url::ParseError| error.to_string())?;
        if let Some(existing) = app.get_webview_window("devtools") {
            existing.navigate(parsed).map_err(|error| error.to_string())?;
            let _ = existing.show();
            let _ = existing.set_focus();
            return Ok(());
        }
        WebviewWindowBuilder::new(app, "devtools", WebviewUrl::External(parsed))
            .title("DSH Desktop · Developer")
            .inner_size(920.0, 640.0)
            .build()
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

fn open_recovery_window(app: &AppHandle, state: Value) -> Result<(), String> {
    let encoded = data_encoding_base64url(&state);
    let url = format!("recovery.html?state={encoded}");
    let handle = app.clone();
    app.run_on_main_thread(move || {
        let _ = open_auxiliary(&handle, "recovery", &url, 720.0, 560.0);
    })
    .map_err(|error| error.to_string())
}

fn open_dialog_window(app: &AppHandle, params: Value) -> Result<Value, String> {
    let buttons = params
        .get("buttons")
        .and_then(Value::as_array)
        .map(|items| items.len())
        .unwrap_or(1);
    let cancel = params
        .get("cancelId")
        .and_then(Value::as_u64)
        .unwrap_or(buttons.saturating_sub(1) as u64) as u32;
    let encoded = data_encoding_base64url(&params);
    if let Some(existing) = app.get_webview_window("dialog") {
        let _ = existing.close();
    }
    let (tx, rx) = mpsc::channel::<u32>();
    WebviewWindowBuilder::new(app, "dialog", WebviewUrl::App(format!("dialog.html?state={encoded}").into()))
        .title("DSH Desktop")
        .inner_size(480.0, 280.0)
        .on_navigation(move |url| {
            if let Some(response) = dialog::parse_dialog_response(url.as_str(), buttons) {
                let _ = tx.send(response);
                return false;
            }
            true
        })
        .build()
        .map_err(|error| error.to_string())?;
    let response = rx.recv_timeout(Duration::from_secs(300)).unwrap_or(cancel);
    if let Some(window) = app.get_webview_window("dialog") {
        let _ = window.close();
    }
    Ok(json!({ "response": response }))
}

fn open_auxiliary(app: &AppHandle, label: &str, url: &str, width: f64, height: f64) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window(label) {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }
    let app_handle = app.clone();
    WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into()))
        .title("DSH Desktop")
        .inner_size(width, height)
        .on_navigation(move |href| {
            if let Some(_action) = recovery::parse_recovery_href(href.as_str()) {
                if let Some(state) = app_handle.try_state::<AppState>() {
                    if let Ok(mut session) = state.session.lock() {
                        let _ = session.dispatch(
                            "shell.openRecovery",
                            json!({ "href": href.as_str() }),
                        );
                        let lifecycle = session.pending_lifecycle.clone();
                        drop(session);
                        if lifecycle.as_deref() == Some("quit") {
                            app_handle.exit(0);
                        }
                        if lifecycle.as_deref() == Some("restart") {
                            app_handle.restart();
                        }
                    }
                }
                return false;
            }
            true
        })
        .build()
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn data_encoding_base64url(value: &Value) -> String {
    let json = serde_json::to_vec(value).unwrap_or_else(|_| b"{}".to_vec());
    URL_SAFE_NO_PAD.encode(json)
}

#[allow(dead_code)]
const _PROTOCOL: u8 = PROTOCOL_VERSION;
