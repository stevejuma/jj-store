//! Fixed application catalog and private tooling selection. No ACP policy changes.
mod install;
mod paths;
mod status;
use super::*;
use jj_store_service::agent_execution::tooling as dto;
use serde::{Deserialize, Serialize};
pub(super) const ADAPTER_VERSION: &str = "1.13.1";
pub(super) const CODEX_VERSION: &str = "0.156.1";
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Selection {
    pub request_id: RequestId,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct InstallIntent {
    pub digest: String,
    pub outcome: dto::Outcome,
}
pub(super) fn recover(state: &mut State) {
    for intent in state.runtime_installs.values_mut() {
        if intent.outcome.state == "installing" {
            // No unselected staging directory is executable authority. Do not replay.
            intent.outcome.state = "interrupted".into();
            intent.outcome.error = Some("installation_interrupted_check_again".into());
        }
    }
}
fn complete(
    state: &mut State,
    request: RequestId,
    result: std::result::Result<(), &'static str>,
) -> Result<()> {
    let intent = state
        .runtime_installs
        .get_mut(&request)
        .ok_or(Error::Invalid)?;
    match result {
        Ok(()) => {
            state.runtime_selection = Some(Selection {
                request_id: request,
            });
            intent.outcome.state = "completed".into();
        }
        Err(reason) => {
            intent.outcome.state = "failed".into();
            intent.outcome.error = Some(reason.into());
        }
    }
    Ok(())
}
impl Supervisor {
    pub(super) fn tooling_paths(&self) -> Result<Vec<PathBuf>> {
        self.read(|s| {
            Ok(paths::directories(
                &self.0.resources.root,
                s.runtime_selection.as_ref(),
            ))
        })
    }
    pub(super) fn tooling_busy(&self) -> Result<bool> {
        self.read(|s| {
            Ok(s.runtime_installs
                .values()
                .any(|i| i.outcome.state == "installing"))
        })
    }
    pub(super) fn runners(&self) -> Result<Vec<Runner>> {
        let paths = self.tooling_paths()?;
        Ok([
            ("codex", "Codex", "codex-acp"),
            ("cursor", "Cursor", "cursor-agent"),
            ("local", "JJ Agent", "lapis-agent"),
        ]
        .into_iter()
        .map(|(id, label, bin)| {
            let available = if id == "local" {
                self.companion("lapis-agent", "JJ_STORE_LOCAL_AGENT_EXECUTABLE")
                    .is_ok()
                    && self
                        .companion("lapis-agent-tools", "JJ_STORE_AGENT_TOOLS_EXECUTABLE")
                        .is_ok()
            } else {
                paths::resolve(&paths, bin).is_some()
            };
            Runner {
                providers: if id == "local" {
                    providers::catalog()
                } else {
                    vec![]
                },
                id: id.into(),
                label: label.into(),
                available,
                qualified: false,
                install_hint: (!available).then(|| "Open Settings → Agents → Runtimes".into()),
            }
        })
        .collect())
    }
    pub(super) fn install_outcome(&self, request: RequestId) -> Result<dto::Outcome> {
        let mut outcome = self.read(|s| {
            s.runtime_installs
                .get(&request)
                .map(|i| i.outcome.clone())
                .ok_or(Error::Invalid)
        })?;
        if outcome.state == "installing"
            && self
                .0
                .installer_task
                .lock()
                .map_err(|_| Error::Unavailable)?
                .as_ref()
                .is_some_and(|task| task.is_finished())
        {
            outcome.state = "outcome_unknown".into();
            outcome.error = Some("runtime_receipt_unavailable_restart".into());
        }
        Ok(outcome)
    }
}
