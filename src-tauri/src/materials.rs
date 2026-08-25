pub fn effective_material(platform: &str, mode: &str, macos: &str, windows: &str, build: Option<u32>) -> &'static str {
    let _ = mode;
    if platform == "linux" {
        return "off";
    }
    if platform == "darwin" {
        return if macos == "off" { "off" } else { "transparent" };
    }
    match windows {
        "mica" if build.unwrap_or(0) >= 22_621 => "mica",
        "mica" if build.unwrap_or(0) >= 17_763 => "acrylic",
        "acrylic" if build.unwrap_or(0) >= 17_763 => "acrylic",
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
            effective_material("linux", "compatibility", "transparent", "acrylic", None),
            "off"
        );
    }
}
