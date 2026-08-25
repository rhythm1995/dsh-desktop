use crate::bootstrap::{self, NativeBootstrap};
use crate::dialog::parse_dialog_response;
use crate::profile::{self, write_profile_state};
use crate::recovery::{parse_recovery_href, RecoveryAction};
use crate::window_ops::{
    folder_drop_script, load_window_bounds, persist_window_bounds, plan_generation, should_open_externally,
    WindowApplyPlan, WindowBounds,
};
use crate::window_spec::ShellPayload;
use crate::zip_store::build_zip_store;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayItem {
    pub id: String,
    pub group: String,
    pub order: i64,
    pub label: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopNotification {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalLaunch {
    pub profile_name: String,
    pub command: Vec<String>,
    pub record_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateOutcome {
    pub status: String,
    pub current_version: String,
    pub latest_version: String,
}

pub struct NativeSession {
    pub user_data: PathBuf,
    pub platform: String,
    pub tray: Vec<TrayItem>,
    pub notifications: Vec<DesktopNotification>,
    pub attention: u32,
    pub terminal_launches: Vec<TerminalLaunch>,
    pub zoom_level: i32,
    pub bounds: Option<WindowBounds>,
    pub last_drop_script: Option<String>,
    pub pending_lifecycle: Option<String>,
    pub last_update: Option<UpdateOutcome>,
    pub last_download: Option<PathBuf>,
    pub last_diagnostics: Option<PathBuf>,
    pub last_plan: Option<WindowApplyPlan>,
    pub theme: Option<String>,
    pub locale: Option<String>,
    pub payload: Option<ShellPayload>,
    generation: u64,
    current_generation: Option<String>,
}

impl NativeSession {
    pub fn new(user_data: PathBuf, platform: impl Into<String>) -> Self {
        let bounds = load_window_bounds(&user_data);
        Self {
            user_data,
            platform: platform.into(),
            tray: Vec::new(),
            notifications: Vec::new(),
            attention: 0,
            terminal_launches: Vec::new(),
            zoom_level: 0,
            bounds,
            last_drop_script: None,
            pending_lifecycle: None,
            last_update: None,
            last_download: None,
            last_diagnostics: None,
            last_plan: None,
            theme: None,
            locale: None,
            payload: None,
            generation: 0,
            current_generation: None,
        }
    }

    pub fn listed_tray_ids(&self) -> Vec<String> {
        self.tray.iter().map(|item| item.id.clone()).collect()
    }

    pub fn dispatch(&mut self, method: &str, params: Value) -> Result<Value, String> {
        match method {
            "shell.schedule" => {
                let payload: ShellPayload =
                    serde_json::from_value(params).map_err(|error| error.to_string())?;
                self.payload = Some(payload);
                self.generation += 1;
                let id = format!("g{}", self.generation);
                self.current_generation = Some(id.clone());
                Ok(json!({ "generationId": id }))
            }
            "shell.mount" => {
                let payload = self
                    .payload
                    .clone()
                    .ok_or_else(|| "no scheduled generation".to_string())?;
                let plan = plan_generation(
                    &self.platform,
                    &payload.mode,
                    if payload.material == "transparent" || payload.material == "off" {
                        payload.material.as_str()
                    } else {
                        "transparent"
                    },
                    if matches!(payload.material.as_str(), "acrylic" | "mica" | "off") {
                        payload.material.as_str()
                    } else {
                        "acrylic"
                    },
                    None,
                    self.zoom_level,
                    self.bounds.clone(),
                );
                self.last_plan = Some(plan);
                Ok(json!({ "ok": true, "mode": self.last_plan.as_ref().map(|plan| plan.mode.clone()) }))
            }
            "shell.release" => {
                self.current_generation = None;
                self.payload = None;
                self.tray.clear();
                Ok(json!({ "ok": true }))
            }
            "shell.show" => Ok(json!({ "ok": true })),
            "shell.tray.upsert" => {
                let item = parse_tray_item(&params)?;
                if let Some(existing) = self.tray.iter_mut().find(|candidate| candidate.id == item.id) {
                    *existing = item;
                } else {
                    self.tray.push(item);
                }
                self.tray.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
                Ok(json!({
                    "ok": true,
                    "ids": self.listed_tray_ids(),
                }))
            }
            "shell.tray.remove" => {
                let id = params
                    .get("id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| "tray id required".to_string())?;
                self.tray.retain(|item| item.id != id);
                Ok(json!({ "ok": true, "ids": self.listed_tray_ids() }))
            }
            "shell.notify" => {
                let notification = DesktopNotification {
                    title: text_field(&params, "title")?,
                    body: text_field(&params, "body")?,
                };
                self.notifications.push(notification.clone());
                Ok(json!({ "ok": true, "queued": self.notifications.len() }))
            }
            "shell.notifyAttention" => {
                self.attention = self.attention.saturating_add(1);
                if params.get("title").and_then(Value::as_str).is_some() {
                    self.notifications.push(DesktopNotification {
                        title: text_field(&params, "title").unwrap_or_else(|_| "DSH Desktop".into()),
                        body: params
                            .get("body")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                    });
                }
                Ok(json!({ "ok": true, "attention": self.attention }))
            }
            "shell.openTerminal" => {
                let launch = record_terminal_launch(&self.user_data, &self.platform, &params)?;
                self.terminal_launches.push(launch.clone());
                Ok(json!({
                    "ok": true,
                    "command": launch.command,
                    "recordPath": launch.record_path,
                }))
            }
            "shell.exportDiagnostics" => {
                let path = export_diagnostics_archive(&self.user_data, "0.1.0")?;
                self.last_diagnostics = Some(path.clone());
                Ok(json!({ "ok": true, "path": path }))
            }
            "shell.confirmUpdate" | "shell.showUpdateResult" => {
                let outcome = parse_update_outcome(&params)?;
                self.last_update = Some(outcome.clone());
                Ok(json!({
                    "ok": true,
                    "status": outcome.status,
                    "currentVersion": outcome.current_version,
                    "latestVersion": outcome.latest_version,
                }))
            }
            "shell.downloadUpdate" => {
                let path = save_update_payload(&self.user_data, &params)?;
                self.last_download = Some(path.clone());
                Ok(json!({ "ok": true, "path": path }))
            }
            "shell.setTheme" => {
                self.theme = params.get("source").and_then(Value::as_str).map(str::to_string);
                Ok(json!({ "ok": true, "source": self.theme }))
            }
            "shell.setLocale" => {
                self.locale = params.get("locale").and_then(Value::as_str).map(str::to_string);
                Ok(json!({ "ok": true, "locale": self.locale }))
            }
            "shell.reportRendererBoot" => Ok(json!({ "ok": true, "status": params.get("status") })),
            "shell.prepareToQuit" => {
                self.pending_lifecycle = Some("quit".into());
                Ok(json!({ "ok": true }))
            }
            "shell.restart" => {
                self.pending_lifecycle = Some("restart".into());
                Ok(json!({ "ok": true, "lifecycle": "restart" }))
            }
            "shell.restartRecovery" => {
                self.pending_lifecycle = Some("recovery".into());
                Ok(json!({ "ok": true, "lifecycle": "recovery" }))
            }
            "shell.writeBootstrap" => {
                let profile = params
                    .get("profileName")
                    .and_then(Value::as_str)
                    .unwrap_or("desktop");
                let mode = params
                    .get("mode")
                    .and_then(Value::as_str)
                    .unwrap_or("compatibility");
                profile::assert_profile_name(profile)?;
                bootstrap::write_native_bootstrap(&self.user_data, profile, mode)
                    .map_err(|error| error.to_string())?;
                Ok(json!({ "ok": true }))
            }
            "shell.openRecovery" => {
                if let Some(action) = params.get("href").and_then(Value::as_str).and_then(parse_recovery_href)
                {
                    return self.apply_recovery(action);
                }
                Ok(json!({ "ok": true, "opened": true }))
            }
            "shell.openProfileCreate" => {
                if let Some(name) = params.get("name").and_then(Value::as_str) {
                    return persist_profile_selection(&self.user_data, name);
                }
                Ok(json!({ "ok": true, "opened": true }))
            }
            "shell.openDialog" => {
                if let Some(href) = params.get("href").and_then(Value::as_str) {
                    let count = params
                        .get("buttons")
                        .and_then(Value::as_array)
                        .map(Vec::len)
                        .unwrap_or(2);
                    let response = parse_dialog_response(href, count)
                        .ok_or_else(|| "invalid dialog href".to_string())?;
                    return Ok(json!({ "response": response }));
                }
                Ok(json!({ "response": params.get("cancelId").and_then(Value::as_u64).unwrap_or(0) }))
            }
            "shell.pickDirectory" => Ok(json!({ "path": params.get("path") })),
            "shell.validateDirectory" => {
                let path = params.get("path").and_then(Value::as_str).unwrap_or("");
                Ok(json!({ "ok": !path.is_empty() && Path::new(path).is_absolute() }))
            }
            "shell.reloadRenderer" | "shell.toggleDeveloperTools" => Ok(json!({ "ok": true })),
            "shell.openDevtools" => {
                let origin = params
                    .get("origin")
                    .and_then(Value::as_str)
                    .ok_or_else(|| "origin required".to_string())?
                    .trim_end_matches('/');
                let tab = params.get("tab").and_then(Value::as_str).unwrap_or("logs");
                if tab != "logs" && tab != "network" {
                    return Err("tab must be logs or network".into());
                }
                Ok(json!({
                    "ok": true,
                    "url": format!("{origin}/_dsh/dev/ui?tab={tab}"),
                }))
            }
            "shell.persistBounds" => {
                let bounds = WindowBounds {
                    x: params.get("x").and_then(Value::as_i64).unwrap_or(0) as i32,
                    y: params.get("y").and_then(Value::as_i64).unwrap_or(0) as i32,
                    width: params.get("width").and_then(Value::as_u64).unwrap_or(0) as u32,
                    height: params.get("height").and_then(Value::as_u64).unwrap_or(0) as u32,
                };
                persist_window_bounds(&self.user_data, &bounds)?;
                self.bounds = Some(bounds);
                Ok(json!({ "ok": true }))
            }
            "shell.dropPaths" => {
                let paths = params
                    .get("paths")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let x = params.get("x").and_then(Value::as_f64).unwrap_or(0.0);
                let y = params.get("y").and_then(Value::as_f64).unwrap_or(0.0);
                let script = folder_drop_script(&paths, x, y);
                self.last_drop_script = Some(script.clone());
                Ok(json!({ "ok": true, "script": script }))
            }
            "shell.openExternal" => {
                let page = params
                    .get("origin")
                    .and_then(Value::as_str)
                    .unwrap_or("http://127.0.0.1/");
                let href = params.get("href").and_then(Value::as_str).unwrap_or("");
                Ok(json!({ "external": should_open_externally(page, href) }))
            }
            other => Err(format!("unknown native method {other}")),
        }
    }

    fn apply_recovery(&mut self, action: RecoveryAction) -> Result<Value, String> {
        match apply_recovery_action(&self.user_data, &action)? {
            RecoveryEffect::Restart => {
                self.pending_lifecycle = Some("restart".into());
                Ok(json!({ "ok": true, "lifecycle": "restart" }))
            }
            RecoveryEffect::Quit => {
                self.pending_lifecycle = Some("quit".into());
                Ok(json!({ "ok": true, "lifecycle": "quit" }))
            }
            RecoveryEffect::Diagnostics(path) => {
                self.last_diagnostics = Some(path.clone());
                Ok(json!({ "ok": true, "path": path }))
            }
            RecoveryEffect::Profile(name) => persist_profile_selection(&self.user_data, &name),
            RecoveryEffect::Ignored => Ok(json!({ "ok": true, "ignored": true })),
        }
    }
}

fn parse_tray_item(params: &Value) -> Result<TrayItem, String> {
    Ok(TrayItem {
        id: text_field(params, "id")?,
        group: params
            .get("group")
            .and_then(Value::as_str)
            .unwrap_or("tools")
            .to_string(),
        order: params.get("order").and_then(Value::as_i64).unwrap_or(0),
        label: text_field(params, "label")?,
        enabled: params.get("enabled").and_then(Value::as_bool).unwrap_or(true),
    })
}

fn text_field(params: &Value, key: &str) -> Result<String, String> {
    params
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("{key} required"))
}

pub fn terminal_command(platform: &str, script_path: &Path) -> Vec<String> {
    match platform {
        "darwin" => vec![
            "open".into(),
            "-a".into(),
            "Terminal".into(),
            script_path.display().to_string(),
        ],
        "win32" => vec![
            "cmd".into(),
            "/c".into(),
            "start".into(),
            "".into(),
            script_path.display().to_string(),
        ],
        _ => vec!["x-terminal-emulator".into(), "-e".into(), script_path.display().to_string()],
    }
}

pub fn record_terminal_launch(
    user_data: &Path,
    platform: &str,
    params: &Value,
) -> Result<TerminalLaunch, String> {
    let profile_name = text_field(params, "profileName")?;
    let profile_dir = params
        .get("profileDir")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let home_dir = params
        .get("homeDir")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let dir = user_data.join("terminal-launches");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let script_path = dir.join(if platform == "win32" {
        format!("{stamp}.cmd")
    } else {
        format!("{stamp}.sh")
    });
    let script = if platform == "win32" {
        format!("@echo off\r\ncd /d {profile_dir}\r\nset DSH_HOME={home_dir}\r\n")
    } else {
        format!("#!/bin/sh\ncd {profile_dir:?}\nexport DSH_HOME={home_dir:?}\nexec \"$SHELL\"\n")
    };
    fs::write(&script_path, script).map_err(|error| error.to_string())?;
    let command = terminal_command(platform, &script_path);
    let record_path = dir.join(format!("{stamp}.json"));
    let record = json!({
        "profileName": profile_name,
        "profileDir": profile_dir,
        "homeDir": home_dir,
        "command": command,
        "scriptPath": script_path,
    });
    fs::write(
        &record_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&record).map_err(|error| error.to_string())?
        ),
    )
    .map_err(|error| error.to_string())?;
    Ok(TerminalLaunch {
        profile_name,
        command,
        record_path,
    })
}

pub fn export_diagnostics_archive(user_data: &Path, app_version: &str) -> Result<PathBuf, String> {
    let mut files = vec![(
        "system-info.txt".to_string(),
        format!(
            "product: DSH Desktop\nversion: {app_version}\nplatform: {}\n",
            std::env::consts::OS
        )
        .into_bytes(),
    )];
    let logs = user_data.join("logs");
    if let Ok(entries) = fs::read_dir(&logs) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("log") {
                if let Ok(bytes) = fs::read(&path) {
                    files.push((
                        format!("logs/{}", path.file_name().unwrap().to_string_lossy()),
                        bytes,
                    ));
                }
            }
        }
    }
    let marker = user_data.join("crash-evidence").join("active-run.json");
    if let Ok(bytes) = fs::read(&marker) {
        files.push(("crash-evidence/active-run.json".into(), bytes));
    }
    let zip = build_zip_store(&files);
    let dir = user_data.join("diagnostics");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let path = dir.join(format!("dsh-diagnostics-{stamp}.zip"));
    fs::write(&path, zip).map_err(|error| error.to_string())?;
    Ok(path)
}

