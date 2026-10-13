//! Native workflow management and assigned-attempt reporting.
use crate::{
    binding::Connection,
    commands::{Format, error},
    presentation,
};
use clap::{Args, Subcommand};
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
    ui::Ui,
};
use jj_store_model::{CommitId, OperationId, RepositoryId, RequestId};
use jj_store_service::workflow as w;
use std::{
    io::{Read as _, Write as _},
    path::PathBuf,
};

#[derive(Clone, Debug, Args)]
pub struct WorkflowArgs {
    #[arg(long,global=true,value_enum,default_value_t=Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<WorkflowCommand>,
}
#[derive(Clone, Debug, Subcommand)]
pub enum WorkflowCommand {
    /// Export the pinned native source tree without publishing a source operation.
    Source(crate::workflow_source::Args),
    /// Inspect semantic committed source events, retaining the returned epoch/cursor.
    Events {
        #[arg(long)]
        epoch: Option<jj_store_model::EpochId>,
        #[arg(long)]
        after: Option<u64>,
        /// JSON array of WorkflowConditions to evaluate at each source publication.
        #[arg(long)]
        filters: Option<PathBuf>,
    },
    List {
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        after: Option<String>,
    },
    Show {
        id: w::WorkflowId,
    },
    Runs {
        #[arg(long)]
        workflow: Option<w::WorkflowId>,
        #[arg(long)]
        run: Option<w::RunId>,
        #[arg(long)]
        after: Option<String>,
        #[arg(long)]
        at: Option<u64>,
    },
    Logs {
        run: w::RunId,
        #[arg(long)]
        attempt: Option<w::ExecutionAttemptId>,
        #[arg(long, default_value_t = 0)]
        after: u64,
        #[arg(long)]
        follow: bool,
    },
    Put {
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        id: Option<w::WorkflowId>,
        #[arg(long)]
        expected: Option<w::VersionId>,
        file: PathBuf,
    },
    Disable {
        id: w::WorkflowId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected: w::VersionId,
    },
    Enable {
        id: w::WorkflowId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected: w::VersionId,
    },
    Remove {
        id: w::WorkflowId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected: w::VersionId,
    },
    Run {
        id: w::WorkflowId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected: w::VersionId,
        #[arg(long = "source-repo", id = "workflow_source_repository")]
        repository: RepositoryId,
        #[arg(long)]
        operation: OperationId,
        #[arg(long)]
        commit: CommitId,
        #[arg(long)]
        bookmark: Option<String>,
    },
    Cancel {
        run: w::RunId,
        #[arg(long)]
        request_id: RequestId,
    },
    Retry {
        run: w::RunId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected: w::ExecutionAttemptId,
    },
    /// Publish one exact typed step/log/artifact report document.
    Report {
        #[arg(long)]
        request_id: RequestId,
        run: w::RunId,
        #[arg(long)]
        attempt: w::ExecutionAttemptId,
        #[arg(long)]
        sequence: u64,
        file: PathBuf,
    },
    /// Publish one typed command document (execution host admission still applies).
    Command {
        #[arg(long)]
        request_id: RequestId,
        file: PathBuf,
    },
    Status {
        request_id: RequestId,
    },
}
fn read(path: &PathBuf) -> Result<Vec<u8>, CommandError> {
    let mut reader: Box<dyn std::io::Read> = if path.as_os_str() == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(error)?)
    };
    let mut bytes = Vec::new();
    reader
        .by_ref()
        .take(2 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(error)?;
    if bytes.len() > 2 * 1024 * 1024 {
        return Err(user_error("Workflow input exceeds two MiB"));
    }
    Ok(bytes)
}
fn output(ui: &mut Ui, format: Format, value: &impl serde::Serialize) -> Result<(), CommandError> {
    let text = if matches!(format, Format::Json) {
        serde_json::to_string(value)
    } else {
        serde_json::to_string_pretty(value)
    }
    .map_err(error)?;
    writeln!(ui.stdout(), "{text}").map_err(error)
}
pub async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    args: WorkflowArgs,
) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["workflow".into()]);
    };
    if let WorkflowCommand::Source(args) = command {
        return crate::workflow_source::run(helper, args).await;
    }
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let client = connection.connect().await.map_err(error)?;
    let (id, command) = match command {
        WorkflowCommand::Source(_) => unreachable!(),
        WorkflowCommand::Events {
            epoch,
            after,
            filters,
        } => {
            return output(
                ui,
                args.format,
                &client
                    .workflow_source_events(&w::EventQuery {
                        epoch,
                        after: after.map(jj_store_model::WalPosition),
                        limit: Some(16),
                        filters: filters
                            .map(|path| std::fs::read_to_string(&path).map_err(error))
                            .transpose()?,
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?,
            );
        }
        WorkflowCommand::List { limit, after } => {
            return output(
                ui,
                args.format,
                &client
                    .workflow_page(&w::Query {
                        limit: Some(limit),
                        after,
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?,
            );
        }
        WorkflowCommand::Show { id } => {
            return output(
                ui,
                args.format,
                &client
                    .workflow_page(&w::Query {
                        workflow: Some(id),
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?,
            );
        }
        WorkflowCommand::Runs {
            workflow,
            run,
            after,
            at,
        } => {
            return output(
                ui,
                args.format,
                &client
                    .workflow_page(&w::Query {
                        workflow,
                        run,
                        after,
                        at: at.map(jj_store_model::WalPosition),
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?,
            );
        }
        WorkflowCommand::Logs {
            run,
            attempt,
            mut after,
            follow,
        } => {
            let mut selected = attempt;
            loop {
                if attempt.is_none() {
                    let overview = client
                        .workflow_page(&w::Query {
                            run: Some(run),
                            limit: Some(1),
                            ..Default::default()
                        })
                        .await
                        .map_err(error)?;
                    let current = overview
                        .runs
                        .first()
                        .ok_or_else(|| user_error("Workflow run not found"))?
                        .current()
                        .id;
                    if selected.is_some_and(|old| old != current) {
                        after = 0;
                    }
                    selected = Some(current);
                }
                let page = client
                    .workflow_page(&w::Query {
                        run: Some(run),
                        attempt: selected,
                        evidence_after: Some(after),
                        limit: Some(256),
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?;
                for evidence in &page.evidence {
                    if let w::EvidenceData::Log { text, .. } = &evidence.data {
                        write!(ui.stdout(), "{text}").map_err(error)?;
                    }
                    after = after.max(evidence.sequence + 1);
                }
                if page.next.is_none()
                    && (!follow
                        || page
                            .runs
                            .first()
                            .is_none_or(|r| r.current().status.terminal()))
                {
                    return Ok(());
                }
                if page.next.is_none() {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
        WorkflowCommand::Status { request_id } => {
            return output(
                ui,
                args.format,
                &client
                    .workflow_command_status(request_id)
                    .await
                    .map_err(error)?,
            );
        }
        WorkflowCommand::Put {
            request_id,
            id,
            expected,
            file,
        } => {
            let bytes = read(&file)?;
            let yaml = std::str::from_utf8(&bytes).map_err(error)?;
            (
                request_id,
                w::Command::Put {
                    workflow: id.unwrap_or_else(|| {
                        w::WorkflowId::new(w::identity(
                            b"cli-definition",
                            &[connection.repository.as_bytes(), request_id.as_bytes()],
                        ))
                    }),
                    expected,
                    definition: w::Definition::parse_yaml(yaml).map_err(error)?,
                },
            )
        }
        WorkflowCommand::Enable {
            id,
            request_id,
            expected,
        }
        | WorkflowCommand::Disable {
            id,
            request_id,
            expected,
        } => {
            let enabled = matches!(&command, WorkflowCommand::Enable { .. });
            let page = client
                .workflow_page(&w::Query {
                    workflow: Some(id),
                    ..Default::default()
                })
                .await
                .map_err(error)?;
            let version = page
                .workflows
                .first()
                .ok_or_else(|| user_error("Workflow not found"))?;
            if version.version != expected {
                return Err(user_error("Workflow version changed"));
            }
            let mut definition = version.definition.clone();
            definition.enabled = enabled;
            (
                request_id,
                w::Command::Put {
                    workflow: id,
                    expected: Some(expected),
                    definition,
                },
            )
        }
        WorkflowCommand::Remove {
            id,
            request_id,
            expected,
        } => (
            request_id,
            w::Command::Remove {
                workflow: id,
                expected,
            },
        ),
        WorkflowCommand::Run {
            id,
            request_id,
            expected,
            repository,
            operation,
            commit,
            bookmark,
        } => (
            request_id,
            w::Command::Run {
                workflow: id,
                expected,
                source: w::Source {
                    repository,
                    operation: Some(operation),
                    commit: Some(commit),
                    bookmark,
                    conversation: None,
                },
            },
        ),
        WorkflowCommand::Cancel { run, request_id } => (request_id, w::Command::Cancel { run }),
        WorkflowCommand::Retry {
            run,
            request_id,
            expected,
        } => (request_id, w::Command::Retry { run, expected }),
        WorkflowCommand::Report {
            request_id,
            run,
            attempt,
            sequence,
            file,
        } => (
            request_id,
            w::Command::Report {
                run,
                attempt,
                sequence,
                data: serde_json::from_slice(&read(&file)?).map_err(error)?,
            },
        ),
        WorkflowCommand::Command { request_id, file } => (
            request_id,
            serde_json::from_slice(&read(&file)?).map_err(error)?,
        ),
    };
    output(
        ui,
        args.format,
        &client
            .apply_workflow_command(&w::Request {
                version: 1,
                request_id: id,
                command,
            })
            .await
            .map_err(error)?,
    )
}
