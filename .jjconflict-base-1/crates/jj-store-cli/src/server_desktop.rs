//! Opt-in desktop owner lifetime and launch-only configuration overrides.
use clap::Args;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_server::config::{Access, Config, Storage};
use std::{net::SocketAddr, path::PathBuf};

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
pub fn owner_pipe() -> AuthorityResult<tokio::io::unix::AsyncFd<std::os::fd::OwnedFd>> {
    let input = rustix::io::dup(std::io::stdin())?;
    if rustix::fs::FileType::from_raw_mode(rustix::fs::fstat(&input)?.st_mode)
        != rustix::fs::FileType::Fifo
    {
        return Err("desktop supervision requires an owner stdin pipe".into());
    }
    let flags = rustix::fs::fcntl_getfl(&input)?;
    rustix::fs::fcntl_setfl(&input, flags | rustix::fs::OFlags::NONBLOCK)?;
    Ok(tokio::io::unix::AsyncFd::new(input)?)
}

#[cfg(unix)]
pub async fn owner_closed(input: tokio::io::unix::AsyncFd<std::os::fd::OwnedFd>) {
    let mut bytes = [0_u8; 64];
    loop {
        let Ok(mut ready) = input.readable().await else {
            return;
        };
        match ready
            .try_io(|fd| rustix::io::read(fd.get_ref(), &mut bytes).map_err(std::io::Error::from))
        {
            Ok(Ok(0)) | Ok(Err(_)) => return,
            _ => {}
        }
    }
}
