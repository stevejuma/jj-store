//! Answer projection follows committed captures and uses ordinary receipt publication.
use super::*;
use super::{Error, Result};
use jj_collab::*;
use jj_store_service::collab_mutations::Request;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SavedAnswerLink {
    pub request: Request,
    pub published: bool,
}
impl Supervisor {
    pub(super) async fn publish_comment_answers(&self, id: &str) -> Result<()> {
        let saved = self.saved(id)?;
        let records: Vec<_> = saved
            .events
            .iter()
            .filter(|e| {
                e.committed
                    && e.payload["type"] == "record"
                    && e.payload["assistantText"]
                        .as_str()
                        .is_some_and(|s| s.contains("<comment-reply"))
            })
            .collect();
        if records.is_empty() {
            return Ok(());
        }
        let feedback = self.feedback_saved(&saved).await?;
        for event in records {
            let Some(request) = event.payload["requestId"]
                .as_str()
                .and_then(|s| s.parse::<RequestId>().ok())
            else {
                continue;
            };
            let delivery = self.read(|s| {
                Ok(s.intents
                    .get(&request)
                    .filter(|i| i.outcome.session_id.as_deref() == Some(id))
                    .and_then(|i| i.prompt.clone()))
            })?;
            let Some(delivery) = delivery.filter(|p| p.state == delivery::DeliveryState::Delivered)
            else {
                continue;
            };
            let Some(prompt) = feedback
                .messages
                .iter()
                .find(|m| m.row_id == format!("{request}/user"))
            else {
                continue;
            };
            let Some(answer) = feedback
                .messages
                .iter()
                .find(|m| m.row_id == format!("{request}/assistant"))
            else {
                continue;
            };
            let text = event.payload["assistantText"]
                .as_str()
                .ok_or(Error::Invalid)?;
            let protocol = delivery.batch.protocol()?;
            for block in parse_comment_replies(text) {
                let Ok(reply) = block.link(
                    prompt.target.clone(),
                    answer.target.clone(),
                    &protocol,
                    text,
                ) else {
                    continue;
                };
                let thread = match &reply.comments[0] {
                    ReactionTargetV1::HumanMessage { thread_id, .. } => *thread_id,
                    ReactionTargetV1::ReviewComment { .. } => {
                        let Some(thread) = self.read(|s| {
                            Ok(saved
                                .view
                                .conversation_id
                                .as_ref()
                                .and_then(|id| s.conversations.get(id))
                                .and_then(|c| c.discussion_thread))
                        })?
                        else {
                            continue;
                        };
                        thread
                    }
                    _ => continue,
                };
                let key = format!(
                    "answer/{id}/{request}/{}/{}",
                    answer.target.exact_key().map_err(|_| Error::Invalid)?,
                    block.answer.start
                );
                let existing = self.read(|s| Ok(s.answer_links.get(&key).cloned()))?;
                if existing.as_ref().is_some_and(|link| link.published) {
                    continue;
                }
                let pending = if let Some(existing) = existing {
                    existing
                } else {
                    let expected_thread = self
                        .home_thread_revision(saved.view.repository, &saved.profile, thread)
                        .await?;
                    let pending = SavedAnswerLink {
                        published: false,
                        request: Request {
                            version: 1,
                            request_id: RequestId::new(publication::identity(&key)),
                            command: CollabCommandV1::Thread {
                                intent: ThreadCommandV1::Link {
                                    thread_id: thread,
                                    expected_thread,
                                    link_id: ThreadLinkId::new(publication::identity(&key)),
                                    target: ThreadLinkTargetV1::CommentReply { reply },
                                    observed_primary: None,
                                },
                            },
                        },
                    };
                    self.update(|s| {
                        if s.answer_links.len() >= 4096 {
                            return Err(Error::Limit);
                        }
                        s.answer_links.insert(key.clone(), pending.clone());
                        Ok(())
                    })?;
                    pending
                };
                let outcome = self
                    .apply_home_thread_command(
                        saved.view.repository,
                        &saved.profile,
                        &pending.request,
                    )
                    .await;
                match outcome {
                    Ok(Some(_)) => {
                        self.update(|s| {
                            s.answer_links
                                .get_mut(&key)
                                .ok_or(Error::Invalid)?
                                .published = true;
                            Ok(())
                        })?;
                    }
                    Err(Error::Stale) => {
                        if self
                            .home_thread_command_not_known(
                                saved.view.repository,
                                &saved.profile,
                                pending.request.request_id,
                            )
                            .await?
                        {
                            let expected = self
                                .home_thread_revision(saved.view.repository, &saved.profile, thread)
                                .await?;
                            // A different, proven-unpublished precondition is a new attempt;
                            // the durable logical link identity and exact answer never change.
                            self.update(|s| {
                                let pending = s.answer_links.get_mut(&key).ok_or(Error::Invalid)?;
                                if let CollabCommandV1::Thread {
                                    intent:
                                        ThreadCommandV1::Link {
                                            expected_thread, ..
                                        },
                                } = &mut pending.request.command
                                {
                                    *expected_thread = expected;
                                }
                                let bytes = serde_json::to_string(&pending.request.command)
                                    .map_err(|_| Error::Invalid)?;
                                pending.request.request_id = RequestId::new(publication::identity(
                                    &format!("{key}/{bytes}"),
                                ));
                                Ok(())
                            })?;
                        }
                        return Err(Error::Stale);
                    }
                    Ok(None) => return Err(Error::Unavailable),
                    Err(error) => return Err(error),
                }
            }
        }
        Ok(())
    }
}
