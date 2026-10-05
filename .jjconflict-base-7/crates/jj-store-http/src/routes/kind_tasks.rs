use super::*;
use jj_store_service::kind_tasks::Kind;

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "PrepareKind")]
pub(super) struct Prepare {
    operation: m::OperationId,
    kind: Kind,
}

pub(super) async fn prepare(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
    body: std::result::Result<Json<Prepare>, axum::extract::rejection::JsonRejection>,
) -> Result<Response> {
    input(args)?;
    let Json(request) = body.map_err(|_| Error::invalid())?;
    let task = state
        .kind_tasks
        .start(context.session, request.operation, request.kind)
        .await?;
    let mut response = state.json(&task)?;
    *response.status_mut() = StatusCode::ACCEPTED;
    Ok(response)
}
pub(super) async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let task = state
        .kind_tasks
        .status(context.session.clone(), context.value("task")?)
        .await?;
    state.json(&task)
}
pub(super) async fn cancel(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let task = state
        .kind_tasks
        .cancel(context.session.clone(), context.value("task")?)
        .await?;
    state.json(&task)
}
