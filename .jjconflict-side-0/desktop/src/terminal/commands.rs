//! Narrow native commands: no arbitrary program, path, environment or HTTP proxy.
use super::{
    Result, Runtime, SessionInfo, Target, TargetResolver,
    wire::{AttachResponse, TerminalMessage, WireViewport},
};
use std::sync::{Arc, Mutex};
use tauri::{State, WebviewWindow, ipc::Channel};

#[derive(Default)]
pub struct TerminalHost {
    pub runtime: Arc<Runtime>,
    pub resolver: Mutex<Option<TargetResolver>>,
}
fn admit(window: &WebviewWindow, state: &TerminalHost) -> Result<TargetResolver> {
    let resolver = state
        .resolver
        .lock()
        .map_err(|_| "Terminal host unavailable")?
        .clone()
        .ok_or("Terminal host is starting")?;
    if window.label() != "main"
        || !crate::readiness::navigation_allowed(
            resolver.origin(),
            &window.url().map_err(|e| e.to_string())?,
        )
    {
        return Err("Terminal is restricted to the managed main window".into());
    }
    Ok(resolver)
}
async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T> + Send + 'static) -> Result<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|_| "Terminal task failed")?
}
#[tauri::command]
pub async fn terminal_targets(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    repository: String,
    profile: String,
) -> Result<Vec<Target>> {
    let resolver = admit(&window, &state)?;
    blocking(move || resolver.list(&repository, &profile)).await
}
#[tauri::command]
pub fn terminal_list(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
) -> Result<Vec<SessionInfo>> {
    admit(&window, &state)?;
    state.runtime.list()
}
#[tauri::command]
pub async fn terminal_open(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    request_id: String,
    repository: String,
    profile: String,
    target_id: String,
) -> Result<SessionInfo> {
    let resolver = admit(&window, &state)?;
    let runtime = state.runtime.clone();
    blocking(move || {
        runtime.open(
            &request_id,
            resolver.resolve(&repository, &profile, &target_id)?,
            80,
            24,
        )
    })
    .await
}
#[tauri::command]
pub async fn terminal_attach(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    on_frame: Channel<TerminalMessage>,
) -> Result<AttachResponse> {
    admit(&window, &state)?;
    let runtime = state.runtime.clone();
    blocking(move || {
        runtime.attach(
            &session_id,
            Arc::new(move |message| on_frame.send(message).map_err(|e| e.to_string())),
        )
    })
    .await
}
#[tauri::command]
pub async fn terminal_close(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
) -> Result<()> {
    admit(&window, &state)?;
    let runtime = state.runtime.clone();
    blocking(move || runtime.close(&session_id)).await
}
#[tauri::command]
pub fn terminal_input(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    data: String,
) -> Result<()> {
    admit(&window, &state)?;
    state.runtime.input(&session_id, &data)
}
#[tauri::command]
pub async fn terminal_resize(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    columns: u16,
    rows: u16,
) -> Result<WireViewport> {
    admit(&window, &state)?;
    let runtime = state.runtime.clone();
    blocking(move || runtime.resize(&session_id, columns, rows)).await
}
#[tauri::command]
pub fn terminal_scroll(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    lines: i32,
) -> Result<()> {
    admit(&window, &state)?;
    state.runtime.scroll(&session_id, lines)
}
#[tauri::command]
pub fn terminal_focus(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    focused: bool,
) -> Result<()> {
    admit(&window, &state)?;
    state.runtime.focus(&session_id, focused)
}
#[tauri::command]
pub fn terminal_ack(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    subscription_id: String,
    sequence: u64,
) -> Result<()> {
    admit(&window, &state)?;
    state
        .runtime
        .acknowledge(&session_id, &subscription_id, sequence)
}
#[tauri::command]
pub fn terminal_viewport_ready(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    subscription_id: String,
    viewport: WireViewport,
) -> Result<()> {
    admit(&window, &state)?;
    state
        .runtime
        .viewport_ready(&session_id, &subscription_id, viewport)
}
#[tauri::command]
pub fn terminal_detach(
    window: WebviewWindow,
    state: State<'_, TerminalHost>,
    session_id: String,
    subscription_id: String,
) -> Result<()> {
    admit(&window, &state)?;
    state.runtime.detach(&session_id, &subscription_id)
}
