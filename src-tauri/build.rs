fn main() {
    println!("cargo:rustc-check-cfg=cfg(dsh_devtools)");
    let explicit = std::env::var("DSH_DEV_TOOLS").ok();
    let profile = std::env::var("DSH_PROFILE").ok();
    let github = std::env::var("GITHUB_ACTIONS").ok();
    let enabled = match explicit.as_deref() {
        Some("0" | "false" | "off") => false,
        Some("1" | "true" | "on") => true,
        _ if matches!(profile.as_deref(), Some("production" | "release" | "prod")) => false,
        _ if github.as_deref() == Some("true") => false,
        _ => true,
    };
    if enabled {
        println!("cargo:rustc-cfg=dsh_devtools");
    }
    println!("cargo:rerun-if-env-changed=DSH_DEV_TOOLS");
    println!("cargo:rerun-if-env-changed=DSH_PROFILE");
    println!("cargo:rerun-if-env-changed=GITHUB_ACTIONS");
    tauri_build::build();
}
