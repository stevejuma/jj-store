//! Managed window, terminal and service lifecycle.
use crate::terminal::{
    TargetResolver,
    commands::{self, TerminalHost},
};
use crate::{installation::Resources, native_window, runtime::ManagedService};
use std::sync::Mutex;
use tauri::{
    Manager, RunEvent, WindowEvent,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_window_state::StateFlags;

struct RuntimeState(Mutex<Option<ManagedService>>);
fn show(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}
pub fn run() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crate::native_glass::set_glass_background, commands::terminal_targets, commands::terminal_list, commands::terminal_open, commands::terminal_attach, commands::terminal_close, commands::terminal_input, commands::terminal_resize, commands::terminal_scroll, commands::terminal_focus, commands::terminal_ack, commands::terminal_viewport_ready, commands::terminal_detach])
        .plugin(tauri_plugin_single_instance::init(|app, _, _| show(app)))
        .plugin(tauri_plugin_window_state::Builder::default().with_state_flags(StateFlags::all() & !StateFlags::VISIBLE).build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(RuntimeState(Mutex::new(None)))
        .manage(TerminalHost::default())
        .setup(|app| {
            let handle = app.handle().clone();
            let show_item = MenuItem::with_id(app, "show", "Show jj-store", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit jj-store", true, Some("CmdOrCtrl+Q"))?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
            let mut tray = TrayIconBuilder::new().menu(&menu).show_menu_on_left_click(true).on_menu_event(|app, event| match event.id.as_ref() { "show" => show(app), "quit" => app.exit(0), _ => {} });
            if let Some(icon) = app.default_window_icon() { tray = tray.icon(icon.clone()); }
            tray.build(app)?;
            let mut root = handle.path().resource_dir()?.join("resources");
            if cfg!(debug_assertions) && !root.is_dir() { root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources"); }
            let state = if cfg!(debug_assertions) {
                std::env::var_os("JJ_STORE_DESKTOP_DATA_DIR").map(std::path::PathBuf::from).unwrap_or(handle.path().app_data_dir()?)
            } else { handle.path().app_data_dir()? };
            std::thread::spawn(move || {
                let result = Resources::from_root(&root).and_then(|resources| ManagedService::start(&state, &resources));
                match result {
                    Ok(service) => {
                        let origin = service.origin.clone();
                        if let Ok(mut resolver) = handle.state::<TerminalHost>().resolver.lock() {
                            *resolver = TargetResolver::new(origin.clone()).ok();
                        }
                        if let Ok(mut owner) = handle.state::<RuntimeState>().0.lock() { *owner = Some(service); } else { handle.exit(1); return; }
                        let window = native_window::build(&handle, origin);
                        if let Err(error) = window { eprintln!("jj-store desktop window: {error}"); handle.dialog().message(format!("Cannot open jj-store: {error}")).blocking_show(); handle.exit(1); return; }
                        loop {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            let failed = match handle.state::<RuntimeState>().0.lock() {
                                Ok(mut owner) => match owner.as_mut() { Some(service) => !service.running().unwrap_or(false), None => return },
                                Err(_) => return,
                            };
                            if failed { handle.dialog().message("The local jj-store service stopped. Relaunch to recover using the retained installation and original pending request identities.").blocking_show(); handle.exit(1); return; }
                        }
                    }
                    Err(error) => { eprintln!("jj-store desktop startup: {error}"); handle.dialog().message(format!("jj-store could not start.\n\n{error}\n\nYour installation has been retained. Relaunch to retry.")).blocking_show(); handle.exit(1); }
                }
            });
            Ok(())
        })
        .build(tauri::generate_context!());
    let app = match app {
        Ok(app) => app,
        Err(error) => {
            eprintln!("Cannot initialize jj-store desktop: {error}");
            return;
        }
    };
    app.run(|handle, event| match event {
        RunEvent::Reopen { .. } => show(handle),
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } if label == "main" => {
            api.prevent_close();
            if let Some(window) = handle.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        RunEvent::ExitRequested { .. } | RunEvent::Exit => {
            if let Err(error) = handle.state::<TerminalHost>().runtime.shutdown() {
                eprintln!("Desktop terminal shutdown: {error}");
            }
            #[cfg(target_os = "macos")]
            crate::macos_chrome::shutdown();
            if let Ok(mut service) = handle.state::<RuntimeState>().0.lock()
                && let Some(mut owner) = service.take()
                && let Err(error) = owner.stop()
            {
                eprintln!("jj-store desktop shutdown: {error}");
            }
        }
        _ => {}
    });
}
