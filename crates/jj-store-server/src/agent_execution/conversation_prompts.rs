//! Reserve every exact context batch before dispatching any participant prompt.
use super::*;
impl Supervisor {
    pub(super) async fn prepare_chat_prompt(
        &self,
        command: &Command,
        submitted: Option<&context::ContextBatch>,
    ) -> Result<Value> {
        let _guard = self.0.mutation.lock().await;
        let id = &command.session_id;
        let saved = self.saved(id)?;
        self.authorized(saved.view.repository, &saved.profile, true)
            .await?;
        for binding in &saved.view.repository_bindings {
            self.authorized(binding.repository, &binding.profile, true)
                .await?;
        }
        self.sender(id)?;
        let Action::Prompt { .. } = &command.action else {
            return Err(Error::Invalid);
        };
        let first = self.read(|s| delivery::needs_initial_context(s, id))?;
        let mut batch = self
            .prompt_feedback(&saved, first)
            .await
            .map_err(|error| match error {
                Error::Limit => Error::FeedbackLimit,
                Error::Denied => Error::Denied,
                _ => Error::FeedbackUnavailable,
            })?;
        if let Some(submitted) = submitted {
            batch.include_submission(submitted);
        }
        if first {
            batch
                .revisions
                .extend(saved.context_replies.iter().cloned());
        }
        let frame = self
            .prompt_frame(&saved, command, &mut batch, first)
            .await?;
        let original = frame["userText"].as_str().ok_or(Error::Invalid)?.to_owned();
        let digest = journal::digest(command)?;
        self.update(|s| {
            delivery::prepare(
                s,
                id,
                command.request_id,
                digest,
                original,
                batch,
                frame.clone(),
                first,
            )
        })?;
        let mut dispatch = frame;
        let body = dispatch.as_object_mut().ok_or(Error::Invalid)?;
        body.remove("userText");
        body.remove("sentContext");
        Ok(dispatch)
    }
}
