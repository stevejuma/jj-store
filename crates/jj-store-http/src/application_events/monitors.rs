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
        if observe(state, repository, local).await.is_err() {
            state.freshness.observe(Change {
                repository,
                domain: Domain::Repository,
                witness: "unavailable".into(),
                identity: None,
                audience: None,
            });
        }
        let leases = Arc::new(AtomicUsize::new(1));
        let weak = Arc::downgrade(state);
        let retained = leases.clone();
        let task = tokio::spawn(run(weak, repository, local, retained));
        let producers = repository.is_none().then(|| super::producers::start(state));
        entries.insert(
            repository,
            Entry {
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
        if leases.load(Ordering::Acquire) == 0 {
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
        if observe(&state, repository, local).await.is_err() {
            state.freshness.observe(Change {
                repository,
                domain: Domain::Repository,
                witness: "unavailable".into(),
                identity: None,
                audience: None,
            });
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
async fn observe(state: &State, repository: Option<m::RepositoryId>, local: bool) -> Result<()> {
    let changes = if let Some(repository) = repository {
        if local {
            let snapshot = crate::local_browser::capture(state, repository).await?;
            let generation = snapshot.code.generation;
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
