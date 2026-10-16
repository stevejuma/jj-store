//! Private device state, separate from all repository metadata.
use crate::{Result, process::OwnedProcess};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct Resources {
    pub server: PathBuf,
    pub jj: PathBuf,
    pub rustfs: PathBuf,
    pub web: PathBuf,
    pub acp: Option<PathBuf>,
    pub helpers: Vec<(String, PathBuf)>,
    pub native_runtime: Option<PathBuf>,
}
impl Resources {
    pub fn from_root(root: &Path) -> Result<Self> {
        let mut value = Self {
            server: root.join("jj-store"),
            jj: root.join("jj-store-jj"),
            rustfs: root.join("jj-store-rustfs"),
            web: root.join("web"),
            helpers: Vec::new(),
            native_runtime: root
                .join("native-runtime")
                .is_dir()
                .then(|| root.join("native-runtime")),
            acp: root.join("jj-acp").exists().then(|| root.join("jj-acp")),
        };
        for file in [
            &value.server,
            &value.jj,
            &value.rustfs,
            &value.web.join("asset-manifest.json"),
        ] {
            if !file.is_file()
                || fs::symlink_metadata(file)
                    .map_err(|e| e.to_string())?
                    .file_type()
                    .is_symlink()
            {
                return Err(format!(
                    "Missing regular bundled resource: {}",
                    file.display()
                ));
            }
        }
        if let Some(acp) = &value.acp
            && (!acp.is_file()
                || fs::symlink_metadata(acp)
                    .map_err(|e| e.to_string())?
                    .file_type()
                    .is_symlink())
        {
            return Err("Invalid bundled ACP executable".into());
        }
        if root.join("agent-resources.json").exists() {
            crate::agent_resources::verify(root)?;
            for (name, variable) in [
                ("jj-store-compute", "JJ_STORE_COMPUTE_EXECUTABLE"),
                ("jj-store-local-agent", "JJ_STORE_LOCAL_AGENT_EXECUTABLE"),
                ("jj-store-agent-tools", "JJ_STORE_AGENT_TOOLS_EXECUTABLE"),
            ] {
                value.helpers.push((variable.into(), root.join(name)));
            }
        }
        Ok(value)
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Device {
    pub version: u32,
    pub port: u16,
}
pub struct Installation {
    pub config: PathBuf,
    pub device: Device,
}

pub fn prepare(root: &Path, resources: &Resources) -> Result<Installation> {
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    check_private(root, true)?;
    let state = root.join("installation");
    let config = state.join("server.toml");
    let device_path = root.join("desktop.json");
    if !state.exists() {
        let mut command = Command::new(&resources.server);
        command
            .args([
                "--no-pager",
                "--color=never",
                "--format=json",
                "init",
                "--local",
                "--state-dir",
            ])
            .arg(&state)
            .arg("--rustfs-executable")
            .arg(&resources.rustfs);
        let mut child = OwnedProcess::spawn(&mut command)?;
        let line = child.first_line(Duration::from_secs(30))?;
        let _: serde_json::Value =
            serde_json::from_str(&line).map_err(|e| format!("Installation initialization: {e}"))?;
        child.finish(Duration::from_secs(30))?;
    }
    if !config.is_file() {
        return Err(
            "Existing desktop installation is incomplete; automatic reset is disabled".into(),
        );
    }
    check_private(&config, false)?;
    let device = if device_path.exists() {
        check_private(&device_path, false)?;
        if fs::metadata(&device_path).map_err(|e| e.to_string())?.len() > 4096 {
            return Err("Desktop settings exceed limit".into());
        }
        serde_json::from_slice::<Device>(&fs::read(&device_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?
    } else {
        let socket = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        let value = Device {
            version: 1,
            port: socket.local_addr().map_err(|e| e.to_string())?.port(),
        };
        use std::io::Write;
        let mut file = tempfile::NamedTempFile::new_in(root).map_err(|e| e.to_string())?;
        file.write_all(&serde_json::to_vec(&value).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        file.as_file().sync_all().map_err(|e| e.to_string())?;
        file.persist_noclobber(&device_path)
            .map_err(|e| e.to_string())?;
        fs::File::open(root)
            .and_then(|dir| dir.sync_all())
            .map_err(|e| e.to_string())?;
        value
    };
    if device.version != 1 || device.port == 0 {
        return Err("Unsupported desktop device settings".into());
    }
    Ok(Installation { config, device })
}

fn check_private(path: &Path, directory: bool) -> Result<()> {
    let details = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if details.file_type().is_symlink() || details.is_dir() != directory {
        return Err("Desktop state must use regular private paths".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if directory && details.permissions().mode() & 0o077 != 0 {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                .map_err(|e| e.to_string())?;
        }
        if !directory && details.permissions().mode() & 0o077 != 0 {
            return Err("Desktop state file is not private".into());
        }
    }
    Ok(())
}
