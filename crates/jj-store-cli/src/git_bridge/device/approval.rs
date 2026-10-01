//! Device-local endpoint admission and private-directory checks.
use super::*;
pub fn id() -> Result<m::RequestId> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes)?;
    Ok(m::RequestId::new(bytes))
}
pub(super) fn private(path: &Path) -> Result<()> {
    if path.exists() {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err("Git private directory type".into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
            if metadata.uid() != fs::metadata(dirs::home_dir().ok_or("device home missing")?)?.uid()
                || metadata.permissions().mode() & 0o077 != 0
            {
                return Err("Git private directory ownership or permissions".into());
            }
        }
    } else {
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt as _;
            builder.mode(0o700);
        }
        builder.create(path)?;
    }
    #[cfg(not(unix))]
    {
        return Err("native Git private transport currently requires Unix".into());
    }
    #[allow(unreachable_code)]
    Ok(())
}
pub(super) fn fingerprint(
    repository: m::RepositoryId,
    remote: m::RequestId,
    approval: m::RequestId,
    url: &str,
) -> m::RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/git-device-binding/v1\0");
    hash.update(repository.as_bytes());
    hash.update(remote.as_bytes());
    hash.update(approval.as_bytes());
    hash.update(url.as_bytes());
    m::RecordDigest::new(hash.finalize().into())
}
pub fn approved_url(cwd: &Path, value: &str) -> Result<String> {
    if value.is_empty()
        || value.len() > 4096
        || value.starts_with('-')
        || value.chars().any(char::is_control)
    {
        return Err("invalid Git endpoint".into());
    }
    if let Ok(url) = url::Url::parse(value) {
        if !matches!(url.scheme(), "https" | "ssh")
            || url.host_str().is_none()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || (url.scheme() == "https" && !url.username().is_empty())
            || (url.scheme() == "ssh"
                && (url.username().starts_with('-')
                    || !url
                        .username()
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))))
            || url
                .host_str()
                .is_some_and(|v| v.starts_with('-') || v.chars().any(char::is_control))
        {
            return Err(
                "Git endpoint requires approved HTTPS/SSH without embedded credentials".into(),
            );
        }
        return Ok(url.to_string());
    }
    if value.contains(':') {
        return Err("use an explicit ssh:// Git endpoint".into());
    }
    Ok(cwd
        .join(value)
        .canonicalize()?
        .to_str()
        .ok_or("Git local endpoint encoding")?
        .into())
}
