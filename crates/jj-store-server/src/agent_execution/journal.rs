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
    #[serde(default)]
    pub options: settings::RuntimeOptions,
    pub context: Option<String>,
    #[serde(default)]
    pub continuation: String,
    #[serde(default)]
    pub delivered_sections: BTreeMap<String, String>,
    #[serde(default)]
    pub coordination: Option<Workspace>,
    pub events: Vec<Event>,
    #[serde(default)]
    pub workspace_ready: bool,
    #[serde(default)]
    pub pending_capture_end: Option<u64>,
    #[serde(default)]
    pub delivered_replies: std::collections::BTreeSet<context::ReplyRevision>,
    #[serde(default)]
    pub context_replies: std::collections::BTreeSet<context::ReplyRevision>,
    #[serde(default)]
    pub context_generation: u64,
    #[serde(default)]
    pub initial_context_delivered: bool,
    #[serde(default)]
    pub code_workspaces: BTreeMap<RepositoryId, Workspace>,
    #[serde(default)]
    pub origin_link: Option<jj_store_service::collab_mutations::Request>,
    #[serde(default)]
    pub origin_linked: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Intent {
    pub digest: String,
    pub outcome: Outcome,
    #[serde(default)]
    pub prompt: Option<super::delivery::PromptDelivery>,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct State {
    #[serde(default)]
    pub chatgpt_accounts: BTreeMap<String, super::chatgpt::Record>,
    #[serde(default)]
    pub chatgpt_host: Option<String>,
    #[serde(default)]
    pub chatgpt_notices: BTreeMap<String, jj_store_service::agent_execution::chatgpt::Attempt>,
    #[serde(default)]
    pub chatgpt_requests:
        BTreeMap<RequestId, (String, jj_store_service::agent_execution::chatgpt::Attempt)>,
    #[serde(default)]
    pub settings: settings::Settings,
    #[serde(default)]
    pub metadata_jobs: BTreeMap<RequestId, super::metadata_jobs::SavedJob>,
    #[serde(default)]
    pub compute: jj_store_service::agent_execution::compute::Settings,
    #[serde(default)]
    pub compute_models: std::collections::BTreeSet<String>,
    #[serde(default)]
    pub compute_runtime_prepared: bool,
    #[serde(default)]
    pub compute_peer_port: Option<u16>,
    #[serde(default)]
    pub compute_generation: u64,
    #[serde(default)]
    pub compute_identity: Option<String>,
    pub owner: Option<PrincipalId>,
    pub revision: u64,
    #[serde(default)]
    pub setup_revision: Option<u64>,
    pub setups: BTreeMap<String, Setup>,
    pub sessions: BTreeMap<String, SavedSession>,
    pub intents: BTreeMap<RequestId, Intent>,
    #[serde(default)]
    pub runtime_installs: BTreeMap<RequestId, super::tooling::InstallIntent>,
    #[serde(default)]
    pub runtime_selection: Option<super::tooling::Selection>,
    #[serde(default)]
    pub worker_generations: BTreeMap<String, u64>,
    #[serde(default)]
    pub warm_agents: std::collections::BTreeSet<String>,
    #[serde(default)]
    pub conversations:
        BTreeMap<String, jj_store_service::agent_execution::conversations::Conversation>,
    #[serde(default)]
    pub personal_home: Option<super::conversation_home::Home>,
    #[serde(default)]
    pub comment_submissions: BTreeMap<RequestId, super::comment_submissions::SavedSubmission>,
    #[serde(default)]
    pub workspace_submissions: BTreeMap<RequestId, super::workspace_freeze::WorkspaceSubmission>,
    #[serde(default)]
    pub file_publications: BTreeMap<RequestId, super::workspace_publication::FilePublication>,
    #[serde(default)]
    pub deferred_file_prompts: BTreeMap<RequestId, super::workspace_deferred::DeferredFilePrompt>,
    #[serde(default)]
    pub answer_links: BTreeMap<String, super::comment_answers::SavedAnswerLink>,
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
    if state.chatgpt_accounts.len() > 64
        || state.chatgpt_requests.len() > 4096
        || state.chatgpt_notices.len() > 64
        || state.workspace_submissions.len() > 4096
        || state.file_publications.len() > 4096
        || state.deferred_file_prompts.len() > 4096
        || state.runtime_installs.len() > 128
        || state.comment_submissions.len() > 4096
        || state.answer_links.len() > 4096
    {
        return Err(Error::Limit);
    }
    if let Some(selected) = &state.runtime_selection
        && !state
            .runtime_installs
            .get(&selected.request_id)
            .is_some_and(|i| i.outcome.state == "completed")
    {
        return Err(Error::Unavailable);
    }
    super::tooling::recover(&mut state);
    // Opening retained history never initiates discovery inference.
    for conversation in state.conversations.values_mut() {
        conversation.metadata_auto_eligible = false;
    }
    if state.metadata_jobs.len() > 4096 {
        return Err(Error::Limit);
    }
    for job in state.metadata_jobs.values_mut() {
        if matches!(
            job.view.state.as_str(),
            "prepared" | "generating" | "publishing"
        ) {
            job.view.state = "outcome_unknown".into();
            job.view.error = Some("interrupted_metadata_request_not_replayed".into());
        }
    }

    if state.sessions.len() > 64 || state.setups.len() > 64 || state.intents.len() > 4096 {
        return Err(Error::Limit);
    }
    if state.warm_agents.len() > 64
        || state.worker_generations.len() > 64
        || state
            .worker_generations
            .iter()
            .any(|(id, generation)| !label(id, 64) || *generation == 0)
        || state
            .warm_agents
            .iter()
            .any(|id| !state.setups.contains_key(id))
        || state.conversations.len() > 128
    {
        return Err(Error::Limit);
    }
    for (id, conversation) in &state.conversations {
        if id != &conversation.id
            || id.parse::<RequestId>().is_err()
            || !label(&conversation.origin.scope, 256)
            || conversation.origin.context.len() > 32768
            || conversation.participants.len() > 64
            || conversation.participants.iter().any(|id| {
                !state.sessions.get(id).is_some_and(|s| {
                    s.view.conversation_id.as_deref() == Some(&conversation.id)
                        && s.view.repository == conversation.home
                })
            })
        {
            return Err(Error::Unavailable);
        }
    }
    avatar::budget(&state)?;
    for saved in state.sessions.values_mut() {
        saved.view.connected = false;
        saved.view.recorded_session_ids.clear();
        saved.view.workspace_name = None;
        saved.view.workspace_operation = None;
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
        if saved.code_workspaces.len() > 8
            || saved.code_workspaces.len() != saved.view.repository_bindings.len()
            || saved
                .view
                .conversation_id
                .as_ref()
                .is_some_and(|id| !state.conversations.contains_key(id))
        {
            return Err(Error::Unavailable);
        }
        for (repository, workspace) in &saved.code_workspaces {
            let expected = saved
                .workspace
                .path
                .join("repositories")
                .join(repository.to_string());
            let binding = saved
                .view
                .repository_bindings
                .iter()
                .find(|b| b.repository == *repository)
                .ok_or(Error::Unavailable)?;
            if workspace.path != expected
                || binding.workspace != expected.to_string_lossy()
                || workspace.operation != binding.operation
                || workspace.commit != binding.commit
                || !matches!(binding.profile.as_str(), "native" | "local_jj_git")
            {
                return Err(Error::Unavailable);
            }
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
            "dispatching" | "queued" | "started" | "configuring" | "stopping"
        ) {
            intent.outcome.state = "outcome_unknown".into();
            intent.outcome.error = Some("process_interrupted_no_replay".into());
        }
    }
    let uncertain: Vec<_> = state
        .intents
        .iter()
        .filter(|(_, i)| i.outcome.state == "outcome_unknown")
        .map(|(request, _)| *request)
        .collect();
    for request in uncertain {
        sent_context::outcome(&mut state, request, "outcome_unknown")?;
    }
    Ok(state)
}
const MAX_BYTES: usize = 32 * 1024 * 1024;
pub(super) fn persist(root: &Path, state: &State) -> Result<()> {
    avatar::budget(state)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn journal_restart_retains_agent_association_but_never_connection() {
        let root = tempfile::tempdir().unwrap();
        let id = "44".repeat(16);
        let path = root.path().join("workspaces").join(format!("agent-{id}"));
        let mut value = json!({
            "owner":null,"revision":1,"setups":{},"intents":{},"sessions":{
                id.clone():{
                    "view":{"id":id,"setup_id":"helper","connected":true,
                        "recorded_session_ids":["aa".repeat(16)],"workspace_name":"untrusted-projection",
                        "workspace_operation":"66".repeat(64),
                        "repository":"11".repeat(16),"target_id":"fixture","runner":"codex",
                        "title":"Original name","avatar":"🤖","workspace":path,
                        "base_commit":"22".repeat(20),"review_commit":"22".repeat(20),
                        "operation":"33".repeat(64),"state":"idle","adapter_session_id":"adapter",
                        "capabilities":{},"preferences":{"model":null,"effort":null,"mode":"ask"},
                        "last_sequence":1,"last_activity_at_millis":1791324000000_u64,
                        "published_sequence":0,"last_error":null},
                    "workspace":{"source":root.path().join("source"),"path":path,
                        "metadata":root.path().join("source/.jj"),"operation":"33".repeat(64),
                        "commit":"22".repeat(20),"name":format!("agent-{id}")},
                    "profile":"local_jj_git","instructions":"Original instructions","context":null,"events":[{
                        "sequence":1,"observed_at_millis":1791323999000_u64,
                        "payload":{"type":"prompt","requestId":"turn","text":"Hello"},"committed":false}]
                }
            }
        });
        fs::write(
            root.path().join("state.json"),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
        let state = read(root.path()).unwrap();
        let saved = &state.sessions[&id];
        assert_eq!(saved.view.setup_id.as_deref(), Some("helper"));
        assert!(!saved.view.connected);
        assert!(saved.view.recorded_session_ids.is_empty());
        assert!(saved.view.workspace_name.is_none());
        assert!(saved.view.workspace_operation.is_none());
        assert_eq!(saved.view.last_activity_at_millis, Some(1791324000000));
        assert_eq!(saved.view.state, "interrupted");
        assert_eq!(saved.instructions, "Original instructions");
        assert_eq!(saved.events[0].observed_at_millis, Some(1791323999000));
        let payload = saved.events[0].payload.clone();
        persist(root.path(), &state).unwrap();
        let reopened = read(root.path()).unwrap();
        assert_eq!(
            reopened.sessions[&id].events[0].observed_at_millis,
            Some(1791323999000)
        );
        assert_eq!(reopened.sessions[&id].events[0].payload, payload);
        // The old journal format predates association, connection and timing.
        value["sessions"][&id]["view"]
            .as_object_mut()
            .unwrap()
            .remove("setup_id");
        value["sessions"][&id]["view"]
            .as_object_mut()
            .unwrap()
            .remove("connected");
        value["sessions"][&id]["view"]
            .as_object_mut()
            .unwrap()
            .remove("last_activity_at_millis");
        fs::write(
            root.path().join("state.json"),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
        value["sessions"][&id]["events"][0]
            .as_object_mut()
            .unwrap()
            .remove("observed_at_millis");
        fs::write(
            root.path().join("state.json"),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
        let legacy = read(root.path()).unwrap();
        assert_eq!(legacy.sessions[&id].events[0].observed_at_millis, None);
        assert_eq!(legacy.sessions[&id].events[0].payload, payload);
        assert_eq!(legacy.sessions[&id].view.setup_id, None);
        assert!(!legacy.sessions[&id].view.connected);
        assert_eq!(legacy.sessions[&id].view.last_activity_at_millis, None);
    }
}
