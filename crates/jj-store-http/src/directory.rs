use crate::*;
use axum::{
    Json,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use jj_store_service::directory as d;
pub(super) fn failure(error: d::Error) -> Error {
    let (status, code) = match error {
        d::Error::Denied => (StatusCode::FORBIDDEN, "directory_denied"),
        d::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_directory_input"),
        d::Error::Stale => (StatusCode::CONFLICT, "directory_changed"),
        d::Error::Limit => (StatusCode::PAYLOAD_TOO_LARGE, "directory_limit"),
        d::Error::Unavailable | d::Error::Corrupt => {
            (StatusCode::SERVICE_UNAVAILABLE, "directory_unavailable")
        }
        d::Error::Unknown => (StatusCode::SERVICE_UNAVAILABLE, "directory_outcome_unknown"),
    };
    Error::new(status, code)
}
pub(super) fn caller(state: &State, headers: &HeaderMap) -> Result<d::Caller> {
    match state.config.access {
        Access::LocalTrusted {
            principal,
            permission: Permission::Admin,
        } => Ok(d::Caller::Local(principal)),
        Access::Authenticated => Ok(d::Caller::Token(crate::repositories::bearer(headers)?)),
        _ => Err(Error::denied()),
    }
}
fn manager(state: &State) -> Result<&Arc<dyn d::Directory>> {
    state
        .config
        .directory
        .as_ref()
        .ok_or_else(Error::unavailable)
}
async fn operator(state: &State, headers: &HeaderMap) -> Result<d::Caller> {
    let token = crate::repositories::bearer(headers)?;
    let policy = state
        .config
        .directory_operators
        .as_ref()
        .ok_or_else(Error::denied)?
        .current()
        .await
        .map_err(|_| Error::denied())?;
    let now =
        jj_store_service::auth::Clock::unix_seconds(&SystemClock).map_err(|_| Error::denied())?;
    let principal = policy
        .authorize(&token, Permission::ManageIdentities, now)
        .map_err(|_| Error::denied())?;
    Ok(d::Caller::Operator(principal.id()))
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Revision {
    revision: String,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Profiles {
    ids: String,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Enroll {
    request_id: m::RequestId,
    principal: m::PrincipalId,
    credential: String,
    expires_at: Option<u64>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Revoke {
    request_id: m::RequestId,
    token: m::TokenId,
}
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/directory/profile", get(profile))
        .route("/directory/profiles", get(profiles))
        .route("/directory/emojis", get(emojis))
        .route("/directory/emoji-snapshot/{id}", get(emoji_snapshot))
        .route("/directory/commands", post(execute))
        .route("/directory/commands/{request_id}", get(receipt))
        .route("/directory/media/{digest}", get(media))
        .route("/directory/identities", get(identities).post(enroll))
        .route("/directory/identity-revoke", post(revoke))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
}
async fn profile(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    state.json(
        &manager(&state)?
            .profile(&caller(&state, &headers)?)
            .await
            .map_err(failure)?,
    )
}
async fn profiles(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<Profiles>,
) -> Result<Response> {
    let ids = routes::input(args)?
        .ids
        .split(',')
        .map(|s| s.parse().map_err(|_| Error::invalid()))
        .collect::<Result<Vec<_>>>()?;
    state.json(
        &manager(&state)?
            .profiles(&caller(&state, &headers)?, ids)
            .await
            .map_err(failure)?,
    )
}
async fn emoji_snapshot(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<m::RequestId>,
    args: routes::Input<Revision>,
) -> Result<Response> {
    let revision = routes::input(args)?.revision;
    state.json(
        &manager(&state)?
            .emoji_snapshot(&caller(&state, &headers)?, id, &revision)
            .await
            .map_err(failure)?,
    )
}
async fn emojis(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    state.json(
        &manager(&state)?
            .emojis(&caller(&state, &headers)?)
            .await
            .map_err(failure)?,
    )
}
async fn execute(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
    Json(request): Json<d::DirectoryRequest>,
) -> Result<Response> {
    routes::input(args)?;
    state.json(
        &manager(&state)?
            .execute(&caller(&state, &headers)?, request)
            .await
            .map_err(failure)?,
    )
}
async fn receipt(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    Path(request): Path<m::RequestId>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let caller = if let Ok(c) = operator(&state, &headers).await {
        c
    } else {
        caller(&state, &headers)?
    };
    state.json(
        &manager(&state)?
            .receipt(&caller, request)
            .await
            .map_err(failure)?,
    )
}
async fn media(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    Path(digest): Path<String>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let media = manager(&state)?
        .media(&caller(&state, &headers)?, &digest)
        .await
        .map_err(failure)?;
    Ok(([(header::CONTENT_TYPE, media.content_type)], media.bytes).into_response())
}
async fn identities(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    state.json(
        &manager(&state)?
            .identities(&operator(&state, &headers).await?)
            .await
            .map_err(failure)?,
    )
}
async fn enroll(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
    Json(request): Json<Enroll>,
) -> Result<Response> {
    routes::input(args)?;
    let credential = zeroize::Zeroizing::new(request.credential);
    let token = BearerToken::parse(&credential).map_err(|_| Error::invalid())?;
    state.json(
        &manager(&state)?
            .enroll(
                &operator(&state, &headers).await?,
                request.request_id,
                request.principal,
                &token,
                request.expires_at,
            )
            .await
            .map_err(failure)?,
    )
}
async fn revoke(
    ExtractState(state): ExtractState<Arc<State>>,
    headers: HeaderMap,
    args: routes::Input<routes::Empty>,
    Json(request): Json<Revoke>,
) -> Result<Response> {
    routes::input(args)?;
    state.json(
        &manager(&state)?
            .revoke(
                &operator(&state, &headers).await?,
                request.request_id,
                request.token,
            )
            .await
            .map_err(failure)?,
    )
}
pub(super) fn document(doc: &mut contract::Document) {
    use contract::Reply;
    doc.endpoint::<Revision, d::EmojiSelection>(
        "get",
        "/directory/emoji-snapshot/{id}",
        "directory_emoji_snapshot",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, d::UserProfile>(
        "get",
        "/directory/profile",
        "directory_profile",
        Reply::Json(200),
    );
    doc.endpoint::<Profiles, Vec<d::PublicProfile>>(
        "get",
        "/directory/profiles",
        "directory_profiles",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, Vec<d::LibraryEmoji>>(
        "get",
        "/directory/emojis",
        "directory_emojis",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, d::DirectoryReceipt>(
        "post",
        "/directory/commands",
        "directory_command",
        Reply::Json(200),
    );
    doc.body::<d::DirectoryRequest>("/v1/directory/commands", "post");
    doc.endpoint::<routes::Empty, Option<d::DirectoryReceipt>>(
        "get",
        "/directory/commands/{request_id}",
        "directory_receipt",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, Vec<d::IdentityBinding>>(
        "get",
        "/directory/identities",
        "directory_identities",
        Reply::Json(200),
    );
    doc.endpoint::<routes::Empty, d::DirectoryReceipt>(
        "post",
        "/directory/identities",
        "directory_enroll",
        Reply::Json(200),
    );
    doc.body::<Enroll>("/v1/directory/identities", "post");
    doc.endpoint::<routes::Empty, d::DirectoryReceipt>(
        "post",
        "/directory/identity-revoke",
        "directory_revoke",
        Reply::Json(200),
    );
    doc.body::<Revoke>("/v1/directory/identity-revoke", "post");
    doc.endpoint::<routes::Empty, serde_json::Value>(
        "get",
        "/directory/media/{digest}",
        "directory_media",
        Reply::Raster,
    );
}
