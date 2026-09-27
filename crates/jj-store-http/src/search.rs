//! Transport parsing over repository-scoped, independently rebuildable projections.
use crate::{
    routes::{Empty, Input, input},
    *,
};
use jj_store_tantivy::paging;

/// A trusted serving composition supplies the projection and its live runner health.
pub struct SearchIndex {
    pub search: jj_store_tantivy::Search,
    pub health: tokio::sync::watch::Receiver<jj_store_adapters::Health>,
    pub limits: paging::Limits,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Query {
    q: String,
    #[serde(default)]
    mode: paging::Mode,
    at_op: Option<m::OperationId>,
    scope: Option<String>,
    #[serde(default = "page_size")]
    page_size: usize,
    minimum_position: Option<m::WalPosition>,
    #[serde(default)]
    wait_millis: u64,
    cursor: Option<String>,
}
fn page_size() -> usize {
    50
}
fn pager<'a>(state: &'a State, context: &Context) -> Result<&'a paging::Pager> {
    state
        .search
        .get(&context.id("repo")?)
        .ok_or_else(|| Error::new(StatusCode::NOT_IMPLEMENTED, "search_not_configured"))
}
pub(crate) async fn search(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Query>,
) -> Result<Response> {
    let args = input(args)?;
    let pager = pager(&state, &context)?;
    let request = paging::Request {
        q: args.q,
        mode: args.mode,
        operation: args.at_op,
        scope: args.scope,
        page_size: args.page_size,
        minimum_position: args.minimum_position,
        wait_millis: args.wait_millis,
    };
    let page = if let Some(cursor) = args.cursor {
        pager.resume(context.session, &cursor, request).await?
    } else {
        pager.begin(context.session, request).await?
    };
    state.json(&page)
}
pub(crate) async fn search_status(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let status = pager(&state, &context)?.status(context.session).await?;
    state.json(&status)
}
