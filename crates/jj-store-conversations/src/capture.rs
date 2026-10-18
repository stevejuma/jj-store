//! One raw owner, injected native privacy, and an exact durable fence before output.
use crate::{control_v3::Stream, privacy::Sanitizer};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::sync::Arc;

const MAX_BATCH_BYTES: usize = 2 * 1024 * 1024;

fn encode(value: &Value) -> Result<Vec<u8>, CaptureError> {
    struct Bounded(Vec<u8>);
    impl std::io::Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.0.len().saturating_add(bytes.len()) > MAX_BATCH_BYTES {
                return Err(std::io::Error::other("native capture byte limit"));
            }
            self.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut output = Bounded(Vec::new());
    serde_json::to_writer(&mut output, value).map_err(|_| CaptureError::Limit)?;
    Ok(output.0)
}

/// Opaque native identities; hosts must never use device paths or credentials here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binding {
    pub scope: String,
    pub worker: String,
    pub request: String,
    pub target: String,
    pub window: String,
    pub admission: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Publication {
    Facts,
    Event { sequence: u64 },
}
pub struct Input<'a> {
    pub worker: &'a str,
    pub ordinal: u64,
    pub publication: Publication,
    pub frame: &'a Value,
}
/// Expected content identity, not by itself proof that a backend persisted it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Receipt(pub [u8; 32]);

pub struct CaptureBatch {
    bytes: Vec<u8>,
    frames: Vec<Value>,
    receipt: Receipt,
}
impl CaptureBatch {
    /// Bounded canonical sanitized capture, including exact scope and progress.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn frames(&self) -> &[Value] {
        &self.frames
    }
    pub fn into_frames(self) -> Vec<Value> {
        self.frames
    }
    pub fn receipt(&self) -> Receipt {
        self.receipt
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CaptureError {
    #[error("invalid native capture binding or progress")]
    Invalid,
    #[error("native capture byte limit")]
    Limit,
    #[error("native capture privacy unavailable")]
    Privacy,
    #[error("native capture rejected")]
    Rejected,
    #[error("native capture outcome unknown")]
    Unknown,
    #[error("native capture receipt mismatch")]
    ReceiptMismatch,
    #[error("native capture owner stopped")]
    Stopped,
}
#[async_trait::async_trait]
pub trait DurableCapture: Send {
    /// Confirm only after the exact bytes/progress are durable. Retain unknown
    /// intent for host-owned recovery and deduplicate exact batches across
    /// owners; never spawn detached persistence or retry an unknown write here.
    async fn persist(&mut self, batch: &CaptureBatch) -> Result<Receipt, CaptureError>;
}

/// One serialized worker lifetime. Native hosts drain admitted calls before
/// releasing authority; this runtime-independent library cannot supervise tasks.
pub struct NativeCapture {
    binding: Binding,
    policy: String,
    sanitizer: Arc<dyn Sanitizer>,
    sink: Box<dyn DurableCapture>,
    stream: Stream,
    ordinal: u64,
    sequence: u64,
    facts: bool,
    stopped: bool,
}
impl NativeCapture {
    pub fn new(
        binding: Binding,
        policy: String,
        sanitizer: Arc<dyn Sanitizer>,
        sink: Box<dyn DurableCapture>,
    ) -> Result<Self, CaptureError> {
        if [
            &binding.scope,
            &binding.worker,
            &binding.request,
            &binding.target,
            &binding.window,
            &binding.admission,
            &policy,
        ]
        .iter()
        .any(|id| id.is_empty() || id.len() > 1024 || id.chars().any(char::is_control))
        {
            return Err(CaptureError::Invalid);
        }
        Ok(Self {
            binding,
            policy,
            sanitizer,
            sink,
            stream: Stream::default(),
            ordinal: 0,
            sequence: 0,
            facts: false,
            stopped: false,
        })
    }
    pub async fn capture(&mut self, input: Input<'_>) -> Result<CaptureBatch, CaptureError> {
        if self.stopped {
            return Err(CaptureError::Stopped);
        }
        // Any failure or abandoned future retains an uncertain owner. No retry
        // may use a possibly already persisted batch as a fresh write.
        self.stopped = true;
        if input.worker != self.binding.worker
            || self.ordinal.checked_add(1) != Some(input.ordinal)
            || !input.frame.is_object()
        {
            return Err(CaptureError::Invalid);
        }
        let publication = match input.publication {
            Publication::Facts if !self.facts => json!({"kind":"facts"}),
            Publication::Event { sequence } if self.sequence.checked_add(1) == Some(sequence) => {
                json!({"kind":"event","sequence":sequence})
            }
            _ => return Err(CaptureError::Invalid),
        };
        encode(input.frame)?;
        let mut frames = self
            .stream
            .frames(&self.binding.worker, &*self.sanitizer, input.frame.clone())
            .map_err(|_| CaptureError::Privacy)?;
        for frame in &mut frames {
            let request = frame["requestId"].as_str().unwrap_or("").to_owned();
            self.stream
                .observe(&self.binding.worker, &request, frame)
                .map_err(|_| CaptureError::Limit)?;
            // Joined assistant text and merged tool snapshots need their own
            // complete-frame scan before the durable sink sees them.
            *frame = self
                .sanitizer
                .sanitize(std::mem::take(frame))
                .map_err(|_| CaptureError::Privacy)?;
        }
        let mut envelope = json!({
            "schema":1,"policy":self.policy,
            "binding":{"scope":self.binding.scope,"worker":self.binding.worker,"request":self.binding.request,"target":self.binding.target,"window":self.binding.window,"admission":self.binding.admission},
            "ordinal":input.ordinal,"publication":publication,"frames":frames,
        });
        // Feature unification may enable serde_json/preserve_order downstream.
        // A capture identity must not depend on source object insertion order.
        envelope.sort_all_objects();
        let bytes = encode(&envelope)?;
        let mut digest = Sha256::new_with_prefix(b"jj-store/native-capture/v1\0");
        digest.update(&bytes);
        let batch = CaptureBatch {
            receipt: Receipt(digest.finalize().into()),
            bytes,
            frames,
        };
        if self.sink.persist(&batch).await? != batch.receipt {
            return Err(CaptureError::ReceiptMismatch);
        }
        self.ordinal = input.ordinal;
        match input.publication {
            Publication::Facts => self.facts = true,
            Publication::Event { sequence } => self.sequence = sequence,
        }
        self.stopped = false;
        Ok(batch)
    }
}
