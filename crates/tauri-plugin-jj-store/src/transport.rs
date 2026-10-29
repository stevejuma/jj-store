mod commands;
use crate::History;
use std::sync::Arc;
use tauri::{Manager, RunEvent, Runtime, WindowEvent, plugin::TauriPlugin};

pub struct Builder {
    history: Arc<History>,
}
impl Builder {
    pub fn new(history: Arc<History>) -> Self {
        Self { history }
    }
    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        tauri::plugin::Builder::new("jj-store")
            .invoke_handler(tauri::generate_handler![
                commands::heads,
                commands::open,
                commands::status,
                commands::read,
                commands::blame,
                commands::close
            ])
            .setup(move |app, _| {
                if !app.manage(self.history) {
                    return Err("JJ history already registered".into());
                }
                Ok(())
            })
            .on_event(|app, event| {
                if let Some(host) = app.try_state::<Arc<History>>() {
                    match event {
                        RunEvent::Exit => report(tauri::async_runtime::block_on(host.shutdown())),
                        RunEvent::WindowEvent {
                            label,
                            event: WindowEvent::Destroyed,
                            ..
                        } => {
                            let host = host.inner().clone();
                            let label = label.clone();
                            tauri::async_runtime::spawn(async move {
                                report(host.revoke(&label).await);
                            });
                        }
                        _ => {}
                    }
                }
            })
            .on_drop(|app| {
                if let Some(host) = app.try_state::<Arc<History>>() {
                    report(tauri::async_runtime::block_on(host.shutdown()));
                }
            })
            .build()
    }
}
fn report(result: crate::Result<()>) {
    if let Err(error) = result {
        eprintln!("JJ history cleanup failed: {error}");
    }
}
