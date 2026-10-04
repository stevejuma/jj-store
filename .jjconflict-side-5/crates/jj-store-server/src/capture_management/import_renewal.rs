//! Original-receipt renewal only; no retained current authority or reader ownership.
use super::*;
use crate::capture_import_state::{
    CaptureImportIntent, CaptureImportJournal, CaptureImportReceipt,
};
use jj_store_capture::{codex_import::CodexImportProbe, source_registry::QualifiedCodexSource};

#[derive(Clone)]
pub(crate) struct CaptureImportRenewal {
    manager: Manager,
    intent: CaptureImportIntent,
    journal: CaptureImportJournal,
    receipt: CaptureImportReceipt,
}
impl CaptureImportSettlement {
    pub(crate) fn prepare_source(&self) -> Result<(QualifiedCodexSource, CodexImportProbe), Error> {
        let qualified = self.qualified_source()?;
        let probe = if let Some(receipt) = self.receipt() {
            CodexImportProbe::reconcile(qualified.clone(), receipt.source.codex())
        } else {
            CodexImportProbe::select(qualified.clone())
        }
        .map_err(|_| Error::Unavailable)?;
        self.check()?;
        Ok((qualified, probe))
    }
    /// Cached physical/header metadata only; no header or transcript I/O.
    pub(crate) fn qualified_source(&self) -> Result<QualifiedCodexSource, Error> {
        self.check()?;
        let runtime = self
            .policy
            .manager
            .binding()?
            .runtime
            .get()
            .ok_or(Error::Unavailable)?;
        let targets = runtime.import_targets().map_err(|_| Error::Unavailable)?;
        let mut matched = None;
        if let Some(receipt) = self.receipt() {
            // Original binding first. An old selector never retargets accepted history.
            for (_, qualified) in &targets.slots {
                if receipt.source.codex().matches_qualified(qualified) {
                    if matched.is_some() {
                        return Err(Error::Unavailable);
                    }
                    matched = Some(qualified.clone());
                }
            }
        } else {
            let (generation, ordinal) = self
                .policy
                .intent
                .selector
                .split_once(':')
                .ok_or(Error::Invalid)?;
            if generation != hex::encode(targets.generation) {
                return Err(Error::SourceStale);
            }
            let ordinal = u64::from_str_radix(ordinal, 16).map_err(|_| Error::Invalid)?;
            let qualified = targets
                .slots
                .iter()
                .find(|(slot, _)| *slot == ordinal)
                .map(|(_, q)| q.clone())
                .ok_or(Error::SourceStale)?;
            matched = Some(qualified);
        }
        self.check()?;
        matched.ok_or(Error::Unavailable)
    }
    pub(crate) fn renewal(&self) -> Result<CaptureImportRenewal, Error> {
        self.check()?;
        Ok(CaptureImportRenewal {
            manager: self.policy.manager.clone(),
            intent: self.policy.intent.clone(),
            journal: self.policy.journal.clone(),
            receipt: self.receipt().ok_or(Error::Unavailable)?.clone(),
        })
    }
}
impl CaptureImportRenewal {
    pub(crate) fn receipt(&self) -> &CaptureImportReceipt {
        &self.receipt
    }
    pub(crate) async fn settle(
        &self,
        admission: &crate::capture_admission::CaptureAdmission,
    ) -> Result<CaptureImportSettlement, Error> {
        let policy = self
            .manager
            .begin_import_owned(self.intent.clone(), self.journal.clone(), admission)
            .await?;
        let receipt = self.receipt.clone();
        admission
            .source_io(move || {
                let mut settlement = policy.into_settlement()?;
                if settlement.receipt() != Some(&receipt) {
                    return Err(Error::Unknown);
                }
                settlement.select(&receipt.source)?;
                Ok(settlement)
            })
            .await
            .map_err(|_| Error::Unavailable)?
    }
}
