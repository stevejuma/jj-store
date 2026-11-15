//! Exact immutable checkpoint review, independent of the capture writer lease.
use super::*;
impl Session {
    pub async fn prepare_push(
        &self,
        remote: &RemoteTransfer,
        id: &str,
        before: Option<String>,
    ) -> Result<PushIntent> {
        prepare(
            &self.binding,
            &self.repo,
            self.checkpoint()?,
            remote,
            id,
            before,
        )
        .await
    }
}
pub(in crate::vault_workspace) async fn prepare(
    binding: &Binding,
    repo: &ReadonlyRepo,
    checkpoint: Option<Checkpoint>,
    remote: &RemoteTransfer,
    id: &str,
    before: Option<String>,
) -> Result<PushIntent> {
    binding.verify_saved()?;
    request_id(id)?;
    if *binding != remote.binding {
        return Err(Error::Binding);
    }
    let checkpoint = checkpoint.ok_or(Error::Binding)?;
    let target = content::retained_commit(binding, repo, &checkpoint.commit).await?;
    if target.has_conflict() {
        return Err(Error::Jj(
            "Resolve backup conflicts before publishing".into(),
        ));
    }
    if let Some(before) = &before {
        if !oid(before) {
            return Err(Error::Binding);
        }
        let before = CommitId::try_from_hex(before).ok_or(Error::Binding)?;
        if !repo
            .index()
            .is_ancestor(&before, target.id())
            .await
            .map_err(jj)?
        {
            return Err(Error::Jj(
                "Fetch and merge the remote backup before publishing".into(),
            ));
        }
    }
    binding.verify_saved()?;
    Ok(PushIntent {
        binding: binding.clone(),
        endpoint: remote.endpoint.clone(),
        branch: remote.branch.clone(),
        request_id: id.into(),
        before,
        after: checkpoint.commit,
    })
}
impl Session {
    pub async fn push_divergence(&self, intent: &PushIntent) -> Result<CommitDivergence> {
        divergence(&self.binding, &self.repo, intent).await
    }
}
pub(in crate::vault_workspace) async fn divergence(
    binding: &Binding,
    repo: &ReadonlyRepo,
    intent: &PushIntent,
) -> Result<CommitDivergence> {
    binding.verify_saved()?;
    if !intent.bound_to(binding) {
        return Err(Error::Binding);
    }
    let local = content::retained_commit(binding, repo, &intent.after).await?;
    let remote = match &intent.before {
        Some(id) => Some(content::retained_commit(binding, repo, id).await?),
        None => None,
    };
    let counts = graph_divergence(repo, local.id(), remote.as_ref().map(|c| c.id())).await?;
    binding.verify_saved()?;
    Ok(counts)
}

pub(in crate::vault_workspace) async fn validate(
    binding: &Binding,
    repo: &ReadonlyRepo,
    intent: &PushIntent,
) -> Result<()> {
    binding.verify_saved()?;
    if !intent.bound_to(binding) || !oid(&intent.after) {
        return Err(Error::Binding);
    }
    let target = content::retained_commit(binding, repo, &intent.after).await?;
    if target.has_conflict() {
        return Err(Error::Binding);
    }
    if let Some(before) = &intent.before {
        if !oid(before) {
            return Err(Error::Binding);
        }
        let before = CommitId::try_from_hex(before).ok_or(Error::Binding)?;
        if !repo
            .index()
            .is_ancestor(&before, target.id())
            .await
            .map_err(jj)?
        {
            return Err(Error::Binding);
        }
    }
    binding.verify_saved()
}
