//! Real unpublished reword candidates, separate from selection/publication.
mod report;
use super::*;
use crate::local_evidence::{self, GitEvidenceCapsuleV1, GitRefTargetV1, OperationArchiveV1};
use jj_lib::{
    backend::CommitId,
    config::{ConfigLayer, ConfigSource, StackedConfig},
    default_backend_factories::default_backend_factories,
    object_id::ObjectId as _,
    op_store::OperationId,
    repo::{Repo as _, RepoLoader},
    settings::UserSettings,
};
use jj_store_model as m;
use std::{collections::BTreeSet, time::Duration};
const TIMEOUT: Duration = Duration::from_secs(30);
const MAX_COMMITS: usize = 4096;
const MAX_BOOKMARKS: usize = 1024;

#[derive(Clone, Debug)]
pub struct Reword {
    /// Composition supplies the authenticated principal and command policy.
    pub principal: m::PrincipalId,
    pub commit: m::GitSha1Id,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rewrite {
    pub change: m::ChangeId,
    pub before: Vec<m::GitSha1Id>,
    pub after: Vec<m::GitSha1Id>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BookmarkEffect {
    pub name: String,
    pub before: GitRefTargetV1,
    pub after: GitRefTargetV1,
}
#[derive(Clone, Debug)]
pub struct PreparedCode {
    pub source: GitEvidenceCapsuleV1,
    pub capsule: GitEvidenceCapsuleV1,
    pub history: OperationArchiveV1,
    pub rewrites: Vec<Rewrite>,
    pub bookmarks: Vec<BookmarkEffect>,
    pub checkout_before: Vec<m::GitSha1Id>,
    pub checkout_after: Vec<m::GitSha1Id>,
}
impl Foreground {
    /// Consumes and returns the lease so cancellation cannot release the actual
    /// foreground lock while a background candidate writer is still running.
    /// A historical request must be looked up by composition before calling this.
    pub async fn prepare_reword(self, request: Reword) -> Result<(Self, PreparedCode)> {
        if request.message.len() > 65536 {
            return Err(Error::Limit);
        }
        if request.message.contains('\0') || request.commit == m::GitSha1Id::new([0; 20]) {
            return Err(Error::Invalid);
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let prepare = async {
                    self.validate_metadata().await?;
                    if self.expected.heads != [self.expected.baseline.operation] {
                        return Err(Error::EpochChanged);
                    }
                    let result = prepare(&self, request).await?;
                    self.validate_metadata().await?;
                    Ok((self, result))
                };
                tokio::select! { biased;
                    _ = cancellation.cancelled() => Err(Error::Canceled),
                    value = tokio::time::timeout(TIMEOUT, prepare) => value.map_err(|_| Error::Limit)?,
                }
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
fn settings(principal: m::PrincipalId) -> Result<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    let mut layer = ConfigLayer::empty(ConfigSource::CommandArg);
    for (key, value) in [
        ("user.name", principal.to_string()),
        ("user.email", format!("{principal}@jj-store.invalid")),
        ("operation.username", principal.to_string()),
        ("operation.hostname", "jj-store".into()),
        ("signing.behavior", "drop".into()),
    ] {
        layer.set_value(key, value).map_err(|_| Error::Invalid)?;
    }
    config.add_layer(layer);
    UserSettings::from_config(config).map_err(|_| Error::Invalid)
}
async fn prepare(lease: &Foreground, request: Reword) -> Result<PreparedCode> {
    let settings = settings(request.principal)?;
    let path = lease.access.metadata_path();
    let source_history = local_evidence::capture_operation_archive(
        &settings,
        path,
        lease.access.repository(),
        lease.expected.baseline.operation,
        Default::default(),
    )
    .await
    .map_err(archive_error)?;
    let source = source_history.capsule().map_err(archive_error)?;
    if source.commits.len() > MAX_COMMITS || source.view.local_bookmarks.len() > MAX_BOOKMARKS {
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
    let visible = visible(&source)?;
    if !visible.contains(&request.commit) {
        return Err(Error::NotCommitted);
    }
    let affected = affected(&source, &visible, request.commit)?;
    // Every affected descendant is checked before candidate writes.
    // Keeping this first profile unsigned and resolved makes tree invariance
    // explicit; wider signing/conflict policies remain separate qualification.
    for id in &affected {
        let commit = source.commits.get(id).ok_or(Error::Incomplete)?;
        if commit.secure_sig.is_some() || commit.root_tree.len() != 1 {
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
    let commit = repo
        .store()
        .get_commit(&CommitId::from_bytes(request.commit.as_bytes()))
        .map_err(|_| Error::Incomplete)?;
    let mut tx = repo.start_transaction();
    tx.set_workspace_name(jj_lib::ref_name::WorkspaceName::new(
        &lease.expected.checkout.workspace,
    ));
    tx.set_attribute(
        "lapis.stacks.principal".into(),
        request.principal.to_string(),
    );
    tx.repo_mut()
        .rewrite_commit(&commit)
        .set_description(request.message.clone())
        .write()
        .await
        .map_err(|_| Error::Unavailable)?;
    tx.repo_mut()
        .rebase_descendants()
        .await
        .map_err(|_| Error::Unavailable)?;
    let candidate = tx
        .write("Prepare Stacks reword")
        .await
        .map_err(|_| Error::Unavailable)?
        .leave_unpublished();
    let id = m::OperationId::from_slice(candidate.operation().id().as_bytes())
        .map_err(|_| Error::Corrupt)?;
    let history = local_evidence::capture_operation_archive(
        &settings,
        path,
        lease.access.repository(),
        id,
        Default::default(),
    )
    .await
    .map_err(archive_error)?;
    if source_history
        .operations
        .iter()
        .any(|(id, data)| history.operations.get(id) != Some(data))
        || source_history
            .views
            .iter()
            .any(|(id, data)| history.views.get(id) != Some(data))
        || source_history
            .commits
            .iter()
            .any(|(id, data)| history.commits.get(id) != Some(data))
    {
        return Err(Error::Corrupt);
    }
    let capsule = history.capsule().map_err(archive_error)?;
    report::build(
        source,
        capsule,
        history,
        &lease.expected,
        &request,
        &affected,
    )
}
fn archive_error(error: crate::local_metadata::Error) -> Error {
    match error {
        crate::local_metadata::Error::Limit => Error::Limit,
        _ => Error::Incomplete,
    }
}
fn visible(source: &GitEvidenceCapsuleV1) -> Result<BTreeSet<m::GitSha1Id>> {
    let mut pending: BTreeSet<_> = source.view.head_ids.iter().copied().collect();
    let mut visible = BTreeSet::new();
    while let Some(id) = pending.pop_first() {
        if !visible.insert(id) {
            continue;
        }
        if visible.len() > MAX_COMMITS {
            return Err(Error::Limit);
        }
        pending.extend(
            source
                .commits
                .get(&id)
                .ok_or(Error::Incomplete)?
                .parents
                .iter()
                .copied(),
        );
    }
    Ok(visible)
}

fn affected(
    source: &GitEvidenceCapsuleV1,
    visible: &BTreeSet<m::GitSha1Id>,
    target: m::GitSha1Id,
) -> Result<BTreeSet<m::GitSha1Id>> {
    let mut children: std::collections::BTreeMap<m::GitSha1Id, BTreeSet<m::GitSha1Id>> =
        std::collections::BTreeMap::new();
    for id in visible {
        for parent in &source.commits.get(id).ok_or(Error::Incomplete)?.parents {
            children.entry(*parent).or_default().insert(*id);
        }
    }
    let mut pending = BTreeSet::from([target]);
    let mut affected = BTreeSet::new();
    while let Some(id) = pending.pop_first() {
        if affected.insert(id) {
            pending.extend(children.get(&id).into_iter().flatten().copied());
        }
    }
    Ok(affected)
}
