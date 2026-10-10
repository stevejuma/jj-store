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
struct CatalogQuery {
    runner: String,
    setup: Option<String>,
}
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
        (post, "/local/threads", configure_thread, "local_threads_configure", routes::Empty, agents::thread_workspace::Request, agents::thread_workspace::Outcome),
        (post, "/local/agents/workspace/inspect", inspect_workspace, "local_agents_workspace_inspect", routes::Empty, agents::workspace_inspection::Request, agents::workspace_inspection::Inspection),
        (get, "/local/compute", compute_status, "local_compute_status", routes::Empty, routes::Empty, agents::compute::Status),
        (post, "/local/compute", compute_command, "local_compute_command", routes::Empty, agents::compute::Command, agents::Outcome),
        (post, "/local/compute/pick", pick_model_file, "local_compute_pick", routes::Empty, routes::Empty, jj_store_service::local_management::ModelFileSelection),
        (get, "/local/agents/chatgpt", chatgpt_status, "local_agents_chatgpt", routes::Empty, routes::Empty, agents::chatgpt::Status),
        (post, "/local/agents/chatgpt", chatgpt_command, "local_agents_chatgpt_command", routes::Empty, agents::chatgpt::Command, agents::chatgpt::Status),
        (get, "/local/agents/settings", settings, "local_agents_settings", routes::Empty, routes::Empty, agents::settings::Settings),
        (post, "/local/agents/settings", save_settings, "local_agents_settings_save", routes::Empty, agents::settings::SettingsCommand, agents::settings::Settings),
        (post, "/local/agents/secrets", secret, "local_agents_secret", routes::Empty, agents::settings::SecretCommand, agents::settings::SecretStatus),
        (post, "/local/agents/catalog", catalog_preview, "local_agents_catalog_preview", routes::Empty, agents::catalog::Preview, agents::catalog::Catalog),
        (get, "/local/agents/catalog", catalog, "local_agents_catalog", CatalogQuery, routes::Empty, agents::catalog::Catalog),
        (get, "/local/agents", status, "local_agents_status", routes::Empty, routes::Empty, agents::Status),
        (get, "/local/agents/runtimes", runtimes, "local_agents_runtimes", routes::Empty, routes::Empty, agents::tooling::Status),
        (post, "/local/agents/runtimes/install", install_runtime, "local_agents_runtime_install", routes::Empty, agents::tooling::Command, agents::tooling::Outcome),
        (get, "/local/agents/runtimes/outcome", runtime_outcome, "local_agents_runtime_outcome", OutcomeQuery, routes::Empty, agents::tooling::Outcome),
        (get, "/local/agents/targets", targets, "local_agents_targets", TargetsQuery, routes::Empty, Vec<agents::Target>),
        (post, "/local/agents/setups", setups, "local_agents_setups", routes::Empty, agents::SetupCommand, agents::Status),
        (get, "/local/agents/metadata", metadata, "local_agents_metadata", agents::metadata::Query, routes::Empty, agents::metadata::View),
        (post, "/local/agents/metadata", metadata_command, "local_agents_metadata_command", routes::Empty, agents::metadata::Command, agents::metadata::Job),
        (get, "/local/agents/metadata/job", metadata_job, "local_agents_metadata_job", OutcomeQuery, routes::Empty, agents::metadata::Job),
        (post, "/local/agents/chat", send_conversation, "local_agents_chat", routes::Empty, agents::conversations::Send, agents::conversations::SendOutcome),
        (post, "/local/agents/control", control_agent, "local_agents_control", routes::Empty, agents::fleet::Control, agents::Outcome),
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
mutation!(
    send_conversation,
    agents::conversations::Send,
    send_conversation
);
mutation!(
    metadata_command,
    agents::metadata::Command,
    metadata_command
);
async fn metadata(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<agents::metadata::Query>,
) -> Result<Response> {
    let query = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.metadata(actor, query).await.map_err(failure)?)
}
async fn metadata_job(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<OutcomeQuery>,
) -> Result<Response> {
    let query = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(
        &m.metadata_job(actor, query.request)
            .await
            .map_err(failure)?,
    )
}
mutation!(control_agent, agents::fleet::Control, control_agent);
mutation!(
    configure_thread,
    agents::thread_workspace::Request,
    thread_workspace
);
mutation!(
    inspect_workspace,
    agents::workspace_inspection::Request,
    inspect_workspace
);
mutation!(setups, agents::SetupCommand, save_setup);
mutation!(launch, agents::Launch, launch);
mutation!(command, agents::Command, command);
mutation!(install_runtime, agents::tooling::Command, install_runtime);

