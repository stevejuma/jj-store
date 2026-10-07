//! Reserve every exact context batch before dispatching any participant prompt.
use super::*;
impl Supervisor {
    pub(super) async fn prepare_chat_prompt(&self, command: &Command) -> Result<Value> {
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
        let Action::Prompt {
            text,
            max_duration_ms,
        } = &command.action
        else {
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
        if first {
            batch
                .revisions
                .extend(saved.context_replies.iter().cloned());
        }
        let context = prompt_context::prepared(&saved, &batch.text, first)?;
        let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
        batch.text = runtime::safe(&sanitizer, json!({"text":batch.text}))?["text"]
            .as_str()
            .ok_or(Error::Invalid)?
            .to_owned();
        if batch.text.len() > 32768 {
            return Err(Error::FeedbackLimit);
        }
        let original = runtime::safe(&sanitizer, json!({"text":text}))?["text"]
            .as_str()
            .ok_or(Error::Invalid)?
            .to_owned();
        let captured = sent_context::capture(&saved, &original, &batch.text, first);
        let mut frame = runtime::safe(
            &sanitizer,
            json!({"version":2,"type":"prompt","requestId":command.request_id.to_string(),"text":text,"context":context,"maxDurationMs":max_duration_ms,"userText":original,"sentContext":captured}),
        )?;
        frame["sentContext"]["wireText"] = frame["text"].clone();
        frame["sentContext"]["wireContext"] = frame["context"].clone();
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
