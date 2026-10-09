//! Read-free physical hints for already authorized, application-owned workspaces.
use super::*;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub(in crate::agent_execution) struct Watch {
    path: PathBuf,
    metadata: PathBuf,
    _watcher: RecommendedWatcher,
}
impl Supervisor {
    pub(super) fn watch_inspected_workspace(
        &self,
        session: &str,
        repository: RepositoryId,
        workspace: &Workspace,
    ) -> Result<()> {
        let mut watches = self
            .0
            .inspection_watches
            .lock()
            .map_err(|_| Error::Unavailable)?;
        let key = (session.to_owned(), repository);
        if watches
            .get(&key)
            .is_some_and(|w| w.path == workspace.path && w.metadata == workspace.metadata)
        {
            return Ok(());
        }
        if watches.len() >= 128 && !watches.contains_key(&key) {
            return Err(Error::Limit);
        }
        let watcher = watch(
            &workspace.path,
            &workspace.metadata,
            self.0.freshness.clone(),
            session.to_owned(),
            repository,
            self.0.resources.actor,
        )?;
        watches.insert(key, watcher);
        Ok(())
    }
}
fn watch(
    root: &Path,
    metadata: &Path,
    signals: jj_store_service::freshness::Signals,
    session: String,
    repository: RepositoryId,
    actor: PrincipalId,
) -> Result<Watch> {
    let physical_root = root.to_owned();
    let mut watcher = RecommendedWatcher::new(
        move |event: notify::Result<notify::Event>| {
            if event.as_ref().is_ok_and(|e| {
                matches!(e.kind, EventKind::Access(_))
                    || (!e.need_rescan()
                        && !e.paths.is_empty()
                        && e.paths.iter().all(|p| {
                            p.strip_prefix(&physical_root).ok().is_some_and(|p| {
                                p.starts_with(".jj") && p != Path::new(".jj/working_copy/checkout")
                            })
                        }))
            }) {
                return;
            }
            signals.notify_to(
                Some(repository),
                jj_store_service::freshness::Domain::Workspaces,
                Some(session.clone()),
                Some(actor),
            );
        },
        Config::default().with_follow_symlinks(false),
    )
    .map_err(|_| Error::Unavailable)?;
    watcher
        .watch(root, RecursiveMode::Recursive)
        .map_err(|_| Error::Unavailable)?;
    watcher
        .watch(&metadata.join("op_heads"), RecursiveMode::NonRecursive)
        .map_err(|_| Error::Unavailable)?;
    Ok(Watch {
        path: root.into(),
        metadata: metadata.into(),
        _watcher: watcher,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn disk_edits_emit_scoped_events_without_polling_or_source_reads() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("work");
        let metadata = temp.path().join("metadata");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(metadata.join("op_heads")).unwrap();
        let signals = jj_store_service::freshness::Signals::default();
        let mut changes = signals.subscribe();
        let repository = RepositoryId::new([1; 16]);
        let actor = PrincipalId::new([2; 16]);
        let watcher = watch(
            &root,
            &metadata,
            signals,
            "owned-session".into(),
            repository,
            actor,
        )
        .unwrap();
        fs::write(root.join("unchanged.txt"), "café 🦀").unwrap();
        let change = tokio::time::timeout(Duration::from_secs(3), changes.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(change.repository, Some(repository));
        assert_eq!(change.identity.as_deref(), Some("owned-session"));
        assert_eq!(change.audience, Some(actor));
        assert_eq!(
            change.domain,
            jj_store_service::freshness::Domain::Workspaces
        );
        drop(watcher);
    }
}
