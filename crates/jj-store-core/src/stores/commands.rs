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
