use super::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::Write as _};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Journal {
    pub version: u32,
    pub host: String,
    pub principal: m::PrincipalId,
    pub home: m::RepositoryId,
    pub generation: m::RequestId,
    pub counter: u64,
    pub cursors: BTreeMap<m::RepositoryId, Cursor>,
    pub pending: Option<w::Request>,
    pub active: Option<Active>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Cursor {
    pub epoch: m::EpochId,
    pub after: m::WalPosition,
    #[serde(default)]
    pub unavailable: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Active {
    pub run: w::RunId,
    pub attempt: w::ExecutionAttemptId,
    pub sequence: u64,
}
impl Host {
    pub(super) fn save(&self) -> Result<()> {
        let bytes = serde_json::to_vec(&self.journal)?;
        if bytes.len() > 8 * 1024 * 1024 {
            return Err("Workflow journal exceeds limit".into());
        }
        let mut file = tempfile::NamedTempFile::new_in(&self.config.state_dir)?;
        file.write_all(&bytes)?;
        file.as_file().sync_all()?;
        file.persist(self.config.state_dir.join("journal.json"))?;
        #[cfg(unix)]
        std::fs::File::open(&self.config.state_dir)?.sync_all()?;
        Ok(())
    }
    pub(super) fn request(&mut self, command: w::Command) -> Result<w::Request> {
        if self.journal.pending.is_some() {
            return Err("Workflow receipt still pending".into());
        }
        self.journal.counter = self
            .journal
            .counter
            .checked_add(1)
            .ok_or("Workflow request counter exhausted")?;
        let id = m::RequestId::new(w::identity(
            b"host-request",
            &[
                self.config.host.as_bytes(),
                self.config.principal.as_bytes(),
                self.journal.home.as_bytes(),
                self.journal.generation.as_bytes(),
                &self.journal.counter.to_be_bytes(),
            ],
        ));
        let request = w::Request {
            version: 1,
            request_id: id,
            command,
        };
        self.journal.pending = Some(request.clone());
        self.save()?;
        Ok(request)
    }
}
