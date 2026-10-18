#![cfg(feature = "native-privacy")]
use futures::executor::block_on;
use jj_store_conversations::{
    capture::{
        Binding, CaptureBatch, CaptureError, DurableCapture, Input, NativeCapture, Publication,
        Receipt,
    },
    native_privacy::{TRANSCRIPT_POLICY, TranscriptSanitizer},
};
use serde_json::{Value, json};
use std::{
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

const TOKEN: &str = "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";
fn privacy() -> Arc<TranscriptSanitizer> {
    static POLICY: OnceLock<(tempfile::TempDir, Arc<TranscriptSanitizer>)> = OnceLock::new();
    POLICY
        .get_or_init(|| {
            let cache = tempfile::tempdir().unwrap();
            let policy = TranscriptSanitizer::new(
                Path::new("/home/example/vault"),
                Path::new("/home/example"),
                Path::new("/tmp/example"),
                cache.path(),
            )
            .unwrap()
            .for_secrets(&["synthetic-secret-value".into()])
            .unwrap();
            (cache, Arc::new(policy))
        })
        .1
        .clone()
}
struct Sink {
    values: Arc<Mutex<Vec<Value>>>,
    unknown: bool,
}
#[async_trait::async_trait]
impl DurableCapture for Sink {
    async fn persist(&mut self, batch: &CaptureBatch) -> Result<Receipt, CaptureError> {
        self.values
            .lock()
            .unwrap()
            .push(serde_json::from_slice(batch.bytes()).unwrap());
        if self.unknown {
            Err(CaptureError::Unknown)
        } else {
            Ok(batch.receipt())
        }
    }
}
fn owner(unknown: bool) -> (NativeCapture, Arc<Mutex<Vec<Value>>>) {
    let values = Arc::new(Mutex::new(Vec::new()));
    let capture = NativeCapture::with_native_privacy(
        Binding {
            scope: "vault".into(),
            worker: "worker".into(),
            request: "open".into(),
            target: "target".into(),
            window: "main".into(),
            admission: "generation".into(),
        },
        privacy(),
        Box::new(Sink {
            values: values.clone(),
            unknown,
        }),
    )
    .unwrap();
    (capture, values)
}
fn event(
    owner: &mut NativeCapture,
    cursor: u64,
    frame: &Value,
) -> Result<CaptureBatch, CaptureError> {
    block_on(owner.capture(Input {
        worker: "worker",
        ordinal: cursor,
        publication: Publication::Event { sequence: cursor },
        frame,
    }))
}
fn text(chunk: &str) -> Value {
    json!({"type":"activity","requestId":"turn","update":{
        "sessionUpdate":"agent_message_chunk","content":{"type":"text","text":chunk}
    }})
}

#[test]
fn native_policy_scans_structured_fields_before_the_receipt() {
    let (mut capture, values) = owner(false);
    let batch = event(&mut capture, 1, &json!({"type":"opened","requestId":"open",
        "payload":{"/home/example/private":["/home/example/vault/note.md", "/tmp/example/file", "synthetic-secret-value", TOKEN]}
    })).unwrap();
    assert_eq!(
        batch.frames()[0]["payload"]["$HOME/private"],
        json!(["$WORKSPACE/note.md", "$TEMP/file", "$SECRET", "[REDACTED]"])
    );
    let saved = values.lock().unwrap();
    assert_eq!(saved[0]["policy"], TRANSCRIPT_POLICY);
    assert_eq!(saved[0]["frames"], json!(batch.frames()));
    let bytes = serde_json::to_string(&*saved).unwrap();
    for private in [
        "/home/example",
        "/tmp/example",
        "synthetic-secret-value",
        TOKEN,
    ] {
        assert!(!bytes.contains(private));
    }
}

#[test]
fn native_stream_preserves_exact_replacements_at_every_utf8_split() {
    let raw = "🦀 /home/example/vault/note /home/example/config /tmp/example/file synthetic-secret-value /";
    let expected = "🦀 $WORKSPACE/note $HOME/config $TEMP/file $SECRET /";
    for (split, _) in raw.char_indices() {
        let (mut capture, values) = owner(false);
        let mut frames = event(&mut capture, 1, &text(&raw[..split]))
            .unwrap()
            .into_frames();
        frames.extend(
            event(&mut capture, 2, &text(&raw[split..]))
                .unwrap()
                .into_frames(),
        );
        let terminal = event(
            &mut capture,
            3,
            &json!({"type":"record","requestId":"turn"}),
        )
        .unwrap();
        assert_eq!(terminal.frames().last().unwrap()["assistantText"], expected);
        frames.extend(terminal.into_frames());
        let display: String = frames
            .iter()
            .filter_map(|frame| frame["update"]["content"]["text"].as_str())
            .collect();
        assert_eq!(display, expected, "split {split}");
        let saved = serde_json::to_string(&*values.lock().unwrap()).unwrap();
        for private in ["/home/example", "/tmp/example", "synthetic-secret-value"] {
            assert!(!saved.contains(private), "split {split}");
        }
    }
}

#[test]
fn native_key_collisions_stop_before_any_durable_attempt() {
    let (mut capture, values) = owner(false);
    let invalid = json!({"/home/example": "one", "$HOME": "two"});
    assert_eq!(
        event(&mut capture, 1, &invalid).err(),
        Some(CaptureError::Privacy)
    );
    assert!(values.lock().unwrap().is_empty());
    assert_eq!(
        event(&mut capture, 1, &text("safe")).err(),
        Some(CaptureError::Stopped)
    );
}

#[test]
fn native_unknown_receipt_keeps_only_safe_candidate_and_never_retries() {
    let (mut capture, values) = owner(true);
    assert_eq!(
        event(&mut capture, 1, &text("synthetic-secret-value")).err(),
        Some(CaptureError::Unknown)
    );
    assert_eq!(
        event(&mut capture, 1, &text("synthetic-secret-value")).err(),
        Some(CaptureError::Stopped)
    );
    let saved = values.lock().unwrap();
    assert_eq!(saved.len(), 1);
    let bytes = serde_json::to_string(&*saved).unwrap();
    assert!(bytes.contains("$SECRET"));
    assert!(!bytes.contains("synthetic-secret-value"));
}

#[test]
fn native_oversized_payload_stops_before_persistence() {
    let (mut capture, values) = owner(false);
    let frame = json!({"large":"x".repeat(1024 * 1024)});
    assert_eq!(
        event(&mut capture, 1, &frame).err(),
        Some(CaptureError::Privacy)
    );
    assert!(values.lock().unwrap().is_empty());
}
