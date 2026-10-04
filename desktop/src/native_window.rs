//! Integrated macOS chrome, with window-only permissions for the managed origin.
use crate::readiness;
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

pub fn build(handle: &AppHandle, origin: url::Url) -> tauri::Result<WebviewWindow> {
    let config = tauri::utils::config::WindowConfig {
        label: "main".into(),
        title: "jj-store".into(),
        url: tauri::WebviewUrl::External(origin.clone()),
        width: 1200.0,
        height: 800.0,
        min_width: Some(900.0),
        min_height: Some(650.0),
        visible: false,
        #[cfg(target_os = "macos")]
        title_bar_style: tauri::TitleBarStyle::Overlay,
        #[cfg(target_os = "macos")]
        hidden_title: true,
        // Configure Tao as well as Wry so restored/resized windows retain the inset.
        #[cfg(target_os = "macos")]
        traffic_light_position: Some(tauri::utils::config::LogicalPosition { x: 16.0, y: 25.0 }),
        ..Default::default()
    };
    let builder = WebviewWindowBuilder::from_config(handle, &config)?;
    #[cfg(target_os = "macos")]
    let builder = {
        handle.add_capability(
            tauri::ipc::CapabilityBuilder::new("managed-main-window-chrome")
                .local(false)
                .window("main")
                .remote(format!("{}*", origin.as_str()))
                .permission("core:window:allow-start-dragging")
                .permission("core:window:allow-internal-toggle-maximize")
                .permission("allow-set-glass-background")
                .permission("allow-terminal-targets")
                .permission("allow-terminal-list")
                .permission("allow-terminal-open")
                .permission("allow-terminal-attach")
                .permission("allow-terminal-close")
                .permission("allow-terminal-input")
                .permission("allow-terminal-resize")
                .permission("allow-terminal-scroll")
                .permission("allow-terminal-focus")
                .permission("allow-terminal-ack")
                .permission("allow-terminal-viewport-ready")
                .permission("allow-terminal-detach"),
        )?;
        builder
            .initialization_script(
                "Object.defineProperty(window, '__JJ_STORE_DESKTOP__', { value: Object.freeze({ version: 1, platform: 'macos', terminal: 1 }) });",
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
                #[cfg(target_os = "macos")]
                crate::macos_chrome::align(&window);
            }
        })
        .build()
        .inspect(|window| {
            #[cfg(target_os = "macos")]
            {
                let chrome = window.clone();
                window.on_window_event(move |event| {
                    if matches!(
                        event,
                        tauri::WindowEvent::Focused(_)
                            | tauri::WindowEvent::Resized(_)
                            | tauri::WindowEvent::Moved(_)
                    ) {
                        crate::macos_chrome::align(&chrome);
                    }
                });
            }
        })
}
