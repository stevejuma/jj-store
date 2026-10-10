use super::*;
use std::path::Path;

#[test]
fn fleet_privacy_isolated_by_participation_preserves_terminal_assistant_text() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let first = TranscriptSanitizer::new(
        Path::new("/Users/synthetic/a"),
        Path::new("/Users/synthetic"),
        Path::new("/tmp/synthetic"),
        &cache,
    )
    .unwrap();
    let second = first
        .for_workspace(Path::new("/Users/synthetic/b"))
        .unwrap();
    let mut stream = Stream::default();
    let chunk = |text| json!({"type":"activity","requestId":"request","update":{"sessionUpdate":"agent_message_chunk","messageId":"message","content":{"type":"text","text":text}}});
    for (id, sanitizer, text) in [
        ("a", &first, "/Users/syn"),
        ("b", &second, "/Users/"),
        ("a", &first, "thetic/a/LICENSE"),
        ("b", &second, "synthetic/b/README.md"),
    ] {
        for mut frame in stream.frames(id, sanitizer, chunk(text)).unwrap() {
            stream.observe(id, "request", &mut frame).unwrap();
        }
    }
    for (id, sanitizer, expected) in [
        ("a", &first, "$WORKSPACE/LICENSE"),
        ("b", &second, "$WORKSPACE/README.md"),
    ] {
        let frames = stream
            .frames(
                id,
                sanitizer,
                json!({"type":"record","requestId":"request"}),
            )
            .unwrap();
        let mut terminal = Value::Null;
        for mut frame in frames {
            stream.observe(id, "request", &mut frame).unwrap();
            terminal = frame;
        }
        assert_eq!(terminal["assistantText"], expected);
    }
    assert!(stream.privacy.is_empty());
    assert!(stream.assistant.is_empty());
}
