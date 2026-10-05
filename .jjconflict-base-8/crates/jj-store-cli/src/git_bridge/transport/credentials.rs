//! Explicit device environment approvals; no checkout or replicated config.
use super::*;
fn approved_file(value: std::ffi::OsString, program: bool) -> Result<std::path::PathBuf> {
    let path = std::path::PathBuf::from(value);
    if !path.is_absolute() {
        return Err("Git approval path must be absolute".into());
    }
    let path = path.canonicalize()?;
    let metadata = std::fs::metadata(&path)?;
    if !metadata.is_file() || (!program && metadata.len() > 1024 * 1024) {
        return Err("Git approval file type/limit".into());
    }
    #[cfg(unix)]
    if program {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o111 == 0 {
            return Err("Git approved program is not executable".into());
        }
    }
    Ok(path)
}
pub(super) fn apply(command: &mut Command) -> Result<()> {
    for key in [
        "JJ_STORE_TOKEN",
        "JJ_STORE_OPERATOR_TOKEN",
        "JJ_STORE_S3_ACCESS_KEY",
        "JJ_STORE_S3_SECRET_KEY",
    ] {
        command.env_remove(key);
    }
    for key in ["GIT_ASKPASS", "SSH_ASKPASS", "GIT_SSH"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, approved_file(value, true)?);
        }
    }
    command.env("GIT_SSH_VARIANT", "ssh");
    if let Some(value) = std::env::var_os("GIT_SSL_CAINFO") {
        command.env("GIT_SSL_CAINFO", approved_file(value, false)?);
    }
    let helper = match std::env::var("JJ_STORE_GIT_CREDENTIAL_HELPER") {
        Ok(value) => value,
        Err(std::env::VarError::NotPresent) => if cfg!(target_os = "macos") {
            "osxkeychain"
        } else {
            "none"
        }
        .into(),
        Err(error) => return Err(error.into()),
    };
    if !matches!(
        helper.as_str(),
        "none" | "osxkeychain" | "manager" | "libsecret"
    ) {
        return Err("Git credential helper approval must name a supported OS helper".into());
    }
    command.args([
        "-c",
        &format!(
            "credential.helper={}",
            if helper == "none" { "" } else { &helper }
        ),
        "-c",
        "http.sslVerify=true",
    ]);
    Ok(())
}
