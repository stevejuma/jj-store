use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum AiHistoryRequest {
    Begin(AiTurnContext),
    Bind(AiTurnContext),
    End {
        conversation: String,
        turn: String,
        outcome: AiTurnOutcome,
    },
    List {
        conversation: String,
        after: Option<String>,
    },
    Read {
        conversation: String,
        turn: String,
    },
    PreviewUndo {
        conversation: String,
        turn: String,
        paths: Vec<String>,
    },
    Undo(AiUndoRequest),
    PendingUndo {},
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum AiHistoryResponse {
    Turn(Box<AiTurnRecord>),
    Page(AiTurnPage),
    Preview(AiUndoPreview),
    Capture(Capture),
    Pending(Vec<AiUndoRequest>),
}
impl Backup {
    pub async fn ai_history(&self, request: AiHistoryRequest) -> Result<AiHistoryResponse, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Ai(request, sender))
            .map_err(|_| "Vault backup is busy or closed".to_string())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
}
impl NativeRepository {
    pub(super) fn ai_recover(&mut self) -> Result<(), String> {
        native::runtime()?.block_on(async {
            jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?
                .recover_ai_turns()
                .map_err(|e| e.to_string())
        })
    }
    pub(super) fn ai_request(
        &mut self,
        request: AiHistoryRequest,
    ) -> Result<AiHistoryResponse, String> {
        native::runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            let result = match request {
                AiHistoryRequest::Begin(context) => session
                    .begin_ai_turn(&context, &self.exclusions)
                    .await
                    .map(|record| AiHistoryResponse::Turn(Box::new(record))),
                AiHistoryRequest::Bind(context) => session
                    .bind_ai_turn(&context)
                    .map(|record| AiHistoryResponse::Turn(Box::new(record))),
                AiHistoryRequest::End {
                    conversation,
                    turn,
                    outcome,
                } => session
                    .end_ai_turn(&conversation, &turn, outcome, &self.exclusions)
                    .await
                    .map(|record| AiHistoryResponse::Turn(Box::new(record))),
                AiHistoryRequest::List {
                    conversation,
                    after,
                } => session
                    .ai_turns(&conversation, after.as_deref())
                    .map(AiHistoryResponse::Page),
                AiHistoryRequest::Read { conversation, turn } => session
                    .ai_turn(&conversation, &turn)
                    .map(|record| AiHistoryResponse::Turn(Box::new(record))),
                AiHistoryRequest::PreviewUndo {
                    conversation,
                    turn,
                    paths,
                } => session
                    .preview_ai_undo(&conversation, &turn, &paths, &self.exclusions)
                    .await
                    .map(AiHistoryResponse::Preview),
                AiHistoryRequest::Undo(request) => session
                    .apply_ai_undo(&request, &self.exclusions)
                    .await
                    .map(AiHistoryResponse::Capture),
                AiHistoryRequest::PendingUndo {} => {
                    session.pending_ai_undo().map(AiHistoryResponse::Pending)
                }
            };
            result.map_err(|e| e.to_string())
        })
    }
}
pub(super) fn run(
    request: AiHistoryRequest,
    reply: oneshot::Sender<Result<AiHistoryResponse, String>>,
    repository: &mut dyn Repository,
    state: &watch::Sender<Status>,
) {
    let mutation = matches!(
        request,
        AiHistoryRequest::Begin(_) | AiHistoryRequest::End { .. } | AiHistoryRequest::Undo(_)
    );
    if mutation {
        state.send_modify(|s| {
            s.phase = Phase::Saving;
            s.error = None;
        });
    }
    let result = repository.ai_history(request);
    if mutation {
        state.send_modify(|s| match &result {
            Ok(value) => {
                s.phase = Phase::Ready;
                s.error = None;
                s.checkpoint = match value {
                    AiHistoryResponse::Turn(turn) => {
                        turn.after.clone().or_else(|| Some(turn.before.clone()))
                    }
                    AiHistoryResponse::Capture(capture) => {
                        capture.checkpoint.clone().or_else(|| s.checkpoint.clone())
                    }
                    _ => s.checkpoint.clone(),
                };
            }
            Err(error) => {
                s.phase = Phase::Failed;
                s.error = Some(error.clone());
            }
        });
    }
    let _ = reply.send(result);
}
