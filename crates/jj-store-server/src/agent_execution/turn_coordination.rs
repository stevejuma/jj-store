//! Adapts the admitted exclusive conversation turn to reusable prompt ports.
use super::*;
use jj_store_conversations::coordination as c;
use std::sync::atomic::{AtomicBool, Ordering};

pub(super) struct Turn;
impl c::Types for Turn {
    type Parent = RequestId;
    type Command = Command;
    type Frame = Value;
    type Outcome = Outcome;
    type Error = Error;
}
pub(super) struct DispatchLease<'a> {
    supervisor: Supervisor,
    _guard: tokio::sync::MutexGuard<'a, ()>,
}
pub(super) async fn lease(supervisor: &Supervisor) -> DispatchLease<'_> {
    DispatchLease {
        supervisor: supervisor.clone(),
        _guard: supervisor.0.conversation_dispatch.lock().await,
    }
}
struct Host<'a, 'owner> {
    _lease: &'a DispatchLease<'owner>,
    supervisor: Supervisor,
    parent: RequestId,
    home: RepositoryId,
    profile: String,
    context: Option<context::ContextBatch>,
    deferred: Option<jj_collab::ConversationSubmissionV1>,
    claimed: AtomicBool,
    commands_digest: String,
}
pub(super) fn coordinator<'a, 'owner: 'a>(
    lease: &'a DispatchLease<'owner>,
    parent: RequestId,
    home: RepositoryId,
    profile: String,
    context: Option<context::ContextBatch>,
    deferred: Option<jj_collab::ConversationSubmissionV1>,
    commands: &[Command],
) -> Result<c::Coordinator<'a, Turn>> {
    let host = Arc::new(Host {
        _lease: lease,
        supervisor: lease.supervisor.clone(),
        parent,
        home,
        profile,
        context,
        deferred,
        claimed: AtomicBool::new(false),
        commands_digest: journal::digest(&commands)?,
    });
    Ok(c::Coordinator::new(
        host.clone(),
        host.clone(),
        host.clone(),
        host,
    ))
}
#[async_trait::async_trait]
impl c::Admission<Turn> for Host<'_, '_> {
    async fn authorize(&self, parent: &RequestId, commands: &[Command]) -> Result<()> {
        if *parent != self.parent
            || !(1..=4).contains(&commands.len())
            || journal::digest(&commands)? != self.commands_digest
        {
            return Err(Error::Invalid);
        }
        for command in commands {
            let saved = self.supervisor.saved(&command.session_id)?;
            self.supervisor
                .authorized(saved.view.repository, &saved.profile, true)
                .await?;
            for binding in &saved.view.repository_bindings {
                self.supervisor
                    .authorized(binding.repository, &binding.profile, true)
                    .await?;
            }
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl c::Context<Turn> for Host<'_, '_> {
    async fn prepare(&self, command: &Command) -> Result<Value> {
        if let Some(submission) = &self.deferred {
            self.supervisor.queue_file_prompt(
                command,
                self.parent,
                self.home,
                &self.profile,
                submission,
            )
        } else {
            self.supervisor
                .prepare_chat_prompt(command, self.context.as_ref())
                .await
        }
    }
}
#[async_trait::async_trait]
impl c::Runtime<Turn> for Host<'_, '_> {
    async fn dispatch(&self, command: &Command, frame: Value) -> c::Dispatch {
        // This in-process enqueue either takes the exact frame or proves not sent.
        // Provider acceptance arrives later through the existing event journal.
        if self.supervisor.send(&command.session_id, frame).is_ok() {
            c::Dispatch::Enqueued
        } else {
            c::Dispatch::NotSent
        }
    }
}
#[async_trait::async_trait]
impl c::Journal<Turn> for Host<'_, '_> {
    async fn begin(&self, parent: &RequestId, commands: &[Command]) -> Result<c::Begin<Outcome>> {
        if self.claimed.swap(true, Ordering::SeqCst) {
            let mut outcomes = Vec::new();
            for command in commands {
                outcomes.push(self.outcome(command).await?);
            }
            return Ok(c::Begin::Replay(outcomes));
        }
        // The caller retains conversation_dispatch and has already durably
        // admitted this exact parent. Earlier duplicates never construct a Host.
        self.supervisor.read(|state| {
            let intent = state.intents.get(parent).ok_or(Error::Stale)?;
            if intent.outcome.state != "preparing_publication" {
                return Err(Error::Conflict);
            }
            Ok(())
        })?;
        Ok(c::Begin::New)
    }
    async fn prepared(&self, parent: &RequestId) -> Result<()> {
        self.supervisor
            .finish(*parent, "prepared", None)
            .map(|_| ())
    }
    async fn release(&self, command: &Command, reason: c::Release) -> Result<()> {
        self.supervisor.update(|state| delivery::observe(state, &command.session_id, &json!({"type":"error", "requestId":command.request_id.to_string(), "code":"dispatch_not_sent"})))?;
        let diagnostic = match reason {
            c::Release::PreparationFailed => "chat_context_preparation_failed_no_prompt",
            c::Release::NotSent => "dispatch_not_sent",
        };
        self.supervisor
            .finish(command.request_id, "failed", Some(diagnostic))
            .map(|_| ())
    }
    async fn unknown(&self, command: &Command) -> Result<()> {
        self.supervisor
            .finish(
                command.request_id,
                "outcome_unknown",
                Some("dispatch_outcome_unknown_no_replay"),
            )
            .map(|_| ())
    }
    async fn outcome(&self, command: &Command) -> Result<Outcome> {
        self.supervisor.read(|state| {
            state
                .intents
                .get(&command.request_id)
                .map(|intent| intent.outcome.clone())
                .ok_or(Error::Stale)
        })
    }
    async fn accepted(&self, parent: &RequestId) -> Result<()> {
        self.supervisor
            .finish(*parent, "accepted", None)
            .map(|_| ())
    }
}
