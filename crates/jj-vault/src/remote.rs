use super::*;

pub(super) enum Request {
    Import(FetchedHead, oneshot::Sender<Result<Checkpoint, String>>),
    Restore(
        VaultRestoreRequest,
        oneshot::Sender<Result<Capture, String>>,
    ),
}
impl Backup {
    /// Network transfer is completed by the host before queuing native import.
    pub async fn import_remote(&self, head: FetchedHead) -> Result<Checkpoint, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Remote(Request::Import(head, sender)))
            .map_err(|_| "Vault backup is busy or closed".to_owned())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_owned())?
    }
    pub async fn restore_remote(&self, request: VaultRestoreRequest) -> Result<Capture, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Remote(Request::Restore(request, sender)))
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
        Request::Import(head, reply) => {
            // The host reports transfer/import progress separately from local
            // capture status; importing changes no working files/checkpoint.
            let _ = reply.send(repository.import_remote(&head));
        }
        Request::Restore(request, reply) => {
            let result = mutation(state, || repository.restore_remote(&request));
            if result.is_ok() {
                schedule.completed(Instant::now());
            } else {
                schedule.failed(Instant::now());
            }
            let _ = reply.send(result);
        }
    }
}
