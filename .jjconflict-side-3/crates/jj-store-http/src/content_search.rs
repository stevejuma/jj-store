use crate::*;
use jj_store_service::content_search as service;
pub(super) async fn page(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: routes::Input<service::ContentQuery>,
) -> Result<Response> {
    state.json(
        &state
            .content
            .page(context.session, routes::input(args)?)
            .await?,
    )
}
