fn main() -> std::process::ExitCode {
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
    runner
        .add_store_factories(jj_store_cli::binding::lazy_factories())
        .add_subcommand(jj_store_cli::commands::run)
        .add_dispatch_hook(jj_store_cli::presentation::dispatch)
        .run()
        .into()
}
