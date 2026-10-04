//! LocalTrusted installation management and strictly typed operation-pinned reads.
use crate::*;
mod agents;
mod conversion;
mod events;
mod reviews;
mod work;
mod working_copy;
use axum::{
    Json,
    extract::{Path, State as ExtractState},
    routing::{get, post},
};
use hmac::{Hmac, Mac as _};
use jj_store_service::local_browser as management;
use management::code as local;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

type Body<T> = std::result::Result<Json<T>, axum::extract::rejection::JsonRejection>;
fn decode_body<T>(body: Body<T>) -> Result<T> {
    body.map(|Json(value)| value).map_err(|error| {
        if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Error::limit()
        } else {
            Error::invalid()
        }
    })
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalCodeQueryRequest")]
pub(crate) struct Query {
    pub operation: m::OperationId,
    pub request: local::Request,
    pub cursor: Option<String>,
}
#[derive(Serialize, JsonSchema)]
#[schemars(rename = "LocalCodeQueryResponse")]
pub(crate) struct QueryResponse {
    pub snapshot: local::Snapshot,
    pub data: local::Response,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}
#[derive(Serialize, JsonSchema)]
#[schemars(rename = "LocalRepositorySnapshot")]
pub(crate) struct Snapshot {
    #[serde(flatten)]
    pub code: local::Heads,
    pub metadata_commit: Option<m::GitSha1Id>,
    pub actor: Option<m::PrincipalId>,
}
#[derive(Serialize, JsonSchema)]
#[schemars(rename = "LocalRepositoryCapabilities")]
pub(crate) struct Capabilities {
    pub version: u32,
    pub profile: &'static str,
    pub code_browse: bool,
    pub collaboration: bool,
    pub agents: bool,
    pub reviews: bool,
    pub review_actions: bool,
    pub review_repository_anchors: bool,
    pub review_code_anchors: bool,
    pub review_agent_anchors: bool,
    pub search: bool,
    pub code_mutations: bool,
    pub working_copy_edit: bool,
    pub conversion: bool,
}
// One inventory owns executable methods, request bodies and response contracts.
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (get, "/local/repositories", list, "local_repository_list", routes::Empty, management::LocalBindings),
        (post, "/local/repositories", register, "local_repository_register", management::RegisterFolder, management::RegistrationOutcome),
        (post, "/local/repositories/inspect", inspect, "local_repository_inspect", management::InspectFolder, management::LocalFolder),
        (post, "/local/repositories/native", create_native, "local_native_repository_create", management::CreateNativeRepository, management::NativeCreationOutcome),
        (get, "/local/repositories/native/requests/{request_id}", native_status, "local_native_repository_status", routes::Empty, management::NativeCreationOutcome),
        (post, "/local/repositories/pick", pick, "local_repository_pick", routes::Empty, jj_store_service::local_management::FolderSelection),
        (get, "/local/repositories/requests/{request_id}", status, "local_repository_status", routes::Empty, management::RegistrationOutcome),
        (post, "/local/repositories/{repo}/reconnect", reconnect, "local_repository_reconnect", management::ReconnectFolder, management::LocalBindings),
        (post, "/local/repositories/{repo}/remove", remove, "local_repository_remove", management::RemoveBinding, management::LocalBindings),
        (get, "/local/repos/{repo}/snapshot", snapshot, "local_code_snapshot", routes::Empty, Snapshot),
        (get, "/local/repos/{repo}/capabilities", capabilities, "local_code_capabilities", routes::Empty, Capabilities),
        (post, "/local/repos/{repo}/query", query, "local_code_query", Query, QueryResponse)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            Router::new()$(.route($path,$method($handler)))*
        };
    }
    endpoints!(register)
        .merge(work::router())
        .merge(working_copy::router())
        .merge(conversion::router())
        .merge(agents::router())
        .merge(reviews::router())
        .merge(events::router())
}
fn admit(state: &State) -> Result<()> {
    if !matches!(
        state.config.access,
        Access::LocalTrusted {
            permission: Permission::Admin,
            ..
        }
    ) {
        return Err(Error::denied());
    }
    Ok(())
}
async fn create_native(
    ExtractState(state): ExtractState<Arc<State>>,
    body: Body<management::CreateNativeRepository>,
) -> Result<Response> {
    let manager = manager(&state)?;
    let request = decode_body(body)?;
    state.json(&manager.create_native(request).await.map_err(map)?)
}
async fn native_status(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(request): Path<m::RequestId>,
) -> Result<Response> {
    let manager = manager(&state)?;
    state.json(&manager.native_creation_status(request).await.map_err(map)?)
}
fn manager(state: &State) -> Result<Arc<dyn management::LocalRepositories>> {
    admit(state)?;
    state.config.local_repositories.clone().ok_or_else(|| {
        Error::new(
            StatusCode::NOT_IMPLEMENTED,
            "local_repositories_unavailable",
        )
    })
}
fn map(error: management::Error) -> Error {
    use management::Error as E;
    match error {
        E::Invalid => Error::new(StatusCode::BAD_REQUEST, "invalid_local_folder"),
        E::Unsupported => Error::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unsupported_local_profile",
        ),
        E::Unavailable => Error::new(StatusCode::SERVICE_UNAVAILABLE, "local_folder_unavailable"),
        E::IdentityChanged => Error::new(StatusCode::CONFLICT, "local_identity_changed"),
        E::Stale => Error::new(StatusCode::CONFLICT, "stale_local_revision"),
        E::Conflict => Error::new(StatusCode::CONFLICT, "local_registration_conflict"),
        E::Limit => Error::limit(),
    }
}
async fn blocking<T: Send + 'static>(
    state: &State,
    run: impl FnOnce() -> Result<T> + Send + 'static,
) -> Result<T> {
    let permit = state
        .local_jobs
        .clone()
        .acquire_owned()
        .await
        .map_err(|_| Error::busy())?;
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        run()
    })
    .await
    .map_err(|_| Error::unavailable())?
}
async fn list(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let manager = manager(&state)?;
    state.json(&blocking(&state, move || manager.list().map_err(map)).await?)
}
async fn inspect(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<routes::Empty>,
    body: Body<management::InspectFolder>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    state.json(&blocking(&state, move || manager.inspect(&request.path).map_err(map)).await?)
}
async fn register(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<routes::Empty>,
    axum::Extension(pending): axum::Extension<mutations::Pending>,
    body: Body<management::RegisterFolder>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    let id = request.request_id;
    pending.set(id);
    let runtime = tokio::runtime::Handle::current();
    state.json(
        &blocking(&state, move || {
            runtime.block_on(manager.register(request)).map_err(map)
        })
        .await?,
    )
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(request): Path<m::RequestId>,
    args: routes::Input<routes::Empty>,
    axum::Extension(pending): axum::Extension<mutations::Pending>,
) -> Result<Response> {
    routes::input(args)?;
    let manager = manager(&state)?;
    pending.set(request);
    let runtime = tokio::runtime::Handle::current();
    state.json(
        &blocking(&state, move || {
            runtime.block_on(manager.status(request)).map_err(map)
        })
        .await?,
    )
}
async fn reconnect(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<management::ReconnectFolder>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    state.json(
        &blocking(&state, move || {
            manager.reconnect(repository, request).map_err(map)
        })
        .await?,
    )
}
async fn remove(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<management::RemoveBinding>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    state.json(
        &blocking(&state, move || {
            manager.remove(repository, request).map_err(map)
        })
        .await?,
    )
}
async fn pick(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<routes::Empty>,
    body: Body<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    decode_body(body)?;
    admit(&state)?;
    let picker = state
        .config
        .local_folder_picker
        .as_ref()
        .ok_or_else(|| Error::new(StatusCode::NOT_IMPLEMENTED, "folder_picker_unavailable"))?;
    let result = picker.pick().await.map_err(|error| {
        use jj_store_service::local_management::PickerError as E;
        match error {
            E::Unavailable => {
                Error::new(StatusCode::SERVICE_UNAVAILABLE, "folder_picker_unavailable")
            }
            E::Busy => Error::new(StatusCode::CONFLICT, "folder_picker_busy"),
            E::Timeout => Error::new(StatusCode::REQUEST_TIMEOUT, "folder_picker_timeout"),
            E::Invalid => Error::new(StatusCode::BAD_GATEWAY, "invalid_folder_picker_response"),
        }
    })?;
    state.json(&result)
}
struct Binding {
    manager: Arc<dyn management::LocalRepositories>,
    folder: management::LocalFolder,
    generation: u64,
}
#[async_trait::async_trait]
impl local::Access for Binding {
    fn repository(&self) -> m::RepositoryId {
        self.folder.repository.expect("registered identity")
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.folder.metadata)
    }
    async fn authorize(&self) -> local::Result<()> {
        let manager = self.manager.clone();
        let repo = self.repository();
        let (folder, generation) = tokio::task::spawn_blocking(move || manager.folder(repo))
            .await
            .map_err(|_| local::Error::Unavailable)?
            .map_err(|_| local::Error::Denied)?;
        if folder != self.folder || generation != self.generation {
            return Err(local::Error::Denied);
        }
        Ok(())
    }
}
async fn binding(state: &State, repo: m::RepositoryId) -> Result<Arc<dyn local::Access>> {
    let manager = manager(state)?;
    let check = manager.clone();
    let (folder, generation) = blocking(state, move || check.folder(repo).map_err(map)).await?;
    Ok(Arc::new(Binding {
        manager,
        folder,
        generation,
    }))
}
async fn snapshot(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    state.json(&capture(&state, repo).await?)
}
async fn capture(state: &State, repo: m::RepositoryId) -> Result<Snapshot> {
    let access = binding(state, repo).await?;
    let code = state.local_reader.heads(access).await?;
    let manager = manager(state)?;
    let (metadata_commit, actor) = blocking(state, move || {
        let repo = manager.open(repo).map_err(map)?;
        let commit = repo
            .metadata_store()
            .read()
            .map_err(|_| Error::unavailable())?
            .map(|s| {
                s.commit()
                    .to_string()
                    .parse()
                    .map_err(|_| Error::unavailable())
            })
            .transpose()?;
        Ok((commit, repo.actor().map_err(|_| Error::unavailable())?))
    })
    .await?;
    work::check(state, repo, code.generation).await?;
    Ok(Snapshot {
        code,
        metadata_commit,
        actor,
    })
}
async fn capabilities(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let _ = binding(&state, repo).await?;
    state.json(&Capabilities {
        version: 1,
        profile: local::PROFILE,
        code_browse: true,
        collaboration: true,
        agents: true,
        reviews: true,
        review_actions: true,
        review_repository_anchors: true,
        review_code_anchors: false,
        review_agent_anchors: false,
        search: false,
        code_mutations: false,
        working_copy_edit: true,
        conversion: state
            .config
            .local_repositories
            .as_ref()
            .is_some_and(|m| m.conversion()),
    })
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    version: u32,
    repository: m::RepositoryId,
    generation: u64,
    operation: m::OperationId,
    digest: String,
    expires: u64,
    anchor: Anchor,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum Anchor {
    Graph(local::ObjectKey),
    Path(String, bool),
}
fn now() -> Result<u64> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|_| Error::unavailable())
}
fn digest(request: &local::Request) -> Result<String> {
    Ok(hex::encode(Sha256::digest(
        serde_json::to_vec(request).map_err(|_| Error::invalid())?,
    )))
}
fn cursor(key: &[u8; 32], value: Cursor) -> Result<String> {
    let mut bytes = serde_json::to_vec(&value).map_err(|_| Error::unavailable())?;
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key");
    mac.update(&bytes);
    bytes.extend_from_slice(&mac.finalize().into_bytes());
    Ok(format!("jlb1_{}", hex::encode(bytes)))
}
fn decode(key: &[u8; 32], text: &str) -> Result<Cursor> {
    if text.len() > 16384 {
        return Err(Error::invalid());
    }
    let bytes = hex::decode(text.strip_prefix("jlb1_").ok_or_else(Error::invalid)?)
        .map_err(|_| Error::invalid())?;
    if bytes.len() <= 32 {
        return Err(Error::invalid());
    }
    let (body, signature) = bytes.split_at(bytes.len() - 32);
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key");
    mac.update(body);
    mac.verify_slice(signature)
        .map_err(|_| Error::new(StatusCode::CONFLICT, "cursor_reset"))?;
    let value: Cursor = serde_json::from_slice(body).map_err(|_| Error::invalid())?;
    if value.version != 1 || value.expires < now()? {
        return Err(Error::new(StatusCode::CONFLICT, "cursor_reset"));
    }
    Ok(value)
}
fn apply_anchor(request: &mut local::Request, anchor: Option<Anchor>) -> Result<()> {
    use local::Request as R;
    let absent = match request {
        R::Graph(_, p) => p.after.is_none(),
        R::Paths(p) => p.anchor.is_none(),
        R::References(p) => p.anchor.is_none(),
        R::DiffPaths(p) => p.anchor.is_none(),
        R::OperationChanges(p) => p.anchor.is_none(),
        _ => true,
    };
    if !absent {
        return Err(Error::invalid());
    }
    if let Some(anchor) = anchor {
        match (request, anchor) {
            (R::Graph(_, p), Anchor::Graph(a)) => p.after = Some(a),
            (R::Paths(p), Anchor::Path(a, back)) => {
                p.anchor = Some(a);
                p.direction = if back {
                    local::Direction::Backward
                } else {
                    local::Direction::Forward
                }
            }
            (R::References(p), Anchor::Path(a, back)) => {
                p.anchor = Some(a);
                p.direction = if back {
                    local::Direction::Backward
                } else {
                    local::Direction::Forward
                }
            }
            (R::DiffPaths(p), Anchor::Path(a, back)) => {
                p.anchor = Some(a);
                p.direction = if back {
                    local::Direction::Backward
                } else {
                    local::Direction::Forward
                }
            }
            (R::OperationChanges(p), Anchor::Path(a, back)) => {
                p.anchor = Some(a);
                p.direction = if back {
                    local::Direction::Backward
                } else {
                    local::Direction::Forward
                }
            }
            _ => return Err(Error::invalid()),
        }
    }
    Ok(())
}
async fn query(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Query>,
) -> Result<Response> {
    routes::input(args)?;
    let mut query = decode_body(body)?;
    let access = binding(&state, repo).await?;
    let generation = access.generation();
    let digest = digest(&query.request)?;
    let anchor = query
        .cursor
        .as_deref()
        .map(|text| {
            let c = decode(&state.repository_cursor_key, text)?;
            if c.repository != repo || c.operation != query.operation || c.digest != digest {
                return Err(Error::invalid());
            }
            if c.generation != generation {
                return Err(Error::new(StatusCode::CONFLICT, "cursor_reset"));
            }
            Ok(c.anchor)
        })
        .transpose()?;
    apply_anchor(&mut query.request, anchor)?;
    let data = state
        .local_reader
        .read(access, query.operation, query.request)
        .await?;
    let make = |anchor| {
        cursor(
            &state.repository_cursor_key,
            Cursor {
                version: 1,
                repository: repo,
                generation,
                operation: query.operation,
                digest: digest.clone(),
                expires: now()? + 300,
                anchor,
            },
        )
    };
    let (next, previous) = match &data {
        local::Response::Graph(p) => (p.next.map(Anchor::Graph), None),
        local::Response::Paths(p) => (
            p.next.clone().map(|a| Anchor::Path(a, false)),
            p.previous.clone().map(|a| Anchor::Path(a, true)),
        ),
        local::Response::References(p) => (
            p.next.clone().map(|a| Anchor::Path(a, false)),
            p.previous.clone().map(|a| Anchor::Path(a, true)),
        ),
        local::Response::DiffPaths(p) => (
            p.next.clone().map(|a| Anchor::Path(a, false)),
            p.previous.clone().map(|a| Anchor::Path(a, true)),
        ),
        local::Response::OperationChanges(p) => (
            p.next.clone().map(|a| Anchor::Path(a, false)),
            p.previous.clone().map(|a| Anchor::Path(a, true)),
        ),
        _ => (None, None),
    };
    state.json(&QueryResponse {
        snapshot: local::Snapshot {
            version: 1,
            profile: local::PROFILE,
            repository: repo,
            generation,
            operation: query.operation,
        },
        data,
        next_cursor: next.map(make).transpose()?,
        previous_cursor: previous.map(make).transpose()?,
    })
}
pub(crate) fn document(document: &mut crate::contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            $(document.local_endpoint::<routes::Empty,$response>(stringify!($method),$path,$id);
            if stringify!($method)=="post" { document.body::<$body>(&format!("/v1{}",$path),"post"); })*
        };
    }
    endpoints!(register);
    work::document(document);
    agents::document(document);
    reviews::document(document);
    working_copy::document(document);
    conversion::document(document);
    events::document(document);
}
