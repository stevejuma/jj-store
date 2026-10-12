//! Explicit repository Write onboarding, reusing native administration receipts.
use crate::*;
use axum::{
    Extension, Json,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use jj_store_service::administration as a;
use std::collections::BTreeSet;
#[derive(Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentAccessSnapshot")]
struct Snapshot {
    epoch: m::EpochId,
    position: u64,
    #[serde(default)]
    code_state_version: u64,
    heads: BTreeSet<m::OperationId>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentAccessGrant")]
struct Grant {
    request_id: m::RequestId,
    expected: Snapshot,
    principal: m::PrincipalId,
    credential: String,
}
#[derive(Serialize, schemars::JsonSchema)]
#[schemars(rename = "AgentAccessOutcome")]
struct Outcome {
    request_id: m::RequestId,
    published: bool,
}
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/repos/{repo}/agent-access", get(snapshot).post(grant))
        .route("/repos/{repo}/agent-access/status", post(status))
        .layer(DefaultBodyLimit::max(16 * 1024))
}
fn session(
    state: &State,
    headers: &HeaderMap,
    repository: m::RepositoryId,
) -> Result<Arc<jj_store_service::native::NativeSession>> {
    if !matches!(state.config.access, Access::Authenticated) {
        return Err(Error::denied());
    }
    let endpoint = state
        .host
        .as_ref()
        .ok_or_else(Error::unavailable)?
        .endpoint(repository)?;
    Ok(endpoint
        .bind(crate::repositories::bearer(headers)?)?
        .into_session())
}
async fn snapshot(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let listing = session(&state, &headers, repository)?
        .tokens()
        .await
        .map_err(reads::Error::from)?;
    state.json(&Snapshot {
        epoch: listing.snapshot.epoch,
        position: listing.snapshot.position.0,
        code_state_version: listing.snapshot.code_state_version,
        heads: listing.snapshot.heads,
    })
}
async fn apply(
    state: &State,
    headers: &HeaderMap,
    repository: m::RepositoryId,
    body: Grant,
    check: bool,
) -> Result<Response> {
    let session = session(state, headers, repository)?;
    session
        .principal(Permission::Admin)
        .await
        .map_err(reads::Error::from)?;
    let bytes = zeroize::Zeroizing::new(body.credential);
    let credential = BearerToken::parse(&bytes).map_err(|_| Error::invalid())?;
    let principal = state
        .config
        .directory
        .as_ref()
        .ok_or_else(Error::unavailable)?
        .principal(&jj_store_service::directory::Caller::Token(
            credential.clone(),
        ))
        .await
        .map_err(|_| Error::denied())?;
    if principal != body.principal {
        return Err(Error::denied());
    }
    let request = a::TokenRequest {
        request_id: body.request_id,
        expected: a::AdministrationSnapshot {
            epoch: body.expected.epoch,
            position: m::WalPosition(body.expected.position),
            code_state_version: body.expected.code_state_version,
            heads: body.expected.heads,
        },
        change: a::TokenChange::Grant {
            credential,
            principal: body.principal,
            permission: Permission::Write,
            expires_at: None,
        },
    };
    let receipt = if check {
        session
            .token_status(&request)
            .await
            .map_err(reads::Error::from)?
    } else {
        match session
            .change_token(&request)
            .await
            .map_err(reads::Error::from)?
        {
            jj_store_service::transport::PublishOutcome::Committed(receipt) => Some(receipt),
            jj_store_service::transport::PublishOutcome::OutcomeUnknown { .. } => None,
        }
    };
    if let Some(receipt) = &receipt {
        let result: a::TokenResult =
            serde_json::from_slice(&receipt.result).map_err(|_| Error::unavailable())?;
        if let a::TokenResult::Granted { token } = result {
            state
                .config
                .registry
                .as_ref()
                .ok_or_else(Error::unavailable)?
                .token_candidate(repository, token.id, true)
                .await
                .map_err(|_| Error::unavailable())?;
        }
    }
    session
        .principal(Permission::Admin)
        .await
        .map_err(|_| Error::unavailable())?;
    state.json(&Outcome {
        request_id: body.request_id,
        published: receipt.is_some(),
    })
}
async fn grant(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<mutations::Pending>,
    Json(body): Json<Grant>,
) -> Result<Response> {
    routes::input(args)?;
    pending.set(body.request_id);
    apply(&state, &headers, repository, body, false).await
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    Extension(pending): Extension<mutations::Pending>,
    Json(body): Json<Grant>,
) -> Result<Response> {
    routes::input(args)?;
    pending.set(body.request_id);
    apply(&state, &headers, repository, body, true).await
}
pub(super) fn document(doc: &mut contract::Document) {
    use contract::Reply;
    doc.endpoint::<routes::Empty, Snapshot>(
        "get",
        "/repos/{repo}/agent-access",
        "agent_access_snapshot",
        Reply::Json(200),
    );
    for (path, id) in [
        ("/repos/{repo}/agent-access", "agent_access_grant"),
        ("/repos/{repo}/agent-access/status", "agent_access_status"),
    ] {
        doc.endpoint::<routes::Empty, Outcome>("post", path, id, Reply::Json(200));
        doc.body::<Grant>(&format!("/v1{path}"), "post");
    }
}
