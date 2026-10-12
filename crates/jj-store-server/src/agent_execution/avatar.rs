//! Community avatars are presentation data, never fetched or interpreted by the server.
use super::*;
pub(super) use crate::presentation_avatar::valid;
const TOTAL_BYTES: usize = 8 * 1024 * 1024;
pub(super) fn budget(state: &State) -> Result<()> {
    reserve(state, 0)
}
pub(super) fn reserve(state: &State, additional: usize) -> Result<()> {
    let bytes: usize = state
        .setups
        .values()
        .map(|s| s.avatar.len())
        .chain(state.sessions.values().map(|s| s.view.avatar.len()))
        .sum();
    if bytes.saturating_add(additional) > TOTAL_BYTES {
        Err(Error::Limit)
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    const IMAGE_BYTES: usize = 1024 * 1024;
    use super::*;
    #[test]
    fn community_avatar_forms_are_bounded_and_inert() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 256 256\"><rect width=\"256\" height=\"256\" rx=\"64\" fill=\"#6e56cf\"/><text x=\"128\" y=\"166\" font-size=\"128\" text-anchor=\"middle\">🤖</text></svg>";
        let uri = |xml: &str| {
            format!(
                "data:image/svg+xml,{}",
                percent_encoding::utf8_percent_encode(xml, percent_encoding::NON_ALPHANUMERIC)
            )
        };
        for avatar in [
            "".to_string(),
            "🤖".into(),
            "https://example.com/avatar.png".into(),
            uri(svg),
            "data:image/png;base64,iVBORw0KGgo=".into(),
        ] {
            assert!(valid(&avatar), "{avatar}");
        }
        for avatar in [
            "https://user:secret@example.com/image",
            "http://example.com/image",
            "javascript:alert(1)",
            "data:text/html,hello",
            "data:image/png;base64,aGVsbG8=",
            "data:image/svg+xml,<svg onload='alert(1)'/>",
        ] {
            assert!(!valid(avatar), "{avatar}");
        }
        assert!(!valid(
            &uri(&svg.replace("🤖", "<script>alert(1)</script>"))
        ));
        let oversized = format!(
            "data:image/png;base64,{}",
            STANDARD.encode(vec![0; IMAGE_BYTES + 1])
        );
        assert!(!valid(&oversized));
        let legacy: Setup = serde_json::from_value(json!({"id":"a","name":"A","avatar":"🤖","instructions":"","runner":"codex","preferences":{"model":null,"effort":null,"mode":null}})).unwrap();
        assert_eq!(legacy.description, "");
        let root = tempfile::tempdir().unwrap();
        let mut state = State::default();
        journal::persist(root.path(), &state).unwrap();
        let before = fs::read(root.path().join("state.json")).unwrap();
        state.setups.insert(
            "a".into(),
            Setup {
                avatar: "x".repeat(TOTAL_BYTES + 1),
                ..legacy
            },
        );
        assert!(matches!(
            journal::persist(root.path(), &state),
            Err(Error::Limit)
        ));
        assert_eq!(fs::read(root.path().join("state.json")).unwrap(), before);
    }
}
