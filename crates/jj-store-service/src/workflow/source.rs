//! Authorised semantic events reconstructed from selected source publications.
use super::*;
use crate::{
    auth::Permission,
    native::NativeSession,
    reads::{OpenRead, ReadLimits},
};
use jj_store_jj::query::content::{ContentReader, Limits, operations};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowEventQuery")]
pub struct EventQuery {
    pub epoch: Option<m::EpochId>,
    pub after: Option<m::WalPosition>,
    pub limit: Option<usize>,
    pub until: Option<m::WalPosition>,
    /// JSON array of bounded WorkflowConditions (portable through URL query transports).
    pub filters: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "WorkflowEventPage")]
pub struct EventPage {
    pub version: u32,
    pub epoch: m::EpochId,
    pub after: m::WalPosition,
    pub upper: m::WalPosition,
    pub events: Vec<SourceEvent>,
    pub unavailable: Vec<String>,
}
impl WorkflowService {
    pub async fn source_events(
        &self,
        session: Arc<NativeSession>,
        query: EventQuery,
    ) -> Result<EventPage> {
        let limit = query.limit.unwrap_or(16);
        if !(1..=32).contains(&limit) {
            return Err(Error::Limit);
        }
        let filters: Vec<Conditions> = match query.filters {
            Some(json) if json.len() <= 24 * 1024 => {
                serde_json::from_str(&json).map_err(|_| Error::Invalid)?
            }
            Some(_) => return Err(Error::Limit),
            None => Vec::new(),
        };
        if filters.len() > 32 {
            return Err(Error::Limit);
        }
        for filter in &filters {
            filter.validate().map_err(|_| Error::Invalid)?;
        }
        session.principal(Permission::Read).await?;
        let snapshot = session.snapshot().await?;
        if query
            .epoch
            .is_some_and(|epoch| epoch != snapshot.manifest().epoch)
        {
            return Err(Error::Stale);
        }
        let mut page = EventPage {
            version: 1,
            epoch: snapshot.manifest().epoch,
            after: query.after.unwrap_or(snapshot.manifest().published),
            upper: query.until.unwrap_or(snapshot.manifest().published),
            events: Vec::new(),
            unavailable: Vec::new(),
        };
        if page.after > page.upper || page.upper > snapshot.manifest().published {
            return Err(Error::Stale);
        }
        let records = snapshot
            .selected_records(session.object_store.clone(), page.after, limit)
            .await?;
        let reader = ContentReader::new(Limits::default(), 1).map_err(|_| Error::Unavailable)?;
        for record in records {
            if record.transaction().after.position > page.upper {
                break;
            }
            for (publication, operations) in session
                .coordinator
                .workflow_source_publications(&record)
                .await?
            {
                for operation in operations {
                    let OpenRead::Pinned(read) = session
                        .open_read(Some(operation), ReadLimits::default())
                        .await
                        .map_err(|_| Error::Unavailable)?
                    else {
                        return Err(Error::Invalid);
                    };
                    let read = Arc::new(read);
                    let diff = read
                        .operation_diff(
                            &reader,
                            operations::Request {
                                operation,
                                parent: None,
                            },
                        )
                        .await
                        .map_err(|_| Error::Unavailable)?;
                    match diff.data {
                        operations::Data::Difference(diff) => {
                            for commit in &diff.view.heads.added {
                                page.events.push(
                                    SourceEvent::new(
                                        publication.clone(),
                                        EventKind::CodePublished,
                                        Source {
                                            repository: session.repository,
                                            operation: Some(operation),
                                            commit: Some(*commit),
                                            bookmark: None,
                                            conversation: None,
                                        },
                                    )
                                    .map_err(|_| Error::Corrupt)?,
                                );
                            }
                            for change in diff.view.local_bookmarks {
                                if let Some(target) = change.after {
                                    if let [Some(commit)] = target.0.as_slice() {
                                        page.events.push(
                                            SourceEvent::new(
                                                publication.clone(),
                                                EventKind::BookmarkChanged,
                                                Source {
                                                    repository: session.repository,
                                                    operation: Some(operation),
                                                    commit: Some(*commit),
                                                    bookmark: Some(change.name),
                                                    conversation: None,
                                                },
                                            )
                                            .map_err(|_| Error::Corrupt)?,
                                        );
                                    } else {
                                        page.unavailable.push(format!(
                                            "{operation}: ambiguous bookmark {}",
                                            change.name
                                        ));
                                    }
                                }
                            }
                        }
                        operations::Data::Root { .. } => {}
                        operations::Data::SelectParent { .. } => page
                            .unavailable
                            .push(format!("{operation}: operation has multiple parents")),
                    }
                    if page.events.len() > 4096 {
                        return Err(Error::Limit);
                    }
                }
            }
            for observed in session
                .coordinator
                .workflow_conversation_events(&record)
                .await?
            {
                let kind = if observed.source.reaction.is_some() {
                    EventKind::ReactionAdded
                } else {
                    EventKind::MessagePosted
                };
                let mut event = SourceEvent::new(
                    format!(
                        "{}/{}/{}",
                        session.repository,
                        record.transaction().after.epoch,
                        record.transaction().receipt.digest
                    ),
                    kind,
                    Source {
                        repository: session.repository,
                        operation: None,
                        commit: None,
                        bookmark: None,
                        conversation: Some(observed.source),
                    },
                )
                .map_err(|_| Error::Corrupt)?;
                let context = event.source.conversation.as_ref().ok_or(Error::Corrupt)?;
                for filter in &filters {
                    let matched = filter.revset.is_none()
                        && filter.matches_content(
                            &observed.actor.to_string(),
                            &observed.text,
                            Some(&context.thread.to_string()),
                            context.message.map(|v| v.to_string()).as_deref(),
                            observed.emoji.as_deref(),
                        );
                    event
                        .filter_results
                        .insert(filter.id().map_err(|_| Error::Invalid)?, matched);
                }
                page.events.push(event);
            }
            page.after = record.transaction().after.position;
        }
        if !filters.is_empty() {
            let cache = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
            let graph = jj_store_jj::query::GraphExecutor::new(
                cache.path().to_path_buf(),
                Default::default(),
                1,
            )
            .map_err(|_| Error::Unavailable)?;
            for event in &mut page.events {
                if event.source.conversation.is_none() {
                    super::matching::code_filters(&session, &graph, event, &filters).await?;
                }
            }
        }
        if page.events.len() > 4096
            || serde_json::to_vec(&page).map_err(|_| Error::Corrupt)?.len() > 3 * 1024 * 1024
        {
            return Err(Error::Limit);
        }
        session.principal(Permission::Read).await?;
        if session.snapshot().await?.manifest().epoch != page.epoch {
            return Err(Error::Stale);
        }
        Ok(page)
    }
}
