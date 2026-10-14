use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
#[command(disable_help_subcommand = true, long_about=include_str!("../../help/stacks.md"))]
pub struct StackArgs {
    /// Discover the JJ workspace from this directory
    #[arg(short = 'C', long, global = true, default_value = ".")]
    pub directory: PathBuf,
    /// Inspect the last snapshot without writing repository or settings state
    #[arg(long, global = true)]
    pub no_snapshot: bool,
    /// Validate and describe a mutation against the last snapshot without writing
    #[arg(long, global = true)]
    pub dry_run: bool,
    /// Confirm deletion, abandonment, discard or reconciliation
    #[arg(long, global = true)]
    pub yes: bool,
    /// Reject a changed operation (also returned by status/hunks JSON)
    #[arg(long, global = true, requires = "expect_revision")]
    pub expect_operation: Option<jj_store_model::OperationId>,
    /// Reject changed workspace configuration
    #[arg(long, global = true, requires = "expect_operation")]
    pub expect_revision: Option<u64>,
    #[command(subcommand)]
    pub command: Option<StackCommand>,
}
#[derive(Debug, Subcommand)]
pub enum StackCommand {
    /// Show workspace, applied stacks, unfinished paths and conflicts
    Status,
    /// Discover top-level stacks relative to the active or explicitly selected target
    List {
        #[arg(long)]
        #[arg(help = "Exact target bookmark; no implicit remote fetch")]
        target: Option<String>,
        /// Include integrated and unavailable bookmark recovery choices
        #[arg(long)]
        all: bool,
    },
    /// Show connected branch cards and one bounded commit page for each
    Show {
        /// Exact target bookmark for browsing before workspace creation
        #[arg(long)]
        target: Option<String>,
        #[arg(help = "Exact bookmark name; aliases retain their native identity")]
        bookmark: String,
        #[arg(long, default_value_t=50, value_parser=clap::value_parser!(u32).range(1..=500))]
        #[arg(help = "Maximum rows per returned page (1-500)")]
        limit: u32,
        /// Continue one exact branch card; requires --card
        #[arg(long, requires = "card")]
        after: Option<String>,
        #[arg(long)]
        #[arg(help = "Exact branch-card bookmark to show or continue")]
        card: Option<String>,
    },
    /// Create the initial merge/WIP workspace, retaining current unfinished work
    Init {
        #[arg(long)]
        #[arg(help = "New workspace or bookmark name")]
        name: String,
        #[arg(long)]
        #[arg(help = "Exact target bookmark; no implicit remote fetch")]
        target: String,
    },
    /// List, create, switch, update or remove named local workspace configurations
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommand,
    },
    /// Apply this exact bookmark tip immediately
    Apply {
        #[arg(help = "Exact bookmark name; aliases retain their native identity")]
        bookmark: String,
    },
    /// Unapply a stack atomically, preserving members shared with other stacks
    Unapply {
        #[arg(help = "Exact bookmark name; aliases retain their native identity")]
        bookmark: String,
        #[arg(long)]
        #[arg(help = "Remove only this explicitly applied bookmark")]
        branch_only: bool,
    },
    /// Create and apply a stack at the target or above an applied branch
    Create {
        #[arg(help = "New workspace or bookmark name")]
        name: String,
        #[arg(long)]
        #[arg(help = "Exact applied branch beneath the new stack")]
        parent: Option<String>,
    },
    /// Create, rename or delete a bookmark
    Branch {
        #[command(subcommand)]
        command: BranchCommand,
    },
    /// Rebase a branch segment onto the workspace target with native propagation
    Rebase {
        #[arg(help = "Exact bookmark name; aliases retain their native identity")]
        bookmark: String,
    },
    /// Squash a branch segment using its proven native boundary
    SquashBranch {
        #[arg(help = "Exact bookmark name; aliases retain their native identity")]
        bookmark: String,
    },
    /// Create a commit on the selected applied branch from exact selected changes
    Commit {
        #[arg(long)]
        #[arg(help = "Exact branch bookmark (net comparisons are inspection only)")]
        branch: String,
        #[command(flatten)]
        message: MessageArgs,
        #[command(flatten)]
        selection: SelectionArgs,
    },
    /// Reword a commit and optionally transfer selected changes into it
    Edit {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
        #[command(flatten)]
        message: MessageArgs,
        #[command(flatten)]
        selection: SelectionArgs,
    },
    /// Extract selected changes from this commit into a new child
    Split {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
        #[command(flatten)]
        message: MessageArgs,
        #[command(flatten)]
        selection: SelectionArgs,
    },
    /// Squash selected content into an exact mutable destination, retaining sources
    Transfer {
        #[arg(long)]
        #[arg(help = "Native JJ selector for the destination commit")]
        to: String,
        #[command(flatten)]
        selection: SelectionArgs,
    },
    /// Change a native commit description
    Reword {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
        #[command(flatten)]
        message: MessageArgs,
    },
    /// Insert an empty commit above or below the selected revision
    Empty {
        #[command(flatten)]
        position: PositionArgs,
        #[command(flatten)]
        message: MessageArgs,
    },
    /// Move one revision above or below the selected destination
    Move {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
        #[command(flatten)]
        position: PositionArgs,
    },
    /// Squash a complete revision into another revision
    Squash {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
        #[arg(long)]
        #[arg(help = "Native JJ selector for the squash destination")]
        into: String,
    },
    /// Move a complete commit's changes back into unfinished work
    Uncommit {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
    },
    /// Abandon a mutable commit; requires --yes
    Abandon {
        #[arg(help = "Native JJ selector resolving exactly one visible commit")]
        revision: String,
    },
    /// Show selected commit/WIP file changes, or a read-only net branch comparison
    Diff {
        #[arg(long, conflicts_with = "branch")]
        #[arg(help = "Native JJ source selector; defaults to the working copy")]
        from: Option<String>,
        #[arg(long)]
        #[arg(help = "Exact branch bookmark (net comparisons are inspection only)")]
        branch: Option<String>,
        /// Canonical path continuation from the previous page
        #[arg(long)]
        after: Option<String>,
        #[arg(long, default_value_t=100, value_parser=clap::value_parser!(u32).range(1..=500))]
        #[arg(help = "Maximum rows per returned page (1-500)")]
        limit: u32,
        /// File/directory path relative to -C (default: repository root)
        #[arg(help = "File path relative to -C")]
        path: Option<String>,
    },
    /// Show exact native hunk IDs and patches with operation/configuration pins
    Hunks {
        #[arg(help = "File path relative to -C")]
        path: String,
        #[arg(long)]
        #[arg(help = "Native JJ source selector; defaults to the working copy")]
        from: Option<String>,
    },
    /// Discard selected WIP content; requires --yes
    Discard {
        #[command(flatten)]
        selection: SelectionArgs,
    },
    /// List WIP conflict paths or inspect a file's ordered native versions
    Conflicts {
        #[arg(help = "File path relative to -C")]
        path: Option<String>,
    },
    /// Submit complete supported text resolution from a file or stdin (-)
    Resolve {
        #[arg(help = "File path relative to -C")]
        path: String,
        #[arg(long)]
        #[arg(help = "Reviewed UTF-8 resolution file, or - for stdin")]
        file: PathBuf,
    },
    /// Snapshot files and refresh the local workspace state
    Refresh,
    /// Explicitly reconcile saved membership and eligible obsolete merges; requires --yes
    Reconcile,
    /// Undo the native repository operation, including ordinary CLI changes
    Undo,
    /// Redo the native repository operation
    Redo,
}
#[derive(Debug, Subcommand)]
pub enum WorkspaceCommand {
    /// Show retained workspaces and their native merge/WIP identities
    List,
    /// Create and switch to an empty workspace inheriting the target
    Create {
        #[arg(help = "New workspace or bookmark name")]
        name: String,
    },
    /// Switch checkout to the retained workspace by exact name or ID
    Switch {
        #[arg(help = "Exact saved workspace name or ID")]
        workspace: String,
    },
    /// Update the active workspace, preserving unspecified settings
    Update {
        #[arg(long)]
        #[arg(help = "New workspace or bookmark name")]
        name: Option<String>,
        #[arg(long)]
        #[arg(help = "Exact target bookmark; no implicit remote fetch")]
        target: Option<String>,
    },
    /// Remove configuration while retaining native unfinished work; requires --yes
    Delete {
        #[arg(help = "Exact saved workspace name or ID")]
        workspace: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum BranchCommand {
    /// Label the selected commit or its single parent
    Create {
        #[arg(help = "New workspace or bookmark name")]
        name: String,
        #[command(flatten)]
        position: PositionArgs,
    },
    /// Rename a bookmark across saved workspace membership
    Rename {
        #[arg(help = "Existing bookmark name")]
        old: String,
        #[arg(help = "New bookmark name")]
        new: String,
    },
    /// Delete a top-level bookmark while retaining lower bookmarks and commits
    Delete {
        #[arg(help = "New workspace or bookmark name")]
        name: String,
    },
}
#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct PositionArgs {
    /// Insert above this exact single native revision
    #[arg(long)]
    pub above: Option<String>,
    /// Insert below this exact single native revision
    #[arg(long)]
    pub below: Option<String>,
}
#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct MessageArgs {
    /// Explicit commit description (empty is allowed only where JJ allows it)
    #[arg(short = 'm', long)]
    pub message: Option<String>,
    /// Read the description from a bounded UTF-8 file
    #[arg(long)]
    pub message_file: Option<PathBuf>,
}
#[derive(Debug, Default, Args)]
pub struct SelectionArgs {
    /// Source revision; defaults to WIP (split defaults to its selected commit)
    #[arg(long, conflicts_with = "selection_file")]
    pub from: Option<String>,
    /// Select all changed paths from the exact source
    #[arg(long, conflicts_with_all=["paths","hunk","empty","selection_file"])]
    pub all: bool,
    /// Deliberately create an empty commit (commit command only)
    #[arg(long, conflicts_with_all=["paths","hunk","selection_file"])]
    pub empty: bool,
    /// Native hunk ID; requires exactly one path and pinned operation/revision
    #[arg(long, conflicts_with = "selection_file")]
    pub hunk: Vec<String>,
    /// Version-1 operation/revision/selections JSON for mixed native sources
    #[arg(long, conflicts_with_all=["paths","hunk"])]
    pub selection_file: Option<PathBuf>,
    /// Exact file paths relative to -C; directories expand only through --all
    #[arg(last = true)]
    pub paths: Vec<String>,
}
