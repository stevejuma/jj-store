//! Explicit installation management; dispatch requires no repository or device state.
use super::*;
use clap::{Args, Subcommand};
use jj_store_model::RequestId;
use jj_store_service::capture_management::*;
mod client;
mod imports;
mod sources;

#[derive(Clone, Debug, Args)]
pub struct CaptureArgs {
    /// Explicit HTTP URL of a LocalTrusted server on a literal loopback address
    #[arg(long)]
    pub server_url: String,
    #[command(subcommand)]
    pub command: CaptureCommand,
}
#[derive(Clone, Debug, Args)]
pub struct PolicyArgs {
    #[arg(long, value_parser = generation)]
    pub generation: String,
    #[arg(long)]
    pub expected_revision: u64,
}
#[derive(Clone, Debug, Args)]
pub struct ControlArgs {
    #[command(flatten)]
    pub policy: PolicyArgs,
    /// Retain this exact identity and policy inputs when retrying
    #[arg(long)]
    pub request_id: RequestId,
}
#[derive(Clone, Debug, Subcommand)]
pub enum CaptureCommand {
    /// Read cached installation capture health and policy
    Status,
    /// List bounded cached source observations without inspecting transcripts
    Sources(sources::SourceArgs),
    /// Accept historical transcript ingestion on an existing retained source worker
    Import(imports::ImportArgs),
    /// Inspect the original receipt, accepted history and continuing tail
    ImportStatus(imports::StatusArgs),
    /// Select durable pause policy and request the existing owner's transition
    Pause(ControlArgs),
    /// Select durable resume policy; cannot enable configuration or start an owner
    Resume(ControlArgs),
    /// Hint discovery at the selected policy without importing historical sessions
    Rescan(PolicyArgs),
}
fn generation(value: &str) -> Result<String, String> {
    if value.len() != 32
        || value
            .bytes()
            .any(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
        || value == "0".repeat(32)
    {
        return Err("generation must be 32 lowercase nonzero hex digits".into());
    }
    Ok(value.into())
}
pub(super) async fn run(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    args: &CaptureArgs,
) -> Result<(), CommandError> {
    let client = client::Client::new(&args.server_url)?;
    if let CaptureCommand::Sources(query) = &args.command {
        return sources::run(ui, helper, format, &client, query).await;
    }
    if matches!(
        args.command,
        CaptureCommand::Import(_) | CaptureCommand::ImportStatus(_)
    ) {
        return imports::run(ui, helper, format, &client, &args.command).await;
    }
    let (value, status, detail) = match &args.command {
        CaptureCommand::Sources(_)
        | CaptureCommand::Import(_)
        | CaptureCommand::ImportStatus(_) => unreachable!("separately dispatched above"),
        CaptureCommand::Status => {
            let status: CaptureStatus = client.request("GET", "/v1/local/capture", None).await?;
            (serde_json::to_value(&status), status, String::new())
        }
        CaptureCommand::Pause(intent) | CaptureCommand::Resume(intent) => {
            if intent.request_id.as_bytes() == &[0; 16] {
                return Err(client::failure("request identity must be nonzero", false));
            }
            let request = CaptureControlCommand {
                version: 1,
                request_id: intent.request_id,
                generation: intent.policy.generation.clone(),
                expected_revision: intent.policy.expected_revision,
                action: if matches!(args.command, CaptureCommand::Pause(_)) {
                    CaptureControlAction::Pause
                } else {
                    CaptureControlAction::Resume
                },
            };
            let outcome: CaptureControlOutcome = client
                .request(
                    "POST",
                    "/v1/local/capture/control",
                    Some(serde_json::to_value(&request).map_err(error)?),
                )
                .await?;
            if outcome.version != 1 || outcome.status.version != 1 {
                return Err(client::failure(
                    "unsupported capture response version",
                    true,
                ));
            }
            if outcome.receipt.request_id != request.request_id
                || outcome.receipt.generation != request.generation
                || outcome.receipt.expected_revision != request.expected_revision
                || request.expected_revision.checked_add(1) != Some(outcome.receipt.revision)
                || outcome.receipt.paused != (request.action == CaptureControlAction::Pause)
                || !outcome.status.enabled
                || outcome.status.policy.as_ref().is_none_or(|policy| {
                    policy.generation != request.generation
                        || policy.revision < outcome.receipt.revision
                })
            {
                return Err(client::failure("capture command outcome unknown", true));
            }
            let detail = format!(
                "Control receipt revision {}{}; runtime transition {}.\n",
                outcome.receipt.revision,
                if outcome.replayed { " (replayed)" } else { "" },
                if outcome.runtime_requested {
                    "requested"
                } else {
                    "unavailable"
                }
            );
            (serde_json::to_value(&outcome), outcome.status, detail)
        }
        CaptureCommand::Rescan(policy) => {
            let request = CaptureRescanCommand {
                version: 1,
                generation: policy.generation.clone(),
                expected_revision: policy.expected_revision,
            };
            let outcome: CaptureRescanOutcome = client
                .request(
                    "POST",
                    "/v1/local/capture/rescan",
                    Some(serde_json::to_value(&request).map_err(error)?),
                )
                .await?;
            if outcome.version != 1 || outcome.status.version != 1 {
                return Err(client::failure(
                    "unsupported capture response version",
                    true,
                ));
            }
            if !outcome.status.enabled
                || outcome.status.policy.as_ref().is_none_or(|selected| {
                    selected.generation != request.generation
                        || selected.revision != request.expected_revision
                })
            {
                return Err(client::failure("capture command outcome unknown", true));
            }
            let detail = format!(
                "Discovery hint {}.\n",
                if outcome.accepted {
                    "accepted"
                } else {
                    "withheld"
                }
            );
            (serde_json::to_value(&outcome), outcome.status, detail)
        }
    };
    let value = value.map_err(error)?;
    if value["version"] != 1 || status.version != 1 {
        return Err(client::failure(
            "unsupported capture response version",
            !matches!(args.command, CaptureCommand::Status),
        ));
    }
    presentation::result(ui, helper, format, &value, &text(&status, &detail))
}
fn text(status: &CaptureStatus, detail: &str) -> String {
    let mut text = format!(
        "Capture {}.\n{detail}",
        if status.enabled {
            "enabled"
        } else {
            "disabled"
        }
    );
    if let Some(policy) = &status.policy {
        text += &format!(
            "Policy: {} (generation {:?}, revision {}).\n",
            if policy.paused { "paused" } else { "unpaused" },
            policy.generation,
            policy.revision
        );
    }
    if let Some(runtime) = &status.runtime {
        text += &format!(
            "Runtime: {:?}; {} sources; {} pending batches; {} degraded.\n",
            runtime.phase, runtime.tracked, runtime.pending_batches, runtime.degraded_sources
        );
        for group in &runtime.harnesses {
            text += &format!(
                "  {} {:?}: {}/{} roots available; {} sources; {} awaiting activity; {} caught up.\n",
                profile_name(group.profile),
                group.product_version,
                group.available_roots,
                group.roots,
                group.tracked,
                group.awaiting_activity,
                group.caught_up
            );
        }
    } else if status.enabled {
        text += "Runtime observation unavailable.\n";
    }
    text
}
fn profile_name(profile: CaptureHarnessProfile) -> &'static str {
    match profile {
        CaptureHarnessProfile::CodexCliJsonlV1 => "Codex CLI",
        CaptureHarnessProfile::CodexDesktopJsonlV1 => "Codex Desktop",
        CaptureHarnessProfile::CursorDesktopTranscriptV1 => "Cursor Desktop",
        CaptureHarnessProfile::OpencodeReadOnlyApiV1 => "OpenCode API",
        CaptureHarnessProfile::OpencodeExportJsonV1 => "OpenCode export",
    }
}
