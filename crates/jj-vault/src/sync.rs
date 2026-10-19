use super::*;
pub(super) enum Request {
    Prepare(String, oneshot::Sender<Result<SyncProposal, String>>),
    Preview(String, oneshot::Sender<Result<VaultRestorePreview, String>>),
    Apply(
        VaultRestoreRequest,
        oneshot::Sender<Result<Capture, String>>,
    ),
    Conflict(
        String,
        String,
        oneshot::Sender<Result<SyncConflict, String>>,
    ),
    Resolve(
        String,
        String,
        SyncResolution,
        oneshot::Sender<Result<Checkpoint, String>>,
    ),
}
impl Backup {
    pub async fn prepare_sync(&self, incoming: String) -> Result<SyncProposal, String> {
        self.sync_request(|reply| Request::Prepare(incoming, reply))
            .await
    }
    pub async fn preview_sync(&self, candidate: String) -> Result<VaultRestorePreview, String> {
        self.sync_request(|reply| Request::Preview(candidate, reply))
            .await
    }
    pub async fn apply_sync(&self, request: VaultRestoreRequest) -> Result<Capture, String> {
        self.sync_request(|reply| Request::Apply(request, reply))
            .await
    }
    pub async fn sync_conflict(
        &self,
        candidate: String,
        path: String,
    ) -> Result<SyncConflict, String> {
        self.sync_request(|reply| Request::Conflict(candidate, path, reply))
            .await
    }
    pub async fn resolve_sync(
        &self,
        candidate: String,
        path: String,
        resolution: SyncResolution,
    ) -> Result<Checkpoint, String> {
        self.sync_request(|reply| Request::Resolve(candidate, path, resolution, reply))
            .await
    }
    async fn sync_request<T>(
        &self,
        request: impl FnOnce(oneshot::Sender<Result<T, String>>) -> Request,
    ) -> Result<T, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Sync(request(sender)))
            .map_err(|_| "Vault backup is busy or closed".to_owned())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_owned())?
    }
}
pub(super) fn run(
    request: Request,
    repository: &mut dyn Repository,
    state: &watch::Sender<Status>,
    schedule: &mut schedule::Schedule,
) {
    match request {
        Request::Prepare(incoming, reply) => {
            state.send_modify(|s| {
                s.phase = Phase::Saving;
                s.error = None;
            });
            let result = repository.prepare_sync(&incoming);
            state.send_modify(|s| match &result {
                Ok(proposal) => {
                    s.phase = Phase::Ready;
                    s.error = None;
                    s.checkpoint = Some(proposal.local.clone());
                }
                Err(error) => {
                    s.phase = Phase::Failed;
                    s.error = Some(error.clone());
                }
            });
            if result.is_ok() {
                schedule.completed(Instant::now());
            } else {
                schedule.failed(Instant::now());
            }
            let _ = reply.send(result);
        }
        Request::Preview(candidate, reply) => {
            let _ = reply.send(repository.preview_sync(&candidate));
        }
        Request::Conflict(candidate, path, reply) => {
            let _ = reply.send(repository.sync_conflict(&candidate, &path));
        }
        Request::Resolve(candidate, path, resolution, reply) => {
            let _ = reply.send(repository.resolve_sync(&candidate, &path, resolution));
        }
        Request::Apply(request, reply) => {
            let result = mutation(state, || repository.apply_sync(&request));
            if result.is_ok() {
                schedule.completed(Instant::now());
            } else {
                schedule.failed(Instant::now());
            }
            let _ = reply.send(result);
        }
    }
}
