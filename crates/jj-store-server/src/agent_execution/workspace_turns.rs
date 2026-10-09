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
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }
    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }
    pub fn defer(&mut self, frame: Value) {
        self.active = None;
        self.pending.push_front(frame);
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

/// Covers the gap between writing a prompt and receiving its started acknowledgement.
/// Worker teardown drops the lease, including when its future is cancelled.
pub(super) struct WorkspaceLease {
    supervisor: Supervisor,
    session: String,
    request: RequestId,
}
impl Drop for WorkspaceLease {
    fn drop(&mut self) {
        if let Ok(mut active) = self.supervisor.0.workspace_activity.lock() {
            if active.get(&self.session) == Some(&self.request) {
                active.remove(&self.session);
            }
        }
    }
}
impl Supervisor {
    pub(super) async fn workspace_dispatch(&self, frame: Value) -> Result<(Value, WorkspaceLease)> {
        let _guard = self.0.workspace_gate.try_lock().map_err(|_| Error::Busy)?;
        let frame = self.prepare_file_prompt(frame).await?;
        self.capture_turn_start(&frame).await?;
        let session = frame["sessionId"]
            .as_str()
            .ok_or(Error::Invalid)?
            .to_owned();
        let request = frame["requestId"]
            .as_str()
            .ok_or(Error::Invalid)?
            .parse()
            .map_err(|_| Error::Invalid)?;
        self.0
            .workspace_activity
            .lock()
            .map_err(|_| Error::Unavailable)?
            .insert(session.clone(), request);
        Ok((
            frame,
            WorkspaceLease {
                supervisor: self.clone(),
                session,
                request,
            },
        ))
    }
}
