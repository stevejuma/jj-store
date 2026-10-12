//! Durable typed links keep origin discussions on the canonical home transcript.
use super::*;
use jj_agent::{AgentObjectKind, AgentObjectRef};
use jj_store_service::collab_mutations::{CollabCommandService, Request};
use jj_store_service::collab_mutations::{
    CollabCommandV1, ThreadCommandV1, ThreadLinkId, ThreadLinkTargetV1,
};
impl Supervisor {
    pub(super) async fn link_origin(&self, id: &str) -> Result<()> {
        let saved = self.saved(id)?;
        let Some(conversation) = saved.view.conversation_id.as_ref() else {
            return Ok(());
        };
        let thread = self.read(|s| {
            Ok(s.conversations
                .get(conversation)
                .ok_or(Error::Stale)?
                .discussion_thread)
        })?;
        let Some(thread) = thread else {
            return Ok(());
        };
        if saved.origin_linked {
            return Ok(());
        }
        let primary = self.read(|s| Ok(s.conversations[conversation].origin.thread.is_some()))?
            && self
                .home_thread_summary(saved.view.repository, &saved.profile, thread)
                .await?
                .value
                .is_user_thread();
        self.authorized(saved.view.repository, &saved.profile, true)
            .await?;
        let session_id = *feedback::recorded_session_ids(&saved)?
            .first()
            .ok_or(Error::Unavailable)?;
        let request = if let Some(request) = &saved.origin_link {
            request.clone()
        } else {
            let (expected_thread, snapshot_id) = if saved.profile == "local_jj_git" {
                let local = self
                    .0
                    .resources
                    .folders
                    .as_ref()
                    .ok_or(Error::Unavailable)?
                    .open(saved.view.repository)
                    .map_err(|_| Error::Denied)?;
                let pin = local
                    .inbox_selection()
                    .await
                    .map_err(|_| Error::Unavailable)?
                    .snapshot
                    .metadata_commit;
                (
                    local
                        .inbox_thread(pin, thread)
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .thread
                        .reference,
                    local
                        .browser_agent_session(pin.ok_or(Error::Unavailable)?, session_id)
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .session
                        .snapshot_id,
                )
            } else {
                let session = self.home_session(saved.view.repository)?;
                let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                    session.clone(),
                    Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
                )
                .await
                .map_err(|_| Error::Unavailable)?;
                let pager = jj_store_service::collaboration::AgentSessionPager::new(
                    jj_store_service::pagination::Limits::default(),
                    Arc::new(jj_store_service::auth::SystemClock),
                )
                .map_err(|_| Error::Unavailable)?;
                (
                    pin.thread(session.clone(), thread)
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .reference,
                    pager
                        .detail(session, session_id)
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .session
                        .snapshot_id,
                )
            };
            let request = Request {
                version: 1,
                request_id: RequestId::new(publication::identity(&format!("origin-link/{id}"))),
                command: CollabCommandV1::Thread {
                    intent: ThreadCommandV1::Link {
                        thread_id: thread,
                        expected_thread,
                        link_id: ThreadLinkId::new(publication::identity(&format!(
                            "origin-link/{id}"
                        ))),
                        target: ThreadLinkTargetV1::Session {
                            session_id,
                            snapshot: AgentObjectRef::new(
                                AgentObjectKind::SessionSnapshot,
                                snapshot_id,
                            ),
                            primary,
                        },
                        observed_primary: None,
                    },
                },
            };
            self.update(|s| {
                s.sessions.get_mut(id).ok_or(Error::Invalid)?.origin_link = Some(request.clone());
                Ok(())
            })?;
            request
        };
        if saved.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.view.repository)
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
        } else {
            if !matches!(
                CollabCommandService
                    .apply(self.home_session(saved.view.repository)?, &request)
                    .await
                    .map_err(|_| Error::Unavailable)?,
                jj_store_service::collab_mutations::Outcome::Committed { .. }
            ) {
                return Err(Error::Unavailable);
            }
        }
        self.update(|s| {
            s.sessions.get_mut(id).ok_or(Error::Invalid)?.origin_linked = true;
            Ok(())
        })
    }
    pub(super) fn home_session(
        &self,
        repository: RepositoryId,
    ) -> Result<Arc<jj_store_service::native::NativeSession>> {
        Ok(self.native_request(repository)?.into_session())
    }
}
