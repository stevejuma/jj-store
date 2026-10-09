//! Source-selected conversation titles, independent of captured agent identity.
use super::*;
use jj_collab::ConversationMetadataTargetV1 as Target;
impl Supervisor {
    pub(super) async fn metadata_title(
        &self,
        saved: &SavedSession,
    ) -> Result<(Option<String>, Option<String>)> {
        let target = if let Some(id) = &saved.view.conversation_id {
            Target::Conversation { id: id.clone() }
        } else {
            Target::Recorded {
                session: *feedback::recorded_session_ids(saved)?
                    .first()
                    .ok_or(Error::Unavailable)?,
            }
        };
        let selected = if saved.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.view.repository)
                .map_err(|_| Error::Denied)?;
            local
                .conversation_metadata(&target)
                .await
                .map_err(|_| Error::Unavailable)?
        } else {
            let session = self.home_session(saved.view.repository)?;
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(|_| Error::Unavailable)?;
            pin.conversation_metadata(session, &target)
                .await
                .map_err(|_| Error::Unavailable)?
        };
        let title = selected
            .as_ref()
            .and_then(|(_, m)| m.value.title())
            .map(str::to_owned);
        let search = selected.as_ref().map(|(_, m)| m.value.search_text());
        Ok((
            title.or_else(|| {
                saved.events.iter().find_map(|e| {
                    if e.payload["type"] != "prompt" {
                        return None;
                    }
                    let text = e.payload["userText"]
                        .as_str()
                        .or_else(|| e.payload["text"].as_str())?;
                    let title = text
                        .lines()
                        .find(|l| !l.trim().is_empty())?
                        .trim()
                        .chars()
                        .take(80)
                        .collect::<String>();
                    (!title.is_empty()).then_some(title)
                })
            }),
            search,
        ))
    }
}
