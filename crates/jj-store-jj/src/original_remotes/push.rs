//! One explicit native bookmark review, never an implicit snapshot or all-bookmark push.
use super::*;
use crate::vault_workspace::{PushIntent, PushOutcome};
use jj_lib::ref_name::RefName;
use serde::Deserialize;

/// Native-private review. Hosts expose only an opaque ID and a sanitized summary.
/// Deserialization alone grants no execution authority; every field is rechecked.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PushReview {
    operation: String,
    configuration: String,
    name: String,
    bookmark: String,
    commit: String,
    target: RemoteTarget,
    intent: PushIntent,
}
impl PushReview {
    pub fn commit(&self) -> &str {
        &self.commit
    }
    pub fn bookmark(&self) -> &str {
        &self.bookmark
    }
}
impl OriginalRepository {
    pub async fn review_push(
        &self,
        name: &str,
        bookmark: &str,
        request_id: &str,
        transfer: &RemoteTransfer,
    ) -> Result<PushReview> {
        self.current().await?;
        self.validate_push_transport(name, transfer)?;
        let selected = self.repo.view().get_local_bookmark(RefName::new(bookmark));
        let commit = selected.as_normal().ok_or(Error::Binding)?;
        let native = self
            .repo
            .store()
            .get_commit(commit)
            .map_err(|_| Error::Native)?;
        if native.has_conflict() {
            return Err(Error::Unsupported);
        }
        let before = transfer.fetch().await.map_err(|_| Error::Transport)?;
        self.current().await?;
        self.validate_push_transport(name, transfer)?;
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        let cache = gix::open(transfer.native_git_path()).map_err(|_| Error::Native)?;
        objects::copy(&backend.git_repo(), &cache, &commit.hex())?;
        let intent = transfer
            .prepare_original_commit(
                request_id,
                &commit.hex(),
                before.map(|head| head.commit().into()),
            )
            .await
            .map_err(|_| Error::Transport)?
            .ok_or(Error::Diverged)?;
        self.current().await?;
        self.validate_push_transport(name, transfer)?;
        Ok(PushReview {
            operation: self.repo.op_id().hex(),
            configuration: self.configuration_revision()?,
            name: name.into(),
            bookmark: bookmark.into(),
            commit: commit.hex(),
            target: transfer.native_target(),
            intent,
        })
    }
    pub async fn push(
        &self,
        review: &PushReview,
        transfer: &RemoteTransfer,
    ) -> Result<PushOutcome> {
        self.current().await?;
        self.validate_push_transport(&review.name, transfer)?;
        if !review
            .intent
            .matches_original_selection(transfer, &review.commit)
            || self.repo.op_id().hex() != review.operation
            || self.configuration_revision()? != review.configuration
            || transfer.native_target() != review.target
            || self
                .repo
                .view()
                .get_local_bookmark(RefName::new(&review.bookmark))
                .as_normal()
                .map(|id| id.hex())
                .as_deref()
                != Some(&review.commit)
        {
            return Err(Error::Stale);
        }
        // Reserve before network. A retained unknown request can only be observed.
        transfer
            .reserve(&review.intent)
            .map_err(|_| Error::Transport)?;
        transfer
            .publish(&review.intent)
            .await
            .map_err(|_| Error::Transport)
    }
}
