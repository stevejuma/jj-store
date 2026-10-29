use futures::{channel::oneshot, executor::block_on, task::noop_waker};
use jj_store_conversations::{
    Error, Result,
    capture::{
        Binding, CaptureBatch, CaptureError, DurableCapture, Input, NativeCapture, Publication,
        Receipt,
    },
    privacy::Sanitizer,
};
use serde_json::{Value, json};
use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

struct Privacy;
impl Sanitizer for Privacy {
    fn sanitize(&self, mut value: Value) -> Result<Value> {
        if value["refuse"] == true || value["assistantText"] == "reject-after-join" {
            return Err(Error::Unavailable);
        }
        if value["expand"] == true {
            value["padding"] = json!("x".repeat(2 * 1024 * 1024));
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
#[derive(Default)]
struct Observed {
    attempted: Vec<Value>,
    committed: Vec<Value>,
}
struct Sink {
    observed: Arc<Mutex<Observed>>,
    failure: Option<CaptureError>,
    mismatch: bool,
    wait: Option<oneshot::Receiver<()>>,
}
#[async_trait::async_trait]
impl DurableCapture for Sink {
    async fn persist(
        &mut self,
        batch: &CaptureBatch,
    ) -> std::result::Result<Receipt, CaptureError> {
        let value = serde_json::from_slice(batch.bytes()).unwrap();
        self.observed.lock().unwrap().attempted.push(value);
        if let Some(wait) = self.wait.take() {
            let _ = wait.await;
        }
        if let Some(failure) = self.failure {
            return Err(failure);
        }
        let value = serde_json::from_slice(batch.bytes()).unwrap();
        self.observed.lock().unwrap().committed.push(value);
        Ok(if self.mismatch {
            Receipt([0; 32])
        } else {
            batch.receipt()
        })
    }
}
fn binding() -> Binding {
    Binding {
        scope: "vault-profile".into(),
        worker: "native-worker".into(),
        request: "open-request".into(),
        target: "admitted-target".into(),
        window: "main".into(),
        admission: "generation".into(),
    }
}
fn sink(observed: &Arc<Mutex<Observed>>) -> Sink {
    Sink {
        observed: observed.clone(),
        failure: None,
        mismatch: false,
        wait: None,
    }
}
fn owner(sink: Sink) -> NativeCapture {
    NativeCapture::new(
        binding(),
        "fixture-policy/v1".into(),
        Arc::new(Privacy),
        Box::new(sink),
    )
    .unwrap()
}
fn input(frame: &Value, ordinal: u64, publication: Publication) -> Input<'_> {
    Input {
        worker: "native-worker",
        ordinal,
        publication,
        frame,
    }
}
fn chunk(text: &str) -> Value {
    json!({"version":3,"type":"activity","requestId":"turn","sessionId":"native-worker","sequence":71,"update":{"sessionUpdate":"agent_message_chunk","messageId":"answer","content":{"type":"text","text":text}}})
}

#[test]
fn bootstrap_facts_and_events_retain_native_binding_and_exact_capture_receipts() {
    block_on(async {
        let observed = Arc::new(Mutex::new(Observed::default()));
        let mut capture = owner(sink(&observed));
        let facts = json!({"version":3,"type":"opened","session":{"label":"facts"}});
        let accepted = capture
            .capture(input(&facts, 1, Publication::Facts))
            .await
            .unwrap();
        assert_eq!(accepted.frames()[0]["scanned"], true);
        {
            let locked = observed.lock().unwrap();
            let stored = &locked.committed[0];
            assert_eq!(stored["binding"]["scope"], "vault-profile");
            assert_eq!(stored["binding"]["worker"], "native-worker");
            assert_eq!(stored["binding"]["request"], "open-request");
            assert_eq!(stored["binding"]["target"], "admitted-target");
            assert_eq!(stored["binding"]["window"], "main");
            assert_eq!(stored["binding"]["admission"], "generation");
            assert_eq!(stored["policy"], "fixture-policy/v1");
            assert_eq!(stored["ordinal"], 1);
            assert_eq!(stored["publication"], json!({"kind":"facts"}));
        }
        let other = owner(sink(&Arc::new(Mutex::new(Observed::default()))))
            .capture(input(&facts, 1, Publication::Facts))
            .await
            .unwrap();
        assert_eq!(accepted.receipt(), other.receipt());
    });
}

#[test]
fn deferred_prefix_and_terminal_projection_are_one_sanitized_durable_batch() {
    block_on(async {
        let observed = Arc::new(Mutex::new(Observed::default()));
        let mut capture = owner(sink(&observed));
        let first = chunk("/private/");
        let second = chunk("root/file");
        let terminal =
            json!({"version":3,"type":"record","requestId":"turn","record":{"status":"completed"}});
        capture
            .capture(input(&first, 1, Publication::Event { sequence: 1 }))
            .await
            .unwrap();
        capture
            .capture(input(&second, 2, Publication::Event { sequence: 2 }))
            .await
            .unwrap();
        let accepted = capture
            .capture(input(&terminal, 3, Publication::Event { sequence: 3 }))
            .await
            .unwrap();
        assert_eq!(accepted.frames().len(), 2);
        assert_eq!(
            accepted.frames()[0]["update"]["content"]["text"],
            "$WORKSPACE/file"
        );
        assert!(accepted.frames()[0].get("sequence").is_none());
        assert_eq!(accepted.frames()[1]["assistantText"], "$WORKSPACE/file");
        assert!(
            accepted
                .frames()
                .iter()
                .all(|frame| frame["scanned"] == true)
        );
        let stored = observed.lock().unwrap();
        assert_eq!(stored.committed.len(), 3);
        assert_eq!(
            stored.committed[2]["publication"],
            json!({"kind":"event","sequence":3})
        );
        assert!(
            !serde_json::to_string(&stored.committed)
                .unwrap()
                .contains("/private/root")
        );
    });
}

#[test]
fn capture_must_finish_before_safe_output_is_returned() {
    let observed = Arc::new(Mutex::new(Observed::default()));
    let mut storage = sink(&observed);
    let (release, wait) = oneshot::channel();
    storage.wait = Some(wait);
    let mut capture = owner(storage);
    let frame = chunk("safe text");
    let mut pending =
        Box::pin(capture.capture(input(&frame, 1, Publication::Event { sequence: 1 })));
    let waker = noop_waker();
    assert!(matches!(
        pending.as_mut().poll(&mut Context::from_waker(&waker)),
        Poll::Pending
    ));
    assert_eq!(observed.lock().unwrap().attempted.len(), 1);
    assert!(observed.lock().unwrap().committed.is_empty());
    release.send(()).unwrap();
    let accepted = block_on(pending).unwrap();
    assert_eq!(observed.lock().unwrap().committed.len(), 1);
    assert_eq!(accepted.frames()[0]["scanned"], true);
}

#[test]
fn refused_unknown_and_mismatched_receipts_never_release_output_or_retry() {
    for expected in [
        CaptureError::Rejected,
        CaptureError::Unknown,
        CaptureError::ReceiptMismatch,
    ] {
        block_on(async {
            let observed = Arc::new(Mutex::new(Observed::default()));
            let mut storage = sink(&observed);
            if expected == CaptureError::ReceiptMismatch {
                storage.mismatch = true;
            } else {
                storage.failure = Some(expected);
            }
            let mut capture = owner(storage);
            let frame = chunk("private data");
            assert_eq!(
                capture
                    .capture(input(&frame, 1, Publication::Event { sequence: 1 }))
                    .await
                    .err(),
                Some(expected)
            );
            assert_eq!(
                capture
                    .capture(input(&frame, 1, Publication::Event { sequence: 1 }))
                    .await
                    .err(),
                Some(CaptureError::Stopped)
            );
            assert_eq!(observed.lock().unwrap().attempted.len(), 1);
        });
    }
}

#[test]
fn dropped_capture_wait_fences_reuse_without_replaying_an_uncertain_write() {
    let observed = Arc::new(Mutex::new(Observed::default()));
    let mut storage = sink(&observed);
    let (_release, wait) = oneshot::channel();
    storage.wait = Some(wait);
    let mut capture = owner(storage);
    let frame = chunk("private data");
    let mut pending =
        Box::pin(capture.capture(input(&frame, 1, Publication::Event { sequence: 1 })));
    let waker = noop_waker();
    assert!(
        pending
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    drop(pending);
    assert_eq!(
        block_on(capture.capture(input(&frame, 1, Publication::Event { sequence: 1 }))).err(),
        Some(CaptureError::Stopped)
    );
    assert_eq!(observed.lock().unwrap().attempted.len(), 1);
}

#[test]
fn scanner_failure_and_foreign_or_noncontiguous_inputs_never_reach_storage() {
    for case in ["scanner", "worker", "ordinal", "cursor", "shape"] {
        block_on(async {
            let observed = Arc::new(Mutex::new(Observed::default()));
            let mut capture = owner(sink(&observed));
            let mut frame = chunk("text");
            if case == "scanner" {
                frame["refuse"] = json!(true);
            }
            if case == "shape" {
                frame = Value::Null;
            }
            let mut value = input(&frame, 1, Publication::Event { sequence: 1 });
            match case {
                "worker" => value.worker = "other-worker",
                "ordinal" => value.ordinal = 2,
                "cursor" => value.publication = Publication::Event { sequence: 2 },
                _ => {}
            }
            assert!(capture.capture(value).await.is_err());
            assert!(observed.lock().unwrap().attempted.is_empty());
        });
    }
}

#[test]
fn joined_projections_are_rescanned_before_any_part_of_the_batch_is_persisted() {
    block_on(async {
        let observed = Arc::new(Mutex::new(Observed::default()));
        let mut capture = owner(sink(&observed));
        capture
            .capture(input(
                &chunk("reject-after-join"),
                1,
                Publication::Event { sequence: 1 },
            ))
            .await
            .unwrap();
        let terminal = json!({"type":"record","requestId":"turn"});
        assert_eq!(
            capture
                .capture(input(&terminal, 2, Publication::Event { sequence: 2 }))
                .await
                .err(),
            Some(CaptureError::Privacy)
        );
        assert_eq!(observed.lock().unwrap().attempted.len(), 1);
    });
}

#[test]
fn facts_consume_an_ordinal_without_advancing_the_native_event_cursor() {
    block_on(async {
        let observed = Arc::new(Mutex::new(Observed::default()));
        let mut capture = owner(sink(&observed));
        let frame = json!({"type":"configured","sequence":71});
        capture
            .capture(input(&frame, 1, Publication::Event { sequence: 1 }))
            .await
            .unwrap();
        capture
            .capture(input(&frame, 2, Publication::Facts))
            .await
            .unwrap();
        let accepted = capture
            .capture(input(&frame, 3, Publication::Event { sequence: 2 }))
            .await
            .unwrap();
        assert_eq!(accepted.frames()[0]["sequence"], 71);
        assert_eq!(
            capture
                .capture(input(&frame, 4, Publication::Facts))
                .await
                .err(),
            Some(CaptureError::Invalid)
        );
        assert_eq!(observed.lock().unwrap().committed.len(), 3);
    });
}

#[test]
fn input_and_post_scan_batch_byte_limits_fail_before_storage() {
    for frame in [
        json!({"large":"x".repeat(2 * 1024 * 1024)}),
        json!({"expand":true}),
    ] {
        block_on(async {
            let observed = Arc::new(Mutex::new(Observed::default()));
            let mut capture = owner(sink(&observed));
            assert_eq!(
                capture
                    .capture(input(&frame, 1, Publication::Facts))
                    .await
                    .err(),
                Some(CaptureError::Limit)
            );
            assert!(observed.lock().unwrap().attempted.is_empty());
        });
    }
}

#[test]
fn content_identity_is_canonical_and_binds_the_host_scope_policy_and_progress() {
    block_on(async {
        let mut receipts = Vec::new();
        for case in [
            "original",
            "reordered",
            "scope",
            "worker",
            "request",
            "target",
            "window",
            "admission",
            "policy",
            "content",
            "event",
        ] {
            let observed = Arc::new(Mutex::new(Observed::default()));
            let mut bound = binding();
            match case {
                "scope" => bound.scope = "other-scope".into(),
                "worker" => bound.worker = "other-worker".into(),
                "request" => bound.request = "other-open-request".into(),
                "target" => bound.target = "other-target".into(),
                _ => {}
            }
            if case == "window" {
                bound.window = "other-window".into();
            }
            if case == "admission" {
                bound.admission = "other-generation".into();
            }
            let worker = bound.worker.clone();
            let policy = if case == "policy" {
                "other-policy"
            } else {
                "fixture-policy/v1"
            };
            let mut capture = NativeCapture::new(
                bound,
                policy.into(),
                Arc::new(Privacy),
                Box::new(sink(&observed)),
            )
            .unwrap();
            let mut frame: Value = serde_json::from_str(if case == "reordered" {
                r#"{"z":2,"a":{"z":2,"a":1}}"#
            } else {
                r#"{"a":{"a":1,"z":2},"z":2}"#
            })
            .unwrap();
            if case == "content" {
                frame["z"] = json!(3);
            }
            let publication = if case == "event" {
                Publication::Event { sequence: 1 }
            } else {
                Publication::Facts
            };
            let mut value = input(&frame, 1, publication);
            value.worker = &worker;
            receipts.push(capture.capture(value).await.unwrap().receipt());
        }
        assert_eq!(receipts[0], receipts[1]);
        assert!(receipts[2..].iter().all(|receipt| *receipt != receipts[0]));
    });
}
