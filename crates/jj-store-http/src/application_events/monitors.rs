//! One metadata monitor per subscribed repository, shared across browser connections.
use super::*;
use jj_store_service::freshness::{Change, Domain};
use std::{
    collections::BTreeMap,
    sync::{
        Weak,
        atomic::{AtomicUsize, Ordering},
    },
};
use tokio::sync::Mutex;
#[derive(Default)]
pub(crate) struct Monitors(Mutex<BTreeMap<Option<m::RepositoryId>, Entry>>);
struct Entry {
    managed: bool,
    leases: Arc<AtomicUsize>,
    task: tokio::task::JoinHandle<()>,
    producers: Option<tokio::task::JoinHandle<()>>,
}
pub(super) struct Lease(Arc<AtomicUsize>);
impl Drop for Lease {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}
impl Monitors {
    pub(crate) async fn shutdown(&self) {
        let entries = std::mem::take(&mut *self.0.lock().await);
        for (_, entry) in entries {
            entry.task.abort();
            if let Some(task) = &entry.producers {
                task.abort();
            }
            let _ = entry.task.await;
            if let Some(task) = entry.producers {
                let _ = task.await;
            }
        }
    }

    pub(crate) async fn activate(
        &self,
        state: &Arc<State>,
        repository: Option<m::RepositoryId>,
        local: bool,
    ) {
        if !matches!(state.config.access, Access::LocalTrusted { .. }) {
            return;
        }
        let Ok(lease) = self.acquire(state, repository, local).await else {
            return;
        };
        if let Some(entry) = self.0.lock().await.get_mut(&repository) {
            entry.managed = true;
        }
        drop(lease);
    }
    pub(super) async fn acquire(
        &self,
        state: &Arc<State>,
        repository: Option<m::RepositoryId>,
        local: bool,
    ) -> Result<Lease> {
        let mut entries = self.0.lock().await;
        if let Some(entry) = entries.get(&repository) {
            entry.leases.fetch_add(1, Ordering::AcqRel);
            return Ok(Lease(entry.leases.clone()));
        }
        if entries.len() >= 257 {
            return Err(Error::busy());
        }
        if let Err(error) = observe(state, repository, local, false, false).await
            && let Some(change) = observation_failure(repository, &error)
        {
            state.freshness.observe(change);
        }
        let leases = Arc::new(AtomicUsize::new(1));
        let weak = Arc::downgrade(state);
        let retained = leases.clone();
        let task = tokio::spawn(run(weak, repository, local, retained));
        let producers = repository.is_none().then(|| super::producers::start(state));
        entries.insert(
            repository,
            Entry {
                managed: false,
                leases: leases.clone(),
                task,
                producers,
            },
        );
        Ok(Lease(leases))
    }
}
impl Drop for Monitors {
    fn drop(&mut self) {
        for entry in self.0.get_mut().values() {
            entry.task.abort();
            if let Some(task) = &entry.producers {
                task.abort();
            }
        }
    }
}
async fn run(
    weak: Weak<State>,
    repository: Option<m::RepositoryId>,
    local: bool,
    leases: Arc<AtomicUsize>,
) {
    let mut hints = weak
        .upgrade()
        .and_then(|s| s.host.as_ref().map(|h| h.publication_hints()));
    let mut health = weak
        .upgrade()
        .and_then(|s| repository.and_then(|id| s.search.health(id)));
    let mut observed_health = health.as_ref().map(|h| h.borrow().clone());
    let mut idle = None;
    let mut physical = if local {
        local_watch(&weak, repository).await
    } else {
        None
    };
    let mut witness = repository_witness(&weak, repository);
    let mut activated = false;
    loop {
        let health_wake = tokio::select! {
            _ = async { if let Some(h) = &mut health { let _ = h.changed().await; } else { std::future::pending::<()>().await; } } => true,
            _ = tokio::time::sleep(Duration::from_secs(2)) => false,
            _ = async { if let Some(watch) = &mut physical { watch.receiver.recv().await; } else { std::future::pending::<()>().await; } } => { tokio::time::sleep(Duration::from_millis(40)).await; false },
            _ = async {
                if let Some(receiver) = &mut hints {
                    loop { match receiver.recv().await { Ok(id) if Some(id) != repository => continue, _ => break } }
                } else { std::future::pending::<()>().await; }
            } => false,
        };
        let Some(state) = weak.upgrade() else {
            return;
        };
        let managed = state
            .event_monitors
            .0
            .lock()
            .await
            .get(&repository)
            .is_some_and(|e| e.managed);
        if leases.load(Ordering::Acquire) == 0 && !managed {
            let since = *idle.get_or_insert_with(Instant::now);
            if since.elapsed() >= Duration::from_secs(15) {
                let mut entries = state.event_monitors.0.lock().await;
                if leases.load(Ordering::Acquire) == 0 {
                    if let Some(entry) = entries.remove(&repository)
                        && let Some(task) = entry.producers
                    {
                        task.abort();
                    }
                    if let Some(id) = repository {
                        state.freshness.forget(id);
                    }
                    return;
                }
            }
        } else {
            idle = None;
        }
        if let Some(repository) = repository {
            let current = state
                .search
                .health(repository)
                .filter(|h| h.has_changed().is_ok());
            let changed = value_changed(&current, &observed_health);
            let value = current.as_ref().map(|h| h.borrow().clone());
            if value != observed_health {
                observed_health = value.clone();
                state.freshness.observe(Change {
                    repository: Some(repository),
                    domain: Domain::Search,
                    witness: format!("{value:?}"),
                    identity: None,
                    audience: None,
                });
            }
            if health
                .as_ref()
                .zip(current.as_ref())
                .is_none_or(|(a, b)| !a.same_channel(b))
            {
                health = current;
            }
            if health_wake && !changed {
                continue;
            }
        }
        let first = managed && !activated;
        if let Err(error) = observe(&state, repository, local, managed, first).await {
            if managed
                && error.invalidates_code_index()
                && let Some(repo) = repository
            {
                state.code_indexes.invalidate(repo);
            }
            if let Some(change) = observation_failure(repository, &error) {
                state.freshness.observe(change);
            }
        } else {
            activated |= managed;
        }
        if local {
            let current = repository_witness(&weak, repository);
            if current != witness {
                witness = current;
                physical = local_watch(&weak, repository).await;
            }
        }
    }
}
fn repository_witness(weak: &Weak<State>, repository: Option<m::RepositoryId>) -> Option<String> {
    weak.upgrade().and_then(|s| {
        s.freshness
            .snapshot()
            .1
            .into_iter()
            .find(|c| c.repository == repository && c.domain == Domain::Repository)
            .map(|c| c.witness)
    })
}
async fn local_watch(
    weak: &Weak<State>,
    repository: Option<m::RepositoryId>,
) -> Option<jj_store_service::local_browser::HintWatch> {
    let state = weak.upgrade()?;
    let manager = state.config.local_repositories.clone()?;
    let repository = repository?;
    tokio::task::spawn_blocking(move || manager.watch(repository))
        .await
        .ok()?
        .ok()
        .flatten()
}
async fn observe(
    state: &State,
    repository: Option<m::RepositoryId>,
    local: bool,
    managed: bool,
    first: bool,
) -> Result<()> {
    let mut local_heads = None;
    let changes = if let Some(repository) = repository {
        if local {
            let snapshot = crate::local_browser::observe(state, repository).await?;
            let generation = snapshot.code.generation;
            local_heads = Some(snapshot.code.clone());
            vec![
                change(
                    Some(repository),
                    Domain::Repository,
                    encode_witness(&generation)?,
                ),
                change(
                    Some(repository),
                    Domain::Code,
                    encode_witness(&snapshot.code)?,
                ),
                // The companion selects all local collaboration domains together.
                change(
                    Some(repository),
                    Domain::Work,
                    encode_witness(&(generation, snapshot.metadata_commit))?,
                ),
                change(
                    Some(repository),
                    Domain::Agents,
                    encode_witness(&(generation, snapshot.metadata_commit))?,
                ),
                change(
                    Some(repository),
                    Domain::Reviews,
                    encode_witness(&(generation, snapshot.metadata_commit))?,
                ),
                change(
                    Some(repository),
                    Domain::Threads,
                    encode_witness(&(generation, snapshot.metadata_commit))?,
                ),
            ]
        } else {
            state
                .host
                .as_ref()
                .ok_or_else(Error::unavailable)?
                .observe_freshness(repository)
                .await
                .map_err(reads::Error::from)?
        }
    } else {
        let registry = state
            .config
            .registry
            .as_ref()
            .ok_or_else(Error::unavailable)?;
        let snapshot = registry
            .snapshot(None)
            .await
            .map_err(|_| Error::unavailable())?;
        let mut changes = vec![change(
            None,
            Domain::Discovery,
            snapshot.generation.to_string(),
        )];
        if let Some(manager) = state.config.local_settings.clone()
            && let Some(witness) = tokio::task::spawn_blocking(move || manager.freshness_witness())
                .await
                .map_err(|_| Error::unavailable())?
        {
            changes.push(change(None, Domain::Configuration, witness));
        }
        if matches!(state.config.access, Access::LocalTrusted { .. }) {
            let mut workspace_witnesses = Vec::new();
            if let Some(manager) = state.config.local_workspaces.clone()
                && let Some(witness) =
                    tokio::task::spawn_blocking(move || manager.freshness_witness())
                        .await
                        .map_err(|_| Error::unavailable())?
            {
                workspace_witnesses.push(witness);
            }
            if let Some(manager) = state.config.local_repositories.clone() {
                let bindings = tokio::task::spawn_blocking(move || manager.list())
                    .await
                    .map_err(|_| Error::unavailable())?
                    .map_err(|_| Error::unavailable())?;
                workspace_witnesses.push(encode_witness(&bindings)?);
            }
            if !workspace_witnesses.is_empty() {
                changes.push(change(
                    None,
                    Domain::Workspaces,
                    encode_witness(&workspace_witnesses)?,
                ));
            }
        }
        changes
    };
    for change in changes {
        if managed
            && change.domain == Domain::Code
            && (first
                || !state.freshness.snapshot().1.iter().any(|old| {
                    old.repository == repository
                        && old.domain == Domain::Code
                        && old.witness == change.witness
                }))
        {
            super::code_indexes::maintain(state, repository, local, local_heads.take()).await?;
        }
        state.freshness.observe(change);
    }
    Ok(())
}
fn change(repository: Option<m::RepositoryId>, domain: Domain, witness: String) -> Change {
    Change {
        repository,
        domain,
        witness,
        identity: None,
        audience: None,
    }
}

fn encode_witness(value: &impl serde::Serialize) -> Result<String> {
    serde_json::to_string(value).map_err(|_| Error::unavailable())
}

fn value_changed(
    current: &Option<tokio::sync::watch::Receiver<jj_store_adapters::Health>>,
    previous: &Option<jj_store_adapters::Health>,
) -> bool {
    current.as_ref().map(|h| h.borrow().clone()) != *previous
}

fn observation_failure(repository: Option<m::RepositoryId>, error: &Error) -> Option<Change> {
    if error.is_observation_pressure() {
        return None;
    }
    Some(change(repository, Domain::Repository, "unavailable".into()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observation_pressure_does_not_invent_repository_changes() {
        let repository = Some(m::RepositoryId::new([7; 16]));
        for error in [Error::busy(), Error::timeout(), Error::limit()] {
            assert!(observation_failure(repository, &error).is_none());
            assert!(!error.invalidates_code_index());
        }
        for error in [Error::denied(), Error::unavailable()] {
            let notice = observation_failure(repository, &error).unwrap();
            assert_eq!(notice.repository, repository);
            assert_eq!(notice.domain, Domain::Repository);
            assert_eq!(notice.witness, "unavailable");
        }
        assert!(Error::denied().invalidates_code_index());
    }
}
