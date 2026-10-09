//! Standalone tool-free completion, with the same provider and credential custody.
use super::*;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Output {
    pub title: String,
    pub summary: String,
    pub keywords: Vec<String>,
    pub goals: Vec<String>,
}
impl Supervisor {
    pub(super) async fn metadata_infer(
        &self,
        config: &metadata::Config,
        transcript: &str,
    ) -> Result<Output> {
        let setup = Setup {
            id: "conversation-metadata".into(),
            name: "Conversation metadata".into(),
            avatar: "".into(),
            description: "".into(),
            instructions: "".into(),
            runner: "local".into(),
            preferences: Preferences {
                model: Some(config.model.clone()),
                effort: None,
                mode: None,
            },
            options: config.options.clone(),
        };
        let executable =
            self.companion("jj-store-local-agent", "JJ_STORE_LOCAL_AGENT_EXECUTABLE")?;
        let mut command = tokio::process::Command::new(executable);
        self.launch_environment_preview(&mut command, &setup, &BTreeMap::new())?;
        self.local_environment(&mut command, &setup, false).await?;
        self.register_launch_secrets(&command, &setup)?;
        let path = self.0.resources.root.join("metadata-runtime");
        journal::private_directory(&path)?;
        let sanitizer = runtime::sanitizer(self, &path).await?;
        command
            .arg("--metadata")
            .env("BUZZ_AGENT_LLM_TIMEOUT_SECS", "120")
            .current_dir(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let mut child = runtime::OwnedChild(command.spawn().map_err(|_| Error::Unavailable)?);
        let input = serde_json::to_vec(&json!({"version":1,"transcript":transcript}))
            .map_err(|_| Error::Invalid)?;
        let result = tokio::time::timeout(Duration::from_secs(125), async {
            let mut stdin = child.0.stdin.take().ok_or(Error::Unavailable)?;
            stdin
                .write_all(&input)
                .await
                .map_err(|_| Error::Unavailable)?;
            stdin.shutdown().await.map_err(|_| Error::Unavailable)?;
            drop(stdin);
            let mut bytes = Vec::new();
            child
                .0
                .stdout
                .take()
                .ok_or(Error::Unavailable)?
                .take(16 * 1024 + 1)
                .read_to_end(&mut bytes)
                .await
                .map_err(|_| Error::Unavailable)?;
            if bytes.len() > 16 * 1024 {
                return Err(Error::Limit);
            }
            let status = child.0.wait().await.map_err(|_| Error::Unavailable)?;
            if !status.success() {
                return Err(Error::Unavailable);
            }
            let value = serde_json::from_slice(&bytes).map_err(|_| Error::Invalid)?;
            serde_json::from_value(runtime::safe(&sanitizer, value)?).map_err(|_| Error::Invalid)
        })
        .await
        .map_err(|_| Error::Unavailable)
        .and_then(|v| v);
        child.stop().await;
        result
    }
}
