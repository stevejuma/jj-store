//! Opt-in desktop owner lifetime and launch-only configuration overrides.
use clap::Args;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_server::config::{Access, Config, Storage};
use std::{net::SocketAddr, path::PathBuf};
#[cfg(unix)]
use tokio::{io::AsyncReadExt, net::unix::pipe::Receiver};

#[derive(Debug, Default, Args)]
pub struct DesktopOptions {
    /// Stop the local service when its desktop owner's stdin pipe closes
    #[arg(long)]
    pub desktop_supervised: bool,
    /// Override the literal-loopback listener for this supervised launch only
    #[arg(long, requires = "desktop_supervised")]
    pub desktop_bind: Option<SocketAddr>,
    /// Use the current app bundle's production browser assets
    #[arg(long, requires = "desktop_supervised")]
    pub desktop_web_root: Option<PathBuf>,
    /// Use the current app bundle's paired RustFS executable
    #[arg(long, requires = "desktop_supervised")]
    pub desktop_rustfs_executable: Option<PathBuf>,
}

impl DesktopOptions {
    pub fn apply(&self, config: &mut Config) -> AuthorityResult<()> {
        if !self.desktop_supervised {
            return Ok(());
        }
        if !matches!(config.access, Access::LocalTrusted { .. })
            || !matches!(config.storage, Storage::EmbeddedRustfs { .. })
            || !config.bind.ip().is_loopback()
            || config.development_browser_annotations
            || config.tls.is_some()
        {
            return Err("desktop supervision requires strict local embedded storage".into());
        }
        if let Some(address) = self.desktop_bind {
            if !address.ip().is_loopback() || address.port() == 0 {
                return Err("desktop bind requires concrete literal loopback".into());
            }
            config.bind = address;
            config.public_url = format!("http://{address}/");
        }
        if let Some(root) = &self.desktop_web_root {
            config.web_root = Some(root.canonicalize()?);
        }
        if let Some(runtime) = &self.desktop_rustfs_executable {
            let Storage::EmbeddedRustfs {
                runtime_executable, ..
            } = &mut config.storage
            else {
                return Err("desktop storage changed".into());
            };
            *runtime_executable = runtime.canonicalize()?;
        }
        config.validate()?;
        Ok(())
    }
}

/// Registers a nonblocking owner pipe before announcing the listener.
#[cfg(unix)]
pub fn owner_pipe() -> AuthorityResult<Receiver> {
    let input = rustix::io::dup(std::io::stdin())?;
    if rustix::fs::FileType::from_raw_mode(rustix::fs::fstat(&input)?.st_mode)
        != rustix::fs::FileType::Fifo
    {
        return Err("desktop supervision requires an owner stdin pipe".into());
    }
    Ok(Receiver::from_owned_fd(input)?)
}

#[cfg(unix)]
pub async fn owner_closed(mut input: Receiver) {
    let mut bytes = [0_u8; 64];
    while matches!(input.read(&mut bytes).await, Ok(count) if count > 0) {}
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn owner_bytes_keep_the_watchdog_open_until_eof() {
        let (mut owner, input) = tokio::net::unix::pipe::pipe().unwrap();
        owner.write_all(&[1; 128]).await.unwrap();
        let mut closed = Box::pin(owner_closed(input));
        assert!(
            tokio::time::timeout(Duration::from_millis(50), &mut closed)
                .await
                .is_err(),
            "owner bytes must be drained without stopping the service"
        );
        drop(owner);
        tokio::time::timeout(Duration::from_secs(5), closed)
            .await
            .expect("closing the owner pipe must complete the watchdog");
    }
}
