mod recovery;
use super::*;
use jj_lib::backend::CommitId;
pub use recovery::PendingPublication;
use std::io::{Read as _, Write as _};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PushIntent {
    binding: Binding,
    endpoint: String,
    branch: String,
    request_id: String,
    before: Option<String>,
    after: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PushOutcome {
    Prepared,
    Complete,
    Unknown,
    Stale,
    Stopped,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    version: u32,
    intent: PushIntent,
    outcome: PushOutcome,
}
pub(in crate::vault_workspace) fn request_id(id: &str) -> Result<()> {
    if id.is_empty()
        || id.len() > 128
        || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
    {
        return Err(Error::Binding);
    }
    Ok(())
}
pub(in crate::vault_workspace) fn save<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let data = serde_json::to_vec(value).map_err(jj)?;
    if data.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    let parent = path.parent().ok_or(Error::Binding)?;
    fs::create_dir_all(parent)?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(&data)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(jj)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
pub(in crate::vault_workspace) fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.is_symlink() || metadata.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    let mut data = Vec::new();
    File::open(path)?
        .take(512 * 1024 + 1)
        .read_to_end(&mut data)?;
    if data.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    serde_json::from_slice(&data).map_err(jj)
}
impl Session {
    /// Pin a current retained checkpoint and prove publication preserves ancestry.
    /// The transfer records the exact intent durably before starting a child.
    pub async fn prepare_push(
        &self,
        remote: &RemoteTransfer,
        id: &str,
        before: Option<String>,
    ) -> Result<PushIntent> {
        self.binding.verify_saved()?;
        request_id(id)?;
        if self.binding != remote.binding {
            return Err(Error::Binding);
        }
        let checkpoint = self.checkpoint()?.ok_or(Error::Binding)?;
        let target = self.retained_commit(&checkpoint.commit).await?;
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
            if !self
                .repo
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
        Ok(PushIntent {
            binding: self.binding.clone(),
            endpoint: remote.endpoint.clone(),
            branch: remote.branch.clone(),
            request_id: id.into(),
            before,
            after: checkpoint.commit,
        })
    }
}
impl RemoteTransfer {
    async fn observes_publication(&self, target: &str) -> Result<bool> {
        let Some(fetched) = self.fetch().await? else {
            return Ok(false);
        };
        if fetched.commit == target {
            return Ok(true);
        }
        // Read ancestry in the fetched Git object database without a capture
        // lease or JJ mutation. A later remote descendant also proves acceptance.
        let (status, _) = self
            .run(&["merge-base", "--is-ancestor", target, &fetched.commit])
            .await?;
        match status {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => Err(Error::Jj(
                "Unable to observe the original backup push".into(),
            )),
        }
    }
    /// Reserve the exact device-local intent before releasing configuration
    /// serialization. No child or network work occurs here.
    pub fn reserve(&self, intent: &PushIntent) -> Result<()> {
        self.binding.verify_saved()?;
        request_id(&intent.request_id)?;
        if !intent.bound_to(&self.binding)
            || intent.endpoint != self.endpoint
            || intent.branch != self.branch
        {
            return Err(Error::Binding);
        }
        let directory = self.binding.state.join("remote-push-receipts");
        fs::create_dir_all(&directory)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(directory.join("publication.lock"))?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        let active = directory.join("active.json");
        if active.exists() {
            let original: PushIntent = read(&active)?;
            if original != *intent {
                return Err(Error::Busy);
            }
        }
        let path = directory.join(format!("{}.json", intent.request_id));
        if path.exists() {
            let receipt: Receipt = read(&path)?;
            if receipt.version != 1 || receipt.intent != *intent {
                return Err(Error::Binding);
            }
            if receipt.outcome.terminal() {
                return Ok(());
            }
        } else {
            save(
                &path,
                &Receipt {
                    version: 1,
                    intent: intent.clone(),
                    outcome: PushOutcome::Prepared,
                },
            )?;
        }
        save(&active, intent)
    }
    pub fn pending_push(&self) -> Result<Option<PushIntent>> {
        self.binding.verify_saved()?;
        let path = self.binding.state.join("remote-push-receipts/active.json");
        if path.exists() {
            Ok(Some(read(&path)?))
        } else {
            Ok(None)
        }
    }
    /// An existing Unknown intent is observed only, never automatically replayed.
    pub async fn publish(&self, intent: &PushIntent) -> Result<PushOutcome> {
        self.binding.verify_saved()?;
        request_id(&intent.request_id)?;
        if !intent.bound_to(&self.binding)
            || intent.endpoint != self.endpoint
            || intent.branch != self.branch
        {
            return Err(Error::Binding);
        }
        if let Some(outcome) = self.finish_if_terminal(intent)? {
            return Ok(outcome);
        }
        // Deserializable receipts do not constitute publication authority.
        // Reprove the pinned target and ancestry through the native repository.
        if !oid(&intent.after) {
            return Err(Error::Binding);
        }
        let session = Session::open(self.binding.clone()).await?;
        let target = session.retained_commit(&intent.after).await?;
        if target.has_conflict() {
            return Err(Error::Binding);
        }
        if let Some(before) = &intent.before {
            if !oid(before) {
                return Err(Error::Binding);
            }
            let before = CommitId::try_from_hex(before).ok_or(Error::Binding)?;
            if !session
                .repo
                .index()
                .is_ancestor(&before, target.id())
                .await
                .map_err(jj)?
            {
                return Err(Error::Binding);
            }
        }
        drop(session);
        let directory = self.binding.state.join("remote-push-receipts");
        fs::create_dir_all(&directory)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(directory.join("publication.lock"))?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        let active = directory.join("active.json");
        if active.exists() {
            let original: PushIntent = read(&active)?;
            if original != *intent {
                request_id(&original.request_id)?;
                let prior_path = directory.join(format!("{}.json", original.request_id));
                let mut prior: Receipt = read(&prior_path)?;
                if prior.version != 1 || prior.intent != original {
                    return Err(Error::Binding);
                }
                if !original.bound_to(&self.binding)
                    || original.endpoint != self.endpoint
                    || original.branch != self.branch
                {
                    return Err(Error::Jj(
                        "An earlier backup push needs observation using its original remote".into(),
                    ));
                }
                if !prior.outcome.terminal() {
                    if !self.observes_publication(&original.after).await? {
                        return Ok(PushOutcome::Unknown);
                    }
                    prior.outcome = PushOutcome::Complete;
                    save(&prior_path, &prior)?;
                }
                fs::remove_file(&active)?;
            }
        }
        let path = directory.join(format!("{}.json", intent.request_id));
        let mut receipt: Receipt = if path.exists() {
            let receipt: Receipt = read(&path)?;
            if receipt.version != 1 || receipt.intent != *intent {
                return Err(Error::Binding);
            }
            receipt
        } else {
            let receipt = Receipt {
                version: 1,
                intent: intent.clone(),
                outcome: PushOutcome::Prepared,
            };
            save(&path, &receipt)?;
            receipt
        };
        if receipt.outcome.terminal() {
            recovery::clear_active(&active, intent)?;
            return Ok(receipt.outcome);
        }
        let observed = self.observe().await?;
        if observed.as_deref() == Some(&intent.after) {
            receipt.outcome = PushOutcome::Complete;
        } else if receipt.outcome == PushOutcome::Unknown {
            if !self.observes_publication(&intent.after).await? {
                return Ok(PushOutcome::Unknown);
            }
            receipt.outcome = PushOutcome::Complete;
        } else if observed != intent.before {
            receipt.outcome = PushOutcome::Stale;
        } else {
            receipt.outcome = PushOutcome::Unknown;
            save(&path, &receipt)?;
            save(&active, intent)?;
            let reference = self.reference();
            let lease = format!(
                "--force-with-lease={reference}:{}",
                intent.before.as_deref().unwrap_or("")
            );
            let update = format!("{}:{reference}", intent.after);
            let result = self
                .run(&[
                    "push",
                    "--atomic",
                    "--no-verify",
                    &lease,
                    "--",
                    &self.endpoint,
                    &update,
                ])
                .await;
            if matches!(result, Ok((Some(0), _)))
                || self.observe().await?.as_deref() == Some(&intent.after)
            {
                receipt.outcome = PushOutcome::Complete;
            }
        }
        save(&path, &receipt)?;
        if receipt.outcome.terminal() && active.exists() {
            let original: PushIntent = read(&active)?;
            if original == *intent {
                fs::remove_file(&active)?;
            }
        }
        Ok(receipt.outcome)
    }
}
