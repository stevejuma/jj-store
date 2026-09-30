//! User-facing ReviewStore commands over the bound native repository.
use crate::{
    binding::Connection,
    commands::{Format, error},
    presentation, review_presentation,
};
use clap::{Args, Subcommand};
use jj_cli::{cli_util::CommandHelper, command_error::CommandError, ui::Ui};
use jj_review::{
    ArtifactAnchorV1, CommentId, CommentRevisionId, ReviewBodyFormatV1, ReviewCommandV1,
    ReviewCommentBodyV1, ReviewThreadId, ThreadSnapshotId,
};
use jj_store_model::RequestId;
use jj_store_service::review_mutations;
use std::path::PathBuf;

const MAX_ANCHOR_BYTES: u64 = 2 * 1024 * 1024;

#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct ReviewArgs {
    #[arg(long, global = true, value_enum, default_value_t=Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<ReviewCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ReviewCommand {
    /// List a bounded page of review threads
    List {
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Show one current-root thread with comments and events
    Show { thread_id: ReviewThreadId },
    /// Create a thread from one complete typed anchor document
    Create {
        #[arg(long)]
        request_id: RequestId,
        #[arg(long)]
        anchor: PathBuf,
        #[command(flatten)]
        body: BodyArgs,
    },
    /// Append a reply using the exact current workflow version
    Reply {
        #[arg(long)]
        request_id: RequestId,
        thread_id: ReviewThreadId,
        #[arg(long)]
        expected_workflow_version: u64,
        #[arg(long)]
        parent_comment_id: Option<CommentId>,
        #[command(flatten)]
        body: BodyArgs,
    },
    /// Replace a visible comment using its exact selected revision
    Edit {
        #[arg(long)]
        request_id: RequestId,
        comment_id: CommentId,
        #[arg(long)]
        expected_revision_id: CommentRevisionId,
        #[command(flatten)]
        body: BodyArgs,
    },
    /// Resolve concurrent comment edits using every exact conflicting revision
    ResolveCommentConflict {
        #[arg(long)]
        request_id: RequestId,
        comment_id: CommentId,
        #[arg(long = "expected-revision-id", num_args = 2..=64)]
        expected_revision_ids: Vec<CommentRevisionId>,
        #[command(flatten)]
        body: BodyArgs,
    },
    /// Replace a comment with a visible tombstone
    Tombstone {
        #[arg(long)]
        request_id: RequestId,
        comment_id: CommentId,
        #[arg(long)]
        expected_revision_id: CommentRevisionId,
        #[arg(long)]
        reason: Option<String>,
    },
    /// Hide a comment with an explicit moderation reason
    Hide {
        #[arg(long)]
        request_id: RequestId,
        comment_id: CommentId,
        #[arg(long)]
        expected_revision_id: CommentRevisionId,
        #[arg(long)]
        moderation_reason: String,
    },
    /// Resolve a thread using its exact selected snapshot ID
    Resolve {
        #[arg(long)]
        request_id: RequestId,
        thread_id: ReviewThreadId,
        #[arg(long)]
        expected_thread_version: ThreadSnapshotId,
        #[arg(long)]
        reason: Option<String>,
    },
    /// Reopen a thread using its exact selected snapshot ID
    Reopen {
        #[arg(long)]
        request_id: RequestId,
        thread_id: ReviewThreadId,
        #[arg(long)]
        expected_thread_version: ThreadSnapshotId,
        #[arg(long)]
        reason: Option<String>,
    },
    /// Inspect the durable receipt state of one exact request ID
    Status { request_id: RequestId },
}

#[derive(Clone, Debug, Args)]
pub struct BodyArgs {
    /// Complete comment text
    #[arg(long)]
    body: String,
    /// Store as plain text instead of safe Markdown
    #[arg(long)]
    plain_text: bool,
}

impl BodyArgs {
    fn value(self) -> ReviewCommentBodyV1 {
        ReviewCommentBodyV1 {
            schema_version: 1,
            format: if self.plain_text {
                ReviewBodyFormatV1::PlainText
            } else {
                ReviewBodyFormatV1::SafeMarkdown
            },
            text: self.body,
            sanitizer_version: "review-markdown-v1".into(),
        }
    }
}

