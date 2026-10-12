//! Shared CLI input admission; both providers receive the same typed intent.
use super::*;
pub(super) fn read(
    command: &AgentCommand,
) -> Result<Option<(m::RequestId, CollabCommandV1)>, CommandError> {
    let (id, intent) = match command {
        AgentCommand::Thread {
            command:
                ThreadCliCommand::Create {
                    request_id,
                    thread_id,
                    title,
                    baseline,
                },
        } => (
            *request_id,
            CollabCommandV1::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: *thread_id,
                    title: title.clone(),
                    baseline: baseline.as_ref().map(|p| document(p)).transpose()?,
                },
            },
        ),
        AgentCommand::Thread {
            command:
                command @ (ThreadCliCommand::Apply { request_id, intent }
                | ThreadCliCommand::Message { request_id, intent }),
        } => {
            let intent: ThreadCommandV1 = document(intent)?;
            if matches!(command, ThreadCliCommand::Message { .. })
                && !matches!(intent, ThreadCommandV1::Message { .. })
            {
                return Err(error("thread message requires a typed message intent"));
            }
            (*request_id, CollabCommandV1::Thread { intent })
        }
        AgentCommand::Reaction {
            command:
                ReactionCliCommand::Set {
                    request_id,
                    target,
                    value,
                    observed,
                },
        } => (
            *request_id,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::Set {
                    emoji: None,
                    target: document(target)?,
                    value: document(value)?,
                    observed: observed
                        .as_ref()
                        .map(|p| document::<BTreeSet<CollabObjectRef>>(p))
                        .transpose()?
                        .unwrap_or_default(),
                },
            },
        ),
        AgentCommand::Reaction {
            command:
                ReactionCliCommand::Clear {
                    request_id,
                    target,
                    observed,
                },
        } => (
            *request_id,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::Clear {
                    target: document(target)?,
                    observed: document(observed)?,
                },
            },
        ),
        AgentCommand::Reaction {
            command:
                ReactionCliCommand::Resolve {
                    request_id,
                    target,
                    observed,
                    value,
                },
        } => (
            *request_id,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::ResolveConflict {
                    target: document(target)?,
                    observed: document(observed)?,
                    value: value.as_ref().map(|p| document(p)).transpose()?,
                },
            },
        ),
        AgentCommand::Reaction {
            command:
                ReactionCliCommand::DefineEmoji {
                    request_id,
                    emoji_id,
                    expected,
                    shortcode,
                    label,
                    asset,
                },
        } => (
            *request_id,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Define {
                    emoji_id: *emoji_id,
                    expected: expected.map(definition),
                    shortcode: shortcode.clone(),
                    label: label.clone(),
                    bytes: bytes(asset, MAX_IMAGE_BYTES)?,
                },
            },
        ),
        AgentCommand::Reaction {
            command:
                ReactionCliCommand::RetireEmoji {
                    request_id,
                    emoji_id,
                    expected,
                },
        } => (
            *request_id,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Retire {
                    emoji_id: *emoji_id,
                    expected: definition(*expected),
                },
            },
        ),
        _ => return Ok(None),
    };
    Ok(Some((id, intent)))
}
