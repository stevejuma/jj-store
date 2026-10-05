use super::*;
use crate::{
    capture_source::{CaptureSourceStep, CaptureStepContext},
    opencode_source::OpenCodePublicationProof,
};
use jj_store_capture::{agent_publication::CaptureDrainScheduler, opencode_ingress::IntegrationId};
pub(super) struct ApiDeliveryWork {
    resources: CodexSupervisorResources,
    scheduler: CaptureDrainScheduler,
}
pub(super) struct ApiDeliveryOutput {
    pub published: bool,
    pub unavailable: bool,
    pub capacity_refusal: Option<jj_store_capture::agent_capture::CaptureCapacityRefusal>,
}
impl ApiDeliveryWork {
    #[cfg(test)]
    pub(super) fn admission_for_test(&mut self, admission: CaptureAdmission) {
        self.resources.admission = admission.for_owned_worker();
    }
    pub fn new(resources: CodexSupervisorResources) -> Self {
        Self {
            resources,
            scheduler: Default::default(),
        }
    }
    async fn advance(&mut self, proofs: &[OpenCodePublicationProof]) -> Result<bool> {
        let r = &mut self.resources;
        let pending: Vec<_> = r
            .spool
            .pending()
            .map_err(|_| Error::Service("OpenCode delivery queue unavailable"))?
            .into_iter()
            .filter(|e| e.cursor.integration == IntegrationId::OpenCode)
            .collect();
        let Some(entry) = self
            .scheduler
            .select(&pending)
            .map_err(|_| Error::Service("OpenCode delivery selection unavailable"))?
        else {
            return Ok(false);
        };
        let Some(proof) = proofs.iter().find(|p| p.batch_id() == entry.batch.batch_id) else {
            return Ok(false);
        };
        let step = proof
            .publish(
                entry,
                &mut CaptureStepContext {
                    authority: &r.authority,
                    host: &r.host,
                    admission: &r.admission,
                    spool: &mut r.spool,
                },
            )
            .await?;
        Ok(matches!(step,CaptureSourceStep::Published(report) if report.committed>0))
    }
}
#[async_trait::async_trait]
impl OwnedWork for ApiDeliveryWork {
    type Command = Vec<OpenCodePublicationProof>;
    type Output = ApiDeliveryOutput;
    async fn step(&mut self, proofs: Self::Command) -> Self::Output {
        let result = self.advance(&proofs).await;
        ApiDeliveryOutput {
            capacity_refusal: result.as_ref().err().and_then(Error::capacity_refusal),
            published: result.as_ref().is_ok_and(|b| *b),
            unavailable: result.is_err(),
        }
    }
}
