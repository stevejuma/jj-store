use super::*;
pub(super) enum Request {
    Adopt(AdoptionRequest, oneshot::Sender<Result<Checkpoint, String>>),
    Stop(String, oneshot::Sender<Result<(), String>>),
}
impl Backup {
    pub async fn adopt_history(&self, request: AdoptionRequest) -> Result<Checkpoint, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Adoption(Request::Adopt(request, sender)))
            .map_err(|_| "Vault backup is busy or closed".to_owned())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_owned())?
    }
    pub async fn stop_adoption(&self, request: String) -> Result<(), String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Adoption(Request::Stop(request, sender)))
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
        Request::Adopt(request, reply) => {
            let mut accepted = None;
            let result = mutation(state, || {
                accepted = Some(repository.adopt_history(&request)?);
                // Publish current status, including later files on receipt replay.
                repository.capture()
            });
            if result.is_ok() {
                schedule.completed(Instant::now());
            } else {
                schedule.failed(Instant::now());
            }
            let _ = reply.send(
                result.and_then(|_| accepted.ok_or_else(|| "Missing adoption receipt".into())),
            );
        }
        Request::Stop(id, reply) => {
            let result = mutation(state, || {
                repository.stop_adoption(&id)?;
                repository.capture()
            });
            if result.is_ok() {
                schedule.completed(Instant::now());
            } else {
                schedule.failed(Instant::now());
            }
            let _ = reply.send(result.map(|_| ()));
        }
    }
}
