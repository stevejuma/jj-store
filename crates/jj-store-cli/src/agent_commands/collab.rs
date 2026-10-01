use super::{AgentCommand, Backend, CommandError, error};
use clap::Subcommand;
use jj_collab::*;
use jj_store_jj::local_metadata::GitOid;
use jj_store_model as m;
use jj_store_service::local_repository::{LocalCollabRequest, LocalRepository};
use serde::{Serialize, de::DeserializeOwned};
use std::result::Result;
use std::{
    collections::BTreeSet,
    io::Read as _,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Subcommand)]
pub enum ThreadCliCommand {
    List {
        #[arg(long, default_value_t = 50)]
        limit: u32,
        #[arg(long)]
        after: Option<ThreadId>,
        #[arg(long)]
        at_metadata: Option<GitOid>,
    },
    Show {
        thread_id: ThreadId,
    },
    Create {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        thread_id: ThreadId,
        #[arg(long)]
        title: String,
        #[arg(long)]
        baseline: Option<PathBuf>,
    },
    /// Apply a bounded typed intent, including rename, lifecycle and exact links
    Apply {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        intent: PathBuf,
    },
    /// Post, revise or tombstone a human message using a typed message intent
    Message {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        intent: PathBuf,
    },
    Status {
        request_id: m::RequestId,
    },
}
#[derive(Clone, Debug, Subcommand)]
pub enum ReactionCliCommand {
    Set {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        value: PathBuf,
        #[arg(long)]
        observed: Option<PathBuf>,
    },
    Clear {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        observed: PathBuf,
    },
    Resolve {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        observed: PathBuf,
        #[arg(long)]
        value: Option<PathBuf>,
    },
    Query {
        #[arg(long)]
        target: PathBuf,
        #[arg(long, default_value_t = 50)]
        limit: u32,
        #[arg(long)]
        after: Option<m::PrincipalId>,
        #[arg(long)]
        at_metadata: Option<GitOid>,
    },
    DefineEmoji {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        emoji_id: CustomEmojiId,
        #[arg(long)]
        expected: Option<CollabObjectId>,
        #[arg(long)]
        shortcode: String,
        #[arg(long)]
        label: String,
        #[arg(long)]
        asset: PathBuf,
    },
    RetireEmoji {
        #[arg(long)]
        request_id: m::RequestId,
        #[arg(long)]
        emoji_id: CustomEmojiId,
        #[arg(long)]
        expected: CollabObjectId,
    },
    Emojis {
        #[arg(long, default_value_t = 50)]
        limit: u32,
        #[arg(long)]
        after: Option<CustomEmojiId>,
        #[arg(long)]
        at_metadata: Option<GitOid>,
    },
    Status {
        request_id: m::RequestId,
    },
}
fn bytes(path: &Path, limit: usize) -> Result<Vec<u8>, CommandError> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .map_err(error)?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(error)?;
    if bytes.is_empty() || bytes.len() > limit {
        return Err(error("collaboration input exceeds its byte limit"));
    }
    Ok(bytes)
}
fn document<T: DeserializeOwned>(path: &Path) -> Result<T, CommandError> {
    serde_json::from_slice(&bytes(path, 16 * 1024 * 1024)?).map_err(error)
}
fn value(data: &impl Serialize) -> Result<serde_json::Value, CommandError> {
    let encoded = serde_json::to_vec(data).map_err(error)?;
    if encoded.len() > 4 * 1024 * 1024 {
        return Err(error("collaboration response exceeds four MiB"));
    }
    serde_json::from_slice(&encoded).map_err(error)
}
fn definition(id: CollabObjectId) -> CollabObjectRef {
    CollabObjectRef {
        kind: CollabObjectKind::CustomEmojiVersion,
        id,
    }
}
fn continuation(
    after: bool,
    expected: Option<GitOid>,
    actual: Option<GitOid>,
    limit: u32,
) -> Result<(), CommandError> {
    if !(1..=256).contains(&limit) || (after && expected.is_none()) {
        return Err(error(
            "continuation requires --at-metadata and a page limit from 1 to 256",
        ));
    }
    if expected.is_some() && expected != actual {
        return Err(error("metadata snapshot changed; refresh explicitly"));
    }
    Ok(())
}
async fn apply(
    repo: &LocalRepository,
    id: m::RequestId,
    command: CollabCommandV1,
) -> Result<serde_json::Value, CommandError> {
    value(
        &repo
            .apply_collab_command(&LocalCollabRequest {
                version: 1,
                request_id: id,
                command,
            })
            .await
            .map_err(error)?,
    )
}
impl Backend {
    pub(super) async fn collab_action(
        &self,
        command: &AgentCommand,
    ) -> Result<Option<serde_json::Value>, CommandError> {
        if !matches!(
            command,
            AgentCommand::Thread { .. } | AgentCommand::Reaction { .. }
        ) {
            return Ok(None);
        }
        let Self::Local(repo) = self else {
            return Err(error(
                "native Thread/Reaction CLI transport is not qualified yet",
            ));
        };
        let output = match command {
            AgentCommand::Thread { command } => match command {
                ThreadCliCommand::List {
                    limit,
                    after,
                    at_metadata,
                } => {
                    let store = repo.thread_store().await.map_err(error)?;
                    continuation(
                        after.is_some(),
                        *at_metadata,
                        store.snapshot.metadata_commit,
                        *limit,
                    )?;
                    let mut threads = Vec::new();
                    for (id, reference) in &store.state.threads {
                        if after.is_some_and(|a| *id <= a) {
                            continue;
                        }
                        let CollabObjectV1::ThreadVersion(thread) =
                            store.object(*reference).map_err(error)?
                        else {
                            return Err(error("invalid selected Thread"));
                        };
                        if thread.status == ThreadStatusV1::Tombstoned {
                            continue;
                        }
                        threads.push(serde_json::json!({"reference":reference,"value":thread}));
                        if threads.len() > *limit as usize {
                            break;
                        }
                    }
                    let more = threads.len() > *limit as usize;
                    threads.truncate(*limit as usize);
                    let next = if more {
                        threads.last().map(|t| t["value"]["thread_id"].clone())
                    } else {
                        None
                    };
                    value(
                        &serde_json::json!({"snapshot":store.snapshot,"threads":threads,"next_id":next}),
                    )?
                }
                ThreadCliCommand::Show { thread_id } => {
                    let store = repo.thread_store().await.map_err(error)?;
                    let reference = store
                        .state
                        .threads
                        .get(thread_id)
                        .ok_or_else(|| error("Thread was not found"))?;
                    let CollabObjectV1::ThreadVersion(thread) =
                        store.object(*reference).map_err(error)?
                    else {
                        return Err(error("invalid selected Thread"));
                    };
                    if thread.status == ThreadStatusV1::Tombstoned {
                        return Err(error("Thread was not found"));
                    }
                    value(
                        &serde_json::json!({"snapshot":store.snapshot,"reference":reference,"value":thread}),
                    )?
                }
                ThreadCliCommand::Create {
                    request_id,
                    thread_id,
                    title,
                    baseline,
                } => {
                    let baseline = baseline.as_ref().map(|p| document(p)).transpose()?;
                    apply(
                        repo,
                        *request_id,
                        CollabCommandV1::Thread {
                            intent: ThreadCommandV1::Create {
                                thread_id: *thread_id,
                                title: title.clone(),
                                baseline,
                            },
                        },
                    )
                    .await?
                }
                ThreadCliCommand::Apply { request_id, intent }
                | ThreadCliCommand::Message { request_id, intent } => {
                    let intent: ThreadCommandV1 = document(intent)?;
                    if matches!(command, ThreadCliCommand::Message { .. })
                        && !matches!(intent, ThreadCommandV1::Message { .. })
                    {
                        return Err(error("thread message requires a typed message intent"));
                    }
                    apply(repo, *request_id, CollabCommandV1::Thread { intent }).await?
                }
                ThreadCliCommand::Status { request_id } => {
                    value(&repo.collab_command_status(*request_id).map_err(error)?)?
                }
            },
            AgentCommand::Reaction { command } => match command {
                ReactionCliCommand::Set {
                    request_id,
                    target,
                    value: reaction,
                    observed,
                } => {
                    let observed = observed
                        .as_ref()
                        .map(|p| document::<BTreeSet<CollabObjectRef>>(p))
                        .transpose()?
                        .unwrap_or_default();
                    apply(
                        repo,
                        *request_id,
                        CollabCommandV1::Reaction {
                            intent: ReactionCommandV1::Set {
                                target: document(target)?,
                                observed,
                                value: document(reaction)?,
                            },
                        },
                    )
                    .await?
                }
                ReactionCliCommand::Clear {
                    request_id,
                    target,
                    observed,
                } => {
                    apply(
                        repo,
                        *request_id,
                        CollabCommandV1::Reaction {
                            intent: ReactionCommandV1::Clear {
                                target: document(target)?,
                                observed: document(observed)?,
                            },
                        },
                    )
                    .await?
                }
                ReactionCliCommand::Resolve {
                    request_id,
                    target,
                    observed,
                    value: reaction,
                } => {
                    apply(
                        repo,
                        *request_id,
                        CollabCommandV1::Reaction {
                            intent: ReactionCommandV1::ResolveConflict {
                                target: document(target)?,
                                observed: document(observed)?,
                                value: reaction.as_ref().map(|p| document(p)).transpose()?,
                            },
                        },
                    )
                    .await?
                }
                ReactionCliCommand::Query {
                    target,
                    limit,
                    after,
                    at_metadata,
                } => value(
                    &repo
                        .reaction_query(&document(target)?, *after, *limit, *at_metadata)
                        .await
                        .map_err(error)?,
                )?,
                ReactionCliCommand::DefineEmoji {
                    request_id,
                    emoji_id,
                    expected,
                    shortcode,
                    label,
                    asset,
                } => {
                    apply(
                        repo,
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
                    )
                    .await?
                }
                ReactionCliCommand::RetireEmoji {
                    request_id,
                    emoji_id,
                    expected,
                } => {
                    apply(
                        repo,
                        *request_id,
                        CollabCommandV1::Emoji {
                            intent: CustomEmojiCommandV1::Retire {
                                emoji_id: *emoji_id,
                                expected: definition(*expected),
                            },
                        },
                    )
                    .await?
                }
                ReactionCliCommand::Emojis {
                    limit,
                    after,
                    at_metadata,
                } => {
                    let store = repo.reaction_store().await.map_err(error)?;
                    continuation(
                        after.is_some(),
                        *at_metadata,
                        store.snapshot.metadata_commit,
                        *limit,
                    )?;
                    value(&store.emojis(*after, *limit).map_err(error)?)?
                }
                ReactionCliCommand::Status { request_id } => {
                    value(&repo.collab_command_status(*request_id).map_err(error)?)?
                }
            },
            _ => return Ok(None),
        };
        Ok(Some(output))
    }
}
