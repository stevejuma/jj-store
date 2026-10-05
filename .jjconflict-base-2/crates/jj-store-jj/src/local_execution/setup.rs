//! Initial real merge/WIP generation; never automatic setup or publication.
pub(super) mod report;
use super::*;
use crate::local_evidence::{self, GitEvidenceCapsuleV1, GitRefTargetV1, OperationArchiveV1};
use jj_lib::{
    backend::CommitId,
    default_backend_factories::default_backend_factories,
    object_id::ObjectId as _,
    op_store::OperationId,
    ref_name::{WorkspaceName, WorkspaceNameBuf},
    repo::{Repo as _, RepoLoader},
};
use jj_store_model as m;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetupBookmark {
    pub name: String,
    pub remote: Option<String>,
    pub head: m::GitSha1Id,
}
#[derive(Clone, Debug)]
pub struct Setup {
    pub principal: m::PrincipalId,
    pub target: SetupBookmark,
}
#[derive(Clone, Debug)]
pub struct PreparedSetup {
    pub source: GitEvidenceCapsuleV1,
    pub capsule: GitEvidenceCapsuleV1,
    pub history: OperationArchiveV1,
    pub target: SetupBookmark,
    pub merge: m::GitSha1Id,
    pub wip: m::GitSha1Id,
}
impl Foreground {
    pub async fn prepare_setup(
        self,
        request: Setup,
        physical: Arc<dyn PhysicalGuard>,
        expected: m::RecordDigest,
    ) -> Result<(Self, PreparedSetup)> {
        if request.principal.as_bytes() == &[0; 16] || expected.as_bytes() == &[0; 32] {
            return Err(Error::Invalid);
        }
        for (text, limit) in std::iter::once((&request.target.name, 1024))
            .chain(request.target.remote.iter().map(|text| (text, 256)))
        {
            if text.trim().is_empty() || text.len() > limit || text.chars().any(char::is_control) {
                return Err(Error::Invalid);
            }
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let prepare = async {
                    self.validate_metadata().await?;
                    if self.expected.heads != [self.expected.baseline.operation] { return Err(Error::EpochChanged); }
                    let binding = RecoveryBinding {
                        repository: self.expected.baseline.repository, generation: self.expected.baseline.generation,
                        physical_workspace: self.expected.physical_workspace, operation: self.expected.baseline.operation,
                        view: self.expected.baseline.view, commit: self.expected.baseline.commit,
                        workspace: self.expected.checkout.workspace.clone(),
                    };
                    physical.validate(&binding, expected).await?;
                    let prepared = prepare(&self, request).await?;
                    self.validate_metadata().await?;
                    physical.validate(&binding, expected).await?;
                    self.validate_metadata().await?;
                    Ok((self, prepared))
                };
                tokio::select! { biased;
                    _ = cancellation.cancelled() => Err(Error::Canceled),
                    value = tokio::time::timeout(Duration::from_secs(30), prepare) => value.map_err(|_| Error::Limit)?,
                }
            }))
        }).await.map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
pub(crate) fn target(source: &GitEvidenceCapsuleV1, target: &SetupBookmark) -> Result<()> {
    let selected = if let Some(remote) = &target.remote {
        source
            .view
            .remote_views
            .get(remote)
            .and_then(|view| view.bookmarks.get(&target.name))
            .map(|r| &r.target)
    } else {
        source.view.local_bookmarks.get(&target.name)
    };
    if selected != Some(&GitRefTargetV1(vec![Some(target.head)])) {
        return Err(Error::EpochChanged);
    }
    if !source.commits.contains_key(&target.head) {
        return Err(Error::NotCommitted);
    }
    Ok(())
}
async fn prepare(lease: &Foreground, request: Setup) -> Result<PreparedSetup> {
    let settings = super::reword::settings(request.principal)?;
    let path = lease.access.metadata_path();
    let source_history = local_evidence::capture_operation_archive(
        &settings,
        path,
        lease.access.repository(),
        lease.expected.baseline.operation,
        Default::default(),
    )
    .await
    .map_err(super::reword::archive_error)?;
    let source = source_history
        .capsule()
        .map_err(super::reword::archive_error)?;
    if source.commits.len() > 4094 || source.view.local_bookmarks.len() > 1024 {
        return Err(Error::Limit);
    }
    if source.view_id != lease.expected.baseline.view
        || source
            .view
            .wc_commit_ids
            .get(&lease.expected.checkout.workspace)
            != Some(&lease.expected.baseline.commit)
    {
        return Err(Error::EpochChanged);
    }
    target(&source, &request.target)?;
    for id in [lease.expected.baseline.commit, request.target.head] {
        let commit = source.commits.get(&id).ok_or(Error::Incomplete)?;
        if commit.root_tree.len() != 1 || commit.conflict_labels != [String::new()] {
            return Err(Error::Invalid);
        }
    }
    lease.validate_metadata().await?;
    let loader = RepoLoader::init_from_file_system(&settings, path, &default_backend_factories())
        .map_err(|_| Error::Unavailable)?;
    let operation = loader
        .load_operation(&OperationId::from_bytes(source.operation_id.as_bytes()))
        .await
        .map_err(|_| Error::Incomplete)?;
    let repo = loader
        .load_at(&operation)
        .await
        .map_err(|_| Error::Incomplete)?;
    let base = repo
        .store()
        .get_commit(&CommitId::from_bytes(request.target.head.as_bytes()))
        .map_err(|_| Error::Incomplete)?;
    let old = repo
        .store()
        .get_commit(&CommitId::from_bytes(
            lease.expected.baseline.commit.as_bytes(),
        ))
        .map_err(|_| Error::Incomplete)?;
    let mut tx = repo.start_transaction();
    tx.set_workspace_name(WorkspaceName::new(&lease.expected.checkout.workspace));
    tx.set_attribute(
        "lapis.stacks.principal".into(),
        request.principal.to_string(),
    );
    tx.set_attribute("lapis.stacks.command".into(), "setup".into());
    let merge = tx
        .repo_mut()
        .new_commit(vec![base.id().clone()], base.tree())
        .set_description("Stack workspace merge")
        .write()
        .await
        .map_err(|_| Error::Unavailable)?;
    let wip = tx
        .repo_mut()
        .new_commit(vec![merge.id().clone()], old.tree())
        .set_description("Stack workspace unfinished work")
        .write()
        .await
        .map_err(|_| Error::Unavailable)?;
    tx.repo_mut()
        .set_wc_commit(
            WorkspaceNameBuf::from(lease.expected.checkout.workspace.as_str()),
            wip.id().clone(),
        )
        .map_err(|_| Error::Unavailable)?;
    let merge = m::GitSha1Id::from_slice(merge.id().as_bytes()).map_err(|_| Error::Corrupt)?;
    let wip = m::GitSha1Id::from_slice(wip.id().as_bytes()).map_err(|_| Error::Corrupt)?;
    let candidate = tx
        .write("Prepare Stacks setup")
        .await
        .map_err(|_| Error::Unavailable)?
        .leave_unpublished();
    let operation = m::OperationId::from_slice(candidate.operation().id().as_bytes())
        .map_err(|_| Error::Corrupt)?;
    let history = local_evidence::capture_operation_archive(
        &settings,
        path,
        lease.access.repository(),
        operation,
        Default::default(),
    )
    .await
    .map_err(super::reword::archive_error)?;
    report::build(
        source_history,
        history,
        request.target,
        merge,
        wip,
        &lease.expected,
    )
}
