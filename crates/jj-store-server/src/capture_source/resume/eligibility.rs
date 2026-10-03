use super::*;
use jj_store_capture::{CaptureResult, source_registry::RegisteredCodexSource};

// An active proof remains bound to its exact snapshot; this is not a saved actor.
#[derive(Clone)]
pub(super) enum Eligibility {
    Admitted(SourceInstanceId),
    Active(Box<ActiveCodexSource>),
}
impl Eligibility {
    pub(super) fn check(
        &self,
        sources: &SourceRegistry,
        qualified: &QualifiedCodexSource,
    ) -> Result<bool> {
        match self {
            Self::Admitted(source) => sources
                .lookup_admitted(qualified)
                .map(|current| current == Some(*source))
                .map_err(|_| Error::Service("capture resume eligibility unavailable")),
            Self::Active(active) => active
                .revalidate()
                .map(|_| true)
                .map_err(|_| Error::Service("capture active snapshot changed")),
        }
    }
    pub(super) fn matches(&self, source: SourceInstanceId) -> bool {
        match self {
            Self::Admitted(expected) => *expected == source,
            Self::Active(_) => true,
        }
    }
    pub(super) fn acquire(
        self,
        sources: &SourceRegistry,
        qualified: QualifiedCodexSource,
    ) -> CaptureResult<Option<RegisteredCodexSource>> {
        match self {
            Self::Admitted(expected) => {
                if sources.lookup_admitted(&qualified)? != Some(expected) {
                    return Err("capture resume channel changed".into());
                }
                sources.acquire_admitted(qualified)
            }
            Self::Active(active) => sources.acquire_active(*active).map(Some),
        }
    }
}
