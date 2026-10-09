//! Authorization and immutable capture projections. Reads never start jobs.
use super::*;
use super::{Error, Result};
use jj_agent::{AgentObjectKind, AgentObjectRef, AgentSessionId};
use jj_collab::*;
use jj_store_service::agent_execution::metadata as api;
use jj_store_service::collab_reads::ReadScope;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(super) struct Capture {
    pub query: api::Query,
    pub repository: RepositoryId,
    pub profile: String,
    pub target: ConversationMetadataTargetV1,
    pub owner: Option<ThreadId>,
    pub revision: Option<CollabObjectRef>,
    pub value: ConversationMetadataValueV1,
    #[serde(default)]
    pub selected_value: ConversationMetadataValueV1,
    pub sources: Vec<ConversationMetadataSourceV1>,
    pub transcript: String,
    pub truncated: bool,
    pub details: Value,
    pub writable: bool,
    #[serde(default)]
    pub suggestions_changed: bool,
}
impl Supervisor {
    pub(super) async fn metadata_capture(&self, query: api::Query) -> Result<Capture> {
        let saved = self.read(|s| {
            Ok(s.sessions
                .values()
                .find(|v| {
                    if let Some(repo) = query.repository {
                        v.view.repository == repo
                            && feedback::recorded_session_ids(v)
                                .unwrap_or_default()
                                .iter()
                                .any(|id| id.to_string() == query.session)
                    } else {
                        v.view.id == query.session
                    }
                })
                .cloned())
        })?;
        let (repository, profile, target, owner, ids) = if let Some(saved) = saved {
            let conversation = saved.view.conversation_id.clone();
            let (owner, ids) = self.read(|s| {
                let owner = conversation
                    .as_ref()
                    .and_then(|id| s.conversations.get(id))
                    .and_then(|c| c.discussion_thread);
                let mut ids = std::collections::BTreeSet::new();
                for item in s.sessions.values().filter(|v| {
                    if let Some(c) = &conversation {
                        v.view.conversation_id.as_ref() == Some(c)
                    } else {
                        v.view.id == saved.view.id
                    }
                }) {
                    ids.extend(feedback::recorded_session_ids(item)?);
                }
                Ok((owner, ids.into_iter().collect::<Vec<_>>()))
            })?;
            (
                saved.view.repository,
                saved.profile,
                conversation
                    .map(|id| ConversationMetadataTargetV1::Conversation { id })
                    .unwrap_or(ConversationMetadataTargetV1::Recorded {
                        session: *ids.first().ok_or(Error::Unavailable)?,
                    }),
                owner,
                ids,
            )
        } else {
            let repo = query.repository.ok_or(Error::Stale)?;
            let id: AgentSessionId = query.session.parse().map_err(|_| Error::Invalid)?;
            let profile = if self
                .0
                .resources
                .folders
                .as_ref()
                .is_some_and(|f| f.open(repo).is_ok())
            {
                "local_jj_git"
            } else {
                "native"
            };
            (
                repo,
                profile.into(),
                ConversationMetadataTargetV1::Recorded { session: id },
                None,
                vec![id],
            )
        };
        self.authorized(repository, &profile, false).await?;
        let (selected, writable) = if profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(repository)
                .map_err(|_| Error::Denied)?;
            (
                local
                    .conversation_metadata(&target)
                    .await
                    .map_err(|_| Error::Unavailable)?,
                true,
            )
        } else {
            let session = self.home_session(repository)?;
            let pin = Box::pin(self.0.metadata_reads.pin(session.clone(), ReadScope::All))
                .await
                .map_err(|_| Error::Unavailable)?;
            (
                pin.conversation_metadata(session, &target)
                    .await
                    .map_err(|_| Error::Unavailable)?,
                pin.metadata_writable(),
            )
        };
        let (revision, mut value, owner) = selected
            .map(|(reference, v)| (Some(reference), v.value, Some(v.thread_id)))
            .unwrap_or((None, ConversationMetadataValueV1::default(), owner));
        let selected_value = value.clone();
        let stored_titles = value.agent_titles.clone();
        self.read(|s| {
            for saved in s.sessions.values().filter(|v| {
                v.view.repository == repository
                    && match &target {
                        ConversationMetadataTargetV1::Conversation { id } => {
                            v.view.conversation_id.as_ref() == Some(id)
                        }
                        ConversationMetadataTargetV1::Recorded { session } => {
                            feedback::recorded_session_ids(v)
                                .unwrap_or_default()
                                .contains(session)
                        }
                    }
            }) {
                for event in &saved.events {
                    let update = &event.payload["update"];
                    if event.committed && update["sessionUpdate"] == "session_info_update" {
                        if update.get("title").is_some_and(Value::is_null) {
                            value.agent_titles.remove(&saved.view.id);
                            value.agent_title_sources.remove(&saved.view.id);
                        } else if let Some(title) =
                            update["title"].as_str().filter(|v| label(v, 1024))
                        {
                            value
                                .agent_titles
                                .insert(saved.view.id.clone(), title.into());
                        }
                    }
                }
            }
            Ok(())
        })?;
        if ids.len() > 16 {
            return Err(Error::Limit);
        }
        let mut sources = Vec::new();
        let mut transcript = String::new();
        let mut details = Vec::new();
        let mut truncated = false;
        for id in ids {
            let detail = if profile == "local_jj_git" {
                let local = self
                    .0
                    .resources
                    .folders
                    .as_ref()
                    .ok_or(Error::Unavailable)?
                    .open(repository)
                    .map_err(|_| Error::Denied)?;
                let selection = local
                    .inbox_selection()
                    .await
                    .map_err(|_| Error::Unavailable)?;
                serde_json::to_value(
                    local
                        .browser_agent_session(
                            selection
                                .snapshot
                                .metadata_commit
                                .ok_or(Error::Unavailable)?,
                            id,
                        )
                        .await
                        .map_err(|_| Error::Unavailable)?,
                )
                .map_err(|_| Error::Unavailable)?
            } else {
                let pager = jj_store_service::collaboration::AgentSessionPager::new(
                    jj_store_service::pagination::Limits::default(),
                    Arc::new(jj_store_service::auth::SystemClock),
                )
                .map_err(|_| Error::Unavailable)?;
                serde_json::to_value(
                    pager
                        .captured_detail(self.home_session(repository)?, id)
                        .await
                        .map_err(|_| Error::Unavailable)?,
                )
                .map_err(|_| Error::Unavailable)?
            };
            let snapshot = detail["session"]["snapshot_id"]
                .as_str()
                .ok_or(Error::Unavailable)?
                .parse()
                .map_err(|_| Error::Unavailable)?;
            sources.push(ConversationMetadataSourceV1 {
                session: id,
                snapshot: AgentObjectRef::new(AgentObjectKind::SessionSnapshot, snapshot),
            });
            let (text, counts, partial) = super::metadata_text::project(&detail);
            truncated |= partial;
            super::metadata_text::append(
                &mut transcript,
                &format!("\n--- Captured session {id} ---\n{text}"),
                &mut truncated,
            );
            details.push(json!({"session":detail["session"],"counts":counts,"partial":partial}));
        }
        self.read(|s| {
            for saved in s.sessions.values().filter(|v| {
                v.view.repository == repository && value.agent_titles.contains_key(&v.view.id)
            }) {
                let aliases = feedback::recorded_session_ids(saved)?;
                if stored_titles.get(&saved.view.id) == value.agent_titles.get(&saved.view.id)
                    && value.agent_title_sources.contains_key(&saved.view.id)
                {
                    continue;
                }
                if let Some(source) = sources
                    .iter()
                    .find(|source| aliases.contains(&source.session))
                {
                    value
                        .agent_title_sources
                        .insert(saved.view.id.clone(), source.clone());
                }
            }
            Ok(())
        })?;
        let comments = self.read(|s| {
            let mut text = String::new();
            let mut seen = std::collections::BTreeSet::new();
            for saved in s.sessions.values().filter(|v| {
                v.view.repository == repository
                    && match &target {
                        ConversationMetadataTargetV1::Conversation { id } => {
                            v.view.conversation_id.as_ref() == Some(id)
                        }
                        ConversationMetadataTargetV1::Recorded { session } => {
                            feedback::recorded_session_ids(v)
                                .unwrap_or_default()
                                .contains(session)
                        }
                    }
            }) {
                for event in saved.events.iter().filter(|e| {
                    e.committed
                        && e.payload["type"] == "context_delivery"
                        && e.payload["sentContext"]["delivery"] == "sent"
                }) {
                    let Some(request) = event.payload["requestId"]
                        .as_str()
                        .and_then(|id| id.parse::<RequestId>().ok())
                    else {
                        continue;
                    };
                    let Some(prompt) = s
                        .intents
                        .get(&request)
                        .and_then(|i| i.prompt.as_ref())
                        .filter(|p| p.state == delivery::DeliveryState::Delivered)
                    else {
                        continue;
                    };
                    for comment in &prompt.batch.comments {
                        if seen.insert(journal::digest(&comment.reference)?) {
                            super::metadata_text::append(
                                &mut text,
                                &format!("\nuser comment: {}\n", comment.body),
                                &mut truncated,
                            );
                        }
                    }
                    for reply in &prompt.batch.replies {
                        if seen.insert(journal::digest(&reply.reply)?) {
                            super::metadata_text::append(
                                &mut text,
                                &format!("\nuser discussion: {}\n", reply.body),
                                &mut truncated,
                            );
                        }
                    }
                }
            }
            Ok(text)
        })?;
        super::metadata_text::append(&mut transcript, &comments, &mut truncated);
        self.authorized(repository, &profile, false).await?;
        let suggestions_changed = stored_titles != value.agent_titles;
        Ok(Capture {
            query,
            repository,
            profile,
            target,
            owner,
            revision,
            value,
            selected_value,
            sources,
            transcript,
            truncated,
            details: json!(details),
            writable,
            suggestions_changed,
        })
    }
}
