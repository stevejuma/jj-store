//! Standalone tool-free completion, with the same provider and credential custody.
use super::*;
use std::process::Stdio;
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
        let executable = self.companion("lapis-agent", "JJ_STORE_LOCAL_AGENT_EXECUTABLE")?;
        let mut command = tokio::process::Command::new(executable);
        self.launch_environment_preview(&mut command, &setup, &BTreeMap::new())?;
        self.local_environment(&mut command, &setup, false).await?;
        let mut chatgpt_lease = self
            .chatgpt_launch(&mut command, &setup, &format!("metadata/{}", setup.id), 0)
            .await?;
        self.register_launch_secrets(&command, &setup)?;
        let path = self.0.resources.root.join("metadata-runtime");
        journal::private_directory(&path)?;
        let sanitizer = runtime::sanitizer(self, &path).await?;
        command
            .env("BUZZ_AGENT_LLM_TIMEOUT_SECS", "120")
            .current_dir(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let plan = lapis_ai_runtime::PreparedLaunch::from_command(command.as_std())
            .map_err(|_| Error::Invalid)?;
        let value = tokio::select! {
            _=chatgpt::broker::cancelled(&mut chatgpt_lease)=>Err(Error::Denied),
            result=lapis_ai_runtime::AgentClient::metadata_prepared(plan,
                json!({"version":1,"transcript":transcript}), native_runtime::limits(16 * 1024), Duration::from_secs(125)) => result.map_err(|_|Error::Unavailable),
        }?;
        serde_json::from_value(runtime::safe(
            &sanitizer
                .for_oauth_secrets(&self.oauth_redactions()?)
                .map_err(|_| Error::Unavailable)?,
            value,
        )?)
        .map_err(|_| Error::Invalid)
    }
}
