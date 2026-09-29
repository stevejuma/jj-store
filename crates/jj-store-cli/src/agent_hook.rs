//! Bounded hook ingestion that durably orders only sanitized observations.
use crate::{
    agent_capture::AgentCaptureSpool,
    agent_capture_files::{restrict_directory, sync_dir},
    agent_integrations::{
        AgentDeviceStatusV1, HarnessSupportV1, IntegrationSetupV1, IntegrationStateV1,
    },
    agent_sanitization::{
        KINGFISHER_POLICY, MAX_HOOK_BYTES, SanitizedHookPayload, kingfisher_cache_ready,
        sanitize_hook_payload,
    },
    agent_setup::installation_state,
};
use jj_agent::{
    CaptureBatchId, CaptureCursorV1, IntegrationId, NormalizerProfileV1, ObservationId,
    SanitizedMediaType, SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourcePositionV1, SourceTrust,
};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const MAX_SPOOL_BYTES: u64 = 64 * 1024 * 1024;
const LOCK_WAIT: Duration = Duration::from_millis(750);
const PUBLISH_WAIT: Duration = Duration::from_secs(3);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HookCaptureAck {
    pub batch_id: CaptureBatchId,
    pub sequence: u64,
    pub omitted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HookSourceStateV1 {
    schema_version: u32,
    integration: IntegrationId,
    source_instance_id: SourceInstanceId,
    generation: u64,
    last_sequence: u64,
}

fn capture_spool_path(state: &Path) -> AuthorityResult<PathBuf> {
    Ok(jj_store_server::policy::outside_checkout(state)?.join("capture/spool"))
}

pub(crate) async fn publish_pending_best_effort(
    state: &Path,
    connection: &crate::binding::Connection,
) {
    let publish = async {
        let client = connection.connect().await?;
        let mut spool = AgentCaptureSpool::open(&capture_spool_path(state)?, MAX_SPOOL_BYTES)?;
        crate::agent_publication::drain_capture_spool(&mut spool, connection.repository, &client)
            .await?;
        AuthorityResult::Ok(())
    };
    let _ = tokio::time::timeout(PUBLISH_WAIT, publish).await;
}

pub fn ingest_stdin(
    workspace: &Path,
    state: &Path,
    repository: RepositoryId,
    integration: IntegrationId,
) -> AuthorityResult<HookCaptureAck> {
    let mut raw = Vec::new();
    std::io::stdin()
        .lock()
        .take(MAX_HOOK_BYTES as u64 + 1)
        .read_to_end(&mut raw)?;
    ingest_hook(&raw, workspace, state, repository, integration)
}

pub fn ingest_hook(
    raw: &[u8],
    workspace: &Path,
    state: &Path,
    repository: RepositoryId,
    integration: IntegrationId,
) -> AuthorityResult<HookCaptureAck> {
    let state = jj_store_server::policy::outside_checkout(state)?;
    let status = load_status(&state, workspace, repository)?;
    verify_installed(&status, &state, integration)?;
    let cache = state.join("kingfisher-cache");
    if !kingfisher_cache_ready(&cache) {
        return Err("Kingfisher cache is unavailable; run agent setup again".into());
    }
    let sanitized = sanitize_hook_payload(raw, workspace, &cache);
    let (payload, omitted) = match sanitized {
        SanitizedHookPayload::Sanitized { payload, .. } => (payload, false),
        SanitizedHookPayload::Omitted => (
            SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"status":"omitted"}"#)?,
            true,
        ),
    };

    let capture = private_directory(&state.join("capture"))?;
    let _lock = bounded_lock(&capture.join("hook.lock"))?;
    let sources = private_directory(&capture.join("sources"))?;
    let path = sources.join(format!("{}.json", integration_name(integration)?));
    let mut source = load_source(&path, integration)?;
    source.last_sequence = source
        .last_sequence
        .checked_add(1)
        .ok_or("agent hook source sequence exhausted")?;
    persist_source(&sources, &path, &source)?;

    let sequence = source.last_sequence;
    let occurred_at_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "system clock precedes the Unix epoch")?
        .as_millis()
        .try_into()
        .map_err(|_| "system clock exceeds the capture timestamp range")?;
    let batch_id = CaptureBatchId::new(derived_id(b"batch", &source, sequence));
    let observation_id = ObservationId::new(derived_id(b"observation", &source, sequence));
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id,
        integration,
        source_instance_id: source.source_instance_id,
        sanitizer_version: KINGFISHER_POLICY.into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id,
            source_sequence: sequence,
            occurred_at_millis: Some(occurred_at_millis),
            source_kind: source_kind(integration),
            source_trust: source_trust(integration),
            payload,
        }],
    };
    let cursor = CaptureCursorV1 {
        schema_version: 1,
        integration,
        source_instance_id: source.source_instance_id,
        generation: source.generation,
        parser_version: profile(integration).identity(),
        native_session_key: None,
        position: SourcePositionV1::NativeSequence { sequence },
    };
    AgentCaptureSpool::open(&capture_spool_path(&state)?, MAX_SPOOL_BYTES)?.enqueue(
        repository,
        profile(integration),
        cursor,
        batch,
    )?;
    Ok(HookCaptureAck {
        batch_id,
        sequence,
        omitted,
    })
}

