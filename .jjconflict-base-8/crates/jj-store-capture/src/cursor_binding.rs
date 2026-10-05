//! Explicit hook opt-in metadata beside a harness transcript. No bodies or credentials.
use crate::{CaptureResult, profile::CaptureProfile};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
const LIMIT: u64 = 32 * 1024;
mod pending;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CursorBinding {
    pub schema_version: u32,
    pub profile: CaptureProfile,
    pub product_version: String,
    pub session: String,
    pub directory: PathBuf,
    pub transcript: PathBuf,
    pub active: bool,
    pub observed_at_millis: u64,
    #[serde(default)]
    pub last_active_at_millis: Option<u64>,
}
fn safe(path: &Path) -> bool {
    path.is_absolute()
        && path.parent().is_some()
        && path
            .to_str()
            .is_some_and(|s| s.len() <= 4096 && !s.chars().any(char::is_control))
        && !path
            .components()
            .any(|c| matches!(c, Component::CurDir | Component::ParentDir))
}
impl CursorBinding {
    fn validate(&self, profile: CaptureProfile, path: &Path) -> CaptureResult<()> {
        if self.schema_version != 1
            || !profile.is_cursor()
            || self.profile != profile
            || self.product_version != profile.product_version()
            || !crate::codex_transcript::native_session_key_valid(&self.session)
            || !safe(&self.directory)
            || !safe(&self.transcript)
            || self.transcript != path
            || path.extension().is_none_or(|e| e != "jsonl")
        {
            return Err("unsupported Cursor transcript binding".into());
        }
        Ok(())
    }
    pub fn witness(&self) -> [u8; 32] {
        let mut h = Sha256::new_with_prefix(b"jj-store/cursor-hook-binding/v1\0");
        for s in [
            self.product_version.as_bytes(),
            self.session.as_bytes(),
            self.directory.as_os_str().as_encoded_bytes(),
            self.transcript.as_os_str().as_encoded_bytes(),
        ] {
            h.update((s.len() as u64).to_be_bytes());
            h.update(s);
        }
        h.finalize().into()
    }
    /// Hook receipt time grants eligibility, never a fabricated message timestamp.
    pub fn fresh_activity(&self) -> bool {
        let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return false;
        };
        let active_at = self
            .last_active_at_millis
            .or_else(|| self.active.then_some(self.observed_at_millis));
        active_at.is_some_and(|time| {
            now.as_millis()
                .checked_sub(time as u128)
                .is_some_and(|age| age <= 60_000)
        })
    }
}
fn binding_path(path: &Path) -> PathBuf {
    path.with_extension("jj-store.json")
}
pub fn read(profile: CaptureProfile, path: &Path) -> CaptureResult<CursorBinding> {
    let b: CursorBinding = serde_json::from_slice(&private_bytes(&binding_path(path))?)
        .map_err(|_| "Cursor binding malformed")?;
    b.validate(profile, path)?;
    Ok(b)
}
fn private_bytes(sidecar: &Path) -> CaptureResult<Vec<u8>> {
    let m = fs::symlink_metadata(sidecar)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > LIMIT {
        return Err("Cursor binding unavailable".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if m.permissions().mode() & 0o077 != 0 {
            return Err("Cursor binding must be private".into());
        }
    }
    let f = File::open(sidecar)?;
    let opened = f.metadata()?;
    if crate::codex_discovery::PhysicalId::from(&m)?
        != crate::codex_discovery::PhysicalId::from(&opened)?
        || opened.len() != m.len()
        || opened.modified()? != m.modified()?
    {
        return Err("Cursor binding changed during open".into());
    }
    let mut bytes = Vec::new();
    f.take(LIMIT + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > LIMIT {
        return Err("Cursor binding exceeds bounds".into());
    }
    let after = fs::symlink_metadata(sidecar)?;
    if after.file_type().is_symlink()
        || crate::codex_discovery::PhysicalId::from(&after)?
            != crate::codex_discovery::PhysicalId::from(&opened)?
        || after.modified()? != opened.modified()?
        || after.len() != opened.len()
    {
        return Err("Cursor binding changed during read".into());
    }
    Ok(bytes)
}
fn replace_private(sidecar: &Path, bytes: &[u8]) -> CaptureResult<()> {
    let parent = sidecar.parent().ok_or("Cursor binding parent")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(bytes)?;
    temp.as_file().sync_all()?;
    temp.persist(sidecar)
        .map_err(|_| "Cursor binding replacement unavailable")?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
/// Explicitly installed hooks hand off nullable paths via bounded metadata only.
pub fn write_hook(raw: &[u8], workspace: &Path, roots: &[PathBuf]) -> CaptureResult<bool> {
    if raw.len() > 1024 * 1024 {
        return Err("Cursor hook exceeds bounds".into());
    }
    let v: serde_json::Value = serde_json::from_slice(raw).map_err(|_| "Cursor hook malformed")?;
    let profile = match v["cursor_version"].as_str() {
        Some("3.22.7") => CaptureProfile::CursorDesktopTranscriptV1,
        Some("2026.09.26-dd393fe") => CaptureProfile::CursorCliTranscriptV1,
        _ => return Err("Cursor hook product version unsupported".into()),
    };
    let native_roots = v["workspace_roots"]
        .as_array()
        .ok_or("Cursor workspace roots missing")?;
    if native_roots.len() != 1 || native_roots[0].as_str() != workspace.to_str() {
        return Err("Cursor hook workspace mismatch".into());
    }
    let session = v["conversation_id"]
        .as_str()
        .ok_or("Cursor session missing")?;
    let event = v["hook_event_name"]
        .as_str()
        .ok_or("Cursor lifecycle missing")?;
    let active = match event {
        "sessionStart" | "beforeSubmitPrompt" | "preToolUse" | "postToolUse"
        | "postToolUseFailure" | "afterFileEdit" | "afterAgentResponse" => true,
        "stop" | "sessionEnd" => false,
        _ => return Ok(false),
    };
    if !crate::codex_transcript::native_session_key_valid(session) || !safe(workspace) {
        return Err("Cursor hook identity unavailable".into());
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .try_into()?;
    let Some(path) = v["transcript_path"].as_str().map(PathBuf::from) else {
        if active {
            pending::remember(profile, session, workspace, roots, now)?;
        }
        return Ok(false);
    };
    if path.extension().is_none_or(|e| e != "jsonl") {
        return Ok(false);
    }
    if !safe(&path)
        || !roots
            .iter()
            .any(|r| safe(r) && path.starts_with(r) && r.canonicalize().is_ok_and(|c| c == *r))
    {
        return Err("Cursor transcript outside approved hook roots".into());
    }
    if active && !path.try_exists()? {
        pending::remember(profile, session, workspace, roots, now)?;
        return Ok(false);
    }
    if path.canonicalize()? != path || !fs::symlink_metadata(&path)?.is_file() {
        return Err("Cursor transcript outside approved hook roots".into());
    }
    let mut b = CursorBinding {
        schema_version: 1,
        profile,
        product_version: profile.product_version().into(),
        session: session.into(),
        directory: workspace.to_owned(),
        transcript: path.clone(),
        active,
        observed_at_millis: now,
        last_active_at_millis: active.then_some(now).or_else(|| {
            roots.iter().find_map(|root| {
                path.starts_with(root)
                    .then(|| pending::completed_activity(profile, session, workspace, root, now))
                    .flatten()
            })
        }),
    };
    b.validate(profile, &path)?;
    let sidecar = binding_path(&path);
    if sidecar.try_exists()? {
        let previous = read(profile, &path)?;
        if !active && previous.witness() == b.witness() {
            b.last_active_at_millis = b.last_active_at_millis.or(previous
                .last_active_at_millis
                .or_else(|| previous.active.then_some(previous.observed_at_millis)));
        }
    }
    replace_private(&sidecar, &serde_json::to_vec(&b)?)?;
    if !active {
        pending::finish(profile, session, workspace, roots);
    }
    Ok(true)
}
