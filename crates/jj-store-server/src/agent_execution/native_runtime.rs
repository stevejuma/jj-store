//! Host admission stays here; Lapis AI owns bounded native transport and cleanup.
use super::*;
use lapis_ai_runtime::{
    AcpClient, AcpCommands, AcpOptions, Config, PreparedLaunch, ProcessLimits, ResourceBundle,
};

pub(super) fn limits(maximum_frame_bytes: usize) -> ProcessLimits {
    ProcessLimits {
        maximum_frame_bytes,
        maximum_request_bytes: 1024 * 1024,
        write_timeout: Duration::from_secs(5),
        ..Default::default()
    }
}
pub(super) async fn start(command: &tokio::process::Command) -> Result<AcpClient> {
    let plan = PreparedLaunch::from_command(command.as_std()).map_err(|_| Error::Invalid)?;
    AcpClient::start_prepared_with_options(
        plan,
        AcpOptions {
            limits: limits(1024 * 1024),
            readiness_timeout: Duration::from_secs(15),
            shutdown_grace: Duration::from_secs(15),
        },
    )
    .await
    .map_err(|_| Error::Unavailable)
}
pub(super) async fn send(input: &AcpCommands, value: Value) -> Result<()> {
    input
        .send_frame(&value)
        .await
        .map_err(|_| Error::Unavailable)
}
pub(crate) fn executable() -> Option<PathBuf> {
    std::env::var_os("JJ_STORE_ACP_EXECUTABLE")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("JJ_STORE_AI_BUNDLE_DIR")
                .map(|root| PathBuf::from(root).join("lapis-acp"))
        })
}
pub(super) fn bundle() -> Result<Option<Config>> {
    let Some(root) = std::env::var_os("JJ_STORE_AI_BUNDLE_DIR").map(PathBuf::from) else {
        return Ok(None);
    };
    let bundle = ResourceBundle::load(&root).map_err(|_| Error::Unavailable)?;
    let mut config = Config::parse("version=1", &root).map_err(|_| Error::Invalid)?;
    bundle.configure(&mut config);
    Ok(Some(config))
}
