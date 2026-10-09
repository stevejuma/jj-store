//! Deferred file prompts retain their complete original intent until queue eligibility.
use super::*;
use jj_collab::ConversationSubmissionV1;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DeferredFilePrompt {
    pub command: Command,
    pub parent: RequestId,
    pub repository: RepositoryId,
    pub profile: String,
    pub submission: ConversationSubmissionV1,
}
impl Supervisor {
    pub(super) fn queue_file_prompt(
        &self,
        command: &Command,
        parent: RequestId,
        repository: RepositoryId,
        profile: &str,
        submission: &ConversationSubmissionV1,
    ) -> Result<Value> {
        submission.validate().map_err(|_| Error::Invalid)?;
        if submission.workspace_comments.is_empty()
            || submission.text.is_some()
            || submission.message_id.is_some()
        {
            return Err(Error::Invalid);
        }
        self.sender(&command.session_id)?;
        let digest = journal::digest(command)?;
        self.update(|s| {
            if s.intents.len() >= 4096
                || s.intents.contains_key(&command.request_id)
                || s.deferred_file_prompts.len() >= 4096
                || s.deferred_file_prompts.contains_key(&command.request_id)
            {
                return Err(Error::Limit);
            }
            s.intents.insert(
                command.request_id,
                Intent {
                    digest,
                    prompt: None,
                    outcome: Outcome {
                        version: 1,
                        request_id: command.request_id,
                        session_id: Some(command.session_id.clone()),
                        state: "queued".into(),
                        error: None,
                        comment_publication: None,
                    },
                },
            );
            s.deferred_file_prompts.insert(
                command.request_id,
                DeferredFilePrompt {
                    command: command.clone(),
                    parent,
                    repository,
                    profile: profile.into(),
                    submission: submission.clone(),
                },
            );
            Ok(())
        })?;
        Ok(
            json!({"version":2,"type":"prompt","requestId":command.request_id,"sessionId":command.session_id,"deferredFiles":true}),
        )
    }
    pub(super) async fn prepare_file_prompt(&self, frame: Value) -> Result<Value> {
        if frame["deferredFiles"] != true {
            return Ok(frame);
        }
        let request = frame["requestId"]
            .as_str()
            .ok_or(Error::Invalid)?
            .parse()
            .map_err(|_| Error::Invalid)?;
        let saved = self.read(|s| {
            s.deferred_file_prompts
                .get(&request)
                .cloned()
                .ok_or(Error::Invalid)
        })?;
        let session = self.saved(&saved.command.session_id)?;
        self.authorized(saved.repository, &saved.profile, true)
            .await?;
        let resolved = self
            .resolve_workspace_comments(saved.parent, &saved.submission)
            .await;
        if matches!(resolved, Err(Error::Stale | Error::Invalid | Error::Denied)) {
            self.update(|s| {
                if let Some(frozen) = s.workspace_submissions.get_mut(&saved.parent) {
                    frozen.refused = true;
                }
                Ok(())
            })?;
        }
        let resolved = resolved?;
        let submitted = self
            .publish_prompt_comments(saved.parent, saved.repository, &saved.profile, &resolved)
            .await?;
        let mut dispatch = self
            .prepare_chat_prompt(&saved.command, Some(&submitted))
            .await?;
        dispatch["sessionId"] = json!(saved.command.session_id);
        dispatch["version"] = json!(if session.view.conversation_id.is_some() {
            3
        } else {
            2
        });
        Ok(dispatch)
    }
}

impl Supervisor {
    /// An explicit repeat may finish publication, but never replay an agent prompt.
    pub(super) async fn recover_file_publication(&self, parent: RequestId) -> Result<()> {
        let deferred = self.read(|s| {
            Ok(s.deferred_file_prompts
                .values()
                .find(|d| d.parent == parent)
                .cloned())
        })?;
        let Some(saved) = deferred else {
            return Ok(());
        };
        let _guard = self.0.workspace_gate.try_lock().map_err(|_| Error::Busy)?;
        // Recovery of an unknown dispatch cannot cause a fresh physical snapshot.
        let complete = self.read(|s| {
            Ok(s.workspace_submissions
                .get(&parent)
                .is_some_and(|f| f.captures.len() == f.original.workspace_comments.len()))
        })?;
        if !complete {
            return Err(Error::Unavailable);
        }
        let resolved = self
            .resolve_workspace_comments(parent, &saved.submission)
            .await?;
        self.publish_prompt_comments(parent, saved.repository, &saved.profile, &resolved)
            .await?;
        Ok(())
    }
}
