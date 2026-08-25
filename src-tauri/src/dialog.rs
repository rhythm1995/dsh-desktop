pub fn parse_dialog_response(href: &str, button_count: usize) -> Option<u32> {
    let prefix = "dsh-desktop-dialog://response?";
    let query = href.strip_prefix(prefix)?;
    if query.contains('&') || query.contains('#') || query.contains('/') {
        return None;
    }
    let id = query.strip_prefix("id=")?;
    if id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if id.len() > 1 && id.starts_with('0') {
        return None;
    }
    let response = id.parse::<u32>().ok()?;
    ((response as usize) < button_count).then_some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_bounded_id() {
        assert_eq!(
            parse_dialog_response("dsh-desktop-dialog://response?id=1", 2),
            Some(1)
        );
        assert_eq!(
            parse_dialog_response("dsh-desktop-dialog://response?id=2", 2),
            None
        );
        assert_eq!(
            parse_dialog_response("dsh-desktop-dialog://response?id=1&command=bad", 2),
            None
        );
        assert_eq!(parse_dialog_response("https://response/?id=1", 2), None);
    }
}
