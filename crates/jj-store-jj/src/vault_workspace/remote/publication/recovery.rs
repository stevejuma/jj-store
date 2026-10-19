//! Device-local recovery never claims an unknown push failed or was cancelled.
use super::*;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PendingPublication {
    pub request_id: String,
    pub before: Option<String>,
    pub after: String,
    pub outcome: PushOutcome,
}
impl PushIntent {
    // A physical-root relink does not change the vault's private repository.
    pub(super) fn bound_to(&self, binding: &Binding) -> bool {
        self.binding.vault_id == binding.vault_id && self.binding.state == binding.state
    }
}
impl PushOutcome {
    pub(super) fn terminal(&self) -> bool {
        matches!(self, Self::Complete | Self::Stale | Self::Stopped)
    }
}
fn summary(receipt: &Receipt) -> PendingPublication {
    PendingPublication {
        request_id: receipt.intent.request_id.clone(),
        before: receipt.intent.before.clone(),
        after: receipt.intent.after.clone(),
        outcome: receipt.outcome.clone(),
    }
}
fn validated(binding: &Binding, intent: &PushIntent) -> Result<Receipt> {
    request_id(&intent.request_id)?;
    if !intent.bound_to(binding)
        || !oid(&intent.after)
        || intent.before.as_ref().is_some_and(|id| !oid(id))
    {
        return Err(Error::Binding);
    }
    let receipt: Receipt = read(
        &binding
            .state
            .join("remote-push-receipts")
            .join(format!("{}.json", intent.request_id)),
    )?;
    if receipt.version != 1 || receipt.intent != *intent {
        return Err(Error::Binding);
    }
    Ok(receipt)
}
fn lock(directory: &Path) -> Result<File> {
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(directory.join("publication.lock"))?;
    lock.try_lock().map_err(|_| Error::Busy)?;
    Ok(lock)
}
pub(super) fn clear_active(active: &Path, intent: &PushIntent) -> Result<()> {
    if active.exists() {
        let selected: PushIntent = read(active)?;
        if selected == *intent {
            fs::remove_file(active)?;
            File::open(active.parent().ok_or(Error::Binding)?)?.sync_all()?;
        }
    }
    Ok(())
}
impl Binding {
    /// A renderer-safe summary, with no filesystem paths or credentials.
    pub fn pending_publication(&self) -> Result<Option<PendingPublication>> {
        self.validate()?;
        let active = self.state.join("remote-push-receipts/active.json");
        let intent: PushIntent = match read(&active) {
            Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            value => value?,
        };
        self.verify_saved()?;
        Ok(Some(summary(&validated(self, &intent)?)))
    }
    /// Explicitly stop tracking one original request. A named receipt survives;
    /// this neither contacts the remote nor undoes a possibly accepted push.
    pub fn stop_publication(&self, id: &str) -> Result<PendingPublication> {
        self.verify_saved()?;
        request_id(id)?;
        let directory = self.state.join("remote-push-receipts");
        let _lock = lock(&directory)?;
        let path = directory.join(format!("{id}.json"));
        let receipt: Receipt = read(&path)?;
        let mut receipt = validated(self, &receipt.intent)?;
        if receipt.intent.request_id != id {
            return Err(Error::Binding);
        }
        let active = directory.join("active.json");
        if !receipt.outcome.terminal() {
            let selected: PushIntent = read(&active)?;
            if selected != receipt.intent {
                return Err(Error::Binding);
            }
            receipt.outcome = PushOutcome::Stopped;
            save(&path, &receipt)?;
        }
        clear_active(&active, &receipt.intent)?;
        Ok(summary(&receipt))
    }
}
impl RemoteTransfer {
    /// Observe only. Even a prepared original request is never sent by this API.
    pub async fn reconcile(&self, id: &str) -> Result<PushOutcome> {
        self.binding.verify_saved()?;
        request_id(id)?;
        let directory = self.binding.state.join("remote-push-receipts");
        let _lock = lock(&directory)?;
        let path = directory.join(format!("{id}.json"));
        let original: Receipt = read(&path)?;
        let mut receipt = validated(&self.binding, &original.intent)?;
        if receipt.intent.request_id != id
            || receipt.intent.endpoint != self.endpoint
            || receipt.intent.branch != self.branch
        {
            return Err(Error::Binding);
        }
        let active = directory.join("active.json");
        if receipt.outcome.terminal() {
            clear_active(&active, &receipt.intent)?;
            return Ok(receipt.outcome);
        }
        let selected: PushIntent = read(&active)?;
        if selected != receipt.intent {
            return Err(Error::Binding);
        }
        if self.observes_publication(&receipt.intent.after).await? {
            receipt.outcome = PushOutcome::Complete;
            save(&path, &receipt)?;
            clear_active(&active, &receipt.intent)?;
        }
        Ok(receipt.outcome)
    }
    pub(super) fn finish_if_terminal(&self, intent: &PushIntent) -> Result<Option<PushOutcome>> {
        let directory = self.binding.state.join("remote-push-receipts");
        if !directory
            .join(format!("{}.json", intent.request_id))
            .exists()
        {
            return Ok(None);
        }
        let _lock = lock(&directory)?;
        let receipt = validated(&self.binding, intent)?;
        if !receipt.outcome.terminal() {
            return Ok(None);
        }
        clear_active(&directory.join("active.json"), intent)?;
        Ok(Some(receipt.outcome))
    }
}
