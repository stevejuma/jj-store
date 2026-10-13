fn main() -> std::process::ExitCode {
    if let Some(code) = jj_store_cli::workflow_host::worker_entry() {
        return code.into();
    }
    if let Some(code) = jj_store_cli::git_bridge::transport_worker() {
        return code.into();
    }
    let runner = jj_cli::cli_util::CliRunner::init();
    if let Some(code) = jj_store_cli::presentation::preflight() {
        return code.into();
    }
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(_) => {
            eprintln!(
                "Error: Could not initialize the native client runtime\nHint: Check local process and thread resource limits."
            );
            return 1.into();
        }
    };
    let _entered = runtime.enter();
    let registry = jj_store_cli::binding::FactoryRegistry::default();
    runner
        .add_store_factories(registry.factories())
        .add_subcommand(jj_store_cli::commands::run)
        .add_dispatch_hook(async move |ui, helper, inner| {
            jj_store_cli::presentation::dispatch(ui, helper, inner, &registry).await
        })
        .run()
        .into()
}
