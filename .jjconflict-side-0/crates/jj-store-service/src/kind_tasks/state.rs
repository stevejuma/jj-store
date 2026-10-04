use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct Limits {
    pub retained: usize,
    pub ttl: Duration,
    pub read: ReadLimits,
    pub preparation: prep::Limits,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            retained: 32,
            ttl: Duration::from_secs(300),
            read: ReadLimits::default(),
            preparation: prep::Limits::default(),
        }
    }
}
impl Limits {
    pub(super) fn validate(self) -> Result<()> {
        self.preparation.validate()?;
        if !(1..=1024).contains(&self.retained)
            || self.ttl.is_zero()
            || self.ttl > Duration::from_secs(3600)
            || self.read.max_objects == 0
            || self.read.max_objects > 2_000_000
            || self.read.timeout.is_zero()
            || self.read.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "KindPreparationKind")]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Operations,
    Copies,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "KindPreparationState")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum State {
    Running,
    Canceling,
    Ready,
    Failed { error: q::Error },
    Canceled,
}
impl State {
    pub(super) fn live(&self) -> bool {
        matches!(self, Self::Running | Self::Canceling)
    }
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "KindPreparationTask")]
pub struct Task {
    pub version: u32,
    pub id: String,
    pub snapshot: ReadSnapshot,
    pub kind: Kind,
    pub progress: prep::Progress,
    #[serde(flatten)]
    pub state: State,
}
pub(super) struct Entry {
    pub(super) task: Task,
    pub(super) owner: m::PrincipalId,
    pub(super) expires: Instant,
    pub(super) cancellation: Cancellation,
}
impl Entry {
    pub(super) fn cancel(&mut self) {
        if self.task.state.live() {
            self.task.state = State::Canceling;
            self.cancellation.cancel();
        }
    }
}
#[derive(Default)]
pub(super) struct Registry(pub(super) BTreeMap<String, Entry>);
impl Registry {
    pub(super) fn prune(&mut self) {
        self.0.retain(|_, entry| {
            if Instant::now() < entry.expires {
                return true;
            }
            entry.cancel();
            entry.task.state.live()
        });
    }
    pub(super) fn existing(
        &self,
        repository: m::RepositoryId,
        epoch: m::EpochId,
        operation: m::OperationId,
        kind: Kind,
        owner: m::PrincipalId,
    ) -> Option<Task> {
        self.0
            .values()
            .find(|entry| {
                entry.owner == owner
                    && entry.task.snapshot.authority.repo_id == repository
                    && entry.task.snapshot.authority.epoch == epoch
                    && entry.task.snapshot.operation_id == operation
                    && entry.task.kind == kind
                    && matches!(entry.task.state, State::Running)
                    && Instant::now() < entry.expires
            })
            .map(|entry| entry.task.clone())
    }
}
