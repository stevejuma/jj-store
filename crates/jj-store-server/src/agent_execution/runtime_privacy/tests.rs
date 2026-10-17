use super::*;
use std::path::Path;

#[test]
fn streamed_privacy_precedes_journal_and_terminal_frames() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/Users/synthetic/workspace"),
        Path::new("/Users/synthetic"),
        Path::new("/tmp/synthetic"),
        &cache,
    )
    .unwrap();
    let mut privacy = StreamPrivacy::default();
    let mut frames = Vec::new();
    for text in [
        "See [LICENSE](<",
        "/",
        "Users",
        "/synthetic",
        "/workspace",
        "/LICENSE:3>) trailing /",
    ] {
        frames.extend(privacy.frames(&sanitizer, json!({
            "type":"activity", "requestId":"request", "sequence":1,
            "update":{"sessionUpdate":"agent_message_chunk","messageId":"answer","content":{"type":"text","text":text}}
        })).unwrap());
    }
    let terminal = privacy
        .frames(&sanitizer, json!({"type":"record","requestId":"request"}))
        .unwrap();
    assert_eq!(terminal.len(), 2);
    assert_eq!(terminal[0]["update"]["messageId"], "answer");
    assert!(terminal[0].get("sequence").is_none());
    assert_eq!(terminal[1]["type"], "record");
    frames.extend(terminal);
    let text: String = frames
        .iter()
        .filter_map(|v| v["update"]["content"]["text"].as_str())
        .collect();
    assert_eq!(text, "See [LICENSE](<$WORKSPACE/LICENSE:3>) trailing /");
    assert!(
        !serde_json::to_string(&frames)
            .unwrap()
            .contains("/Users/synthetic")
    );
    assert!(!privacy.has_pending_text());
}

#[test]
fn message_and_tool_boundaries_flush_without_joining_distinct_answers() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace"),
        Path::new("/home/synthetic"),
        Path::new("/tmp/synthetic"),
        &cache,
    )
    .unwrap();
    let mut privacy = StreamPrivacy::default();
    let frame = |message, text| json!({"type":"activity","requestId":"request","update":{"sessionUpdate":"agent_message_chunk","messageId":message,"content":{"type":"text","text":text}}});
    privacy.frames(&sanitizer, frame("a", "A /work")).unwrap();
    let next = privacy.frames(&sanitizer, frame("b", "B")).unwrap();
    assert_eq!(next[0]["update"]["messageId"], "a");
    assert_eq!(next[0]["update"]["content"]["text"], "/work");
    assert_eq!(next[1]["update"]["messageId"], "b");
    let tool = privacy
        .frames(
            &sanitizer,
            json!({"type":"activity","update":{"sessionUpdate":"tool_call","toolCallId":"tool"}}),
        )
        .unwrap();
    assert_eq!(tool.last().unwrap()["update"]["toolCallId"], "tool");
    assert!(!privacy.has_pending_text());
}
