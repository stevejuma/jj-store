use crate::{Error, Result};
use jj_store_object::s3::{self, S3Store};
use std::{net::SocketAddr, path::PathBuf, process::Stdio, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader},
    process::{Child, ChildStdin, Command},
};

#[derive(Clone)]
pub struct EmbeddedStorageConfig {
    pub runtime_executable: PathBuf,
    pub volume: PathBuf,
    pub address: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub bucket: String,
    pub max_object_bytes: u64,
    pub timeout: Duration,
}

impl std::fmt::Debug for EmbeddedStorageConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EmbeddedStorageConfig")
            .field("runtime_executable", &self.runtime_executable)
            .field("volume", &self.volume)
            .field("address", &self.address)
            .field("credentials", &"[REDACTED]")
            .field("region", &self.region)
            .field("bucket", &self.bucket)
            .field("max_object_bytes", &self.max_object_bytes)
            .field("timeout", &self.timeout)
            .finish()
    }
}

pub struct EmbeddedStorageRuntime {
    child: Child,
    shutdown: Option<ChildStdin>,
    logs: tokio::task::JoinHandle<()>,
    store: Arc<S3Store>,
}

impl EmbeddedStorageRuntime {
    pub async fn start(config: EmbeddedStorageConfig) -> Result<Self> {
        if !config.runtime_executable.is_absolute()
            || !config.volume.is_absolute()
            || !config.volume.is_dir()
        {
            return Err(Error::Config(
                "embedded storage executable and existing volume paths must be absolute",
            ));
        }
        let address: SocketAddr = config
            .address
            .parse()
            .map_err(|_| Error::Config("embedded storage address"))?;
        if !address.ip().is_loopback() || address.port() == 0 {
            return Err(Error::Config(
                "embedded storage requires a concrete loopback address",
            ));
        }
        let mut child = Command::new(&config.runtime_executable)
            .arg(&config.address)
            .arg(&config.volume)
            .arg(&config.region)
            .env("JJ_STORE_INTERNAL_RUSTFS_ACCESS_KEY", &config.access_key)
            .env("JJ_STORE_INTERNAL_RUSTFS_SECRET_KEY", &config.secret_key)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;
        let shutdown = child
            .stdin
            .take()
            .ok_or(Error::Service("embedded storage shutdown channel"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or(Error::Service("embedded storage readiness channel"))?;
        let mut output = BufReader::new(stdout);
        let ready = tokio::time::timeout(config.timeout, async {
            let mut observed = 0_usize;
            loop {
                let mut line = String::new();
                let read = output.read_line(&mut line).await?;
                if read == 0 {
                    return Ok::<_, std::io::Error>(false);
                }
                observed = observed.saturating_add(read);
                if observed > 64 * 1024 {
                    return Err(std::io::Error::other(
                        "embedded storage startup output exceeded limit",
                    ));
                }
                if line.trim() == "READY" {
                    return Ok(true);
                }
            }
        })
        .await
        .map_err(|_| Error::Service("embedded storage readiness timed out"))?
        .map_err(Error::Io)?;
        if !ready {
            let _ = child.wait().await;
            return Err(Error::Service("embedded storage startup failed"));
        }
        let logs = tokio::spawn(async move {
            let _ = tokio::io::copy(&mut output, &mut tokio::io::sink()).await;
        });
        let store = Arc::new(
            S3Store::new(s3::Config {
                endpoint: format!("http://{address}"),
                bucket: config.bucket,
                region: config.region,
                credentials: s3::Credentials {
                    access_key: config.access_key,
                    secret_key: config.secret_key,
                    session_token: None,
                },
                allow_http_loopback: true,
                max_object_bytes: config.max_object_bytes,
                timeout: config.timeout,
            })
            .map_err(|_| Error::Config("embedded storage client"))?,
        );
        Ok(Self {
            child,
            shutdown: Some(shutdown),
            logs,
            store,
        })
    }

    pub fn store(&self) -> Arc<S3Store> {
        self.store.clone()
    }

    pub fn process_id(&self) -> Option<u32> {
        self.child.id()
    }

    pub async fn shutdown(mut self) {
        self.stop().await;
    }

    async fn stop(&mut self) {
        if let Some(mut shutdown) = self.shutdown.take() {
            let _ = shutdown.write_all(b"shutdown\n").await;
            let _ = shutdown.shutdown().await;
        }
        if tokio::time::timeout(Duration::from_secs(10), self.child.wait())
            .await
            .is_err()
        {
            let _ = self.child.kill().await;
            let _ = self.child.wait().await;
        }
        self.logs.abort();
    }
}

impl Drop for EmbeddedStorageRuntime {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}
