//! Body-free scheduling input; every owned step must reacquire current policy.
use super::*;
use crate::capture_import_state::{CaptureImportIntent, CaptureImportJournal};
use jj_store_capture::source_registry::QualifiedCodexSource;

#[derive(Clone)]
pub struct CaptureImportDispatch {
    manager: Manager,
    intent: CaptureImportIntent,
    journal: CaptureImportJournal,
    pub(crate) qualified: QualifiedCodexSource,
}
impl CaptureImportPolicy {
    /// Select only retained metadata under current gates, then release those gates.
    /// This is volatile scheduling input, not durable acceptance or body authority.
    pub fn into_dispatch(self) -> Result<CaptureImportDispatch, Error> {
        let settlement = self.into_settlement()?;
        let qualified = settlement.qualified_source()?;
        let dispatch = settlement.dispatch(qualified);
        settlement.check()?;
        Ok(dispatch)
    }
}
impl CaptureImportSettlement {
    pub(crate) fn dispatch(&self, qualified: QualifiedCodexSource) -> CaptureImportDispatch {
        CaptureImportDispatch {
            manager: self.policy.manager.clone(),
            intent: self.policy.intent.clone(),
            journal: self.policy.journal.clone(),
            qualified,
        }
    }
}
impl CaptureImportDispatch {
    pub(crate) fn request_identity(&self) -> (PrincipalId, jj_store_model::RequestId) {
        (self.intent.principal, self.intent.request)
    }
    pub(crate) fn same_request(&self, other: &Self) -> bool {
        self.intent == other.intent && self.qualified.fingerprint() == other.qualified.fingerprint()
    }
    pub(crate) async fn begin(
        &self,
        admission: &crate::capture_admission::CaptureAdmission,
    ) -> Result<CaptureImportPolicy, Error> {
        self.manager
            .begin_import_owned(self.intent.clone(), self.journal.clone(), admission)
            .await
    }
}