async fn runtimes(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.runtime_status(actor).await.map_err(failure)?)
}
async fn runtime_outcome(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<OutcomeQuery>,
) -> Result<Response> {
    let q = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    state.json(&m.runtime_outcome(actor, q.request).await.map_err(failure)?)
}
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

async fn catalog(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<CatalogQuery>,
) -> Result<Response> {
    let q = routes::input(query)?;
    let (m, actor) = manager(&state)?;
    let value = if let Some(setup) = q.setup {
        m.catalog_for_setup(actor, q.runner, setup).await
    } else {
        m.catalog(actor, q.runner).await
    };
    state.json(&value.map_err(failure)?)
}

mutation!(
    save_settings,
    agents::settings::SettingsCommand,
    save_settings
);
mutation!(secret, agents::settings::SecretCommand, secret);
async fn settings(
    axum::extract::State(state): axum::extract::State<Arc<crate::State>>,
) -> Result<Json<agents::settings::Settings>> {
    let (manager, actor) = manager(&state)?;
    Ok(Json(manager.settings(actor).await.map_err(failure)?))
}

mutation!(compute_command, agents::compute::Command, compute_command);
async fn compute_status(
    axum::extract::State(state): axum::extract::State<Arc<crate::State>>,
) -> Result<Json<agents::compute::Status>> {
    let (manager, actor) = manager(&state)?;
    let mut value = manager.compute_status(actor).await.map_err(failure)?;
    value.model_file_picker = state
        .config
        .local_folder_picker
        .as_ref()
        .is_some_and(|picker| picker.model_files_supported());
    Ok(Json(value))
}

async fn pick_model_file(
    axum::extract::State(state): axum::extract::State<Arc<crate::State>>,
    query: routes::Input<routes::Empty>,
    request: std::result::Result<Json<routes::Empty>, JsonRejection>,
) -> Result<Json<jj_store_service::local_management::ModelFileSelection>> {
    manager(&state)?;
    routes::input(query)?;
    body(request)?;
    let picker = state
        .config
        .local_folder_picker
        .as_ref()
        .filter(|picker| picker.model_files_supported())
        .ok_or_else(|| {
            Error::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "model_file_picker_unavailable",
            )
        })?;
    let selection = picker.pick_model_file().await.map_err(|error| {
        use jj_store_service::local_management::PickerError;
        let (status, code) = match error {
            PickerError::Unavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "model_file_picker_unavailable",
            ),
            PickerError::Busy => (StatusCode::CONFLICT, "model_file_picker_busy"),
            PickerError::Timeout => (StatusCode::REQUEST_TIMEOUT, "model_file_picker_timeout"),
            PickerError::Invalid => (StatusCode::BAD_GATEWAY, "model_file_picker_invalid_output"),
        };
        Error::new(status, code)
    })?;
    Ok(Json(selection))
}

async fn catalog_preview(
    ExtractState(state): ExtractState<Arc<State>>,
    body: std::result::Result<Json<agents::catalog::Preview>, JsonRejection>,
) -> Result<Response> {
    let (m, actor) = manager(&state)?;
    let Json(preview) = body.map_err(|_| Error::invalid())?;
    state.json(&m.catalog_preview(actor, preview).await.map_err(failure)?)
}

async fn chatgpt_status(ExtractState(state): ExtractState<Arc<State>>) -> Result<Response> {
    let (m, actor) = manager(&state)?;
    state.json(&m.chatgpt_status(actor).await.map_err(failure)?)
}
mutation!(chatgpt_command, agents::chatgpt::Command, chatgpt_command);
