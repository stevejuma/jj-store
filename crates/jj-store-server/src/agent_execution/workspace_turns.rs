//! Capture the exact workspace boundary immediately before a prompt leaves the controller.
use super::*;
impl Supervisor {
    pub(super) async fn capture_turn_start(&self, frame: &Value) -> Result<()> {
        if frame["type"] != "prompt" {
            return Ok(());
        }
        let id = frame["sessionId"].as_str().ok_or(Error::Invalid)?;
        let request = frame["requestId"].as_str().ok_or(Error::Invalid)?;
        let saved = self.saved(id)?;
        let mut workspaces = saved.code_workspaces.clone();
        if saved.view.conversation_id.is_none() && saved.workspace_ready {
            workspaces.insert(saved.view.repository, saved.workspace.clone());
        }
        for (repository, workspace) in &workspaces {
            if saved.events.iter().any(|e| {
                e.payload["type"] == "repository_turn_start"
                    && e.payload["requestId"] == request
                    && e.payload["repository"] == repository.to_string()
            }) {
                // A physical effect with an uncertain result is not implicitly retried.
                return Err(Error::Stale);
            }
            let profile = if saved.view.conversation_id.is_none() {
                saved.profile.as_str()
            } else {
                &saved
                    .view
                    .repository_bindings
                    .iter()
                    .find(|b| b.repository == *repository)
                    .ok_or(Error::Stale)?
                    .profile
            };
            self.authorized(*repository, profile, true).await?;
            self.append(id, json!({"type":"repository_turn_start","requestId":request,"repository":repository,"state":"preparing"}))?;
            let snapshot = self.0.resources.driver.snapshot(workspace).await;
            self.append(id, json!({"type":"repository_turn_start","requestId":request,"repository":repository,"state":if snapshot.is_ok(){"completed"}else{"outcome_unknown"},"operation":snapshot.as_ref().ok().map(|w|w.operation),"commit":snapshot.as_ref().ok().map(|w|w.commit.clone())}))?;
            snapshot?;
        }
        Ok(())
    }
}

/// The ACP worker is single-turn. Keep queued frames controller-owned until that
/// turn completes so a future start never captures another turn's intermediate files.
#[derive(Default)]
pub(super) struct PromptQueue {
    pending: std::collections::VecDeque<Value>,
    active: Option<(String, String)>,
}
impl PromptQueue {
    pub fn push(&mut self, frame: Value) -> Result<()> {
        let session = frame["sessionId"].as_str().ok_or(Error::Invalid)?;
        if self.pending.len() >= 64
            || self
                .pending
                .iter()
                .filter(|f| f["sessionId"] == session)
                .count()
                >= 16
        {
            return Err(Error::Limit);
        }
        self.pending.push_back(frame);
        Ok(())
    }
    pub fn next(&mut self) -> Option<Value> {
        if self.active.is_some() {
            return None;
        }
        let frame = self.pending.pop_front()?;
        self.active = Some((
            frame["sessionId"].as_str()?.into(),
            frame["requestId"].as_str()?.into(),
        ));
        Some(frame)
    }
    pub fn completed(&mut self, frame: &Value) {
        if matches!(frame["type"].as_str(), Some("record" | "error"))
            && self
                .active
                .as_ref()
                .is_some_and(|(s, r)| frame["sessionId"] == *s && frame["requestId"] == *r)
        {
            self.active = None;
        }
    }
    pub fn cancel(&mut self, session: &str, target: &str) -> Option<Value> {
        let index = self
            .pending
            .iter()
            .position(|f| f["sessionId"] == session && f["requestId"] == target)?;
        self.pending.remove(index)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queued_turns_wait_for_the_exact_active_completion_and_cancel_without_dispatch() {
        let mut queue = PromptQueue::default();
        for request in ["a", "b", "c"] {
            queue
                .push(json!({"type":"prompt","sessionId":"s","requestId":request}))
                .unwrap();
        }
        assert_eq!(queue.next().unwrap()["requestId"], "a");
        queue.completed(&json!({"type":"record","sessionId":"other","requestId":"a"}));
        assert!(queue.next().is_none());
        assert_eq!(queue.cancel("s", "b").unwrap()["requestId"], "b");
        queue.completed(&json!({"type":"record","sessionId":"s","requestId":"a"}));
        assert_eq!(queue.next().unwrap()["requestId"], "c");
    }
}