fn load_status(
    state: &Path,
    workspace: &Path,
    repository: RepositoryId,
) -> AuthorityResult<AgentDeviceStatusV1> {
    let status: AgentDeviceStatusV1 = serde_json::from_slice(
        &jj_store_server::policy::read_regular(&state.join("registry.json"), 1024 * 1024)?,
    )
    .map_err(|_| "invalid agent device registry")?;
    status.validate()?;
    if status.repository != repository || status.workspace != workspace.canonicalize()? {
        return Err("agent registry belongs to another repository or workspace".into());
    }
    Ok(status)
}

fn verify_installed(
    status: &AgentDeviceStatusV1,
    state: &Path,
    integration: IntegrationId,
) -> AuthorityResult<()> {
    let value = status
        .integration(integration)
        .ok_or("unsupported agent integration")?;
    if value.support != HarnessSupportV1::Supported
        || value.state != IntegrationStateV1::Enabled
        || value.setup != IntegrationSetupV1::Installed
        || installation_state(status, state, integration)? != IntegrationSetupV1::Installed
    {
        return Err("agent integration is not enabled and installed".into());
    }
    Ok(())
}

fn private_directory(path: &Path) -> AuthorityResult<PathBuf> {
    if fs::symlink_metadata(path)
        .is_ok_and(|value| !value.is_dir() || value.file_type().is_symlink())
    {
        return Err("agent hook state must be a directory".into());
    }
    fs::create_dir_all(path)?;
    restrict_directory(path)?;
    Ok(path.to_owned())
}

fn bounded_lock(path: &Path) -> AuthorityResult<File> {
    if fs::symlink_metadata(path)
        .is_ok_and(|value| !value.is_file() || value.file_type().is_symlink())
    {
        return Err("agent hook lock must be a regular file".into());
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    let deadline = Instant::now() + LOCK_WAIT;
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(std::fs::TryLockError::WouldBlock) => {
                if Instant::now() >= deadline {
                    return Err("agent hook state is busy".into());
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(std::fs::TryLockError::Error(_)) => {
                return Err("agent hook state lock failed".into());
            }
        }
    }
}

fn load_source(path: &Path, integration: IntegrationId) -> AuthorityResult<HookSourceStateV1> {
    if path.exists() {
        let value: HookSourceStateV1 =
            serde_json::from_slice(&jj_store_server::policy::read_regular(path, 64 * 1024)?)
                .map_err(|_| "invalid agent hook source state")?;
        if value.schema_version != 1
            || value.integration != integration
            || value.generation != 1
            || value.source_instance_id == SourceInstanceId::new([0; 16])
        {
            return Err("invalid agent hook source state".into());
        }
        return Ok(value);
    }
    let mut source = [0; 16];
    getrandom::fill(&mut source)?;
    Ok(HookSourceStateV1 {
        schema_version: 1,
        integration,
        source_instance_id: SourceInstanceId::new(source),
        generation: 1,
        last_sequence: 0,
    })
}

fn persist_source(
    directory: &Path,
    path: &Path,
    source: &HookSourceStateV1,
) -> AuthorityResult<()> {
    let mut temporary = tempfile::NamedTempFile::new_in(directory)?;
    temporary.write_all(&serde_json::to_vec(source)?)?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    sync_dir(directory)
}

fn derived_id(label: &[u8], source: &HookSourceStateV1, sequence: u64) -> [u8; 16] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/agent-hook-id/v1\0");
    hash.update(label);
    hash.update(source.source_instance_id.as_bytes());
    hash.update(source.generation.to_be_bytes());
    hash.update(sequence.to_be_bytes());
    hash.finalize()[..16]
        .try_into()
        .expect("fixed SHA-256 prefix")
}

fn profile(integration: IntegrationId) -> NormalizerProfileV1 {
    match integration {
        IntegrationId::Codex => NormalizerProfileV1::codex_0_156_1(),
        IntegrationId::Cursor => NormalizerProfileV1::cursor_3_22_7(),
        IntegrationId::OpenCode => NormalizerProfileV1::opencode_1_18_30(),
        IntegrationId::OwnedController => unreachable!("not a hook integration"),
    }
}

fn integration_name(integration: IntegrationId) -> AuthorityResult<&'static str> {
    match integration {
        IntegrationId::Codex => Ok("codex"),
        IntegrationId::Cursor => Ok("cursor"),
        IntegrationId::OpenCode => Ok("opencode"),
        IntegrationId::OwnedController => Err("owned controller has no hook integration".into()),
    }
}

fn source_kind(integration: IntegrationId) -> SourceKind {
    match integration {
        IntegrationId::OpenCode => SourceKind::Plugin,
        _ => SourceKind::Hook,
    }
}

fn source_trust(integration: IntegrationId) -> SourceTrust {
    match integration {
        IntegrationId::OpenCode => SourceTrust::LocallyTrustedPlugin,
        _ => SourceTrust::LocalUserHook,
    }
}
