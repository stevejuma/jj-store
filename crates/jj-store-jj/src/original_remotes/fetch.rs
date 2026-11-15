use super::*;
use gix::refs::{
    Target,
    transaction::{Change, LogChange, PreviousValue, RefEdit},
};
use jj_lib::{backend::CommitId, git::GitImportOptions};
use std::collections::HashMap;

impl OriginalRepository {
    /// Uses the independent admitted transfer cache. No snapshot, checkout or file capture.
    pub async fn fetch(&mut self, name: &str, transfer: &RemoteTransfer) -> Result<Snapshot> {
        self.current().await?;
        self.validate_transport(name, transfer)?;
        let target = transfer.native_target();
        let reference = format!("refs/remotes/{name}/{}", target.branch);
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        let database = backend.git_repo();
        let before = database
            .try_find_reference(&reference)
            .map_err(|_| Error::Native)?
            .map(|reference| reference.inner.target.clone());
        let fetched = transfer.fetch().await.map_err(|_| Error::Transport)?;
        self.current().await?;
        self.validate_transport(name, transfer)?;
        if let Some(fetched) = &fetched {
            let cache = gix::open(transfer.native_git_path()).map_err(|_| Error::Native)?;
            objects::copy(&cache, &database, fetched.commit())?;
            let id = CommitId::try_from_hex(fetched.commit()).ok_or(Error::Binding)?;
            backend
                .import_head_commits([&id])
                .map_err(|_| Error::Native)?;
        }
        self.current().await?;
        self.validate_transport(name, transfer)?;
        let change = match (&before, &fetched) {
            (_, Some(fetched)) => Some(Change::Update {
                log: LogChange::default(),
                expected: before.map_or(
                    PreviousValue::MustNotExist,
                    PreviousValue::MustExistAndMatch,
                ),
                new: Target::Object(
                    gix::ObjectId::from_hex(fetched.commit().as_bytes())
                        .map_err(|_| Error::Binding)?,
                ),
            }),
            (Some(before), None) => Some(Change::Delete {
                expected: PreviousValue::MustExistAndMatch(before.clone()),
                log: gix::refs::transaction::RefLog::AndReference,
            }),
            (None, None) => None,
        };
        if let Some(change) = change {
            database
                .edit_references([RefEdit {
                    name: reference.try_into().map_err(|_| Error::Binding)?,
                    change,
                    deref: false,
                }])
                .map_err(|_| Error::Stale)?;
        }
        let mut transaction = self.repo.start_transaction();
        let options = GitImportOptions {
            abandon_unreachable_commits: false,
            record_synthetic_predecessors: false,
            remote_auto_track_bookmarks: HashMap::new(),
        };
        git::import_some_refs(transaction.repo_mut(), &options, |kind, symbol| {
            kind == git::GitRefKind::Bookmark
                && symbol.remote.as_str() == name
                && symbol.name.as_str() == target.branch
        })
        .await
        .map_err(|_| Error::Native)?;
        self.current().await?;
        if transaction.repo_mut().has_changes() {
            self.repo = transaction
                .commit(format!(
                    "fetch original remote {name} branch {}",
                    target.branch
                ))
                .await
                .map_err(|_| Error::Native)?;
        }
        self.authorize_mutation().await?;
        self.snapshot().await
    }
}
