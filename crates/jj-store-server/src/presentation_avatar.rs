//! Bounded presentation-only avatar formats, without runtime authority.
use base64::{Engine as _, engine::general_purpose::STANDARD};
const IMAGE_BYTES: usize = 1024 * 1024;
pub(crate) fn valid(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    if value.starts_with("https://") {
        return value.len() <= 2048
            && url::Url::parse(value).is_ok_and(|url| {
                url.host_str().is_some() && url.username().is_empty() && url.password().is_none()
            })
            && !value.chars().any(char::is_control);
    }
    if let Some(svg) = value.strip_prefix("data:image/svg+xml,") {
        return svg.len() <= 4096
            && percent_encoding::percent_decode_str(svg)
                .decode_utf8()
                .is_ok_and(|svg| valid_emoji_svg(&svg));
    }
    if let Some(data) = value.strip_prefix("data:image/") {
        let Some((format, data)) = data.split_once(";base64,") else {
            return false;
        };
        if data.len() > IMAGE_BYTES.div_ceil(3) * 4 {
            return false;
        }
        let Ok(bytes) = STANDARD.decode(data) else {
            return false;
        };
        if bytes.len() > IMAGE_BYTES {
            return false;
        }
        return match format {
            "png" => bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
            "jpeg" => bytes.starts_with(b"\xff\xd8\xff"),
            "gif" => bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a"),
            "webp" => bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP"),
            _ => false,
        };
    }
    value.len() <= 128
        && !value.chars().any(char::is_control)
        && !value.contains([':', '/', '<', '>'])
}
fn valid_emoji_svg(svg: &str) -> bool {
    for radius in [64, 128] {
        for color in [
            "#f5d90a", "#ff8a4c", "#e5484d", "#ab4aba", "#6e56cf", "#0091ff", "#30a46c", "#687076",
        ] {
            let prefix = format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 256 256\"><rect width=\"256\" height=\"256\" rx=\"{radius}\" fill=\"{color}\"/><text x=\"128\" y=\"166\" font-size=\"128\" text-anchor=\"middle\">"
            );
            if let Some(text) = svg
                .strip_prefix(&prefix)
                .and_then(|s| s.strip_suffix("</text></svg>"))
            {
                let decoded = text
                    .replace("&amp;", "&")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">");
                let escaped = decoded
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                return !decoded.is_empty()
                    && decoded.len() <= 128
                    && !decoded.chars().any(char::is_control)
                    && escaped == text;
            }
        }
    }
    false
}
