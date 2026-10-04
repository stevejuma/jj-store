//! Strict LocalTrusted Admin installation conversion; private archives never cross HTTP.
use super::*;
use jj_store_service::conversion::contracts as conversion;
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repositories/conversion/preview", preview, "local_repository_conversion_preview", conversion::ConversionIntent, conversion::ConversionPreview),
        (post, "/local/repositories/conversion", start, "local_repository_conversion_start", conversion::StartConversion, conversion::ConversionOutcome),
        (get, "/local/repositories/conversion/requests/{request_id}", status, "local_repository_conversion_status", routes::Empty, conversion::ConversionOutcome)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register { ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => { Router::new()$(.route($path,$method($handler)))* }; }
    endpoints!(register)
}
async fn preview(
    ExtractState(state): ExtractState<Arc<State>>,
    body: Body<conversion::ConversionIntent>,
) -> Result<Response> {
    let manager = manager(&state)?;
    let intent = decode_body(body)?;
    let _permit = state
        .local_jobs
        .clone()
        .acquire_owned()
        .await
        .map_err(|_| Error::busy())?;
    state.json(&manager.preview_conversion(intent).await.map_err(map)?)
}
async fn start(
    ExtractState(state): ExtractState<Arc<State>>,
    body: Body<conversion::StartConversion>,
) -> Result<Response> {
    let manager = manager(&state)?;
    let request = decode_body(body)?;
    let _permit = state
        .local_jobs
        .clone()
        .acquire_owned()
        .await
        .map_err(|_| Error::busy())?;
    state.json(&manager.start_conversion(request).await.map_err(map)?)
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(request): Path<m::RequestId>,
    args: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(args)?;
    let manager = manager(&state)?;
    let _permit = state
        .local_jobs
        .clone()
        .acquire_owned()
        .await
        .map_err(|_| Error::busy())?;
    state.json(&manager.conversion_status(request).await.map_err(map)?)
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
