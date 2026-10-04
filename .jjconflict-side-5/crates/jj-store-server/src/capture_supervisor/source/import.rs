//! Import-only preparation, ingress and publication share one retained source slot.
use super::*;
use crate::{
    capture_management::CaptureImportDispatch,
    capture_source::{
        CaptureImportStep, CodexCaptureImport, ImportedCodexCapturePipeline,
        ParkedImportedCodexCapturePipeline,
    },
};
use jj_store_capture::agent_publication::CaptureDrainScheduler;
enum Stage {
    New,
    Preparing(CodexCaptureImport),
    Live(ImportedCodexCapturePipeline),
    Parked(ParkedImportedCodexCapturePipeline),
}
pub(super) struct ImportWork {
    dispatch: CaptureImportDispatch,
    stage: Stage,
    publish_next: bool,
    delivery: CaptureDrainScheduler,
}
impl ImportWork {
    pub fn new(dispatch: CaptureImportDispatch) -> Self {
        Self {
            dispatch,
            stage: Stage::New,
            publish_next: true,
            delivery: Default::default(),
        }
    }
    pub async fn advance(
        &mut self,
        raw: bool,
        r: &mut CodexSupervisorResources,
        snapshot: &mut SourceSnapshot,
    ) -> Result<bool> {
        snapshot.needs_frame = true;
        if !raw {
            return Ok(false);
        }
        if matches!(self.stage, Stage::New) {
            let policy = self
                .dispatch
                .begin(&r.admission)
                .await
                .map_err(|_| Error::Service("capture import policy unavailable"))?;
            self.stage = Stage::Preparing(CodexCaptureImport::prepare(
                policy,
                r.sources.clone(),
                r.privacy.clone(),
                &r.spool,
            )?);
        }
        let mut context = CaptureStepContext {
            authority: &r.authority,
            host: &r.host,
            admission: &r.admission,
            spool: &mut r.spool,
        };
        let step = match &mut self.stage {
            Stage::Preparing(import) => import.step(&mut context).await?,
            Stage::Parked(parked) => parked.resume_current(&mut context).await?,
            Stage::Live(_) => return self.run_live(&mut context, snapshot).await,
            Stage::New => unreachable!(),
        };
        if let CaptureImportStep::Ready(pipeline) = step {
            self.stage = Stage::Live(*pipeline);
            snapshot.observed = true;
            snapshot.awaiting_activity = false;
            return self.run_live(&mut context, snapshot).await;
        }
        Ok(false)
    }
    async fn run_live(
        &mut self,
        context: &mut CaptureStepContext<'_>,
        snapshot: &mut SourceSnapshot,
    ) -> Result<bool> {
        let Stage::Live(pipeline) = &mut self.stage else {
            unreachable!()
        };
        let partition = pipeline.partition();
        let pending: Vec<_> = context
            .spool
            .pending()
            .map_err(|e| Error::capture_spool(e, "capture import delivery unavailable"))?
            .into_iter()
            .filter(|e| {
                e.batch.integration == partition.integration
                    && e.batch.source_instance_id == partition.source
            })
            .collect();
        let entry = self
            .delivery
            .select(&pending)
            .map_err(|_| Error::Service("capture import delivery selection unavailable"))?
            .cloned();
        let step = if self.publish_next
            && let Some(entry) = &entry
        {
            self.publish_next = false;
            pipeline.publish_step(entry, context).await?
        } else {
            self.publish_next = true;
            pipeline.ingest_step(context).await?
        };
        snapshot.caught_up = step == CaptureSourceStep::Waiting && entry.is_none();
        Ok(matches!(
            step,
            CaptureSourceStep::Enqueued | CaptureSourceStep::Published(_)
        ))
    }
    pub fn settle(&mut self, snapshot: &mut SourceSnapshot) {
        if let Stage::Preparing(import) = &mut self.stage {
            import.release_policy();
        }
        if let Stage::Live(pipeline) = &mut self.stage {
            snapshot.anchor = Some(pipeline.anchor().clone());
            pipeline.compact_idle();
            let Stage::Live(pipeline) = std::mem::replace(&mut self.stage, Stage::New) else {
                unreachable!()
            };
            self.stage = Stage::Parked(pipeline.park());
        }
        snapshot.compact = match &mut self.stage {
            Stage::New => true,
            Stage::Preparing(import) => import.compact(),
            Stage::Parked(parked) => {
                snapshot.anchor = parked.anchor().cloned();
                parked.compact_idle()
            }
            Stage::Live(_) => unreachable!(),
        };
        snapshot.caught_up &= snapshot.compact;
    }
}
