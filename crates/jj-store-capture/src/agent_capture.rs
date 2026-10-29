//! Exact acknowledgement shared by source readers and host-owned durable journals.
use jj_agent::{CaptureBatchId, SourcePositionV1};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableCaptureAck {
    pub batch_id: CaptureBatchId,
    pub position: SourcePositionV1,
}

#[cfg(feature = "application")]
mod journal;
#[cfg(feature = "application")]
pub use journal::{
    AgentCaptureSpool, CaptureCapacityRefusal, PendingCaptureStateV1, PendingCaptureV1,
};
#[cfg(feature = "application")]
pub(crate) use journal::{MAX_ENTRIES, ack, same_capture};
