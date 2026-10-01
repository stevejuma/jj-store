use crate::{
    Access, Context, Error, Result, State,
    routes::{Empty, Input, input},
};
use axum::{
    Json,
    extract::{State as ExtractState, rejection::JsonRejection},
    http::StatusCode,
    response::Response,
};
use jj_store_service::{
    auth::Permission,
    local_management::{self as management, LinkWorkspace, WorkspaceManager},
};
use std::sync::Arc;

async fn manager<'a>(state: &'a State, context: &Context) -> Result<&'a Arc<dyn WorkspaceManager>> {
    if !matches!(
        state.config.access,
        Access::LocalTrusted {
            permission: Permission::Admin,
            ..
        }
    ) {
        return Err(Error::denied());
    }
    context
        .session
        .principal(Permission::Admin)
        .await
        .map_err(crate::reads::Error::from)?;
    state
        .config
        .local_workspaces
        .as_ref()
        .ok_or_else(Error::unavailable)
}

fn failure(error: management::Error) -> Error {
    let (status, code) = match error {
        management::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_workspace_binding"),
        management::Error::UnsupportedProfile => {
            (StatusCode::BAD_REQUEST, "workspace_profile_unavailable")
        }
        management::Error::StaleRevision => (StatusCode::CONFLICT, "workspace_links_changed"),
        management::Error::Limit => (StatusCode::PAYLOAD_TOO_LARGE, "workspace_link_limit"),
        management::Error::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "workspace_links_unavailable",
        ),
    };
    Error::new(status, code)
}

pub(crate) async fn local_workspaces(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    state.json(&manager.list(context.id("repo")?).await.map_err(failure)?)
}

pub(crate) async fn link_workspace(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<Json<LinkWorkspace>, JsonRejection>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(request) = body.map_err(|_| Error::invalid())?;
    state.json(
        &manager
            .link(context.id("repo")?, request)
            .await
            .map_err(failure)?,
    )
}
