use super::*;
use crate::binding::Connection;
use jj_store_rpc::client::NativeClient;
use jj_store_service::{collab_mutations as mutations, collab_pages::Query};
pub(super) async fn apply(
    connection: &Connection,
    id: m::RequestId,
    command: CollabCommandV1,
) -> Result<serde_json::Value, CommandError> {
    let client = connection.connect().await.map_err(error)?;
    let request = mutations::Request {
        version: 1,
        request_id: id,
        command,
    };
    let result = match client.apply_collab_command(&request).await {
        Ok(result) => result,
        Err(status)
            if matches!(
                status.code(),
                tonic::Code::DeadlineExceeded
                    | tonic::Code::Unavailable
                    | tonic::Code::Cancelled
                    | tonic::Code::Unknown
                    | tonic::Code::Internal
                    | tonic::Code::DataLoss
                    | tonic::Code::ResourceExhausted
            ) =>
        {
            mutations::Outcome::OutcomeUnknown { request_id: id }
        }
        Err(status) => return Err(error(status)),
    };
    value(&result)
}
fn cursor(
    after: bool,
    metadata: Option<GitOid>,
    cursor: &Option<String>,
) -> Result<Option<&str>, CommandError> {
    if after || metadata.is_some() {
        return Err(error(
            "native pages use --cursor; --after and --at-metadata belong to the local Git profile",
        ));
    }
    Ok(cursor.as_deref())
}
async fn page(
    client: &NativeClient,
    query: Query,
    cursor: Option<&str>,
) -> Result<serde_json::Value, CommandError> {
    value(&client.collab_page(&query, cursor).await.map_err(error)?)
}
pub(super) async fn read(
    connection: &Connection,
    command: &AgentCommand,
) -> Result<serde_json::Value, CommandError> {
    let client = connection.connect().await.map_err(error)?;
    match command {
        AgentCommand::Thread { command } => match command {
            ThreadCliCommand::List {
                limit,
                after,
                at_metadata,
                cursor: token,
            } => {
                page(
                    &client,
                    Query::Threads { limit: *limit },
                    cursor(after.is_some(), *at_metadata, token)?,
                )
                .await
            }
            ThreadCliCommand::Show { thread_id } => {
                value(&client.collab_thread(*thread_id).await.map_err(error)?)
            }
            ThreadCliCommand::Status { request_id } => value(
                &client
                    .collab_command_status(*request_id)
                    .await
                    .map_err(error)?,
            ),
            _ => Err(error("collaboration intent dispatch failed")),
        },
        AgentCommand::Reaction { command } => match command {
            ReactionCliCommand::Query {
                target,
                limit,
                after,
                at_metadata,
                cursor: token,
            } => {
                page(
                    &client,
                    Query::Reactions {
                        target: document(target)?,
                        limit: *limit,
                    },
                    cursor(after.is_some(), *at_metadata, token)?,
                )
                .await
            }
            ReactionCliCommand::Emojis {
                limit,
                after,
                at_metadata,
                cursor: token,
            } => {
                page(
                    &client,
                    Query::Emojis { limit: *limit },
                    cursor(after.is_some(), *at_metadata, token)?,
                )
                .await
            }
            ReactionCliCommand::Status { request_id } => value(
                &client
                    .collab_command_status(*request_id)
                    .await
                    .map_err(error)?,
            ),
            _ => Err(error("collaboration intent dispatch failed")),
        },
        _ => Err(error("collaboration read dispatch failed")),
    }
}
