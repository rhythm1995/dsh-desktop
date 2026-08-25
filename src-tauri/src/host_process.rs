use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};

#[allow(dead_code)]
pub fn host_entry_path() -> PathBuf {
    host_entry_path_from(None)
}

pub fn host_entry_path_from(resource_dir: Option<&Path>) -> PathBuf {
    if let Ok(path) = std::env::var("DSH_HOST_ENTRY") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }
    let mut candidates = Vec::new();
    if let Some(dir) = resource_dir {
        candidates.push(dir.join("host/dist/main.js"));
        candidates.push(dir.join("dist/main.js"));
        candidates.push(dir.join("main.js"));
    }
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../host/dist/main.js"));
    candidates
        .into_iter()
        .find(|path| path.exists())
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../host/dist/main.js"))
}

pub fn node_binary() -> PathBuf {
    if let Ok(path) = std::env::var("DSH_NODE_BINARY") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }
    let mut search = std::env::var("PATH").unwrap_or_default();
    for extra in ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"] {
        if !search.split(':').any(|item| item == extra) {
            search = format!("{extra}:{search}");
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let nvm = PathBuf::from(&home).join(".nvm/versions/node");
        if let Ok(entries) = std::fs::read_dir(&nvm) {
            let mut versions: Vec<PathBuf> = entries.flatten().map(|entry| entry.path().join("bin/node")).collect();
            versions.sort();
            if let Some(node) = versions.into_iter().rev().find(|path| path.exists()) {
                return node;
            }
        }
        let fnm = PathBuf::from(&home).join("Library/Application Support/fnm/node-versions");
        if let Ok(entries) = std::fs::read_dir(fnm) {
            let mut versions: Vec<PathBuf> = entries
                .flatten()
                .map(|entry| entry.path().join("installation/bin/node"))
                .collect();
            versions.sort();
            if let Some(node) = versions.into_iter().rev().find(|path| path.exists()) {
                return node;
            }
        }
    }
    for candidate in ["/opt/homebrew/bin/node", "/usr/local/bin/node", "/usr/bin/node"] {
        if Path::new(candidate).exists() {
            return PathBuf::from(candidate);
        }
    }
    PathBuf::from("node")
}

pub fn dsh_plugin_desktop_root(resource_dir: Option<&Path>) -> PathBuf {
    if let Ok(path) = std::env::var("DSH_PLUGIN_DESKTOP_ROOT") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }
    let mut candidates = Vec::new();
    if let Some(dir) = resource_dir {
        candidates.push(dir.join("dsh-plugin-desktop"));
    }
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../anywhere-labs-dsh-desktop/dsh-plugin-desktop"));
    candidates
        .into_iter()
        .find(|path| path.join("lib/tauri-host.js").exists())
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../anywhere-labs-dsh-desktop/dsh-plugin-desktop")
        })
}

pub fn spawn_host(
    entry: &Path,
    user_data: &Path,
    recovery: bool,
    resource_dir: Option<&Path>,
) -> std::io::Result<Child> {
    std::fs::create_dir_all(user_data)?;
    let mut command = Command::new(node_binary());
    let mut path = std::env::var("PATH").unwrap_or_default();
    for extra in ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"] {
        if !path.split(':').any(|item| item == extra) {
            path = format!("{extra}:{path}");
        }
    }
    let plugin_root = dsh_plugin_desktop_root(resource_dir);
    command
        .arg(entry)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .env("DSH_DESKTOP_USER_DATA", user_data)
        .env("PATH", path)
        .env("DSH_DEV_TOOLS", if cfg!(dsh_devtools) { "1" } else { "0" })
        .env("DSH_PLUGIN_DESKTOP_ROOT", &plugin_root)
        .env("DSH_OFFICIAL_HOST", "1")
        .current_dir(user_data);
    if recovery {
        command.arg("--recovery");
    }
    command.spawn()
}

pub fn take_stdin(child: &mut Child) -> Option<ChildStdin> {
    child.stdin.take()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_entry_points_at_compiled_host() {
        let path = host_entry_path();
        assert!(path.ends_with("host/dist/main.js") || path.ends_with("host\\dist\\main.js"));
    }

    #[test]
    fn node_binary_is_resolvable() {
        let path = node_binary();
        assert!(!path.as_os_str().is_empty());
    }

    #[test]
    fn main_window_config_defers_creation_to_setup() {
        let conf: serde_json::Value =
            serde_json::from_str(include_str!("../tauri.conf.json")).expect("tauri.conf.json");
        let main = conf["app"]["windows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|window| window["label"] == "main")
            .expect("main window");
        assert_eq!(main["create"], false);
        let lib = include_str!("lib.rs");
        assert!(lib.contains("create_main_window(app)?"));
        assert!(lib.contains("WebviewWindowBuilder::from_config"));
        assert!(!lib.contains("if let Some(existing) = app.get_webview_window(\"main\")"));
        let create = lib
            .find("create_main_window(app)?")
            .expect("setup must create the main window");
        let spawn = lib
            .find("spawn_host_bridge(handle.clone());")
            .expect("setup must spawn the Host bridge");
        assert!(
            create < spawn,
            "create_main_window must run before spawn_host_bridge so shell.mount can find main"
        );
        let mount_arm = lib
            .split("\"shell.mount\" =>")
            .nth(1)
            .expect("shell.mount arm")
            .split("\"shell.")
            .next()
            .expect("shell.mount arm end");
        assert!(
            mount_arm.contains("run_on_main_thread"),
            "shell.mount must hop to the UI thread before navigating the hidden splash"
        );
        assert!(mount_arm.contains("mount_main_window"));
    }

    #[test]
    fn spawn_host_creates_missing_user_data_dir() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let dir = std::env::temp_dir().join(format!(
            "dsh-host-ud-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        assert!(!dir.exists());
        let entry = host_entry_path();
        assert!(entry.exists(), "{}", entry.display());
        let mut child = spawn_host(&entry, &dir, false, None).expect("spawn host");
        let created = dir.is_dir();
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(created);
    }
}
