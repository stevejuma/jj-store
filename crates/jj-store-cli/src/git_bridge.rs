//! Parsed, backend-aware routing for explicitly requested native Git operations.
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
};

/// Discover the backend without loading operation heads, snapshotting files or
/// creating a second WorkspaceCommandHelper. Creation remains upstream-owned.
pub fn native_operation(helper: &CommandHelper) -> Result<Option<String>, CommandError> {
    let Some(git) = helper.matches().subcommand_matches("git") else {
        return Ok(None);
    };
    let Some((name, args)) = git.subcommand() else {
        return Ok(None);
    };
    if matches!(name, "init" | "clone") {
        return Ok(None);
    }
    let workspace = helper.load_workspace()?;
    if workspace.repo_loader().store().backend().name() != "jj-store" {
        return Ok(None);
    }
    let operation = if name == "remote" {
        format!("remote {}", args.subcommand_name().unwrap_or(""))
    } else {
        name.to_owned()
    };
    Ok(Some(operation))
}

pub fn unavailable(operation: &str) -> CommandError {
    user_error(format!(
        "NATIVE_GIT_CAPABILITY: git {operation} is not yet qualified for native repositories"
    ))
    .hinted("Inspect `jj store capabilities` for the current native command matrix.")
}
