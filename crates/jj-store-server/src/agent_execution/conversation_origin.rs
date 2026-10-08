//! A receipt-backed metadata thread links conversations and exact agent-message origins.
use super::*;
use jj_store_service::collab_mutations::{
    CollabCommandService, CollabCommandV1, Request, ThreadCommandV1, ThreadId,
};
impl Supervisor {
    pub(super) async fn prepare_origin_discussion(&self, id: &str, profile: &str) -> Result<()> {
        let conversation = self.read(|s| s.conversations.get(id).cloned().ok_or(Error::Stale))?;
        if conversation.discussion_thread.is_some() {
            return Ok(());
        }
        if let Some(thread) = conversation.origin.thread {
            self.home_thread_revision(conversation.home, profile, thread)
                .await?;
            self.update(|s| {
                s.conversations
                    .get_mut(id)
                    .ok_or(Error::Stale)?
                    .discussion_thread = Some(thread);
                Ok(())
            })?;
            return Ok(());
        }
        let thread = publication::identity(&format!("conversation-thread/{id}"));
        let request = Request {
            version: 1,
            request_id: RequestId::new(publication::identity(&format!(
                "conversation-thread-create/{id}"
            ))),
            command: CollabCommandV1::Thread {
                intent: if let Some(target) = conversation.origin.agent_target {
                    ThreadCommandV1::CreateAgentDiscussion {
                        thread_id: ThreadId::new(thread),
                        title: "Agent conversation replies".into(),
                        target,
                    }
                } else {
                    ThreadCommandV1::Create {
                        thread_id: ThreadId::new(thread),
                        title: "Agent conversation".into(),
                        baseline: None,
                    }
                },
            },
        };
        self.authorized(conversation.home, profile, true).await?;
        if profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(conversation.home)
                .map_err(|_| Error::Denied)?;
            let outcome = local
                .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
                    version: 1,
                    request_id: request.request_id,
                    command: request.command,
                })
                .await
                .map_err(|_| Error::Unavailable)?;
            if !matches!(
                outcome,
                jj_store_service::local_repository::LocalCollabOutcome::Committed { .. }
            ) {
                return Err(Error::Unavailable);
            }
        } else if !matches!(
            CollabCommandService
                .apply(self.home_session(conversation.home)?, &request)
                .await
                .map_err(|_| Error::Unavailable)?,
            jj_store_service::collab_mutations::Outcome::Committed { .. }
        ) {
            return Err(Error::Unavailable);
        }
        self.update(|s| {
            s.conversations
                .get_mut(id)
                .ok_or(Error::Stale)?
                .discussion_thread = Some(ThreadId::new(thread));
            Ok(())
        })
    }
}
