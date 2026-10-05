//! Private state path policy shared by capture and server composition.
use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
};
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid private state path: {0}")]
    Config(&'static str),
    #[error("private state filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

/// Resolve existing ancestors before checking checkout boundaries. This also
/// works for a not-yet-created state directory, without creating it first.
pub fn outside_checkout(path: &Path) -> Result<PathBuf> {
    let development = std::env::var_os("JJ_STORE_DEVELOPMENT_STATE_ROOT").map(PathBuf::from);
    outside_checkout_for_development(path, development.as_deref())
}

/// Permit one explicit ignored `.local` subtree for the development harness.
/// Production callers omit the environment override and retain the checkout
/// rejection.
pub fn outside_checkout_for_development(
    path: &Path,
    development_root: Option<&Path>,
) -> Result<PathBuf> {
    let resolved = resolve_control_path(path)?;
    let development_root = development_root.map(resolve_control_path).transpose()?;
    for parent in resolved.ancestors() {
        if parent.join(".jj").exists() || parent.join(".git").exists() {
            let local = parent.join(".local");
            if development_root.as_ref().is_some_and(|root| {
                root.starts_with(&local) && root != &local && resolved.starts_with(root)
            }) {
                return Ok(resolved);
            }
            return Err(Error::Config(
                "credentials and process state must be outside checkouts",
            ));
        }
    }
    Ok(resolved)
}

fn resolve_control_path(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    if fs::symlink_metadata(&absolute).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err(Error::Config("control paths must not be symlinks"));
    }
    let mut ancestor = absolute.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(
            ancestor
                .file_name()
                .ok_or(Error::Config("control path"))?
                .to_owned(),
        );
        ancestor = ancestor.parent().ok_or(Error::Config("control path"))?;
    }
    let mut resolved = ancestor.canonicalize()?;
    for component in suffix.into_iter().rev() {
        resolved.push(component);
    }
    Ok(resolved)
}
pub fn exclusive_lock(path: &Path) -> Result<File> {
    let path = outside_checkout(path)?;
    if let Ok(metadata) = fs::symlink_metadata(&path)
        && !metadata.is_file()
    {
        return Err(Error::Config("expected regular lock file"));
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    file.try_lock()
        .map_err(|_| Error::Config("another process holds this state or policy lock"))?;
    Ok(file)
}
