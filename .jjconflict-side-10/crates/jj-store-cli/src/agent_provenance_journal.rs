//! Private exact-request journal. Selected receipts alone complete publication.
use crate::agent_capture_files::{restrict_directory, sync_dir};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use jj_store_service::local_repository::{
    LocalAgentCommand, LocalAgentOutcome, LocalAgentRequest, LocalRepository,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};
const MAX_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    version: u32,
    repository: RepositoryId,
    actor: PrincipalId,
    intent: jj_store_model::IntentDigest,
    request: LocalAgentRequest,
}
pub(crate) struct ProvenanceJournal {
    path: PathBuf,
    repository: RepositoryId,
    actor: PrincipalId,
    _lock: fs::File,
}
impl ProvenanceJournal {
    pub fn open(
        state: &Path,
        repository: RepositoryId,
        actor: PrincipalId,
    ) -> AuthorityResult<Self> {
        let path = jj_store_server::policy::outside_checkout(&state.join("provenance"))?;
        if fs::symlink_metadata(&path).is_ok_and(|v| v.file_type().is_symlink()) {
            return Err("provenance journal must not be a symlink".into());
        }
        fs::create_dir_all(&path)?;
        restrict_directory(&path)?;
        let lock = jj_store_server::policy::exclusive_lock(&path.join("journal.lock"))?;
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|v| v.starts_with(".tmp"))
            {
                if !entry.file_type()?.is_file() {
                    return Err("unexpected provenance journal temporary entry".into());
                }
                fs::remove_file(entry.path())?;
                sync_dir(&path)?;
            }
        }
        Ok(Self {
            path,
            repository,
            actor,
            _lock: lock,
        })
    }
    fn filename(&self, id: RequestId) -> PathBuf {
        self.path.join(format!("{id}.json"))
    }
    pub fn pending(&self, id: RequestId) -> AuthorityResult<Option<LocalAgentRequest>> {
        let path = self.filename(id);
        if !path.try_exists()? {
            return Ok(None);
        }
        let bytes = jj_store_server::policy::read_regular(&path, MAX_BYTES)?;
        let entry: Entry =
            serde_json::from_slice(&bytes).map_err(|_| "invalid provenance journal")?;
        if entry.version != 1
            || entry.repository != self.repository
            || entry.actor != self.actor
            || entry.request.request_id != id
            || serde_json::to_vec(&entry)? != bytes
            || !matches!(entry.request.command, LocalAgentCommand::Provenance { .. })
        {
            return Err("provenance journal binding or encoding mismatch".into());
        }
        entry.request.validate()?;
        if entry.intent != entry.request.intent_for(entry.actor)? {
            return Err("provenance journal intent mismatch".into());
        }
        Ok(Some(entry.request))
    }
    pub fn prepare(&self, request: &LocalAgentRequest) -> AuthorityResult<()> {
        request.validate()?;
        if let Some(old) = self.pending(request.request_id)? {
            if old != *request {
                return Err("provenance retry differs from original request".into());
            }
            return Ok(());
        }
        let bytes = serde_json::to_vec(&Entry {
            version: 1,
            repository: self.repository,
            actor: self.actor,
            intent: request.intent_for(self.actor)?,
            request: request.clone(),
        })?;
        let mut total = bytes.len() as u64;
        let mut files = 0;
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.file_name() == "journal.lock" {
                continue;
            }
            let metadata = fs::symlink_metadata(entry.path())?;
            if !metadata.is_file()
                || metadata.file_type().is_symlink()
                || entry.path().extension().is_none_or(|v| v != "json")
            {
                return Err("unexpected provenance journal entry".into());
            }
            total = total
                .checked_add(metadata.len())
                .ok_or("provenance journal byte limit")?;
            files += 1;
        }
        if total > MAX_BYTES || files >= 1024 {
            return Err("provenance journal capacity exceeded".into());
        }
        let mut temporary = tempfile::Builder::new()
            .prefix(".tmp")
            .tempfile_in(&self.path)?;
        temporary.write_all(&bytes)?;
        temporary.as_file().sync_all()?;
        temporary
            .persist_noclobber(self.filename(request.request_id))
            .map_err(|v| v.error)?;
        sync_dir(&self.path)
    }
    pub async fn publish(
        &self,
        repository: &LocalRepository,
        request: &LocalAgentRequest,
    ) -> AuthorityResult<LocalAgentOutcome> {
        if repository.repository()? != Some(self.repository)
            || repository.actor()? != Some(self.actor)
        {
            return Err("provenance publication repository binding mismatch".into());
        }
        if self.pending(request.request_id)?.as_ref() != Some(request) {
            return Err("provenance request is not durably prepared".into());
        }
        let outcome = repository.apply_agent_command(request).await?;
        if let LocalAgentOutcome::Committed { receipt } = &outcome {
            if receipt.repository != self.repository
                || receipt.actor != self.actor
                || receipt.request_id != request.request_id
                || receipt.intent != request.intent_for(self.actor)?
                || receipt.batch.is_some()
            {
                return Err("provenance receipt mismatch".into());
            }
            fs::remove_file(self.filename(request.request_id))?;
            sync_dir(&self.path)?;
        }
        Ok(outcome)
    }
}
