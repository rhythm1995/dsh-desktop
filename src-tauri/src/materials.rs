pub fn effective_material(platform: &str, mode: &str, macos: &str, windows: &str, build: Option<u32>) -> &'static str {
    let _ = mode;
    if platform == "linux" {
        return "off";
    }
    if platform == "darwin" {
        return if macos == "off" { "off" } else { "transparent" };
    }
    let n = build.unwrap_or(0);
    let acrylic = n >= 17_763;
    let rounded = n >= 22_000;
    let system_backdrop = n >= 22_621;
    let legacy_acrylic = acrylic && !rounded;
    match windows {
        "mica" if system_backdrop => "mica",
        "mica" if legacy_acrylic => "acrylic",
        "acrylic" if system_backdrop || legacy_acrylic => "acrylic",
        _ => "off",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gates_windows_backdrops() {
        assert_eq!(
            effective_material("win32", "compatibility", "transparent", "mica", Some(22_631)),
            "mica"
        );
        assert_eq!(
            effective_material("win32", "extended", "transparent", "mica", Some(19_045)),
            "acrylic"
        );
        assert_eq!(
            effective_material("win32", "extended", "transparent", "mica", Some(22_000)),
            "off"
        );
        assert_eq!(
            effective_material("win32", "extended", "transparent", "acrylic", Some(22_000)),
            "off"
        );
        assert_eq!(
            effective_material("linux", "compatibility", "transparent", "acrylic", None),
            "off"
        );
    }
}
