//! WorkStore keeps its companion pin and CLI actor, independently from code.
use super::*;
use jj_store_service::{local_repository as repo, work as service};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalWorkPageQuery")]
struct PageQuery {
    metadata_commit: repo::MetadataCommit,
    query: service::WorkQuery,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalWorkBrowseQuery")]
struct BrowseQuery {
    metadata_commit: repo::MetadataCommit,
    query: service::WorkBrowseQuery,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalWorkActivityQuery")]
struct ActivityQuery {
    metadata_commit: repo::MetadataCommit,
    query: service::WorkActivityQuery,
}
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repos/{repo}/work/page", page, "local_work_page", PageQuery, repo::LocalWorkPage),
        (post, "/local/repos/{repo}/work/browse", browse, "local_work_browse", BrowseQuery, repo::LocalWorkBrowsePage),
        (post, "/local/repos/{repo}/work/activity", activity, "local_work_activity", ActivityQuery, repo::LocalWorkActivityPage),
        (post, "/local/repos/{repo}/work/commands", apply, "local_work_apply", service::Request, repo::LocalWorkOutcome),
        (get, "/local/repos/{repo}/work/commands/{request_id}", status, "local_work_status", routes::Empty, repo::LocalWorkOutcome)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            Router::new()$(.route($path,$method($handler)))*
        };
    }
    endpoints!(register)
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    version: u32,
    repository: m::RepositoryId,
    generation: u64,
    digest: String,
    expires: u64,
    continuation: String,
}
pub(super) fn digest<T: Serialize>(kind: &str, query: &T) -> Result<String> {
    let mut hash = Sha256::new_with_prefix(kind.as_bytes());
    hash.update(serde_json::to_vec(query).map_err(|_| Error::invalid())?);
    Ok(hex::encode(hash.finalize()))
}
fn sign(key: &[u8; 32], value: Cursor) -> Result<String> {
    let mut body = serde_json::to_vec(&value).map_err(|_| Error::unavailable())?;
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key");
    mac.update(b"jj-store/local-work-cursor/v1\0");
    mac.update(&body);
    body.extend_from_slice(&mac.finalize().into_bytes());
    Ok(format!("jlw1_{}", hex::encode(body)))
}
pub(super) fn resume(
    state: &State,
    text: Option<String>,
    repository: m::RepositoryId,
    generation: u64,
    digest: &str,
) -> Result<Option<String>> {
    text.map(|text| {
        if text.len() > 16384 {
            return Err(Error::invalid());
        }
        let bytes = hex::decode(text.strip_prefix("jlw1_").ok_or_else(Error::invalid)?)
            .map_err(|_| Error::invalid())?;
        if bytes.len() <= 32 {
            return Err(Error::invalid());
        }
        let (body, signature) = bytes.split_at(bytes.len() - 32);
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&state.repository_cursor_key).expect("HMAC key");
        mac.update(b"jj-store/local-work-cursor/v1\0");
        mac.update(body);
        mac.verify_slice(signature)
            .map_err(|_| Error::new(StatusCode::CONFLICT, "cursor_reset"))?;
        let c: Cursor = serde_json::from_slice(body).map_err(|_| Error::invalid())?;
        if c.repository != repository || c.digest != digest {
            return Err(Error::invalid());
        }
        if c.version != 1 || c.generation != generation || c.expires < now()? {
            return Err(Error::new(StatusCode::CONFLICT, "cursor_reset"));
        }
        Ok(c.continuation)
    })
    .transpose()
}
pub(super) fn continuation(
    state: &State,
    raw: Option<String>,
    repository: m::RepositoryId,
    generation: u64,
    digest: String,
) -> Result<Option<String>> {
    raw.map(|continuation| {
        sign(
            &state.repository_cursor_key,
            Cursor {
                version: 1,
                repository,
                generation,
                digest,
                expires: now()? + 300,
                continuation,
            },
        )
    })
    .transpose()
}
pub(super) async fn check(
    state: &State,
    repository: m::RepositoryId,
    generation: u64,
) -> Result<()> {
    let manager = manager(state)?;
    blocking(state, move || {
        if manager.folder(repository).map_err(map)?.1 != generation {
            return Err(Error::new(StatusCode::CONFLICT, "cursor_reset"));
        }
        Ok(())
    })
    .await
}
async fn page(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<PageQuery>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.query.cursor.take();
    let digest = digest("local-work-page", &request)?;
    request.query.cursor = resume(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let mut page = blocking(&state, move || {
        manager
            .open(repository)
            .map_err(map)?
            .work_page_at(request.metadata_commit, &request.query)
            .map_err(Error::from)
    })
    .await?;
    check(&state, repository, generation).await?;
    page.next_cursor = continuation(&state, page.next_cursor, repository, generation, digest)?;
    state.json(&page)
}
async fn browse(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<BrowseQuery>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.query.cursor.take();
    let digest = digest("local-work-browse", &request)?;
    request.query.cursor = resume(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let mut page = blocking(&state, move || {
        manager
            .open(repository)
            .map_err(map)?
            .work_browse_at(request.metadata_commit, &request.query)
            .map_err(Error::from)
    })
    .await?;
    check(&state, repository, generation).await?;
    page.next_cursor = continuation(&state, page.next_cursor, repository, generation, digest)?;
    state.json(&page)
}
async fn apply(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    axum::Extension(pending): axum::Extension<mutations::Pending>,
    body: Body<service::Request>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    let _ = binding(&state, repository).await?;
    pending.set(request.request_id);
    let outcome = blocking(&state, move || {
        manager
            .open(repository)
            .map_err(map)?
            .apply_work_command(&request)
            .map_err(Error::from)
    })
    .await?;
    Ok(state
        .json(&outcome)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
async fn activity(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<ActivityQuery>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.query.cursor.take();
    let digest = digest("local-work-activity", &request)?;
    request.query.cursor = resume(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let mut page = blocking(&state, move || {
        manager
            .open(repository)
            .map_err(map)?
            .work_activity_at(request.metadata_commit, &request.query)
            .map_err(Error::from)
    })
    .await?;
    check(&state, repository, generation).await?;
    page.next_cursor = continuation(&state, page.next_cursor, repository, generation, digest)?;
    state.json(&page)
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((repository, request)): Path<(m::RepositoryId, m::RequestId)>,
    args: routes::Input<routes::Empty>,
    axum::Extension(pending): axum::Extension<mutations::Pending>,
) -> Result<Response> {
    routes::input(args)?;
    let manager = manager(&state)?;
    let _ = binding(&state, repository).await?;
    pending.set(request);
    let outcome = blocking(&state, move || {
        manager
            .open(repository)
            .map_err(map)?
            .work_command_status(request)
            .map_err(Error::from)
    })
    .await?;
    Ok(state
        .json(&outcome)
        .unwrap_or_else(|_| pending.response().unwrap()))
}
pub(super) fn document(document: &mut crate::contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            $(document.local_endpoint::<routes::Empty,$response>(stringify!($method),$path,$id);
            if stringify!($method)=="post" { document.body::<$body>(&format!("/v1{}",$path),"post"); })*
        };
    }
    endpoints!(register);
}
