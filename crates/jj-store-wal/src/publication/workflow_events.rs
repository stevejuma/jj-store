//! Selected collaboration publications exposed to authorised source readers.
use super::collab_store::collab_error;
use super::*;
use jj_collab::{CollabCommandResultV1, CollabObjectV1, ReactionTargetV1, ReactionValueV1};

/// Transient source content. Workflow evaluations persist references and condition results only.
pub struct WorkflowConversationEvent {
    pub source: jj_workflow::Conversation,
    pub actor: PrincipalId,
    pub text: String,
    pub emoji: Option<String>,
}
impl Coordinator {
    pub async fn workflow_conversation_events(
        &self,
        record: &SelectedRecord,
    ) -> Result<Vec<WorkflowConversationEvent>> {
        let tx = record.transaction();
        if tx.repository != self.records.repository {
            return Err(Error::Denied);
        }
        if !matches!(
            tx.kind,
            SemanticKind::ThreadStateChanged | SemanticKind::ReactionStateChanged
        ) || !tx.origins.is_empty()
        {
            return Ok(Vec::new());
        }
        let receipt: Receipt = self.records.get(tx.receipt, RecordKind::Receipt).await?;
        let collab = CollabCommandReceiptV1::decode(&receipt.result)
            .map_err(|_| Error::Corrupt("workflow collaboration receipt"))?;
        let domains = tx
            .after_domains
            .ok_or(Error::Corrupt("workflow collaboration domains"))?;
        let mut events = Vec::new();
        match collab.result {
            CollabCommandResultV1::Thread { result } => {
                let Some(revision) = result.message else {
                    return Ok(events);
                };
                let root = domains
                    .thread
                    .ok_or(Error::Corrupt("workflow thread root"))?
                    .root;
                let threads = self.native_thread_store(root).await?;
                let CollabObjectV1::HumanMessageRevision(message) =
                    threads.graph.get(revision).map_err(collab_error)?
                else {
                    return Err(Error::Corrupt("workflow message kind"));
                };
                if message.previous.is_none()
                    && let Some(body) = &message.body
                {
                    events.push(WorkflowConversationEvent {
                        source: jj_workflow::Conversation {
                            thread: message.thread_id,
                            message: Some(message.message_id),
                            revision,
                            reaction: None,
                            position: tx.after.position,
                        },
                        actor: message.actor,
                        text: body.clone(),
                        emoji: None,
                    });
                }
            }
            CollabCommandResultV1::Reaction { result } => {
                let root = domains
                    .reaction
                    .ok_or(Error::Corrupt("workflow reaction root"))?
                    .root;
                let reactions = self.native_reaction_store(root).await?;
                let CollabObjectV1::ReactionSlot(slot) =
                    reactions.graph.get(result.slot).map_err(collab_error)?
                else {
                    return Err(Error::Corrupt("workflow reaction slot"));
                };
                let mut previous = BTreeSet::new();
                if let Some(root) = tx.before_domains.and_then(|d| d.reaction) {
                    let before = self.native_reaction_store(root.root).await?;
                    if let Some(reference) = before
                        .graph
                        .catalog(before.root.slots)
                        .map_err(collab_error)?
                        .get(&slot.key().map_err(collab_error)?)
                    {
                        let CollabObjectV1::ReactionSlot(old) =
                            before.graph.get(*reference).map_err(collab_error)?
                        else {
                            return Err(Error::Corrupt("workflow old reaction slot"));
                        };
                        previous = old.additions.clone();
                    }
                }
                for reference in slot.additions.difference(&previous) {
                    let CollabObjectV1::Reaction(reaction) =
                        reactions.graph.get(*reference).map_err(collab_error)?
                    else {
                        return Err(Error::Corrupt("workflow reaction kind"));
                    };
                    let (thread, message, revision) = match reaction.target {
                        ReactionTargetV1::HumanMessage {
                            thread_id,
                            message_id,
                            revision,
                        } => (thread_id, Some(message_id), revision),
                        ReactionTargetV1::Thread { thread_id } => {
                            let root = domains
                                .thread
                                .ok_or(Error::Corrupt("workflow reaction thread root"))?
                                .root;
                            let threads = self.native_thread_store(root).await?;
                            let revision = *threads
                                .state
                                .threads
                                .get(&thread_id)
                                .ok_or(Error::Corrupt("workflow reaction thread"))?;
                            (thread_id, None, revision)
                        }
                        _ => continue,
                    };
                    let emoji = match &reaction.value {
                        ReactionValueV1::Like => "+".into(),
                        ReactionValueV1::Dislike => "-".into(),
                        ReactionValueV1::Emoji { sequence } => sequence.clone(),
                        ReactionValueV1::Custom { emoji_id, .. } => format!("custom:{emoji_id}"),
                    };
                    events.push(WorkflowConversationEvent {
                        source: jj_workflow::Conversation {
                            thread,
                            message,
                            revision,
                            reaction: Some(*reference),
                            position: tx.after.position,
                        },
                        actor: reaction.actor,
                        text: String::new(),
                        emoji: Some(emoji),
                    });
                }
            }
            CollabCommandResultV1::Emoji { .. } => {}
        }
        Ok(events)
    }
}
