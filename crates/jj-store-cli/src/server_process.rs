//! Foreground composition and readiness shared by ordinary and supervised CLI.
use crate::{commands::Format, server_desktop::DesktopOptions};
use jj_cli::ui::Ui;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_server::{
    config::Config,
    process::{self, PreparedServer},
};
use std::io::Write as _;

pub async fn serve(
    ui: &Ui,
    path: &std::path::Path,
    format: Format,
    local: bool,
    options: DesktopOptions,
) -> AuthorityResult<()> {
    serve_with_workflows(ui, path, format, local, options, None).await
}

pub async fn serve_with_workflows(
    ui: &Ui,
    path: &std::path::Path,
    format: Format,
    local: bool,
    options: DesktopOptions,
    workflow_host: Option<std::path::PathBuf>,
) -> AuthorityResult<()> {
    if let Some(path) = &workflow_host {
        crate::workflow_host::Config::load(path)?;
    }
    let mut config = Config::load(path)?;
    options.apply(&mut config)?;
    #[cfg(unix)]
    let owner = if options.desktop_supervised {
        Some(crate::server_desktop::owner_pipe()?)
    } else {
        None
    };
    #[cfg(not(unix))]
    if options.desktop_supervised {
        return Err("desktop supervision requires Unix".into());
    }
    let verifier = match &config.access {
        jj_store_server::config::Access::LocalTrusted { rpc_socket, .. } => Some(
            std::sync::Arc::new(crate::workspace_management::NativeWorkspaceVerifier::new(
                rpc_socket,
            )?)
                as std::sync::Arc<dyn jj_store_service::local_management::WorkspaceVerifier>,
        ),
        _ => None,
    };
    let server = PreparedServer::prepare_with_browser(
        &config,
        local,
        verifier,
        crate::folder_picker::native_picker(),
        Some(path),
        Some(std::sync::Arc::new(crate::local_browser::LocalFolders)),
    )
    .await?;
    let address = server.address()?;
    let signal = process::shutdown_signal()?;
    if format == Format::Json {
        writeln!(
            ui.stdout(),
            "{}",
            serde_json::json!({"version":1,"event":"listening","address":address,"transport":if config.tls.is_some(){"tls"}else{"explicit-loopback"},"storage_readiness":"unverified"})
        )?;
    } else {
        writeln!(
            ui.stdout(),
            "Listening on {address}\nStorage readiness: use jj-store doctor for a known repository."
        )?;
    }
    ui.stdout().flush()?;
    let host = workflow_host.map(|path| {
        tokio::spawn(async move {
            if crate::workflow_host::run(crate::workflow_host::Args {
                config: path,
                once: false,
                check: false,
            })
            .await
            .is_err()
            {
                eprintln!("Workflow host stopped; repository service remains available");
            }
        })
    });
    let result = server.run(async move {
        #[cfg(unix)]
        if let Some(input) = owner {
            tokio::select! { _ = signal => {}, _ = crate::server_desktop::owner_closed(input) => {} }
            return;
        }
        signal.await;
    }).await;
    if let Some(host) = host {
        host.abort();
        let _ = host.await;
    }
    result?;
    Ok(())
}
