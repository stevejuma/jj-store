use crate::agent_capture_files::{restrict_directory, sync_dir};
use jj_agent::IntegrationId;
use jj_store_jj::operation_stores::AuthorityResult;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, Permissions},
    io::Write as _,
    path::{Component, Path, PathBuf},
};

const MAX_CONFIG_BYTES: u64 = 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InstallRecordV1 {
    schema_version: u32,
    integration: IntegrationId,
    target: PathBuf,
    pub(crate) original_sha256: Option<String>,
    pub(crate) owned_sha256: String,
}

pub(crate) struct TargetBytes {
    pub(crate) bytes: Option<Vec<u8>>,
    pub(crate) permissions: Option<Permissions>,
}

pub(crate) fn integration_name(integration: IntegrationId) -> AuthorityResult<&'static str> {
    match integration {
        IntegrationId::Codex => Ok("codex"),
        IntegrationId::Cursor => Ok("cursor"),
        IntegrationId::OpenCode => Ok("opencode"),
        IntegrationId::OwnedController => Err("owned controller has no harness setup".into()),
    }
}

pub(crate) fn install_path(state: &Path, integration: IntegrationId) -> AuthorityResult<PathBuf> {
    Ok(state.join("install").join(integration_name(integration)?))
}

pub(crate) fn read_target(path: &Path) -> AuthorityResult<TargetBytes> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(TargetBytes {
            bytes: None,
            permissions: None,
        });
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > MAX_CONFIG_BYTES
    {
        return Err("agent integration target must be a bounded regular file".into());
    }
    Ok(TargetBytes {
        bytes: Some(jj_store_server::policy::read_regular(
            path,
            MAX_CONFIG_BYTES,
        )?),
        permissions: Some(metadata.permissions()),
    })
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub(crate) fn read_record(
    install: &Path,
    integration: IntegrationId,
    target: &Path,
) -> AuthorityResult<InstallRecordV1> {
    let metadata = fs::symlink_metadata(install)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err("agent setup ownership path must be a directory".into());
    }
    let record: InstallRecordV1 = serde_json::from_slice(&jj_store_server::policy::read_regular(
        &install.join("record.json"),
        64 * 1024,
    )?)?;
    let backup = install.join("original.bin");
    let original = if record.original_sha256.is_some() {
        Some(jj_store_server::policy::read_regular(
            &backup,
            MAX_CONFIG_BYTES,
        )?)
    } else if backup.exists() {
        return Err("unexpected agent setup backup".into());
    } else {
        None
    };
    if record.schema_version != 1
        || record.integration != integration
        || record.target != target
        || record.owned_sha256.len() != 64
        || !record
            .owned_sha256
            .bytes()
            .all(|value| value.is_ascii_hexdigit())
        || original.as_deref().map(digest) != record.original_sha256
    {
        return Err("invalid agent setup ownership record".into());
    }
    Ok(record)
}

pub(crate) fn persist_record(
    state: &Path,
    integration: IntegrationId,
    target: &Path,
    original: Option<&[u8]>,
    owned_sha256: &str,
) -> AuthorityResult<()> {
    let root = private_directory(&state.join("install"))?;
    let destination = root.join(integration_name(integration)?);
    if destination.exists() {
        return Err("agent setup ownership record already exists".into());
    }
    let temporary = tempfile::Builder::new().prefix(".tmp").tempdir_in(&root)?;
    restrict_directory(temporary.path())?;
    if let Some(bytes) = original {
        write_file(&temporary.path().join("original.bin"), bytes, None)?;
    }
    let record = InstallRecordV1 {
        schema_version: 1,
        integration,
        target: target.to_owned(),
        original_sha256: original.map(digest),
        owned_sha256: owned_sha256.into(),
    };
    write_file(
        &temporary.path().join("record.json"),
        &serde_json::to_vec_pretty(&record)?,
        None,
    )?;
    sync_dir(temporary.path())?;
    fs::rename(temporary.keep(), &destination)?;
    sync_dir(&root)
}

fn private_directory(path: &Path) -> AuthorityResult<PathBuf> {
    if let Ok(metadata) = fs::symlink_metadata(path)
        && (!metadata.is_dir() || metadata.file_type().is_symlink())
    {
        return Err("agent setup state must be a directory".into());
    }
    fs::create_dir_all(path)?;
    restrict_directory(path)?;
    Ok(path.to_owned())
}

pub(crate) fn write_target(
    workspace: &Path,
    path: &Path,
    bytes: &[u8],
    permissions: Option<Permissions>,
) -> AuthorityResult<()> {
    ensure_parent(workspace, path)?;
    write_file(path, bytes, permissions)
}

pub(crate) fn validate_target_parent(workspace: &Path, path: &Path) -> AuthorityResult<()> {
    visit_parent(workspace, path, false)
}

fn ensure_parent(workspace: &Path, path: &Path) -> AuthorityResult<()> {
    visit_parent(workspace, path, true)
}

fn visit_parent(workspace: &Path, path: &Path, create: bool) -> AuthorityResult<()> {
    let parent = path.parent().ok_or("agent setup target has no parent")?;
    let mut current = workspace.to_owned();
    for component in parent.strip_prefix(workspace)?.components() {
        let Component::Normal(name) = component else {
            return Err("invalid agent setup target".into());
        };
        current.push(name);
        if let Ok(metadata) = fs::symlink_metadata(&current) {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err("agent setup directory must not be a symlink".into());
            }
        } else if create {
            fs::create_dir(&current)?;
        }
    }
    Ok(())
}

fn write_file(path: &Path, bytes: &[u8], permissions: Option<Permissions>) -> AuthorityResult<()> {
    let parent = path.parent().ok_or("agent setup file has no parent")?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    if let Some(permissions) = permissions {
        temporary.as_file().set_permissions(permissions)?;
    }
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    sync_dir(parent)
}
