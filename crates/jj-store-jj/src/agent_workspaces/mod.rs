//! Fixed native-aware JJ executable boundary for application-owned workspaces.
mod cleanup;
mod command;
mod ignores;
pub mod inspection;
mod provision;
use jj_lib::{
    protos::local_working_copy::Checkout,
    workspace::{DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _},
};
use jj_store_model::OperationId;
use prost::Message as _;
use std::{
    fs,
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, &'static str>;
#[derive(Clone, Debug)]
pub struct Selection {
    pub root: PathBuf,
    pub metadata: PathBuf,
    pub operation: OperationId,
    pub commit: String,
    pub name: String,
}
#[derive(Clone)]
pub struct Provider {
    pub executable: PathBuf,
}
impl Provider {
    /// Read an existing checkout and exact recorded revision without snapshotting.
    pub async fn inspect(&self, root: &Path) -> Result<Selection> {
        let (root, metadata, operation, name) = identity(root)?;
        let commit = self
            .run(
                &root,
                &[
                    "--ignore-working-copy",
                    "--at-op",
                    &operation.to_string(),
                    "log",
                    "-r",
                    "@",
                    "--no-graph",
                    "-T",
                    "commit_id",
                ],
            )
            .await?;
        let commit = commit.trim().to_owned();
        if !valid_commit(&commit) {
            return Err("invalid_commit");
        }
        Ok(Selection {
            root,
            metadata,
            operation,
            commit,
            name,
        })
    }
    /// Revalidate the checkout identity without launching JJ. An unchanged exact
    /// operation and workspace name select the same immutable view/commit.
    pub fn verify(&self, expected: &Selection) -> Result<()> {
        let (root, metadata, operation, name) = identity(&expected.root)?;
        if root != expected.root
            || metadata != expected.metadata
            || operation != expected.operation
            || name != expected.name
        {
            return Err("workspace_changed");
        }
        Ok(())
    }
    /// Stable destination/name ownership makes creation retries inspectable.
    pub async fn create(
        &self,
        source: &Selection,
        destination: &Path,
        operation: OperationId,
        commit: &str,
        name: &str,
    ) -> Result<Selection> {
        if !valid_commit(commit) || !name.starts_with("agent-") || name.len() > 80 {
            return Err("invalid_selection");
        }
        if self.inspect(&source.root).await?.metadata != source.metadata {
            return Err("workspace_changed");
        }
        if destination.exists() {
            let existing = self.inspect(destination).await?;
            if existing.metadata != source.metadata || existing.name != name {
                return Err("workspace_changed");
            }
            self.verify_base(&existing, commit).await?;
            return Ok(existing);
        }
        let path = destination.to_str().ok_or("invalid_destination")?;
        self.run(
            &source.root,
            &[
                "agent-workspace",
                "--operation",
                &operation.to_string(),
                "--name",
                name,
                "--revision",
                commit,
                path,
            ],
        )
        .await?;
        let created = self.inspect(destination).await?;
        if created.metadata != source.metadata || created.name != name {
            return Err("workspace_changed");
        }
        self.verify_base(&created, commit).await?;
        Ok(created)
    }
    async fn verify_base(&self, selected: &Selection, commit: &str) -> Result<()> {
        let parents = self
            .run(
                &selected.root,
                &[
                    "--ignore-working-copy",
                    "--at-op",
                    &selected.operation.to_string(),
                    "log",
                    "-r",
                    &selected.commit,
                    "--no-graph",
                    "-T",
                    "parents.map(|c| c.commit_id()).join(\",\")",
                ],
            )
            .await?;
        let changes = self
            .run(
                &selected.root,
                &[
                    "--ignore-working-copy",
                    "--at-op",
                    &selected.operation.to_string(),
                    "diff",
                    "--from",
                    commit,
                    "--to",
                    &selected.commit,
                    "--summary",
                ],
            )
            .await?;
        if parents.trim() != commit || !changes.trim().is_empty() {
            return Err("workspace_base_unverified");
        }
        Ok(())
    }
    /// Explicit turn completion snapshots only the isolated physical workspace.
    pub async fn snapshot(&self, workspace: &Selection) -> Result<Selection> {
        let current = self.inspect(&workspace.root).await?;
        if current.metadata != workspace.metadata || current.name != workspace.name {
            return Err("workspace_changed");
        }
        self.run(&workspace.root, &["status"]).await?;
        self.inspect(&workspace.root).await
    }
}
fn valid_commit(value: &str) -> bool {
    matches!(value.len(), 40 | 128)
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn directory(path: &Path) -> Result<()> {
    let stat = fs::symlink_metadata(path).map_err(|_| "workspace_unavailable")?;
    if !stat.is_dir() || stat.file_type().is_symlink() {
        return Err("invalid_workspace");
    }
    Ok(())
}
fn regular(path: &Path, limit: u64) -> Result<Vec<u8>> {
    use std::io::Read as _;
    let stat = fs::symlink_metadata(path).map_err(|_| "workspace_unavailable")?;
    if !stat.is_file() || stat.file_type().is_symlink() || stat.len() > limit {
        return Err("invalid_workspace");
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|_| "workspace_unavailable")?
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "workspace_unavailable")?;
    if bytes.len() as u64 > limit {
        return Err("workspace_limit");
    }
    Ok(bytes)
}

fn identity(root: &Path) -> Result<(PathBuf, PathBuf, OperationId, String)> {
    directory(root)?;
    let root = root.canonicalize().map_err(|_| "workspace_unavailable")?;
    for path in [root.join(".jj"), root.join(".jj/working_copy")] {
        directory(&path)?;
    }
    let checkout = regular(&root.join(".jj/working_copy/checkout"), 65536)?;
    let checkout = Checkout::decode(checkout.as_slice()).map_err(|_| "invalid_checkout")?;
    let operation =
        OperationId::from_slice(&checkout.operation_id).map_err(|_| "invalid_operation")?;
    let metadata = DefaultWorkspaceLoaderFactory
        .create(&root)
        .map_err(|_| "workspace_unavailable")?
        .repo_path()
        .canonicalize()
        .map_err(|_| "workspace_unavailable")?;
    let name = if checkout.workspace_name.is_empty() {
        "default".into()
    } else {
        checkout.workspace_name
    };
    Ok((root, metadata, operation, name))
}
