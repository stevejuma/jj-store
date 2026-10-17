//! Explicit durable turn coordination; publication and runtime ownership stay injected.
use std::sync::Arc;

pub trait Types: Send + Sync + 'static {
    type Parent: Sync;
    type Command: Send + Sync;
    type Frame: Send;
    type Outcome: Send;
    type Error: Send;
}
pub enum Begin<O> {
    New,
    Replay(Vec<O>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dispatch {
    /// Accepted by the injected runtime queue, not proof of provider execution.
    Enqueued,
    /// The runtime proves the command did not leave this owner.
    NotSent,
    /// A side effect may have occurred. Never release or replay this command.
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Release {
    PreparationFailed,
    NotSent,
}
#[async_trait::async_trait]
pub trait Admission<T: Types>: Send + Sync {
    /// Recheck current host authority and bound the complete recipient set.
    async fn authorize(&self, parent: &T::Parent, commands: &[T::Command]) -> Result<(), T::Error>;
}
#[async_trait::async_trait]
pub trait Context<T: Types>: Send + Sync {
    /// Retain exact private delivery intent before returning its dispatch frame.
    async fn prepare(&self, command: &T::Command) -> Result<T::Frame, T::Error>;
}
#[async_trait::async_trait]
pub trait Runtime<T: Types>: Send + Sync {
    async fn dispatch(&self, command: &T::Command, frame: T::Frame) -> Dispatch;
}
#[async_trait::async_trait]
pub trait Journal<T: Types>: Send + Sync {
    /// Claim a durable explicit intent or verify an exclusive admitted host lease.
    /// Existing/interrupted intents return retained outcomes without replay.
    async fn begin(
        &self,
        parent: &T::Parent,
        commands: &[T::Command],
    ) -> Result<Begin<T::Outcome>, T::Error>;
    async fn prepared(&self, parent: &T::Parent) -> Result<(), T::Error>;
    async fn release(&self, command: &T::Command, reason: Release) -> Result<(), T::Error>;
    async fn unknown(&self, command: &T::Command) -> Result<(), T::Error>;
    async fn outcome(&self, command: &T::Command) -> Result<T::Outcome, T::Error>;
    async fn accepted(&self, parent: &T::Parent) -> Result<(), T::Error>;
}

pub struct Coordinator<'a, T: Types> {
    admission: Arc<dyn Admission<T> + 'a>,
    context: Arc<dyn Context<T> + 'a>,
    journal: Arc<dyn Journal<T> + 'a>,
    runtime: Arc<dyn Runtime<T> + 'a>,
}
impl<'a, T: Types> Coordinator<'a, T> {
    pub fn new(
        admission: Arc<dyn Admission<T> + 'a>,
        context: Arc<dyn Context<T> + 'a>,
        journal: Arc<dyn Journal<T> + 'a>,
        runtime: Arc<dyn Runtime<T> + 'a>,
    ) -> Self {
        Self {
            admission,
            context,
            journal,
            runtime,
        }
    }
    pub async fn send(
        &self,
        parent: &T::Parent,
        commands: &[T::Command],
    ) -> Result<Vec<T::Outcome>, T::Error> {
        self.admission.authorize(parent, commands).await?;
        if let Begin::Replay(outcomes) = self.journal.begin(parent, commands).await? {
            return Ok(outcomes);
        }
        let mut prepared = Vec::new();
        for command in commands {
            match self.context.prepare(command).await {
                Ok(frame) => prepared.push((command, frame)),
                Err(error) => {
                    for (prior, _) in &prepared {
                        self.journal
                            .release(prior, Release::PreparationFailed)
                            .await?;
                    }
                    return Err(error);
                }
            }
        }
        self.journal.prepared(parent).await?;
        let mut outcomes = Vec::new();
        for (command, frame) in prepared {
            match self.runtime.dispatch(command, frame).await {
                Dispatch::Enqueued => {}
                Dispatch::NotSent => self.journal.release(command, Release::NotSent).await?,
                Dispatch::Unknown => self.journal.unknown(command).await?,
            }
            outcomes.push(self.journal.outcome(command).await?);
        }
        self.journal.accepted(parent).await?;
        Ok(outcomes)
    }
}
