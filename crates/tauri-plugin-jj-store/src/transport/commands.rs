use crate::{Context, Error, History, Result, frames, wire};
use std::sync::Arc;
use tauri::{Runtime, State, WebviewWindow};

fn caller<R: Runtime>(window: &WebviewWindow<R>) -> Result<Context> {
    Ok(Context {
        window: window.label().into(),
        url: window.url().map_err(|_| Error::Denied)?.to_string(),
    })
}
#[tauri::command]
pub async fn heads<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: String,
) -> Result<String> {
    let request: frames::Vault = wire::decode(&request)?;
    wire::encode(host.heads(&caller(&window)?, &request.vault).await?)
}
#[tauri::command]
pub async fn open<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: String,
) -> Result<String> {
    let request: frames::Open = wire::decode(&request)?;
    wire::encode(
        host.inner()
            .open(&caller(&window)?, &request.vault, request.operation)
            .await?,
    )
}
#[tauri::command]
pub async fn status<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: String,
) -> Result<String> {
    let request: frames::Selection = wire::decode(&request)?;
    wire::encode(
        host.status(&caller(&window)?, &request.vault, request.handle)
            .await?,
    )
}
#[tauri::command]
pub async fn read<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: String,
) -> Result<String> {
    let request: frames::Read = wire::decode(&request)?;
    wire::encode(
        host.read(
            &caller(&window)?,
            &request.vault,
            request.handle,
            request.query,
        )
        .await?,
    )
}
#[tauri::command]
pub async fn blame<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: String,
) -> Result<String> {
    let request: frames::Blame = wire::decode(&request)?;
    wire::encode(
        host.blame(
            &caller(&window)?,
            &request.vault,
            request.handle,
            request.query,
        )
        .await?,
    )
}
#[tauri::command]
pub async fn close<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: String,
) -> Result<String> {
    let request: frames::Selection = wire::decode(&request)?;
    wire::encode(
        host.close(&caller(&window)?, &request.vault, request.handle)
            .await?,
    )
}
