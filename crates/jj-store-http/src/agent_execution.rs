//! Execution is installation-local and always requires LocalTrusted Admin.
use crate::*;
use axum::{
    Json,
    extract::{DefaultBodyLimit, rejection::JsonRejection},
    routing::{get, post},
};
use jj_store_service::agent_execution as agents;
mod stream;
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct TargetsQuery {
    repository: Option<m::RepositoryId>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct OutcomeQuery {
    request: m::RequestId,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct EventsQuery {
    session: String,
    #[serde(default)]
    after: u64,
}
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (get, "/local/agents", status, "local_agents_status", routes::Empty, routes::Empty, agents::Status),
        (get, "/local/agents/targets", targets, "local_agents_targets", TargetsQuery, routes::Empty, Vec<agents::Target>),
        (post, "/local/agents/setups", setups, "local_agents_setups", routes::Empty, agents::SetupCommand, agents::Status),
        (post, "/local/agents/launch", launch, "local_agents_launch", routes::Empty, agents::Launch, agents::Outcome),
        (post, "/local/agents/command", command, "local_agents_command", routes::Empty, agents::Command, agents::Outcome),
        (get, "/local/agents/outcome", outcome, "local_agents_outcome", OutcomeQuery, routes::Empty, agents::Outcome),
        (get, "/local/agents/feedback", feedback, "local_agents_feedback", EventsQuery, routes::Empty, agents::Feedback),
        (get, "/local/agents/events", events, "local_agents_events", EventsQuery, routes::Empty, agents::EventPage)
    } };
}
pub(crate) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:path,$id:literal,$query:ty,$body:ty,$response:ty)),*) => {
            Router::new()$(.route($path,$method($handler).layer(DefaultBodyLimit::max(if $path == "/local/agents/setups" { 2 * 1024 * 1024 } else { 512 * 1024 }))))*
        };
    }
    endpoints!(register).route("/local/agents/stream", get(stream::events))
}
pub(crate) fn document(document: &mut contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:path,$id:literal,$query:ty,$body:ty,$response:ty)),*) => {
            $(document.local_endpoint::<$query,$response>(stringify!($method),$path,$id);
            if stringify!($method)=="post" { document.body::<$body>(&format!("/v1{}",$path),"post"); })*
        };
    }
    endpoints!(register);
    document.local_agent_events::<EventsQuery, agents::EventPage>(
        "/local/agents/stream",
        "local_agents_stream",
    );
}
fn manager(state: &State) -> Result<(&agents::AgentManager, m::PrincipalId)> {
    let Access::LocalTrusted {
        principal,
        permission: Permission::Admin,
    } = state.config.access
    else {
        return Err(Error::denied());
    };
    Ok((
        state
            .config
            .local_agents
            .as_ref()
            .ok_or_else(Error::unavailable)?,
        principal,
    ))
}
fn failure(error: agents::Error) -> Error {
    let (status, code) = match error {
        agents::Error::Denied => (StatusCode::FORBIDDEN, "agent_execution_denied"),
        agents::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_agent_execution"),
        agents::Error::Stale => (StatusCode::CONFLICT, "agent_selection_changed"),
        agents::Error::Conflict => (StatusCode::CONFLICT, "agent_request_identity_conflict"),
        agents::Error::Busy => (StatusCode::CONFLICT, "agent_execution_busy"),
        agents::Error::FeedbackLimit => (StatusCode::PAYLOAD_TOO_LARGE, "agent_feedback_limit"),
        agents::Error::FeedbackUnavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "agent_feedback_unavailable",
        ),
        agents::Error::Limit => (StatusCode::PAYLOAD_TOO_LARGE, "agent_execution_limit"),
        agents::Error::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "agent_execution_unavailable",
        ),
    };
    Error::new(status, code)
}
fn body<T>(body: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    body.map(|Json(value)| value).map_err(|e| {
        if e.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Error::limit()
        } else {
            Error::invalid()
        }
    })
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.status(actor).await.map_err(failure)?)
}
async fn targets(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<TargetsQuery>,
) -> Result<Response> {
    let q = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.targets(actor, q.repository).await.map_err(failure)?)
}
async fn outcome(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<OutcomeQuery>,
) -> Result<Response> {
    let q = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.outcome(actor, q.request).await.map_err(failure)?)
}
async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<EventsQuery>,
) -> Result<Response> {
    let q = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.events(actor, q.session, q.after).await.map_err(failure)?)
}
macro_rules! mutation {
    ($name:ident,$input:ty,$method:ident) => {
        async fn $name(
            ExtractState(state): ExtractState<Arc<State>>,
            query: routes::Input<routes::Empty>,
            input: std::result::Result<Json<$input>, JsonRejection>,
        ) -> Result<Response> {
            routes::input(query)?;
            let (m, actor) = manager(&state)?;
            let input = body(input)?;
            state.json(&m.$method(actor, input).await.map_err(failure)?)
        }
    };
}
mutation!(setups, agents::SetupCommand, save_setup);
mutation!(launch, agents::Launch, launch);
mutation!(command, agents::Command, command);
#[cfg(test)]
mod tests;

async fn feedback(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<EventsQuery>,
) -> Result<Response> {
    let q = routes::input(query)?;
    if q.after != 0 {
        return Err(Error::invalid());
    }
    let (m, actor) = manager(&state)?;
    state.json(&m.feedback(actor, q.session).await.map_err(failure)?)
}
