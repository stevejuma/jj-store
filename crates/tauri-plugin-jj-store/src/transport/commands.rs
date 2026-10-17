use crate::{Context, Error, Handle, History, OperationId, Result, Status, local};
use serde::Deserialize;
use std::sync::Arc;
use tauri::{Runtime, State, WebviewWindow};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vault {
    vault: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Open {
    vault: String,
    operation: OperationId,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    vault: String,
    handle: Handle,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Read {
    vault: String,
    handle: Handle,
    query: local::Request,
}

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
    request: Vault,
) -> Result<local::Heads> {
    host.heads(&caller(&window)?, &request.vault).await
}
#[tauri::command]
pub async fn open<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: Open,
) -> Result<Handle> {
    host.inner()
        .open(&caller(&window)?, &request.vault, request.operation)
        .await
}
#[tauri::command]
pub async fn status<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: Selection,
) -> Result<Status> {
    host.status(&caller(&window)?, &request.vault, request.handle)
        .await
}
#[tauri::command]
pub async fn read<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: Read,
) -> Result<local::Response> {
    host.read(
        &caller(&window)?,
        &request.vault,
        request.handle,
        request.query,
    )
    .await
}
#[tauri::command]
pub async fn close<R: Runtime>(
    window: WebviewWindow<R>,
    host: State<'_, Arc<History>>,
    request: Selection,
) -> Result<()> {
    host.close(&caller(&window)?, &request.vault, request.handle)
        .await
}

#[cfg(test)]
mod tests;
