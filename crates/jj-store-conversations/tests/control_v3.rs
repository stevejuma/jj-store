use jj_store_conversations::{
    Error, Result,
    control_v3::Stream,
    privacy::{Sanitizer, StreamPrivacy},
};
use serde_json::{Value, json};

/// A deterministic injected policy fixture; real native policy is qualified by
/// the server consumer's original scanner and streaming privacy tests.
struct FixturePrivacy;
impl Sanitizer for FixturePrivacy {
    fn sanitize(&self, mut value: Value) -> Result<Value> {
        if value["reject"] == true {
            return Err(Error::Unavailable);
        }
        value["scanned"] = json!(true);
        Ok(value)
    }
    fn replace_stream_text(
        &self,
        pending: &mut String,
        chunk: &str,
        complete: bool,
    ) -> Result<String> {
        pending.push_str(chunk);
        Ok(if complete {
            std::mem::take(pending).replace("/private/root", "$WORKSPACE")
        } else {
            String::new()
        })
    }
}
fn chunk(message: &str, text: &str) -> Value {
    json!({"type":"activity","requestId":"request","sequence":7,"update":{
        "sessionUpdate":"agent_message_chunk","messageId":message,"content":{"type":"text","text":text}
    }})
}

#[test]
fn injected_privacy_flushes_before_boundaries_and_every_emitted_frame_is_scanned() {
    let mut privacy = StreamPrivacy::default();
    let mut frames = privacy
        .frames(&FixturePrivacy, chunk("a", "/private/"))
        .unwrap();
    assert!(privacy.has_pending_text());
    frames.extend(
        privacy
            .frames(&FixturePrivacy, chunk("a", "root/file"))
            .unwrap(),
    );
    let boundary = privacy.frames(&FixturePrivacy, chunk("b", "next")).unwrap();
    assert_eq!(boundary[0]["update"]["messageId"], "a");
    assert!(boundary[0].get("sequence").is_none());
    assert_eq!(boundary[0]["update"]["content"]["text"], "$WORKSPACE/file");
    frames.extend(boundary);
    frames.extend(
        privacy
            .frames(&FixturePrivacy, json!({"type":"record"}))
            .unwrap(),
    );
    assert!(frames.iter().all(|f| f["scanned"] == true));
    assert!(
        !serde_json::to_string(&frames)
            .unwrap()
            .contains("/private/root")
    );
    assert!(!privacy.has_pending_text());
}

#[test]
fn participants_keep_separate_assistant_and_tool_snapshots() {
    let mut stream = Stream::default();
    for id in ["a", "b"] {
        for mut frame in stream
            .frames(id, &FixturePrivacy, chunk("answer", id))
            .unwrap()
        {
            stream.observe(id, "request", &mut frame).unwrap();
        }
        let mut tool = json!({"type":"activity","update":{"toolCallId":"same","title":id}});
        stream.observe(id, "request", &mut tool).unwrap();
        assert_eq!(tool["toolSnapshot"]["title"], id);
    }
    for id in ["a", "b"] {
        let frames = stream
            .frames(id, &FixturePrivacy, json!({"type":"record"}))
            .unwrap();
        let mut terminal = Value::Null;
        for mut frame in frames {
            stream.observe(id, "request", &mut frame).unwrap();
            terminal = frame;
        }
        assert_eq!(terminal["assistantText"], id);
    }
    assert!(stream.is_drained());
}

#[test]
fn malformed_text_and_scanner_failure_produce_fixed_safe_errors() {
    let mut privacy = StreamPrivacy::default();
    let mut invalid = chunk("a", "");
    invalid["update"]["content"]["text"] = json!(123);
    assert_eq!(
        privacy.frames(&FixturePrivacy, invalid),
        Err(Error::Invalid)
    );
    assert_eq!(
        privacy.frames(&FixturePrivacy, json!({"type":"error","reject":true})),
        Err(Error::Unavailable)
    );
}

#[test]
fn participation_capacity_refuses_new_ids_without_blocking_an_existing_participant() {
    let mut stream = Stream::default();
    for id in 0..256 {
        stream
            .frames(&id.to_string(), &FixturePrivacy, json!({"type":"activity"}))
            .unwrap();
    }
    assert_eq!(
        stream.frames("extra", &FixturePrivacy, json!({"type":"activity"})),
        Err(Error::Limit)
    );
    assert!(
        stream
            .frames("0", &FixturePrivacy, json!({"type":"record"}))
            .is_ok()
    );
    assert!(
        stream
            .frames("extra", &FixturePrivacy, json!({"type":"activity"}))
            .is_ok()
    );
}

#[test]
fn assistant_and_tool_accumulation_retains_existing_bounds() {
    let mut stream = Stream::default();
    let mut first = chunk("a", &"x".repeat(512 * 1024));
    stream.observe("id", "request", &mut first).unwrap();
    assert_eq!(
        stream.observe("id", "request", &mut chunk("a", "more")),
        Err(Error::Limit)
    );
    for index in 0..256 {
        stream
            .observe(
                "id",
                "request",
                &mut json!({"type":"activity","update":{"toolCallId":index.to_string()}}),
            )
            .unwrap();
    }
    assert_eq!(
        stream.observe(
            "id",
            "request",
            &mut json!({"type":"activity","update":{"toolCallId":"extra"}})
        ),
        Err(Error::Limit)
    );
}
