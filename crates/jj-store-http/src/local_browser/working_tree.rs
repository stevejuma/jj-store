//! Registered physical observation. No client paths, contents, staging or mutations.
use super::*;
use jj_store_service::working_tree as tree;
use local::Access as _;
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repos/{repo}/working-tree/observe", observe, "local_working_tree_observe", tree::ObserveWorkingTree, tree::ObservedWorkingTree)
    } };
}
pub(super) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:ident,$id:literal,$body:ty,$response:ty)),*) => { Router::new()$(.route($path,$method($handler)))* };
    }
    endpoints!(register)
}
fn error(error: tree::Error) -> Error {
    match error {
        tree::Error::Invalid => Error::invalid(),
        tree::Error::Limit => Error::limit(),
        tree::Error::Unavailable => Error::unavailable(),
        tree::Error::Denied => Error::denied(),
        tree::Error::Busy => Error::new(StatusCode::SERVICE_UNAVAILABLE, "working_tree_busy"),
        tree::Error::Stale => Error::new(StatusCode::CONFLICT, "stale_working_tree"),
        tree::Error::Unsupported => {
            Error::new(StatusCode::UNPROCESSABLE_ENTITY, "unsupported_working_tree")
        }
        tree::Error::Cancelled => Error::new(StatusCode::REQUEST_TIMEOUT, "working_tree_cancelled"),
    }
}
async fn observe(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repo): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<tree::ObserveWorkingTree>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    if request.version != 1 {
        return Err(Error::invalid());
    }
    let access = bound_folder(&state, repo).await?;
    if access.generation() != request.binding_generation {
        return Err(error(tree::Error::Stale));
    }
    let root = std::path::PathBuf::from(&access.folder.path);
    let value = state
        .working_tree
        .observe(access.clone(), root, request.operation)
        .await
        .map_err(error)?;
    access.authorize().await?;
    state.json(&value.summary().map_err(error)?)
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
#[cfg(test)]
mod tests;
