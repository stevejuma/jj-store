//! Recorded AgentStore queries with a distinct companion pin and signed cursor.
use super::*;
use jj_store_service::local_repository as repo;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Pinned<Q> {
    metadata_commit: repo::MetadataCommit,
    query: Q,
    cursor: Option<String>,
}
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Sessions {
    limit: usize,
}
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Session {
    session_id: jj_store_service::collaboration::AgentSessionIdentity,
}
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Relations {
    subject: repo::LocalAgentSubject,
    limit: usize,
}
#[derive(Serialize, JsonSchema)]
struct Page<T> {
    data: T,
    next_cursor: Option<String>,
}
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repos/{repo}/agents/sessions", sessions, "local_agent_sessions", Pinned<Sessions>, Page<repo::LocalBrowserAgentPage>),
        (post, "/local/repos/{repo}/agents/session", session, "local_agent_session", Pinned<Session>, repo::LocalBrowserAgentDetail),
        (post, "/local/repos/{repo}/agents/transcript", transcript, "local_agent_transcript", Pinned<repo::LocalBrowserTranscriptRequest>, Page<repo::LocalBrowserTranscriptPage>),
        (post, "/local/repos/{repo}/agents/relations", relations, "local_agent_relations", Pinned<Relations>, Page<repo::LocalBrowserAgentRelations>)
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
fn digest<T: Serialize>(kind: &str, query: &Pinned<T>) -> Result<String> {
    work::digest(kind, query)
}
fn anchor<T: DeserializeOwned>(
    state: &State,
    raw: Option<String>,
    repository: m::RepositoryId,
    generation: u64,
    digest: &str,
) -> Result<Option<T>> {
    work::resume(state, raw, repository, generation, digest)?
        .map(|raw| serde_json::from_str(&raw).map_err(|_| Error::invalid()))
        .transpose()
}
fn next<T: Serialize>(
    state: &State,
    value: Option<T>,
    repository: m::RepositoryId,
    generation: u64,
    digest: String,
) -> Result<Option<String>> {
    let raw = value
        .map(|value| serde_json::to_string(&value).map_err(|_| Error::unavailable()))
        .transpose()?;
    work::continuation(state, raw, repository, generation, digest)
}
async fn sessions(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Pinned<Sessions>>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.cursor.take();
    let digest = digest("local-agent-sessions", &request)?;
    let after = anchor(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .browser_agent_sessions(request.metadata_commit, request.query.limit, after)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    let next_cursor = next(&state, data.next_after, repository, generation, digest)?;
    state.json(&Page { data, next_cursor })
}
async fn session(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Pinned<Session>>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    if request.cursor.is_some() {
        return Err(Error::invalid());
    }
    let generation = binding(&state, repository).await?.generation();
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .browser_agent_session(request.metadata_commit, request.query.session_id)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    state.json(&data)
}
async fn transcript(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Pinned<repo::LocalBrowserTranscriptRequest>>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.cursor.take();
    let digest = digest("local-agent-transcript", &request)?;
    let after = anchor(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .browser_agent_transcript(request.metadata_commit, request.query, after)
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    let next_cursor = next(
        &state,
        data.next_after_sequence,
        repository,
        generation,
        digest,
    )?;
    state.json(&Page { data, next_cursor })
}
async fn relations(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<Pinned<Relations>>,
) -> Result<Response> {
    routes::input(args)?;
    let mut request = decode_body(body)?;
    let generation = binding(&state, repository).await?.generation();
    let raw = request.cursor.take();
    let digest = digest("local-agent-relations", &request)?;
    let after = anchor(&state, raw, repository, generation, &digest)?;
    let manager = manager(&state)?;
    let runtime = tokio::runtime::Handle::current();
    let data = blocking(&state, move || {
        runtime.block_on(async {
            manager
                .open(repository)
                .map_err(map)?
                .browser_agent_relations(
                    request.metadata_commit,
                    request.query.subject,
                    request.query.limit,
                    after,
                )
                .await
                .map_err(Error::from)
        })
    })
    .await?;
    work::check(&state, repository, generation).await?;
    let next_cursor = next(&state, data.next_after, repository, generation, digest)?;
    state.json(&Page { data, next_cursor })
}
pub(super) fn document(document: &mut crate::contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => {
            $(document.local_endpoint::<routes::Empty,$response>(stringify!($method),$path,$id);
            document.body::<$body>(&format!("/v1{}",$path),"post");)*
        };
    }
    endpoints!(register);
}
