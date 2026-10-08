//! Bounded no-prompt ACP discovery in a private directory, separate from chats.
use super::*;
use jj_store_service::agent_execution::catalog::Catalog;
use std::{process::Stdio, sync::atomic::Ordering};
use tokio::{io::BufReader, process::Command};

pub(super) type Cache = BTreeMap<String, (tokio::time::Instant, Catalog)>;
struct Reservation<'a>(&'a std::sync::atomic::AtomicBool);
impl Drop for Reservation<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}
impl Supervisor {
    pub(super) async fn model_catalog(
        &self,
        runner: String,
        setup: Option<Setup>,
    ) -> Result<Catalog> {
        self.model_catalog_preview(runner, setup, &BTreeMap::new())
            .await
    }
    pub(super) async fn model_catalog_preview(
        &self,
        runner: String,
        setup: Option<Setup>,
        secrets: &BTreeMap<String, String>,
    ) -> Result<Catalog> {
        if !matches!(runner.as_str(), "codex" | "cursor" | "local") {
            return Err(Error::Invalid);
        }
        let setup = if let Some(setup) = setup {
            setup
        } else {
            self.read(|s| {
                Ok(configuration::resolve(
                    &Setup {
                        id: "catalog".into(),
                        name: "Catalog".into(),
                        avatar: String::new(),
                        description: String::new(),
                        instructions: String::new(),
                        runner: runner.clone(),
                        preferences: Preferences {
                            model: None,
                            effort: None,
                            mode: None,
                        },
                        options: settings::RuntimeOptions {
                            inherit_defaults: true,
                            ..Default::default()
                        },
                    },
                    &s.settings,
                ))
            })?
        };
        if runner == "local" {
            return self.provider_catalog(&setup, secrets).await;
        }
        let key = journal::digest(&(
            self.read(|s| configuration::launch_digest(&setup, s))?,
            secrets,
        ))?;
        let mut cache = self.0.catalogs.lock().await;
        if self.0.stopping.load(Ordering::Acquire) {
            return Err(Error::Unavailable);
        }
        if let Some((at, value)) = cache.get(&key)
            && at.elapsed() < Duration::from_secs(300)
        {
            return Ok(value.clone());
        }
        let guard = self.0.mutation.lock().await;
        if self.tooling_busy()? {
            return Err(Error::Unavailable);
        }
        // Admission shares the fleet lock with worker spawn; discovery occupies one slot.
        {
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            let legacy = self.0.runtimes.lock().map_err(|_| Error::Unavailable)?;
            if workers.values().filter(|w| !w.sender.is_closed()).count() + legacy.len() >= 4 {
                return Err(Error::Busy);
            }
            self.0.catalog_active.store(true, Ordering::Release);
        }
        let _reservation = Reservation(&self.0.catalog_active);
        let path = self.0.resources.root.join("catalog");
        journal::private_directory(&path)?;
        let paths = std::env::join_paths(self.tooling_paths()?).map_err(|_| Error::Unavailable)?;
        let mut command = Command::new(&self.0.resources.executable);
        command.arg("serve");
        self.launch_environment_preview(&mut command, &setup, secrets)?;
        self.local_launch(&mut command, &setup, false).await?;
        self.register_launch_secrets(&command, &setup)?;
        let sanitizer = runtime::sanitizer(self, &path).await?;

        command
            .env("PATH", paths)
            .args([
                "--control-version",
                "2",
                "--runner",
                &runner,
                "--max-turn-duration",
                "30",
                "--idle-timeout",
                "20",
                "--cancel-grace",
                "2",
                "--reap-timeout",
                "2",
                "--cwd",
            ])
            .arg(&path)
            .current_dir(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let mut child = runtime::OwnedChild(command.spawn().map_err(|_| Error::Unavailable)?);
        drop(guard);
        // Installation checks the reserved slot; Stop/shutdown uses independent signals.
        let result = tokio::time::timeout(Duration::from_secs(28), async {
            let mut input = child.0.stdin.take().ok_or(Error::Unavailable)?;
            let mut output = BufReader::new(child.0.stdout.take().ok_or(Error::Unavailable)?);
            let mut bytes = Vec::new();
            let mut facts = None;
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        if self.0.stopping.load(Ordering::Acquire) { return Err(Error::Unavailable); }
                    },
                    line = runtime::bounded_line(&mut output, &mut bytes) => {
                        if !line? { return Err(Error::Unavailable); }
                        let value: Value = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
                        bytes.clear();
                        if value["version"] != 2 { return Err(Error::Unavailable); }
                        match value["type"].as_str() {
                            Some("ready") => runtime::write(&mut input, json!({"version":2,"type":"open","requestId":"catalog-open"})).await?,
                            Some("opened") if value["requestId"] == "catalog-open" && facts.is_none() => {
                                // Keep only advertised configuration, never transcripts or adapter session IDs.
                                facts = Some(runtime::safe(&sanitizer, json!({
                                    "configOptions":value["session"]["configOptions"],
                                    "models":value["session"]["models"],
                                    "modes":value["session"]["modes"],
                                }))?);
                                runtime::write(&mut input, json!({"version":2,"type":"close","requestId":"catalog-close"})).await?;
                            },
                            Some("closed") if value["requestId"] == "close" && value["confirmedStopped"] == true => return facts.ok_or(Error::Unavailable),
                            Some("permission" | "error" | "started" | "activity") => return Err(Error::Unavailable),
                            _ => {},
                        }
                    }
                }
            }
        }).await.map_err(|_| Error::Unavailable).and_then(|v| v);
        child.stop().await;
        let value = Catalog {
            status: None,
            runner: runner.clone(),
            facts: result?,
        };
        cache.insert(key, (tokio::time::Instant::now(), value.clone()));
        Ok(value)
    }
}
