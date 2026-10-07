use crate::{
    Result,
    installation::{Resources, prepare},
    process::OwnedProcess,
    readiness,
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    path::Path,
    process::Command,
    time::Duration,
};

pub struct ManagedService {
    process: OwnedProcess,
    pub origin: url::Url,
}
impl Drop for ManagedService {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
impl ManagedService {
    pub fn start(state: &Path, resources: &Resources) -> Result<Self> {
        let installation = prepare(state, resources)?;
        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, installation.device.port));
        let mut command = Command::new(&resources.server);
        command
            .current_dir(state)
            .args([
                "--no-pager",
                "--color=never",
                "--format=json",
                "run",
                "--config",
            ])
            .arg(&installation.config)
            .args([
                "--loopback-only",
                "--desktop-supervised",
                "--desktop-bind",
                &address.to_string(),
                "--desktop-web-root",
            ])
            .arg(&resources.web)
            .arg("--desktop-rustfs-executable")
            .arg(&resources.rustfs);
        command.env("JJ_STORE_AGENT_JJ_EXECUTABLE", &resources.jj);
        for (variable, path) in &resources.helpers {
            command.env(variable, path);
        }
        if let Some(native) = &resources.native_runtime {
            command.env("JJ_STORE_COMPUTE_NATIVE_RUNTIME_DIR", native);
        }
        if let Some(acp) = &resources.acp {
            command.env("JJ_STORE_ACP_EXECUTABLE", acp);
        }
        let mut process = OwnedProcess::spawn(&mut command)?;
        let line = process.first_line(Duration::from_secs(30))?;
        readiness::listening(&line, address)?;
        readiness::bootstrap(address)?;
        if !process.running()? {
            return Err(format!(
                "Service stopped during startup: {}",
                process.diagnostics()
            ));
        }
        Ok(Self {
            process,
            origin: url::Url::parse(&format!("http://{address}/")).map_err(|e| e.to_string())?,
        })
    }
    pub fn running(&mut self) -> Result<bool> {
        self.process.running()
    }
    pub fn stop(&mut self) -> Result<()> {
        self.process.finish(Duration::from_secs(40))
    }
}
