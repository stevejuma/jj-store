//! One retained owner for bounded pipeline work; automatic composition is separate.
use crate::{
    Error, Result,
    capture_admission::CaptureAdmission,
    capture_authority::CaptureAuthority,
    capture_source::{CaptureSourceStep, CaptureStepContext, CodexCapturePipeline},
};
use jj_store_capture::agent_capture::{AgentCaptureSpool, PendingCaptureV1};
use jj_store_service::administration::RepositoryHost;
use std::{sync::Arc, time::Duration};
pub(crate) mod executor;
use executor::{Completion, OwnedWork, OwnedWorker};

pub enum CaptureWorkerCommand {
    Ingest,
    Publish(Box<PendingCaptureV1>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureWorkerPhase {
    Ready,
    Running,
    Overdue,
    Draining,
    Paused,
    Stopped,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureWorkerPoll {
    Idle,
    Running,
    Completed(CaptureSourceStep),
    Cancelled,
    Deadline,
    Unavailable,
}
// Raw paths, source state and proof-bearing context have no Debug/serialization.
struct PipelineState {
    pipeline: CodexCapturePipeline,
    authority: Arc<CaptureAuthority>,
    host: RepositoryHost,
    admission: CaptureAdmission,
    spool: AgentCaptureSpool,
}
#[async_trait::async_trait]
impl OwnedWork for PipelineState {
    type Command = CaptureWorkerCommand;
    type Output = Result<CaptureSourceStep>;
    async fn step(&mut self, command: Self::Command) -> Self::Output {
        let mut context = CaptureStepContext {
            authority: &self.authority,
            host: &self.host,
            admission: &self.admission,
            spool: &mut self.spool,
        };
        match command {
            CaptureWorkerCommand::Ingest => self.pipeline.ingest_step(&mut context).await,
            CaptureWorkerCommand::Publish(entry) => {
                self.pipeline.publish_step(&entry, &mut context).await
            }
        }
    }
}
pub struct CapturePipelineWorker(OwnedWorker<PipelineState>);
impl CapturePipelineWorker {
    pub fn new(
        pipeline: CodexCapturePipeline,
        authority: Arc<CaptureAuthority>,
        host: RepositoryHost,
        admission: CaptureAdmission,
        spool: AgentCaptureSpool,
        deadline: Duration,
    ) -> Result<Self> {
        Ok(Self(OwnedWorker::new(
            PipelineState {
                pipeline,
                authority,
                host,
                admission: admission.for_owned_worker(),
                spool,
            },
            deadline,
        )?))
    }
    pub fn phase(&self) -> CaptureWorkerPhase {
        self.0.phase()
    }
    pub fn start(&mut self, command: CaptureWorkerCommand) -> Result<()> {
        self.0.start(command)
    }
    pub fn pause(&mut self) {
        self.0.pause();
    }
    pub fn resume(&mut self) -> Result<()> {
        self.0.resume()
    }
    /// Never wait on running blocking I/O; settlement restores the same owner.
    pub async fn poll(&mut self) -> CaptureWorkerPoll {
        match self.0.poll().await {
            Completion::Idle => CaptureWorkerPoll::Idle,
            Completion::Running => CaptureWorkerPoll::Running,
            Completion::Completed(Ok(step)) => CaptureWorkerPoll::Completed(step),
            Completion::Completed(Err(_)) | Completion::Lost => CaptureWorkerPoll::Unavailable,
            Completion::Cancelled => CaptureWorkerPoll::Cancelled,
            Completion::Deadline => CaptureWorkerPoll::Deadline,
        }
    }
    pub async fn shutdown(&mut self) -> Result<()> {
        self.0.shutdown().await
    }
}
