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
    /// Optional bounded changed-path record outside the exported tree.
    #[arg(long)]
    pub changed_paths: Option<PathBuf>,
    /// Compare with this exact visible native revision.
    #[arg(long, conflicts_with = "empty_baseline")]
    pub before: Option<m::CommitId>,
    /// Compare a newly created bookmark with an explicit empty tree.
    #[arg(long)]
    pub empty_baseline: bool,
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
    let changed = if args.changed_paths.is_some() {
        let baseline = if args.empty_baseline {
            None
        } else if let Some(before) = args.before {
            let before = CommitId::from_bytes(before.as_bytes());
            if !repo.index().has_id(&before).await? {
                return Err(user_error("Comparison revision is not visible"));
            }
            Some(repo.store().get_commit_async(&before).await?)
        } else if let [parent] = commit.parent_ids() {
            Some(repo.store().get_commit_async(parent).await?)
        } else {
            None
        };
        if baseline.is_none() && !args.empty_baseline {
            None
        } else {
            let mut before = std::collections::BTreeMap::new();
            if let Some(baseline) = baseline {
                for (path, value) in baseline.tree().entries() {
                    before.insert(path.as_internal_file_string().to_owned(), value?);
                }
            }
            let mut paths = std::collections::BTreeSet::new();
            for (path, value) in commit.tree().entries() {
                let name = path.as_internal_file_string().to_owned();
                if before.remove(&name).as_ref() != Some(&value?) {
                    paths.insert(name);
                }
            }
            paths.extend(before.into_keys());
            if paths.len() > 100_000 {
                return Err(user_error("Changed path limit"));
            }
            Some(paths.into_iter().collect::<Vec<_>>())
        }
    } else {
        None
    };
    if args
        .changed_paths
        .as_ref()
        .is_some_and(|p| !p.is_absolute() || p.exists() || p.starts_with(&args.destination))
    {
        return Err(user_error(
            "Changed paths need a fresh absolute file outside the source",
        ));
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
        if let Some(path) = &args.changed_paths {
            let data = serde_json::to_vec(&changed).map_err(error)?;
            if data.len() > 8 * 1024 * 1024 {
                return Err(user_error("Changed paths byte limit"));
            }
            let mut out = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(path)
                .map_err(error)?;
            out.write_all(&data).map_err(error)?;
        }
        Ok(())
    }
    .await;
    if result.is_err() {
        let _ = fs::remove_dir_all(&args.destination);
    }
    result
}
