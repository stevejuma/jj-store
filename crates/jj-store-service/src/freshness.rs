//! Bounded process-local change notices; never repository publication authority.
pub(crate) mod native;
use jj_store_model::RepositoryId;
use schemars::JsonSchema;
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};
use tokio::sync::{broadcast, watch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Repository,
    Code,
    Threads,
    Agents,
    AgentExecution,
    AgentSettings,
    Reviews,
    Work,
    Storage,
    Sync,
    Search,
    IndexTasks,
    Capture,
    Configuration,
    Discovery,
    Profiles,
    EmojiLibrary,
    Workspaces,
    Conversion,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Change {
    pub repository: Option<RepositoryId>,
    pub domain: Domain,
    pub witness: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(skip)]
    #[schemars(skip)]
    pub audience: Option<jj_store_model::PrincipalId>,
}
#[derive(Clone, Default)]
pub struct Signals(Arc<SignalState>);
#[derive(Default)]
struct SignalState {
    sender: Mutex<Option<broadcast::Sender<Change>>>,
    revision: AtomicU64,
}
impl Signals {
    pub fn notify(
        &self,
        repository: Option<RepositoryId>,
        domain: Domain,
        identity: Option<String>,
    ) {
        self.notify_to(repository, domain, identity, None);
    }
    pub fn notify_to(
        &self,
        repository: Option<RepositoryId>,
        domain: Domain,
        identity: Option<String>,
        audience: Option<jj_store_model::PrincipalId>,
    ) {
        let witness = self
            .0
            .revision
            .fetch_add(1, Ordering::AcqRel)
            .checked_add(1)
            .expect("freshness signal exhaustion");
        self.send(Change {
            repository,
            domain,
            witness: witness.to_string(),
            identity,
            audience,
        });
    }
    pub fn subscribe(&self) -> broadcast::Receiver<Change> {
        let mut state = self.0.sender.lock().expect("freshness signals");
        state
            .get_or_insert_with(|| broadcast::channel(128).0)
            .subscribe()
    }
    pub fn send(&self, change: Change) {
        if let Some(sender) = &*self.0.sender.lock().expect("freshness signals") {
            let _ = sender.send(change);
        }
    }
}
#[derive(Clone)]
pub struct Hub(Arc<Inner>);
struct Inner {
    state: Mutex<State>,
    signal: watch::Sender<u64>,
    capacity: usize,
}
#[derive(Default)]
struct State {
    revision: u64,
    records: VecDeque<(u64, Change)>,
    witnesses: BTreeMap<
        (
            Option<RepositoryId>,
            Domain,
            Option<jj_store_model::PrincipalId>,
        ),
        Change,
    >,
}
impl Hub {
    pub fn new(capacity: usize) -> Self {
        assert!((1..=4096).contains(&capacity));
        Self(Arc::new(Inner {
            state: Mutex::new(State::default()),
            signal: watch::channel(0).0,
            capacity,
        }))
    }
    /// Establishes the first baseline without inventing a change. Semantic comparisons suppress scheduler churn.
    pub fn observe(&self, mut change: Change) {
        Self::bound(&mut change);
        let mut state = self.0.state.lock().expect("freshness hub");
        let key = (change.repository, change.domain, change.audience);
        let previous = state.witnesses.insert(key, change.clone());
        while state.witnesses.len() > 4096 {
            state.witnesses.pop_first();
        }
        if previous.as_ref().is_none_or(|before| before == &change) {
            return;
        }
        Self::record(&self.0, &mut state, change);
    }
    /// State-owner settlement is an event even when no earlier observation was captured.
    pub fn changed(&self, mut change: Change) {
        Self::bound(&mut change);
        let mut state = self.0.state.lock().expect("freshness hub");
        if state
            .witnesses
            .get(&(change.repository, change.domain, change.audience))
            == Some(&change)
        {
            return;
        }
        state.witnesses.insert(
            (change.repository, change.domain, change.audience),
            change.clone(),
        );
        while state.witnesses.len() > 4096 {
            state.witnesses.pop_first();
        }
        Self::record(&self.0, &mut state, change);
    }
    fn bound(change: &mut Change) {
        if change.witness.len() > 128 {
            change.witness = hex::encode(Sha256::digest(change.witness.as_bytes()));
        }
        if change.identity.as_ref().is_some_and(|id| id.len() > 128) {
            change.identity = None;
        }
    }
    fn record(inner: &Inner, state: &mut State, change: Change) {
        state.revision = state
            .revision
            .checked_add(1)
            .expect("freshness revision exhaustion");
        state.records.push_back((state.revision, change));
        while state.records.len() > inner.capacity {
            state.records.pop_front();
        }
        inner.signal.send_replace(state.revision);
    }
    pub fn subscribe(&self) -> watch::Receiver<u64> {
        self.0.signal.subscribe()
    }
    pub fn snapshot(&self) -> (u64, Vec<Change>) {
        let state = self.0.state.lock().expect("freshness hub");
        (state.revision, state.witnesses.values().cloned().collect())
    }
    pub fn after(&self, revision: u64) -> Option<Vec<(u64, Change)>> {
        let state = self.0.state.lock().expect("freshness hub");
        if revision > state.revision
            || state
                .records
                .front()
                .is_some_and(|(first, _)| revision < first.saturating_sub(1))
        {
            return None;
        }
        Some(
            state
                .records
                .iter()
                .filter(|(seq, _)| *seq > revision)
                .cloned()
                .collect(),
        )
    }
    pub fn forget(&self, repository: RepositoryId) {
        self.0
            .state
            .lock()
            .expect("freshness hub")
            .witnesses
            .retain(|(repo, _, _), _| *repo != Some(repository));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn change(domain: Domain, witness: &str) -> Change {
        Change {
            repository: None,
            domain,
            witness: witness.into(),
            identity: None,
            audience: None,
        }
    }
    #[test]
    fn unchanged_witness_does_not_notify_and_replay_overflow_requires_reset() {
        let hub = Hub::new(2);
        hub.observe(change(Domain::Storage, "a"));
        hub.observe(change(Domain::Storage, "a"));
        assert_eq!(hub.snapshot().0, 0);
        hub.observe(change(Domain::Storage, "b"));
        hub.changed(change(Domain::IndexTasks, "ready"));
        assert_eq!(hub.after(0).unwrap().len(), 2);
        hub.changed(change(Domain::Capture, "paused"));
        assert!(hub.after(0).is_none());
        assert_eq!(hub.after(1).unwrap().len(), 2);
        assert!(hub.after(4).is_none());
    }
    #[test]
    fn task_witnesses_retain_independent_principal_audiences() {
        let hub = Hub::new(8);
        let mut first = change(Domain::IndexTasks, "ready");
        first.audience = Some(jj_store_model::PrincipalId::new([1; 16]));
        first.identity = Some("one".into());
        let mut second = first.clone();
        second.audience = Some(jj_store_model::PrincipalId::new([2; 16]));
        second.identity = Some("two".into());
        hub.changed(first.clone());
        hub.changed(second);
        hub.changed(first);
        assert_eq!(hub.snapshot().1.len(), 2);
        assert_eq!(hub.after(0).unwrap().len(), 2);
    }
}
