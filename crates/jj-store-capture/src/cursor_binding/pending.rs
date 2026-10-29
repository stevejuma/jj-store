//! Bounded metadata handoff for native hooks whose transcript path is not ready.
use super::*;
const MAX_PENDING: usize = 256;
const MAX_SESSION_AGE: u64 = 24 * 60 * 60 * 1000;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    schema_version: u32,
    profile: CaptureProfile,
    session: String,
    directory: PathBuf,
    started_at_millis: u64,
}
fn key(profile: CaptureProfile, session: &str, workspace: &Path) -> String {
    let mut hash = Sha256::new_with_prefix(b"jj-store/cursor-pending-hook/v1\0");
    for value in [
        profile.product_version().as_bytes(),
        session.as_bytes(),
        workspace.as_os_str().as_encoded_bytes(),
    ] {
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
    }
    format!("{}.json", lower_hex(hash.finalize()))
}
fn lower_hex(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0xf) as usize] as char);
    }
    out
}
fn directory(root: &Path, create: bool) -> CaptureResult<PathBuf> {
    if !safe(root) || root.canonicalize()? != root {
        return Err("Cursor pending root unavailable".into());
    }
    let path = root.join(".jj-store-bindings");
    if create && !path.try_exists()? {
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(&path)?;
    }
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() || path.canonicalize()? != path {
        return Err("Cursor pending directory unavailable".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err("Cursor pending directory must be private".into());
        }
    }
    Ok(path)
}
fn read(path: &Path) -> CaptureResult<Pending> {
    let value: Pending = serde_json::from_slice(&private_bytes(path)?)?;
    if value.schema_version != 1
        || !value.profile.is_cursor()
        || !crate::codex_transcript::native_session_key_valid(&value.session)
        || !safe(&value.directory)
        || path.file_name().is_none_or(|name| {
            name != key(value.profile, &value.session, &value.directory).as_str()
        })
    {
        return Err("Cursor pending identity unavailable".into());
    }
    Ok(value)
}
pub(super) fn remember(
    profile: CaptureProfile,
    session: &str,
    workspace: &Path,
    roots: &[PathBuf],
    now: u64,
) -> CaptureResult<()> {
    for root in roots {
        let directory = directory(root, true)?;
        let path = directory.join(key(profile, session, workspace));
        let mut retained = 0;
        for entry in fs::read_dir(&directory)?.take(MAX_PENDING + 1) {
            let entry = entry?;
            retained += 1;
            if let Ok(previous) = read(&entry.path())
                && now
                    .checked_sub(previous.started_at_millis)
                    .is_some_and(|age| age > MAX_SESSION_AGE)
            {
                fs::remove_file(entry.path())?;
                retained -= 1;
            }
        }
        if retained >= MAX_PENDING && !path.try_exists()? {
            return Err("Cursor pending hook budget exhausted".into());
        }
        if path.try_exists()? {
            read(&path)?;
        }
        replace_private(
            &path,
            &serde_json::to_vec(&Pending {
                schema_version: 1,
                profile,
                session: session.into(),
                directory: workspace.into(),
                started_at_millis: now,
            })?,
        )?;
    }
    Ok(())
}
/// A terminal lifecycle receipt is recent activity only after an exact start witness.
pub(super) fn completed_activity(
    profile: CaptureProfile,
    session: &str,
    workspace: &Path,
    root: &Path,
    now: u64,
) -> Option<u64> {
    let path = directory(root, false)
        .ok()?
        .join(key(profile, session, workspace));
    let previous = read(&path).ok()?;
    (previous.profile == profile
        && previous.session == session
        && previous.directory == workspace
        && now
            .checked_sub(previous.started_at_millis)
            .is_some_and(|age| age <= MAX_SESSION_AGE))
    .then_some(now)
}
pub(super) fn finish(profile: CaptureProfile, session: &str, workspace: &Path, roots: &[PathBuf]) {
    for root in roots {
        if let Ok(directory) = directory(root, false) {
            let path = directory.join(key(profile, session, workspace));
            if read(&path).is_ok() {
                let _ = fs::remove_file(path);
            }
        }
    }
}
