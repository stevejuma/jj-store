//! Integrated macOS chrome, with window-only permissions for the managed origin.
use crate::readiness;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

pub fn build(handle: &AppHandle, origin: url::Url) -> tauri::Result<WebviewWindow> {
    let builder = WebviewWindowBuilder::new(handle, "main", WebviewUrl::External(origin.clone()))
        .title("jj-store")
        .inner_size(1200.0, 800.0)
        .min_inner_size(900.0, 650.0)
        .visible(false);
    #[cfg(target_os = "macos")]
    let builder = {
        handle.add_capability(
            tauri::ipc::CapabilityBuilder::new("managed-main-window-chrome")
                .local(false)
                .window("main")
                .remote(format!("{}*", origin.as_str()))
                .permission("core:window:allow-start-dragging")
                .permission("core:window:allow-internal-toggle-maximize"),
        )?;
        builder
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true)
            .traffic_light_position(tauri::LogicalPosition::new(16.0, 25.0))
            .initialization_script(
                "Object.defineProperty(window, '__JJ_STORE_DESKTOP__', { value: Object.freeze({ version: 1, platform: 'macos' }) });",
            )
    };
    let links = handle.clone();
    builder
        .on_navigation(move |target| {
            if readiness::navigation_allowed(&origin, target) {
                return true;
            }
            if matches!(target.scheme(), "https" | "http")
                && target.username().is_empty()
                && target.password().is_none()
            {
                let _ = links.opener().open_url(target.as_str(), None::<&str>);
            }
            false
        })
        .on_page_load(|window, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished {
                eprintln!("jj-store desktop: managed page loaded");
                let _ = window.show();
                let _ = window.set_focus();
            }
        })
        .build()
}
