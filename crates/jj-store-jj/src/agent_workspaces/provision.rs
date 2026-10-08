//! Fixed commands to provision app-owned personal and registered native sources.
use super::*;
impl Provider {
    pub async fn initialize_personal(&self, path: &Path) -> Result<Selection> {
        if path.join(".jj").exists() {
            return self.inspect(path).await;
        }
        let parent = path.parent().ok_or("invalid_destination")?;
        self.provision(
            parent,
            &["git", "init", path.to_str().ok_or("invalid_destination")?],
        )
        .await?;
        self.inspect(path).await
    }
    pub async fn materialize(
        &self,
        repository: jj_store_model::RepositoryId,
        path: &Path,
        socket: &Path,
    ) -> Result<Selection> {
        if path.join(".jj").exists() {
            return self.inspect_materialization(repository, path, socket).await;
        }
        let parent = path.parent().ok_or("invalid_destination")?;
        let state = parent.join(format!("attachment-{repository}"));
        let cache = parent.join(format!("cache-{repository}"));
        self.provision(
            parent,
            &[
                "store",
                "attach",
                path.to_str().ok_or("invalid_destination")?,
                "--repo-id",
                &repository.to_string(),
                "--unix-socket",
                socket.to_str().ok_or("invalid_socket")?,
                "--state-dir",
                state.to_str().ok_or("invalid_destination")?,
                "--cache",
                cache.to_str().ok_or("invalid_destination")?,
            ],
        )
        .await?;
        self.inspect_materialization(repository, path, socket).await
    }
    async fn inspect_materialization(
        &self,
        repository: jj_store_model::RepositoryId,
        path: &Path,
        socket: &Path,
    ) -> Result<Selection> {
        let selected = self.inspect(path).await?;
        let connection = regular(&selected.metadata.join("store-connection.json"), 65536)?;
        let connection: serde_json::Value =
            serde_json::from_slice(&connection).map_err(|_| "invalid_connection")?;
        if connection["format"] != 2
            || connection["repository"] != repository.to_string()
            || connection["unix_socket"].as_str() != socket.to_str()
        {
            return Err("workspace_changed");
        }
        Ok(selected)
    }
    async fn provision(&self, parent: &Path, args: &[&str]) -> Result<()> {
        let mut command = tokio::process::Command::new(&self.executable);
        command
            .current_dir(parent)
            .args(["--no-pager", "--config", "fsmonitor.backend=none"])
            .args(args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let mut child = command.spawn().map_err(|_| "jj_executable_unavailable")?;
        let outcome = tokio::time::timeout(std::time::Duration::from_secs(90), child.wait()).await;
        match outcome {
            Ok(Ok(status)) if status.success() => Ok(()),
            _ => {
                #[cfg(unix)]
                if let Some(pid) = child
                    .id()
                    .and_then(|p| rustix::process::Pid::from_raw(p as i32))
                {
                    let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
                }
                let _ = child.start_kill();
                let _ = tokio::time::timeout(std::time::Duration::from_secs(2), child.wait()).await;
                Err("provisioning_outcome_unknown")
            }
        }
    }
}
