//! Physical-file reads and Save are separate from recorded repository commands.
use super::*;
use jj_store_service::working_copy as wc;
macro_rules! endpoints {
    ($register:ident) => { $register! {
        (post, "/local/repos/{repo}/working-copy/read", read, "local_working_file_read", wc::ReadWorkingFile, wc::WorkingFile),
        (post, "/local/repos/{repo}/working-copy/save", save, "local_working_file_save", wc::SaveWorkingFile, wc::WorkingFile)
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
fn error(error: wc::Error) -> Error {
    match error {
        wc::Error::Invalid => Error::invalid(),
        wc::Error::Stale => Error::new(StatusCode::CONFLICT, "stale_working_file"),
        wc::Error::Unsupported => {
            Error::new(StatusCode::UNPROCESSABLE_ENTITY, "unsupported_working_file")
        }
        wc::Error::Limit => Error::limit(),
        wc::Error::Unavailable => Error::unavailable(),
    }
}
async fn read(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<wc::ReadWorkingFile>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    let value = blocking(&state, move || {
        let (folder, generation) = manager.folder(repository).map_err(map)?;
        let value = wc::read(
            std::path::Path::new(&folder.path),
            repository,
            generation,
            &request.path,
        )
        .map_err(error)?;
        if manager.folder(repository).map_err(map)? != (folder, generation) {
            return Err(Error::new(StatusCode::CONFLICT, "stale_working_file"));
        }
        Ok(value)
    })
    .await?;
    state.json(&value)
}
async fn save(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    body: Body<wc::SaveWorkingFile>,
) -> Result<Response> {
    routes::input(args)?;
    let request = decode_body(body)?;
    let manager = manager(&state)?;
    let value = blocking(&state, move || {
        let (folder, generation) = manager.folder(repository).map_err(map)?;
        wc::save(
            std::path::Path::new(&folder.path),
            repository,
            generation,
            &request.selection,
            &request.contents,
        )
        .map_err(error)
    })
    .await?;
    state.json(&value)
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