pub async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    args: ReviewArgs,
) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["review".into()]);
    };
    let connection = Connection::read(helper.workspace_loader()?.repo_path()).map_err(error)?;
    let client = connection.connect().await.map_err(error)?;
    match command {
        ReviewCommand::List { limit, cursor } => {
            let page = client
                .review_threads(cursor.as_deref(), limit)
                .await
                .map_err(error)?;
            review_presentation::page(ui, helper, args.format, &page)
        }
        ReviewCommand::Show { thread_id } => {
            let detail = client.review_thread(thread_id).await.map_err(error)?;
            review_presentation::detail(ui, helper, args.format, &detail)
        }
        ReviewCommand::Status { request_id } => {
            let status = client
                .review_command_status(request_id)
                .await
                .map_err(error)?;
            review_presentation::result(ui, helper, args.format, &status)
        }
        command => {
            let request = request(connection.repository, command).map_err(error)?;
            let outcome = client.apply_review_command(&request).await.map_err(error)?;
            review_presentation::result(ui, helper, args.format, &outcome)
        }
    }
}

fn request(
    repository: jj_store_model::RepositoryId,
    command: ReviewCommand,
) -> Result<review_mutations::Request, String> {
    let (request_id, command) = match command {
        ReviewCommand::Create {
            request_id,
            anchor,
            body,
        } => {
            let bytes = crate::binding::bounded_read(&anchor, MAX_ANCHOR_BYTES)
                .map_err(|_| "could not read bounded review anchor".to_owned())?;
            let anchor: ArtifactAnchorV1 = serde_json::from_slice(&bytes)
                .map_err(|_| "invalid review anchor document".to_owned())?;
            anchor
                .validate()
                .map_err(|_| "invalid review anchor document".to_owned())?;
            if anchor.repository_id != repository {
                return Err("review anchor belongs to another repository".into());
            }
            (
                request_id,
                ReviewCommandV1::CreateThread {
                    anchor: Box::new(anchor),
                    body: body.value(),
                },
            )
        }
        ReviewCommand::Reply {
            request_id,
            thread_id,
            expected_workflow_version,
            parent_comment_id,
            body,
        } => (
            request_id,
            ReviewCommandV1::AppendReply {
                thread_id,
                expected_workflow_version,
                parent_comment_id,
                body: body.value(),
            },
        ),
        ReviewCommand::Edit {
            request_id,
            comment_id,
            expected_revision_id,
            body,
        } => (
            request_id,
            ReviewCommandV1::EditComment {
                comment_id,
                expected_revision_id,
                replacement_body: body.value(),
            },
        ),
        ReviewCommand::ResolveCommentConflict {
            request_id,
            comment_id,
            mut expected_revision_ids,
            body,
        } => {
            expected_revision_ids.sort();
            (
                request_id,
                ReviewCommandV1::ResolveCommentConflict {
                    comment_id,
                    expected_revision_ids,
                    replacement_body: body.value(),
                },
            )
        }
        ReviewCommand::Tombstone {
            request_id,
            comment_id,
            expected_revision_id,
            reason,
        } => (
            request_id,
            ReviewCommandV1::TombstoneComment {
                comment_id,
                expected_revision_id,
                reason,
            },
        ),
        ReviewCommand::Hide {
            request_id,
            comment_id,
            expected_revision_id,
            moderation_reason,
        } => (
            request_id,
            ReviewCommandV1::HideComment {
                comment_id,
                expected_revision_id,
                moderation_reason,
            },
        ),
        ReviewCommand::Resolve {
            request_id,
            thread_id,
            expected_thread_version,
            reason,
        } => (
            request_id,
            ReviewCommandV1::ResolveThread {
                thread_id,
                expected_thread_version,
                reason,
            },
        ),
        ReviewCommand::Reopen {
            request_id,
            thread_id,
            expected_thread_version,
            reason,
        } => (
            request_id,
            ReviewCommandV1::ReopenThread {
                thread_id,
                expected_thread_version,
                reason,
            },
        ),
        ReviewCommand::List { .. } | ReviewCommand::Show { .. } | ReviewCommand::Status { .. } => {
            return Err("invalid review command dispatch".into());
        }
    };
    command
        .validate()
        .map_err(|_| "invalid review command".to_owned())?;
    Ok(review_mutations::Request {
        version: 1,
        request_id,
        command,
    })
}
