use super::*;
use crate::capture_source::{
    CaptureParkedResumeStep, CaptureResumeStep, CaptureSourceStep, CaptureStepContext,
    CodexCapturePipeline, CodexCaptureResume, ParkedCodexCapturePipeline,
};
use jj_store_capture::codex_activity::{ActivityPoll, CodexActivityProbe};
mod import;
use import::ImportWork;
#[cfg(all(test, unix))]
mod tests;
#[derive(Clone, Default)]
pub(super) struct SourceSnapshot {
    pub unavailable: bool,
    pub capacity_refusal: Option<jj_store_capture::agent_capture::CaptureCapacityRefusal>,
    pub retired: bool,
    pub observed: bool,
    pub awaiting_activity: bool,
    pub caught_up: bool,
    pub needs_frame: bool,
    pub compact: bool,
    pub anchor: Option<VerifiedCaptureWorkspace>,
}
enum Stage {
    New,
    Probe {
        probe: CodexActivityProbe,
        resume: Option<Box<CodexCaptureResume>>,
    },
    Resume(Box<CodexCaptureResume>),
    Parked(ParkedCodexCapturePipeline),
    Live(CodexCapturePipeline),
}
pub(super) struct SourceWork {
    qualified: QualifiedCodexSource,
    resources: CodexSupervisorResources,
    stage: Stage,
    snapshot: SourceSnapshot,
    import: Option<ImportWork>,
}
pub(super) struct SourceOutput {
    pub snapshot: SourceSnapshot,
    pub enqueued: bool,
    pub unavailable: bool,
}
impl SourceWork {
    pub fn new(qualified: QualifiedCodexSource, resources: CodexSupervisorResources) -> Self {
        Self {
            qualified,
            resources,
            stage: Stage::New,
            import: None,
            snapshot: SourceSnapshot {
                awaiting_activity: true,
                needs_frame: true,
                ..Default::default()
            },
        }
    }
    pub(super) fn install_import(
        &mut self,
        dispatch: crate::capture_management::CaptureImportDispatch,
    ) -> bool {
        if self.import.is_some() {
            return true;
        }
        if !self.replaceable() {
            return false;
        }
        self.stage = Stage::New;
        self.import = Some(ImportWork::new(dispatch));
        true
    }
    fn replaceable(&mut self) -> bool {
        // A cancelled live frame may be newer than the cached compact observation.
        self.settle();
        !self.snapshot.retired && (self.snapshot.compact || matches!(self.stage, Stage::New))
    }
    async fn advance(&mut self, raw: bool) -> Result<bool> {
        self.snapshot.caught_up = false;
        self.snapshot.needs_frame = false;
        if let Some(import) = &mut self.import {
            return import
                .advance(raw, &mut self.resources, &mut self.snapshot)
                .await;
        }
        // Only a settled body-free frame can relinquish tracking. Failed header
        // inspection retains it; a negative physical/header binding grants no
        // native eligibility to a newly discovered source.
        if self.snapshot.compact
            && !self
                .qualified
                .is_current()
                .map_err(|_| Error::Service("capture source binding unavailable"))?
        {
            self.stage = Stage::New;
            self.snapshot.retired = true;
            return Ok(false);
        }
        let r = &mut self.resources;
        let mut context = CaptureStepContext {
            authority: &r.authority,
            host: &r.host,
            admission: &r.admission,
            spool: &mut r.spool,
        };
        if matches!(self.stage, Stage::New) {
            if !raw {
                self.snapshot.needs_frame = true;
                return Ok(false);
            }
            self.stage = if let Some(resume) = CodexCaptureResume::prepare(
                r.sources.clone(),
                self.qualified.clone(),
                r.privacy.clone(),
                context.spool,
            )? {
                self.snapshot.awaiting_activity = false;
                self.snapshot.observed = true;
                Stage::Resume(Box::new(resume))
            } else {
                Stage::Probe {
                    probe: CodexActivityProbe::arm(self.qualified.clone())
                        .map_err(|_| Error::Service("capture activity unavailable"))?,
                    resume: None,
                }
            };
        }
        match &mut self.stage {
            Stage::New => unreachable!(),
            Stage::Probe { probe, resume } => {
                if !raw {
                    if probe
                        .poll_idle()
                        .map_err(|_| Error::Service("capture idle probe unavailable"))?
                        .is_none()
                    {
                        self.snapshot.needs_frame = true;
                    }
                    return Ok(false);
                }
                let activity = match probe.poll() {
                    Ok(activity) => activity,
                    Err(_) => {
                        if probe.requires_rearm() {
                            // Only unadmitted probe/preparation state is invalid.
                            // Arm on a later reserved step; current starts remain historical.
                            self.stage = Stage::New;
                            self.snapshot.needs_frame = true;
                        }
                        return Err(Error::Service("capture activity unavailable"));
                    }
                };
                self.snapshot.observed |= probe.is_compact();
                if activity != ActivityPoll::Active {
                    return Ok(false);
                }
                if resume.is_none() {
                    let active = probe
                        .proof()
                        .map_err(|_| Error::Service("capture activity proof unavailable"))?
                        .ok_or(Error::Service("capture activity proof changed"))?;
                    *resume = Some(Box::new(CodexCaptureResume::prepare_active(
                        r.sources.clone(),
                        active,
                        r.privacy.clone(),
                        context.spool,
                    )?));
                }
                match resume
                    .as_mut()
                    .expect("active preparation")
                    .step(&mut context)
                    .await
                {
                    Ok(CaptureResumeStep::Ready(source)) => {
                        self.snapshot.awaiting_activity = false;
                        self.stage = Stage::Live(source.into_pipeline());
                    }
                    Ok(_) => {}
                    Err(error) => {
                        *resume = None;
                        return Err(error);
                    }
                }
            }
            Stage::Resume(resume) => {
                if !raw {
                    self.snapshot.needs_frame = true;
                    return Ok(false);
                }
                if let CaptureResumeStep::Ready(source) = resume.step(&mut context).await? {
                    self.stage = Stage::Live(source.into_pipeline());
                }
            }
            Stage::Parked(parked) => {
                if !raw {
                    let idle = parked.idle_unchanged(&r.sources, &mut context).await;
                    self.snapshot.caught_up = idle.as_ref().is_ok_and(|unchanged| *unchanged);
                    self.snapshot.needs_frame = !self.snapshot.caught_up;
                    idle?;
                    return Ok(false);
                }
                if let CaptureParkedResumeStep::Ready(pipeline) = parked
                    .resume_current(r.sources.clone(), &mut context)
                    .await?
                {
                    self.stage = Stage::Live(*pipeline);
                }
            }
            Stage::Live(_) => {}
        }
        if let Stage::Live(pipeline) = &mut self.stage {
            let step = pipeline.ingest_step(&mut context).await?;
            self.snapshot.caught_up = step == CaptureSourceStep::Waiting;
            return Ok(step == CaptureSourceStep::Enqueued);
        }
        Ok(false)
    }
    fn settle(&mut self) {
        if let Some(import) = &mut self.import {
            import.settle(&mut self.snapshot);
            return;
        }
        if self.snapshot.retired {
            self.snapshot.compact = true;
            self.snapshot.needs_frame = false;
            self.snapshot.caught_up = false;
            self.snapshot.anchor = None;
            return;
        }
        if let Stage::Live(pipeline) = &mut self.stage {
            self.snapshot.anchor = Some(pipeline.anchor().clone());
            self.snapshot.compact = pipeline.compact_idle();
            let Stage::Live(pipeline) = std::mem::replace(&mut self.stage, Stage::New) else {
                unreachable!()
            };
            self.stage = Stage::Parked(pipeline.park_automatic());
        }
        self.snapshot.compact = match &mut self.stage {
            Stage::Probe { probe, resume } => probe.is_compact() && resume.is_none(),
            Stage::Parked(parked) => {
                self.snapshot.anchor = parked.anchor().cloned();
                parked.compact_idle()
            }
            _ => false,
        };
        self.snapshot.caught_up &= self.snapshot.compact;
    }
}
#[async_trait::async_trait]
impl OwnedWork for SourceWork {
    type Command = bool;
    type Output = SourceOutput;
    async fn step(&mut self, raw: bool) -> Self::Output {
        let result = self.advance(raw).await;
        self.snapshot.unavailable = result.is_err();
        self.snapshot.capacity_refusal = result.as_ref().err().and_then(Error::capacity_refusal);
        self.settle();
        SourceOutput {
            snapshot: self.snapshot.clone(),
            enqueued: result.as_ref().is_ok_and(|v| *v),
            unavailable: result.is_err(),
        }
    }
    fn settled(&mut self) {
        if self.import.is_some() {
            self.settle();
        }
    }
}
