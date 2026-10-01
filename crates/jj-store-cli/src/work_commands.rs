//! Native project and task commands over the repository's local service.
use crate::{
    binding::Connection,
    commands::{Format, error},
    presentation,
};
use clap::{Args, Subcommand};
use jj_cli::{cli_util::CommandHelper, command_error::CommandError, ui::Ui};
use jj_store_model::{RepositoryId, RequestId};
use jj_store_service::work::{
    ProjectId, Request, TaskId, TaskStateV1, WorkCommandV1, WorkCommentId, WorkPage, WorkQuery,
};
use sha2::{Digest as _, Sha256};
mod output;
mod task;

#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct ProjectArgs {
    #[arg(long, global = true, value_enum, default_value_t = Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<ProjectCommand>,
}
#[derive(Clone, Debug, Subcommand)]
pub enum ProjectCommand {
    /// List one pinned page of native projects
    List {
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Show a project and a pinned page of its tasks
    Show {
        id: ProjectId,
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Create a project home record; omitted entity IDs derive from the request ID
    Create {
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        id: Option<ProjectId>,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        body: String,
    },
    /// Replace project title and body using its exact current version
    Edit {
        id: ProjectId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        title: String,
        #[arg(long)]
        body: String,
    },
    /// Link one stable repository ID without granting code access
    Link {
        project: ProjectId,
        repository: RepositoryId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected_version: u64,
    },
    /// Retire a link while preserving existing task targets
    Unlink {
        project: ProjectId,
        repository: RepositoryId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected_version: u64,
    },
    /// Read the durable receipt for an exact request ID
    Status { request_id: RequestId },
}
pub use task::{TaskArgs, run_task};

fn entity_id(repository: RepositoryId, request: RequestId, kind: &[u8]) -> [u8; 16] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/work-cli-entity/v1\0");
    hash.update(repository.as_bytes());
    hash.update(request.as_bytes());
    hash.update(kind);
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hash.finalize()[..16]);
    bytes
}
async fn apply(
    ui: &mut Ui,
    helper: &CommandHelper,
    format: Format,
    client: &jj_store_rpc::client::NativeClient,
    id: RequestId,
    command: WorkCommandV1,
) -> Result<(), CommandError> {
    let outcome = client
        .apply_work_command(&Request {
            version: 1,
            request_id: id,
            command,
        })
        .await
        .map_err(error)?;
    output::result(ui, helper, format, &outcome)
}
pub async fn run_project(
    ui: &mut Ui,
    helper: &CommandHelper,
    args: ProjectArgs,
) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["project".into()]);
    };
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let client = connection.connect().await.map_err(error)?;
    let (id, command) = match command {
        ProjectCommand::List { limit, cursor } => {
            return output::page(
                ui,
                helper,
                args.format,
                &client
                    .work_page(&WorkQuery {
                        limit,
                        cursor,
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?,
            );
        }
        ProjectCommand::Show { id, limit, cursor } => {
            return output::page(
                ui,
                helper,
                args.format,
                &client
                    .work_page(&WorkQuery {
                        project: Some(id),
                        limit,
                        cursor,
                        ..Default::default()
                    })
                    .await
                    .map_err(error)?,
            );
        }
        ProjectCommand::Status { request_id } => {
            return output::result(
                ui,
                helper,
                args.format,
                &client
                    .work_command_status(request_id)
                    .await
                    .map_err(error)?,
            );
        }
        ProjectCommand::Create {
            request_id,
            id,
            title,
            body,
        } => (
            request_id,
            WorkCommandV1::CreateProject {
                id: id.unwrap_or_else(|| {
                    ProjectId::new(entity_id(connection.repository, request_id, b"project"))
                }),
                title,
                body,
            },
        ),
        ProjectCommand::Edit {
            id,
            request_id,
            expected_version,
            title,
            body,
        } => (
            request_id,
            WorkCommandV1::UpdateProject {
                id,
                expected_version,
                title,
                body,
            },
        ),
        ProjectCommand::Link {
            project,
            repository,
            request_id,
            expected_version,
        } => (
            request_id,
            WorkCommandV1::SetRepository {
                project,
                repository,
                expected_version,
                active: true,
            },
        ),
        ProjectCommand::Unlink {
            project,
            repository,
            request_id,
            expected_version,
        } => (
            request_id,
            WorkCommandV1::SetRepository {
                project,
                repository,
                expected_version,
                active: false,
            },
        ),
    };
    apply(ui, helper, args.format, &client, id, command).await
}