pub fn save_update_payload(user_data: &Path, params: &Value) -> Result<PathBuf, String> {
    let version = text_field(params, "version")?;
    let destination = params
        .get("destinationPath")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            user_data
                .join("updates")
                .join(format!("DSH-Desktop-{version}.bin"))
        });
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let body = if let Some(text) = params.get("body").and_then(Value::as_str) {
        text.as_bytes().to_vec()
    } else if let Some(bytes) = params.get("bytes").and_then(Value::as_array) {
        bytes
            .iter()
            .filter_map(Value::as_u64)
            .map(|value| value as u8)
            .collect()
    } else {
        return Err("empty-body".into());
    };
    if body.is_empty() {
        return Err("empty-body".into());
    }
    if body.len() > 1024 * 1024 * 1024 {
        return Err("response-too-large".into());
    }
    fs::write(&destination, &body).map_err(|error| error.to_string())?;
    Ok(destination)
}

fn parse_update_outcome(params: &Value) -> Result<UpdateOutcome, String> {
    let status = params
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("up-to-date")
        .to_string();
    if status != "up-to-date" && status != "update-available" {
        return Err("invalid update status".into());
    }
    Ok(UpdateOutcome {
        status,
        current_version: text_field(params, "currentVersion").or_else(|_| text_field(params, "current_version"))?,
        latest_version: text_field(params, "latestVersion").or_else(|_| text_field(params, "latest_version"))?,
    })
}

