//! Ephemeral local-agent catalog subprocess, isolated from fleet and conversation state.
use super::*;
use jj_store_service::agent_execution::catalog::Catalog;
use std::{process::Stdio, sync::atomic::Ordering};
use tokio::process::Command;
struct Reservation<'a>(&'a std::sync::atomic::AtomicBool);
impl Drop for Reservation<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}
impl Supervisor {
    pub(super) async fn provider_catalog(
        &self,
        setup: &Setup,
        secrets: &BTreeMap<String, String>,
    ) -> Result<Catalog> {
        if setup.options.provider.as_deref() == Some("chatgpt-plan") {
            let Some(account) = setup.options.account.as_deref() else {
                return Ok(unavailable("authentication_required"));
            };
            return match self.chatgpt_models(account).await {
                Ok(value) => Ok(value),
                Err(Error::Denied | Error::Invalid) => Ok(unavailable("authentication_required")),
                Err(_) => Ok(unavailable("unavailable")),
            };
        }
        let mut cache = self.0.catalogs.lock().await;
        let guard = self.0.mutation.lock().await;
        if self.0.stopping.load(Ordering::Acquire) || self.tooling_busy()? {
            return Err(Error::Unavailable);
        }
        {
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            let legacy = self.0.runtimes.lock().map_err(|_| Error::Unavailable)?;
            if workers.values().filter(|w| !w.sender.is_closed()).count() + legacy.len() >= 4 {
                return Err(Error::Busy);
            }
            self.0.catalog_active.store(true, Ordering::Release);
        }
        let _reservation = Reservation(&self.0.catalog_active);
        let executable = self.companion("lapis-agent", "JJ_STORE_LOCAL_AGENT_EXECUTABLE")?;
        let mut command = Command::new(executable);
        if let Err(error) = self.launch_environment_preview(&mut command, setup, secrets) {
            return if matches!(error, Error::Unavailable) {
                Ok(unavailable("authentication_required"))
            } else {
                Err(error)
            };
        }
        if self
            .local_environment(&mut command, setup, false)
            .await
            .is_err()
        {
            return Ok(unavailable("unavailable"));
        }
        // Only discovery may use a placeholder to parse provider configuration before model selection.
        if setup.preferences.model.is_none() {
            command.env("BUZZ_AGENT_MODEL", "auto");
        }
        self.register_launch_secrets(&command, setup)?;
        let resolved: Vec<_> = command
            .as_std()
            .get_envs()
            .map(|(k, v)| (k.to_string_lossy(), v.map(|v| v.to_string_lossy())))
            .collect();
        let key = journal::digest(&(
            self.read(|s| configuration::launch_digest(setup, s))?,
            resolved,
        ))?;
        if let Some((at, value)) = cache.get(&key)
            && at.elapsed() < Duration::from_secs(300)
        {
            return Ok(value.clone());
        }
        let path = self.0.resources.root.join("catalog");
        journal::private_directory(&path)?;
        let sanitizer = runtime::sanitizer(self, &path).await?;
        command
            .current_dir(&path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let plan = lapis_ai_runtime::PreparedLaunch::from_command(command.as_std())
            .map_err(|_| Error::Invalid)?;
        drop(guard);
        let result = tokio::select! {
            result = lapis_ai_runtime::AgentClient::models_prepared(plan, native_runtime::limits(2 * 1024 * 1024), Duration::from_secs(25)) => result.map_err(|_|Error::Unavailable),
            _ = async { while !self.0.stopping.load(Ordering::Acquire) { tokio::time::sleep(Duration::from_millis(100)).await; } } => Err(Error::Unavailable),
        };
        let value = result?;
        let status = value["status"]
            .as_str()
            .filter(|s| {
                matches!(
                    *s,
                    "ready" | "empty" | "authentication_required" | "unavailable"
                )
            })
            .ok_or(Error::Unavailable)?;
        let value = Catalog {
            runner: "local".into(),
            status: Some(status.into()),
            facts: runtime::safe(&sanitizer, value["facts"].clone())?,
        };
        if value.status.as_deref() == Some("ready") {
            cache.insert(key, (tokio::time::Instant::now(), value.clone()));
        }
        // Bound distinct configuration previews; no credentials or facts go to disk.
        while cache.len() > 32 {
            if let Some(key) = cache.keys().next().cloned() {
                cache.remove(&key);
            }
        }
        Ok(value)
    }
}
fn unavailable(status: &str) -> Catalog {
    Catalog {
        runner: "local".into(),
        status: Some(status.into()),
        facts: json!({}),
    }
}
