//! Buzz's opaque-NSWindow / transparent-WKWebView vibrancy composition.
//! Adapted from Buzz window_vibrancy.rs at ef2aa1ae (Apache-2.0).
#[tauri::command]
pub fn set_glass_background(window: tauri::WebviewWindow, enabled: bool) -> Result<(), String> {
    if window.label() != "main" {
        return Err("Glass is restricted to the managed main window".into());
    }
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{NSVisualEffectMaterial, apply_vibrancy, clear_vibrancy};
        // Clear before applying so repeated requests never stack effect views.
        clear_vibrancy(&window).map_err(|error| error.to_string())?;
        if enabled {
            apply_vibrancy(&window, NSVisualEffectMaterial::Sidebar, None, None)
                .map_err(|error| error.to_string())?;
            // WebviewWindow's setter also changes NSWindow; target only WKWebView.
            let webview: &tauri::Webview<_> = window.as_ref();
            webview
                .set_background_color(Some(tauri::window::Color(0, 0, 0, 0)))
                .map_err(|error| error.to_string())?;
        }
        // Wry's drawsBackground flag is one-way; glass-off CSS paints opaque again.
        crate::macos_chrome::align(&window);
    }
    #[cfg(not(target_os = "macos"))]
    let _ = enabled;
    Ok(())
}
