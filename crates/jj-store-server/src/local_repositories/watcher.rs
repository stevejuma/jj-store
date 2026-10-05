//! Physical hints only. The ordinary bounded reader validates committed selections.
use super::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
pub(super) fn watch(
    folder: &LocalFolder,
    admission: Option<Arc<admission::FolderAdmission>>,
) -> Result<HintWatch> {
    let (send, receiver) = tokio::sync::mpsc::channel(1);
    let mut watcher: RecommendedWatcher =
        notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            if event
                .as_ref()
                .is_ok_and(|e| matches!(e.kind, notify::EventKind::Access(_)))
            {
                return;
            }
            if let Some(admission) = &admission {
                admission.invalidate();
            }
            let _ = send.try_send(());
        })
        .map_err(|_| Error::Unavailable)?;
    let metadata = Path::new(&folder.metadata);
    let mut paths = vec![
        (metadata.to_path_buf(), RecursiveMode::NonRecursive),
        (metadata.join("op_heads"), RecursiveMode::NonRecursive),
    ];
    if let Some(parent) = metadata.parent() {
        paths.push((parent.into(), RecursiveMode::NonRecursive));
    }
    let local_git = metadata.join("store/git");
    let git = if local_git.is_dir() {
        Some(local_git)
    } else {
        crate::policy::read_regular(&local_git, 4096)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .map(|value| metadata.join("store").join(value.trim()))
    };
    if let Some(git) = git {
        paths.push((git.clone(), RecursiveMode::NonRecursive)); // packed refs and atomic replacements
        paths.push((git.join("refs/lapis"), RecursiveMode::Recursive));
        paths.push((git.join("refs"), RecursiveMode::NonRecursive)); // missing companion ref gets created here
    }
    let mut watched = false;
    for (path, mode) in paths {
        if path.is_dir() && watcher.watch(&path, mode).is_ok() {
            watched = true;
        }
    }
    if !watched {
        return Err(Error::Unavailable);
    }
    Ok(HintWatch {
        receiver,
        retained: Box::new(watcher),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn heads_and_atomic_packed_refs_replacement_send_hints_without_history_reads() {
        let root = tempfile::tempdir().unwrap();
        let metadata = root.path().join(".jj/repo");
        std::fs::create_dir_all(metadata.join("op_heads")).unwrap();
        let git = metadata.join("store/git");
        std::fs::create_dir_all(git.join("refs/lapis")).unwrap();
        let folder = LocalFolder {
            path: root.path().to_str().unwrap().into(),
            metadata: metadata.to_str().unwrap().into(),
            repository: None,
        };
        let admission = Arc::new(admission::FolderAdmission::default());
        admission
            .validate(&folder, 1, true, || Ok((folder.clone(), 1)))
            .unwrap();
        let mut watched = watch(&folder, Some(admission.clone())).unwrap();
        std::fs::write(metadata.join("op_heads/head"), b"operation").unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(3), watched.receiver.recv())
            .await
            .unwrap()
            .unwrap();
        assert!(!admission.cached(&folder, 1));
        while watched.receiver.try_recv().is_ok() {}
        std::fs::write(git.join("packed-refs.new"), b"committed-ref").unwrap();
        std::fs::rename(git.join("packed-refs.new"), git.join("packed-refs")).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(3), watched.receiver.recv())
            .await
            .unwrap()
            .unwrap();
    }
}
