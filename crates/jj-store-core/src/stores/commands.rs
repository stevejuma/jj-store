use jj_agent::{
    AgentObjectRef, AgentObjectV1, CaptureBatchId, NormalizerProfileV1, SanitizedObservationBatchV1,
};
use jj_store_model::{RecordDigest, portable::CodeEvidenceRefV1};

#[derive(Clone, Debug)]
pub struct CodeEvidence {
    pub reference: CodeEvidenceRefV1,
    pub bytes: Vec<u8>,
}
#[derive(Clone, Debug)]
pub enum AgentCommand {
    Prepared {
        delta: Box<PreparedAgentDelta>,
    },
    Initialize,
    Capture {
        profile: NormalizerProfileV1,
        batch: Box<SanitizedObservationBatchV1>,
        binding: Option<jj_collab::ThreadBindingV1>,
    },
    Provenance {
        expected: Option<AgentObjectRef>,
        object: Box<AgentObjectV1>,
        evidence: Vec<CodeEvidence>,
    },
}
#[derive(Clone, Debug)]
pub struct AgentResult {
    pub root: RecordDigest,
    pub batch: Option<CaptureBatchId>,
}
#[derive(Clone, Debug)]
pub enum ReviewCommand {
    Initialize,
    RepositoryThread {
        body: jj_review::ReviewCommentBodyV1,
    },
    Apply {
        command: Box<jj_review::ReviewCommandV1>,
        evidence: Vec<CodeEvidence>,
    },
}
#[derive(Clone, Debug)]
pub enum ReactionCommand {
    Reaction(jj_collab::ReactionCommandV1),
    Emoji(jj_collab::CustomEmojiCommandV1),
}

/// Stable native preparation data. Persist this command with its original
/// request/publication IDs before retrying a potentially published outcome.
#[derive(Clone, Debug)]
pub struct PreparedAgentDelta {
    pub actor: jj_store_model::PrincipalId,
    pub delta: jj_agent::AgentDeltaV1,
    pub result: Vec<u8>,
    pub thread_binding: Option<PreparedThreadBinding>,
}
#[derive(Clone, Debug)]
pub struct PreparedThreadBinding {
    pub binding: jj_collab::ThreadBindingV1,
    pub accepted_at_millis: i64,
}

impl AgentCommand {
    pub fn has_thread_binding(&self) -> bool {
        match self {
            Self::Capture { binding, .. } => binding.is_some(),
            Self::Prepared { delta } => delta.thread_binding.is_some(),
            _ => false,
        }
    }
}
