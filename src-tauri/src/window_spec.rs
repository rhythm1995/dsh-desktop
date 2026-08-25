use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ShellPayload {
    pub mode: String,
    pub material: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "minWidth")]
    pub min_width: u32,
    #[serde(rename = "minHeight")]
    pub min_height: u32,
    pub url: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "windowTitle")]
    pub window_title: String,
    #[serde(rename = "themeSource")]
    pub theme_source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowChrome {
    pub titlebar_height: u32,
    pub macos_traffic_light_top: u32,
}

pub fn chrome_for_mode(mode: &str) -> WindowChrome {
    if mode == "advanced" {
        WindowChrome {
            titlebar_height: 32,
            macos_traffic_light_top: 16,
        }
    } else {
        WindowChrome {
            titlebar_height: 36,
            macos_traffic_light_top: 12,
        }
    }
}

pub fn linux_forces_compatibility(platform: &str, mode: &str) -> bool {
    platform == "linux" && mode != "compatibility"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatibility_uses_36px_frame() {
        assert_eq!(chrome_for_mode("compatibility").titlebar_height, 36);
        assert_eq!(chrome_for_mode("extended").titlebar_height, 36);
        assert_eq!(chrome_for_mode("advanced").titlebar_height, 32);
    }

    #[test]
    fn linux_rejects_advanced_modes() {
        assert!(linux_forces_compatibility("linux", "advanced"));
        assert!(!linux_forces_compatibility("darwin", "advanced"));
    }
}
