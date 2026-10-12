//! Authenticated execution only; installation management remains LocalTrusted.
use crate::*;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::{
    Json,
    extract::{DefaultBodyLimit, rejection::JsonRejection},
    routing::{get, post},
};
use jj_store_service::agent_execution as a;
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
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct TargetsQuery {
    repository: Option<m::RepositoryId>,
}
macro_rules! endpoints { ($register:ident) => { $register! {
    (get,"/agents",status,"agents_status",routes::Empty,routes::Empty,a::Status),
    (get,"/agents/targets",targets,"agents_targets",TargetsQuery,routes::Empty,Vec<a::Target>),
    (post,"/agents/chat",send,"agents_chat",routes::Empty,a::conversations::Send,a::conversations::SendOutcome),
    (post,"/agents/command",command,"agents_command",routes::Empty,a::Command,a::Outcome),
    (post,"/agents/workspace/inspect",inspect,"agents_workspace_inspect",routes::Empty,a::workspace_inspection::Request,a::workspace_inspection::Inspection),
    (get,"/agents/outcome",outcome,"agents_outcome",OutcomeQuery,routes::Empty,a::Outcome),
    (get,"/agents/events",events,"agents_events",EventsQuery,routes::Empty,a::EventPage),
    (get,"/agents/feedback",feedback,"agents_feedback",EventsQuery,routes::Empty,a::Feedback)
} }; }
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register { ($(($method:ident,$path:literal,$handler:path,$id:literal,$query:ty,$body:ty,$response:ty)),*) => {
        Router::new()$(.route($path,$method($handler)))*
    }; }
    endpoints!(register)
        .route("/agents/stream", get(stream))
        .layer(DefaultBodyLimit::max(512 * 1024))
}
pub(super) fn document(document: &mut contract::Document) {
    macro_rules! register { ($(($method:ident,$path:literal,$handler:path,$id:literal,$query:ty,$body:ty,$response:ty)),*) => {
        $(document.authenticated_agent_endpoint::<$query,$response>(stringify!($method),$path,$id);
        if stringify!($method)=="post" {document.body::<$body>(&format!("/v1{}",$path),"post");})*
    }; }
    endpoints!(register);
    document
        .authenticated_agent_events::<EventsQuery, a::EventPage>("/agents/stream", "agents_stream");
}
async fn manager(state: &State, headers: &HeaderMap) -> Result<(m::PrincipalId, a::AgentManager)> {
    if !matches!(state.config.access, Access::Authenticated) {
        return Err(Error::denied());
    }
    let token = crate::repositories::bearer(headers)?;
    state
        .config
        .remote_agents
        .as_ref()
        .ok_or_else(Error::unavailable)?
        .admit(token)
        .await
        .map_err(super::agent_execution::failure)
}
async fn status(
    ExtractState(s): ExtractState<Arc<State>>,
    h: HeaderMap,
    q: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(q)?;
    let (p, m) = manager(&s, &h).await?;
    s.json(&m.status(p).await.map_err(super::agent_execution::failure)?)
}
async fn targets(
    ExtractState(s): ExtractState<Arc<State>>,
    h: HeaderMap,
    q: routes::Input<TargetsQuery>,
) -> Result<Response> {
    let q = routes::input(q)?;
    let (p, m) = manager(&s, &h).await?;
    s.json(
        &m.targets(p, q.repository)
            .await
            .map_err(super::agent_execution::failure)?,
    )
}
async fn outcome(
    ExtractState(s): ExtractState<Arc<State>>,
    h: HeaderMap,
    q: routes::Input<OutcomeQuery>,
) -> Result<Response> {
    let q = routes::input(q)?;
    let (p, m) = manager(&s, &h).await?;
    s.json(
        &m.outcome(p, q.request)
            .await
            .map_err(super::agent_execution::failure)?,
    )
}
async fn events(
    ExtractState(s): ExtractState<Arc<State>>,
    h: HeaderMap,
    q: routes::Input<EventsQuery>,
) -> Result<Response> {
    let q = routes::input(q)?;
    let (p, m) = manager(&s, &h).await?;
    s.json(
        &m.events(p, q.session, q.after)
            .await
            .map_err(super::agent_execution::failure)?,
    )
}
async fn feedback(
    ExtractState(s): ExtractState<Arc<State>>,
    h: HeaderMap,
    q: routes::Input<EventsQuery>,
) -> Result<Response> {
    let q = routes::input(q)?;
    if q.after != 0 {
        return Err(Error::invalid());
    }
    let (p, m) = manager(&s, &h).await?;
    s.json(
        &m.feedback(p, q.session)
            .await
            .map_err(super::agent_execution::failure)?,
    )
}
macro_rules! mutation {
    ($name:ident,$input:ty,$method:ident) => {
        async fn $name(
            ExtractState(s): ExtractState<Arc<State>>,
            h: HeaderMap,
            q: routes::Input<routes::Empty>,
            input: std::result::Result<Json<$input>, JsonRejection>,
        ) -> Result<Response> {
            routes::input(q)?;
            let (p, m) = manager(&s, &h).await?;
            let input = super::agent_execution::body(input)?;
            s.json(
                &m.$method(p, input)
                    .await
                    .map_err(super::agent_execution::failure)?,
            )
        }
    };
}
mutation!(send, a::conversations::Send, send_conversation);
mutation!(command, a::Command, command);
mutation!(inspect, a::workspace_inspection::Request, inspect_workspace);
async fn stream(
    ExtractState(s): ExtractState<Arc<State>>,
    h: HeaderMap,
    q: routes::Input<EventsQuery>,
) -> Result<Response> {
    let q = routes::input(q)?;
    let (p, m) = manager(&s, &h).await?;
    m.events(p, q.session.clone(), q.after)
        .await
        .map_err(super::agent_execution::failure)?;
    let updates = m.subscribe();
    let deadline = tokio::time::Instant::now() + s.config.event_lifetime;
    let stream = futures::stream::try_unfold(
        (m, updates, q, false),
        move |(m, mut updates, mut q, delivered)| async move {
            if tokio::time::Instant::now() >= deadline {
                return Ok::<_, std::io::Error>(None);
            }
            if delivered {
                tokio::select! {_=updates.recv()=>{},_=tokio::time::sleep(Duration::from_secs(2))=>{}}
            }
            let page = m
                .events(p, q.session.clone(), q.after)
                .await
                .map_err(std::io::Error::other)?;
            if let Some(event) = page.events.last() {
                q.after = event.sequence;
            }
            let more = page.next.is_some();
            let event = Event::default()
                .event("agent-activity")
                .id(q.after.to_string())
                .json_data(page)
                .map_err(std::io::Error::other)?;
            Ok(Some((event, (m, updates, q, !more))))
        },
    );
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response())
}

#[cfg(test)]
mod tests;
