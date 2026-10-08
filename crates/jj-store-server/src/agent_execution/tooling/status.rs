use super::*;
impl Supervisor {
    pub(in crate::agent_execution) async fn tooling_status(&self) -> Result<dto::Status> {
        let paths = self.tooling_paths()?;
        let node = paths::resolve(&paths, "node");
        let node_version = match node {
            Some(ref p) => paths::probe(p, &paths).await,
            None => None,
        };
        let npm_available = paths::resolve(&paths, "npm").is_some();
        let prerequisites = node_version
            .as_deref()
            .and_then(paths::version)
            .is_some_and(|v| v.0 >= 22)
            && npm_available;
        let supported = matches!(std::env::consts::OS, "macos" | "linux");
        let live = !self
            .0
            .runtimes
            .lock()
            .map_err(|_| Error::Unavailable)?
            .is_empty();
        let (selection, pending) = self.read(|s| {
            Ok((
                s.runtime_selection.clone(),
                s.runtime_installs
                    .values()
                    .find(|i| i.outcome.state == "installing")
                    .map(|i| i.outcome.clone()),
            ))
        })?;
        let pending = pending
            .map(|p| self.install_outcome(p.request_id))
            .transpose()?;
        let adapter = paths::resolve(&paths, "codex-acp");
        let codex = paths::resolve(&paths, "codex");
        let mut adapter_version = adapter
            .as_deref()
            .and_then(|p| paths::package_version(p, "@agentclientprotocol/codex-acp"));
        if adapter_version.is_none()
            && let Some(ref p) = adapter
        {
            adapter_version = paths::probe(p, &paths).await;
        }
        let cli_version = match codex {
            Some(ref p) => paths::probe(p, &paths).await,
            None => None,
        };
        let managed = selection.as_ref().is_some_and(|s| {
            adapter
                .as_ref()
                .is_some_and(|p| p.starts_with(paths::prefix(&self.0.resources.root, s.request_id)))
        });
        let old = adapter_version
            .as_deref()
            .and_then(paths::version)
            .is_some_and(|v| v < paths::version(ADAPTER_VERSION).unwrap())
            || (managed
                && cli_version
                    .as_deref()
                    .and_then(paths::version)
                    .is_some_and(|v| v < paths::version(CODEX_VERSION).unwrap()));
        let unknown = adapter.is_some() && adapter_version.is_none();
        let state = if pending
            .as_ref()
            .is_some_and(|p| p.state == "outcome_unknown")
        {
            "unknown"
        } else if pending.is_some() {
            "installing"
        } else if codex.is_none() {
            "cli_missing"
        } else if adapter.is_none() {
            "adapter_needed"
        } else if !prerequisites {
            "node_needed"
        } else if old {
            "update_available"
        } else if unknown {
            "unknown"
        } else {
            "ready"
        };
        let action = if adapter.is_none() || codex.is_none() {
            Some(dto::Action::Install)
        } else if old || unknown {
            Some(dto::Action::Update)
        } else {
            None
        };
        let cursor = paths::resolve(&paths, "cursor-agent");
        let cursor_version = match cursor {
            Some(ref p) => paths::probe(p, &paths).await,
            None => None,
        };
        let local_ready = self
            .runners()?
            .iter()
            .any(|r| r.id == "local" && r.available);
        Ok(dto::Status {
            version:1, node_version, npm_available, install_supported:supported && prerequisites && !live && pending.is_none(),
            install_unavailable: if !supported { Some("Managed installation is available on macOS and Linux.".into()) }
                else if !prerequisites { Some("Install Node.js 22 or newer with npm, then Check again.".into()) }
                else if live { Some("End the connected agent session before installing or updating tooling.".into()) }
                else { None }, pending,
            runtimes:vec![dto::Runtime {
                id:"codex".into(),label:"Codex".into(),state:state.into(),adapter_version,cli_version,managed,action,
                detail:format!("Codex CLI and ACP adapter. Private install uses adapter {ADAPTER_VERSION} and CLI {CODEX_VERSION}. Authenticate with Codex before connecting."),
                guide_url:"https://developers.openai.com/codex/cli".into(),target_version:Some(ADAPTER_VERSION.into()),target_cli_version:Some(CODEX_VERSION.into()),
            },dto::Runtime {
                id:"cursor".into(),label:"Cursor".into(),state:if cursor.is_some() {"ready"} else {"cli_missing"}.into(),
                adapter_version:None,cli_version:cursor_version,managed:false,action:None,
                detail:"Cursor supplies its own ACP runtime. Install or update using Cursor's guide, then run cursor-agent login.".into(),
                guide_url:"https://cursor.com/docs/cli/installation".into(),target_version:None,target_cli_version:None,
            },dto::Runtime {
                id:"local".into(),label:"jj-agent".into(),state:if local_ready {"ready"} else {"bundle_needed"}.into(),
                adapter_version:None,cli_version:None,managed:true,action:None,
                detail:if local_ready {"Bundled ACP agent and coding tools, adapted from buzz-agent. Choose a local model, paired compute or an OpenAI compatible provider."} else {"The bundled ACP agent or coding-tool helper is missing. Use a complete jj-store distribution or configure the trusted companion executable paths."}.into(),
                guide_url:"/help/guides/agent-execution/".into(),target_version:None,target_cli_version:None,
            }],
        })
    }
}
