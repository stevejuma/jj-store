//! Exact captured entities selected for execution feedback, not a wire format.
use jj_store_wal::publication::{MessageId, ToolCallId};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeedbackSubject {
    Message(MessageId),
    Tool(ToolCallId),
}
