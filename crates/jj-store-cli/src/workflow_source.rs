//! Read an exact native revision without creating source operations or workspaces.
use crate::{binding::Connection, commands::error};
use futures::io::AsyncReadExt as _;
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
};
use jj_lib::{
    backend::{CommitId, TreeValue},
    op_store::OperationId,
    repo::Repo as _,
};
use jj_store_model as m;
use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    path::PathBuf,
};

#[derive(Clone, Debug, clap::Args)]
pub struct Args {
    #[arg(long = "repo-id", id = "workflow_repository")]
    pub repository: m::RepositoryId,
    #[arg(long)]
    pub operation: m::OperationId,
    #[arg(long)]
    pub commit: m::CommitId,
    /// A fresh absolute directory; no repository metadata is exported.
    pub destination: PathBuf,
}
pub async fn run(helper: &CommandHelper, args: Args) -> Result<(), CommandError> {
    if !args.destination.is_absolute() || args.destination.exists() {
        return Err(user_error("Source export needs a new absolute directory"));
    }
    let workspace = helper.load_workspace()?;
    let binding = Connection::read(workspace.repo_path()).map_err(error)?;
    if binding.repository != args.repository {
        return Err(user_error("Source repository binding differs"));
    }
    let loader = workspace.repo_loader();
    let operation = loader
        .load_operation(&OperationId::from_bytes(args.operation.as_bytes()))
        .await?;
    let repo = loader.load_at(&operation).await?;
    let id = CommitId::from_bytes(args.commit.as_bytes());
    if !repo.index().has_id(&id).await? {
        return Err(user_error(
            "Revision is not visible at the selected operation",
        ));
    }
    let commit = repo.store().get_commit_async(&id).await?;
    // Validate the complete tree before creating anything. Symlinks, conflicts and
    // submodules require separate export profiles and are refused in this profile.
    let mut files = Vec::new();
    for (path, value) in commit.tree().entries() {
        let value = value?;
        let Some(Some(TreeValue::File { id, executable, .. })) = value.as_resolved() else {
            return Err(user_error(
                "Source export contains a conflict, symlink or submodule",
            ));
        };
        let name = path.as_internal_file_string();
        if name
            .split('/')
            .any(|part| matches!(part, "" | "." | ".." | ".jj" | ".git") || part.contains('\\'))
        {
            return Err(user_error("Source export contains an unsupported path"));
        }
        files.push((path.clone(), id.clone(), *executable));
        if files.len() > 100_000 {
            return Err(user_error("Source export file limit"));
        }
    }
    fs::create_dir(&args.destination).map_err(error)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&args.destination, fs::Permissions::from_mode(0o700)).map_err(error)?;
    }
    let result = async {
        let mut total = 0u64;
        for (path, id, executable) in files {
            let mut bytes = Vec::new();
            repo.store()
                .read_file(&path, &id)
                .await?
                .take(32 * 1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .await
                .map_err(error)?;
            total += bytes.len() as u64;
            if bytes.len() > 32 * 1024 * 1024 || total > 512 * 1024 * 1024 {
                return Err(user_error("Source export byte limit"));
            }
            let target = args.destination.join(path.as_internal_file_string());
            fs::create_dir_all(target.parent().expect("file parent")).map_err(error)?;
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&target)
                .map_err(error)?;
            file.write_all(&bytes).map_err(error)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                fs::set_permissions(
                    &target,
                    fs::Permissions::from_mode(if executable { 0o700 } else { 0o600 }),
                )
                .map_err(error)?;
            }
        }
        Ok(())
    }
    .await;
    if result.is_err() {
        let _ = fs::remove_dir_all(&args.destination);
    }
    result
}