pub(crate) enum RecoveryEffect {
    Restart,
    Quit,
    Diagnostics(PathBuf),
    Profile(String),
    Ignored,
}

pub fn apply_recovery_action(user_data: &Path, action: &RecoveryAction) -> Result<RecoveryEffect, String> {
    match action.action.as_str() {
        "restart" => Ok(RecoveryEffect::Restart),
        "quit" => Ok(RecoveryEffect::Quit),
        "save-diagnostics" => Ok(RecoveryEffect::Diagnostics(export_diagnostics_archive(
            user_data, "0.1.0",
        )?)),
        "add-profile" | "switch-profile" => {
            let name = action
                .name
                .clone()
                .ok_or_else(|| "profile name required".to_string())?;
            write_profile_state(
                &user_data.join("profile-selection").join("state.json"),
                &name,
            )?;
            let mode = bootstrap::load_native_bootstrap(user_data).mode;
            bootstrap::write_native_bootstrap(user_data, &name, &mode)
                .map_err(|error| error.to_string())?;
            Ok(RecoveryEffect::Profile(name))
        }
        _ => Ok(RecoveryEffect::Ignored),
    }
}

fn persist_profile_selection(user_data: &Path, name: &str) -> Result<Value, String> {
    write_profile_state(
        &user_data.join("profile-selection").join("state.json"),
        name,
    )?;
    let mode = bootstrap::load_native_bootstrap(user_data).mode;
    bootstrap::write_native_bootstrap(user_data, name, &mode).map_err(|error| error.to_string())?;
    let NativeBootstrap { profile_name, .. } = bootstrap::load_native_bootstrap(user_data);
    Ok(json!({ "ok": true, "profileName": profile_name }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn session() -> NativeSession {
        let dir = std::env::temp_dir().join(format!(
            "dsh-native-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        NativeSession::new(dir, "darwin")
    }

    #[test]
    fn stdio_request_line_upserts_tray() {
        let line = r#"{"v":1,"type":"req","id":"1","method":"shell.tray.upsert","params":{"id":"tray-1","group":"tools","order":10,"label":"Open Terminal","enabled":true}}"#;
        let decoded = crate::protocol::decode_message(line).unwrap();
        let crate::protocol::RpcMessage::Request { method, params, .. } = decoded else {
            panic!("production tray upsert must be a request, not an event");
        };
        let mut native = session();
        native.dispatch(&method, params.unwrap_or(json!({}))).unwrap();
        assert_eq!(native.listed_tray_ids(), vec!["tray-1".to_string()]);
    }

    #[test]
    fn open_devtools_returns_inspector_url() {
        let mut native = session();
        let result = native
            .dispatch(
                "shell.openDevtools",
                json!({ "origin": "http://127.0.0.1:9/", "tab": "network" }),
            )
            .unwrap();
        assert_eq!(result["url"], "http://127.0.0.1:9/_dsh/dev/ui?tab=network");
    }

    #[test]
    fn tray_upsert_is_retained_and_listed() {
        let mut native = session();
        let result = native
            .dispatch(
                "shell.tray.upsert",
                json!({ "id": "tray-1", "group": "tools", "order": 2, "label": "Open Terminal", "enabled": true }),
            )
            .unwrap();
        assert_eq!(result["ids"][0], "tray-1");
        assert_eq!(native.listed_tray_ids(), vec!["tray-1".to_string()]);
        native
            .dispatch("shell.tray.remove", json!({ "id": "tray-1" }))
            .unwrap();
        assert!(native.listed_tray_ids().is_empty());
    }

    #[test]
    fn diagnostics_write_produces_zip() {
        let mut native = session();
        fs::create_dir_all(native.user_data.join("logs")).unwrap();
        fs::write(native.user_data.join("logs").join("dsh.log"), "boot ok").unwrap();
        let result = native.dispatch("shell.exportDiagnostics", json!({})).unwrap();
        let path = PathBuf::from(result["path"].as_str().unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_eq!(&bytes[0..4], b"PK\x03\x04");
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("system-info.txt"));
        assert!(text.contains("dsh.log"));
        assert!(text.contains("boot ok"));
    }

    #[test]
    fn terminal_launch_is_recorded() {
        let mut native = session();
        let result = native
            .dispatch(
                "shell.openTerminal",
                json!({
                    "profileName": "desktop",
                    "profileDir": "/tmp/profile",
                    "homeDir": "/tmp/home"
                }),
            )
            .unwrap();
        assert_eq!(result["command"][0], "open");
        let record = fs::read_to_string(result["recordPath"].as_str().unwrap()).unwrap();
        assert!(record.contains("\"profileName\": \"desktop\""));
    }

    #[test]
    fn notification_and_attention_are_queued() {
        let mut native = session();
        native
            .dispatch("shell.notify", json!({ "title": "Ready", "body": "Host mounted" }))
            .unwrap();
        native
            .dispatch("shell.notifyAttention", json!({ "title": "Job", "body": "done" }))
            .unwrap();
        assert_eq!(native.notifications.len(), 2);
        assert_eq!(native.attention, 1);
    }

    #[test]
    fn update_download_writes_payload() {
        let mut native = session();
        native
            .dispatch(
                "shell.showUpdateResult",
                json!({
                    "status": "update-available",
                    "currentVersion": "0.1.0",
                    "latestVersion": "0.2.0"
                }),
            )
            .unwrap();
        let result = native
            .dispatch(
                "shell.downloadUpdate",
                json!({ "version": "0.2.0", "body": "installer-bytes" }),
            )
            .unwrap();
        let path = PathBuf::from(result["path"].as_str().unwrap());
        assert_eq!(fs::read_to_string(path).unwrap(), "installer-bytes");
        assert_eq!(native.last_update.as_ref().unwrap().status, "update-available");
    }

    #[test]
    fn recovery_profile_action_persists_state() {
        let mut native = session();
        let result = native
            .dispatch(
                "shell.openRecovery",
                json!({ "href": "dsh-recovery://switch-profile?name=web" }),
            )
            .unwrap();
        assert_eq!(result["profileName"], "web");
        let state = fs::read_to_string(native.user_data.join("profile-selection").join("state.json")).unwrap();
        assert!(state.contains("\"active\": \"web\""));
        let bootstrap = fs::read_to_string(native.user_data.join("native-bootstrap.json")).unwrap();
        assert!(bootstrap.contains("\"profileName\": \"web\""));
    }

    #[test]
    fn dialog_href_returns_bounded_id() {
        let mut native = session();
        let result = native
            .dispatch(
                "shell.openDialog",
                json!({
                    "href": "dsh-desktop-dialog://response?id=1",
                    "buttons": ["Keep", "Replace"]
                }),
            )
            .unwrap();
        assert_eq!(result["response"], 1);
        let rejected = native.dispatch(
            "shell.openDialog",
            json!({
                "href": "dsh-desktop-dialog://response?id=1&command=bad",
                "buttons": ["Keep", "Replace"]
            }),
        );
        assert!(rejected.is_err());
    }

    #[test]
    fn drop_paths_and_external_links_via_dispatch() {
        let mut native = session();
        let drop = native
            .dispatch(
                "shell.dropPaths",
                json!({ "paths": ["/Users/me/proj"], "x": 12.0, "y": 40.0 }),
            )
            .unwrap();
        let script = drop["script"].as_str().unwrap();
        assert!(script.contains("dsh-desktop-folder-drop"));
        assert!(script.contains("/Users/me/proj"));
        assert_eq!(native.last_drop_script.as_deref(), Some(script));
        let https = native
            .dispatch(
                "shell.openExternal",
                json!({ "origin": "http://127.0.0.1:9/", "href": "https://example.com/docs" }),
            )
            .unwrap();
        assert_eq!(https["external"], true);
        let loopback = native
            .dispatch(
                "shell.openExternal",
                json!({ "origin": "http://127.0.0.1:9/", "href": "http://127.0.0.1:9/app" }),
            )
            .unwrap();
        assert_eq!(loopback["external"], false);
        let mailto = native
            .dispatch(
                "shell.openExternal",
                json!({ "origin": "http://127.0.0.1:9/", "href": "mailto:ops@example.com" }),
            )
            .unwrap();
        assert_eq!(mailto["external"], true);
    }

    #[test]
    fn mount_records_chrome_plan() {
        let mut native = session();
        native
            .dispatch(
                "shell.schedule",
                json!({
                    "mode": "advanced",
                    "material": "transparent",
                    "width": 1280,
                    "height": 800,
                    "minWidth": 900,
                    "minHeight": 600,
                    "url": "http://127.0.0.1:9/",
                    "productName": "DSH Desktop",
                    "windowTitle": "DSH Desktop",
                    "themeSource": "system"
                }),
            )
            .unwrap();
        native.dispatch("shell.mount", json!({ "generationId": "g1" })).unwrap();
        let plan = native.last_plan.unwrap();
        assert_eq!(plan.mode, "advanced");
        assert_eq!(plan.chrome.titlebar_height, 32);
        assert!(plan.intercept_external_links);
    }
}
