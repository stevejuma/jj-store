//! Live source dependencies are admitted through the selected store participants.
use super::*;
use jj_collab::{
    ReactionTargetV1 as Target, ThreadCommandV1 as Thread, ThreadLinkTargetV1 as Link,
};

pub fn reaction_source_store(target: &Target) -> Option<StoreKind> {
    Some(match target {
        Target::Thread { .. } | Target::HumanMessage { .. } => StoreKind::Thread,
        Target::AgentMessage { .. }
        | Target::ToolInvocation { .. }
        | Target::ToolOutput { .. }
        | Target::AgentCheckpoint { .. } => StoreKind::Agent,
        Target::ReviewThread { .. } | Target::ReviewComment { .. } => StoreKind::Review,
        Target::WorkComment { .. } => StoreKind::Work,
        // This source has its own explicit, separately authorized capability.
        Target::RepositoryReviewComment { .. } => return None,
    })
}
pub fn thread_source_stores(command: &Thread) -> BTreeSet<StoreKind> {
    let mut sources = BTreeSet::new();
    for source in command.conversation_sources() {
        sources.extend(reaction_source_store(&source.target));
    }
    for target in command.comment_references() {
        sources.extend(reaction_source_store(target));
    }
    match command {
        Thread::CreateAgentDiscussion { target, .. } => {
            sources.extend(reaction_source_store(target));
        }
        Thread::SetMetadata { value, .. } if !value.source_refs().is_empty() => {
            sources.insert(StoreKind::Agent);
        }
        Thread::Link { target, .. } => match target {
            Link::CommentReply { reply } => {
                for target in reply.targets() {
                    sources.extend(reaction_source_store(target));
                }
            }
            Link::CommentBatch { .. } => {}
            Link::CaptureGap { .. } | Link::Session { .. } => {
                sources.insert(StoreKind::Agent);
            }
            Link::Review { .. } => {
                sources.insert(StoreKind::Review);
            }
            Link::Code { evidence, .. } => {
                if evidence.is_some() {
                    sources.insert(StoreKind::Agent);
                }
            }
        },
        _ => {}
    }
    sources
}
pub fn review_source_stores(command: &jj_review::ReviewCommandV1) -> Result<BTreeSet<StoreKind>> {
    let mut sources = BTreeSet::new();
    if let jj_review::ReviewCommandV1::CreateThread { anchor, .. } = command
        && !anchor
            .agent_dependencies()
            .map_err(|_| Error::Invalid("review anchor"))?
            .is_empty()
    {
        sources.insert(StoreKind::Agent);
    }
    Ok(sources)
}
