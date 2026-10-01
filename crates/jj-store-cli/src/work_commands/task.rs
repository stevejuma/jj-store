use super::*;
use clap::ValueEnum;

#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct TaskArgs {
    #[arg(long, global = true, value_enum, default_value_t = Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<TaskCommand>,
}
#[derive(Clone, Debug, Args)]
pub struct TaskFields {
    #[arg(long)]
    title: String,
    #[arg(long)]
    body: String,
    #[arg(long, value_enum)]
    state: State,
    #[arg(long)]
    repository: Option<RepositoryId>,
    #[arg(long)]
    parent: Option<TaskId>,
}
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum State {
    Open,
    InProgress,
    Blocked,
    Done,
}
impl State {
    fn value(self) -> TaskStateV1 {
        match self {
            Self::Open => TaskStateV1::Open,
            Self::InProgress => TaskStateV1::InProgress,
            Self::Blocked => TaskStateV1::Blocked,
            Self::Done => TaskStateV1::Done,
        }
    }
}
#[derive(Clone, Debug, Subcommand)]
pub enum TaskCommand {
    /// List a pinned page of tasks in one project
    List {
        #[arg(long)]
        project: ProjectId,
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Show one task and a pinned page of its threaded comments
    Show {
        id: TaskId,
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Create a task; parent and child may target different linked repositories
    Create {
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        id: Option<TaskId>,
        #[arg(long)]
        project: ProjectId,
        #[arg(long)]
        expected_project_version: u64,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        body: String,
        #[arg(long)]
        repository: Option<RepositoryId>,
        #[arg(long)]
        parent: Option<TaskId>,
    },
    /// Replace task fields using exact task and project versions
    Edit {
        id: TaskId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        expected_project_version: u64,
        #[command(flatten)]
        fields: TaskFields,
    },
    /// Add a root comment or reply using the exact task version
    Comment {
        task: TaskId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        id: Option<WorkCommentId>,
        #[arg(long)]
        expected_task_version: u64,
        #[arg(long)]
        reply_to: Option<WorkCommentId>,
        #[arg(long)]
        body: String,
    },
    /// Edit a comment using the exact comment and task versions
    EditComment {
        id: WorkCommentId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        expected_task_version: u64,
        #[arg(long)]
        body: String,
    },
    /// Tombstone a comment while retaining immutable history
    TombstoneComment {
        id: WorkCommentId,
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        expected_task_version: u64,
    },
    Status {
        request_id: RequestId,
    },
}
pub async fn run_task(
    ui: &mut Ui,
    helper: &CommandHelper,
    args: TaskArgs,
) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["task".into()]);
    };
    let client = client::Client::open(helper).await?;
    let (id, command) = match command {
        TaskCommand::List {
            project,
            limit,
            cursor,
        } => {
            return output::page(
                ui,
                helper,
                args.format,
                &client
                    .work_page(&WorkQuery {
                        project: Some(project),
                        limit,
                        cursor,
                        ..Default::default()
                    })
                    .await?,
            );
        }
        TaskCommand::Show { id, limit, cursor } => {
            return output::page(
                ui,
                helper,
                args.format,
                &client
                    .work_page(&WorkQuery {
                        task: Some(id),
                        limit,
                        cursor,
                        ..Default::default()
                    })
                    .await?,
            );
        }
        TaskCommand::Status { request_id } => {
            return output::result(
                ui,
                helper,
                args.format,
                &client.work_command_status(request_id).await?,
            );
        }
        TaskCommand::Create {
            request_id,
            id,
            project,
            expected_project_version,
            title,
            body,
            repository,
            parent,
        } => (
            request_id,
            WorkCommandV1::CreateTask {
                id: id.unwrap_or_else(|| TaskId::new(client.entity_id(request_id, b"task"))),
                project,
                expected_project_version,
                title,
                body,
                repository,
                parent,
            },
        ),
        TaskCommand::Edit {
            id,
            request_id,
            expected_version,
            expected_project_version,
            fields,
        } => (
            request_id,
            WorkCommandV1::UpdateTask {
                id,
                expected_version,
                expected_project_version,
                title: fields.title,
                body: fields.body,
                state: fields.state.value(),
                repository: fields.repository,
                parent: fields.parent,
            },
        ),
        TaskCommand::Comment {
            task,
            request_id,
            id,
            expected_task_version,
            reply_to,
            body,
        } => (
            request_id,
            WorkCommandV1::AddComment {
                id: id.unwrap_or_else(|| {
                    WorkCommentId::new(client.entity_id(request_id, b"comment"))
                }),
                task,
                expected_task_version,
                reply_to,
                body,
            },
        ),
        TaskCommand::EditComment {
            id,
            request_id,
            expected_version,
            expected_task_version,
            body,
        } => (
            request_id,
            WorkCommandV1::EditComment {
                id,
                expected_version,
                expected_task_version,
                body,
            },
        ),
        TaskCommand::TombstoneComment {
            id,
            request_id,
            expected_version,
            expected_task_version,
        } => (
            request_id,
            WorkCommandV1::TombstoneComment {
                id,
                expected_version,
                expected_task_version,
            },
        ),
    };
    apply(ui, helper, args.format, &client, id, command).await
}
