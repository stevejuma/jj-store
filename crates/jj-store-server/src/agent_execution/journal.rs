//! Device journal: atomic private writes, bounded state and no prompt replay.
use super::*;
use serde::{Deserialize, Serialize};
use std::io::{Read as _, Write as _};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SavedSession {
    pub view: Session,
    pub workspace: Workspace,
    pub profile: String,
    pub instructions: String,
    pub context: Option<String>,
    pub events: Vec<Event>,
    #[serde(default)]
    pub workspace_ready: bool,
    #[serde(default)]
    pub pending_capture_end: Option<u64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Intent {
    pub digest: String,
    pub outcome: Outcome,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct State {
    pub owner: Option<PrincipalId>,
    pub revision: u64,
    #[serde(default)]
    pub setup_revision: Option<u64>,
    pub setups: BTreeMap<String, Setup>,
    pub sessions: BTreeMap<String, SavedSession>,
    pub intents: BTreeMap<RequestId, Intent>,
}
pub(super) fn read(root: &Path) -> Result<State> {
    let path = root.join("state.json");
    if !path.exists() {
        return Ok(State::default());
    }
    let metadata = fs::symlink_metadata(&path).map_err(|_| Error::Unavailable)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > MAX_BYTES as u64
    {
        return Err(Error::Unavailable);
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|_| Error::Unavailable)?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Unavailable)?;
    if bytes.len() > MAX_BYTES {
        return Err(Error::Limit);
    }
    let mut state: State = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
    if state.sessions.len() > 64 || state.setups.len() > 64 || state.intents.len() > 4096 {
        return Err(Error::Limit);
    }
    for saved in state.sessions.values_mut() {
        let expected = root
            .join("workspaces")
            .join(format!("agent-{}", saved.view.id));
        if saved.view.id.parse::<RequestId>().is_err()
            || saved.workspace.path != expected
            || saved.workspace.name != format!("agent-{}", saved.view.id)
            || saved.view.workspace != expected.to_string_lossy()
            || !matches!(saved.profile.as_str(), "native" | "local_jj_git")
            || saved.events.len() > 8192
            || saved.view.last_sequence != saved.events.len() as u64
            || saved.view.published_sequence > saved.view.last_sequence
            || saved.pending_capture_end.is_some_and(|end| {
                end <= saved.view.published_sequence || end > saved.view.last_sequence
            })
            || saved
                .events
                .iter()
                .enumerate()
                .any(|(i, e)| e.sequence != i as u64 + 1)
        {
            return Err(Error::Unavailable);
        }
        if !matches!(
            saved.view.state.as_str(),
            "ended" | "cleaned" | "interrupted"
        ) {
            saved.view.state = "interrupted".into();
            saved.view.last_error = Some("process_interrupted_no_replay".into());
        }
    }
    for intent in state.intents.values_mut() {
        if matches!(
            intent.outcome.state.as_str(),
            "dispatching" | "queued" | "started"
        ) {
            intent.outcome.state = "outcome_unknown".into();
            intent.outcome.error = Some("process_interrupted_no_replay".into());
        }
    }
    Ok(state)
}
const MAX_BYTES: usize = 32 * 1024 * 1024;
pub(super) fn persist(root: &Path, state: &State) -> Result<()> {
    let bytes = serde_json::to_vec(state).map_err(|_| Error::Unavailable)?;
    if bytes.len() > MAX_BYTES {
        return Err(Error::Limit);
    }
    let mut temporary = tempfile::NamedTempFile::new_in(root).map_err(|_| Error::Unavailable)?;
    temporary
        .write_all(&bytes)
        .map_err(|_| Error::Unavailable)?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|_| Error::Unavailable)?;
    temporary
        .persist(root.join("state.json"))
        .map_err(|_| Error::Unavailable)?;
    fs::File::open(root)
        .and_then(|f| f.sync_all())
        .map_err(|_| Error::Unavailable)
}
pub(super) fn private_directory(path: &Path) -> Result<()> {
    if fs::symlink_metadata(path).is_ok_and(|m| !m.is_dir() || m.file_type().is_symlink()) {
        return Err(Error::Unavailable);
    }
    fs::create_dir_all(path).map_err(|_| Error::Unavailable)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| Error::Unavailable)?;
    }
    Ok(())
}
pub(super) fn digest(value: &impl Serialize) -> Result<String> {
    use sha2::{Digest as _, Sha256};
    Ok(hex::encode(Sha256::digest(
        serde_json::to_vec(value).map_err(|_| Error::Invalid)?,
    )))
}
