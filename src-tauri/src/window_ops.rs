use crate::materials::effective_material;
use crate::window_spec::{chrome_for_mode, linux_forces_compatibility, WindowChrome};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowApplyPlan {
    pub mode: String,
    pub chrome: WindowChrome,
    pub material: String,
    pub zoom_level: i32,
    pub intercept_external_links: bool,
    pub bounds: Option<WindowBounds>,
}

#[derive(Serialize, Deserialize)]
struct BoundsDocument {
    version: u8,
    bounds: WindowBounds,
}

pub fn clamp_zoom(level: i32) -> i32 {
    level.clamp(-4, 4)
}

pub fn apply_zoom_delta(current: i32, delta: i32) -> i32 {
    clamp_zoom(current + delta)
}

pub fn origin_of(href: &str) -> Option<String> {
    let scheme_end = href.find("://")?;
    let scheme = &href[..scheme_end];
    if scheme != "http" && scheme != "https" {
        return None;
    }
    let rest = &href[scheme_end + 3..];
    let hostport = rest.split('/').next().unwrap_or(rest);
    if hostport.is_empty() {
        return None;
    }
    Some(format!("{scheme}://{hostport}"))
}

pub fn is_loopback_href(href: &str) -> bool {
    href.starts_with("http://127.0.0.1")
        || href.starts_with("http://[::1]")
        || href.starts_with("http://localhost")
}

pub fn should_open_externally(page_origin: &str, href: &str) -> bool {
    if href.starts_with("mailto:") {
        return true;
    }
    if is_loopback_href(href) {
        return false;
    }
    let Some(target) = origin_of(href) else {
        return false;
    };
    let Some(page) = origin_of(page_origin) else {
        return true;
    };
    target != page
}

pub fn open_external_href(
    page_origin: &str,
    href: &str,
    opener: impl FnOnce(&str) -> Result<(), String>,
) -> Result<bool, String> {
    if !should_open_externally(page_origin, href) {
        return Ok(false);
    }
    opener(href)?;
    Ok(true)
}

pub fn folder_drop_script(paths: &[String], x: f64, y: f64) -> String {
    let detail = serde_json::json!({ "paths": paths, "x": x, "y": y });
    format!("window.dispatchEvent(new CustomEvent('dsh-desktop-folder-drop', {{ detail: {detail} }}));")
}

pub fn persist_window_bounds(user_data: &Path, bounds: &WindowBounds) -> Result<(), String> {
    fs::create_dir_all(user_data).map_err(|error| error.to_string())?;
    let document = BoundsDocument {
        version: 1,
        bounds: bounds.clone(),
    };
    fs::write(
        user_data.join("main-window-state.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&document).map_err(|error| error.to_string())?
        ),
    )
    .map_err(|error| error.to_string())
}

pub fn load_window_bounds(user_data: &Path) -> Option<WindowBounds> {
    let text = fs::read_to_string(user_data.join("main-window-state.json")).ok()?;
    let document: BoundsDocument = serde_json::from_str(&text).ok()?;
    if document.version != 1 || document.bounds.width == 0 || document.bounds.height == 0 {
        return None;
    }
    Some(document.bounds)
}

pub fn plan_generation(
    platform: &str,
    requested_mode: &str,
    macos_material: &str,
    windows_material: &str,
    windows_build: Option<u32>,
    zoom: i32,
    bounds: Option<WindowBounds>,
) -> WindowApplyPlan {
    let mode = if linux_forces_compatibility(platform, requested_mode) {
        "compatibility"
    } else {
        requested_mode
    };
    WindowApplyPlan {
        mode: mode.to_string(),
        chrome: chrome_for_mode(mode),
        material: effective_material(platform, mode, macos_material, windows_material, windows_build)
            .to_string(),
        zoom_level: apply_zoom_delta(0, zoom),
        intercept_external_links: true,
        bounds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn linux_advanced_falls_back_to_compatibility_off() {
        let plan = plan_generation("linux", "advanced", "transparent", "mica", None, 0, None);
        assert_eq!(plan.mode, "compatibility");
        assert_eq!(plan.chrome.titlebar_height, 36);
        assert_eq!(plan.material, "off");
        assert!(plan.intercept_external_links);
    }

    #[test]
    fn advanced_macos_uses_compact_caption() {
        let plan = plan_generation("darwin", "advanced", "transparent", "acrylic", None, 9, None);
        assert_eq!(plan.chrome.titlebar_height, 32);
        assert_eq!(plan.chrome.macos_traffic_light_top, 16);
        assert_eq!(plan.material, "transparent");
        assert_eq!(plan.zoom_level, 4);
        assert_eq!(apply_zoom_delta(3, 2), 4);
        assert_eq!(apply_zoom_delta(-3, -2), -4);
    }

    #[test]
    fn drop_script_forwards_native_path() {
        let script = folder_drop_script(&["/Users/me/proj".into()], 12.0, 40.0);
        assert!(script.contains("dsh-desktop-folder-drop"));
        assert!(script.contains("/Users/me/proj"));
        assert!(script.contains("12"));
    }

    #[test]
    fn external_links_leave_loopback_origin() {
        assert!(should_open_externally(
            "http://127.0.0.1:9/",
            "https://example.com/docs"
        ));
        assert!(!should_open_externally(
            "http://127.0.0.1:9/",
            "http://127.0.0.1:9/app"
        ));
        assert!(should_open_externally("http://127.0.0.1:9/", "mailto:ops@example.com"));
        assert!(!should_open_externally("tauri://localhost/", "http://127.0.0.1:9/"));
        let mut opened = Vec::new();
        assert!(open_external_href(
            "http://127.0.0.1:9/",
            "https://example.com/docs",
            |url| {
                opened.push(url.to_string());
                Ok(())
            },
        )
        .unwrap());
        assert_eq!(opened, vec!["https://example.com/docs".to_string()]);
        assert!(!open_external_href(
            "http://127.0.0.1:9/",
            "http://127.0.0.1:9/app",
            |_| Err("must not open loopback".into()),
        )
        .unwrap());
    }

    #[test]
    fn window_bounds_round_trip() {
        let dir = std::env::temp_dir().join(format!(
            "dsh-bounds-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        persist_window_bounds(
            &dir,
            &WindowBounds {
                x: 12,
                y: 24,
                width: 1280,
                height: 800,
            },
        )
        .unwrap();
        let loaded = load_window_bounds(&dir).unwrap();
        assert_eq!(loaded.width, 1280);
        assert_eq!(loaded.x, 12);
    }
}
