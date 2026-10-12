use super::*;
use jj_store_model::{PrincipalId, RequestId};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Record {
    actor: PrincipalId,
    command: Command,
    outcome: Outcome,
}
#[derive(Serialize, Deserialize)]
pub(super) struct Journal {
    #[serde(skip)]
    path: PathBuf,
    pub generation: u64,
    records: Vec<Record>,
}
impl Journal {
    pub fn open(root: &Path) -> crate::Result<Self> {
        let path = root.join("configuration-reloads.json");
        let mut journal = if path.exists() {
            serde_json::from_slice::<Self>(&policy::read_regular(&path, 8 * 1024 * 1024)?)
                .map_err(|_| crate::Error::Config("reload journal"))?
        } else {
            Self {
                path: path.clone(),
                generation: 0,
                records: vec![],
            }
        };
        if journal.records.len() > 4096 {
            return Err(crate::Error::Config("reload receipt bound"));
        }
        journal.path = path;
        for record in &mut journal.records {
            if record.outcome.state == State::Running {
                record.outcome.state = State::Unknown;
                for result in &mut record.outcome.results {
                    if result.state == State::Running {
                        result.state = State::Unknown;
                    }
                }
            }
        }
        Ok(journal)
    }
    fn persist(&self) -> Result<(), Error> {
        use std::io::Write;
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Unavailable)?;
        if bytes.len() > 8 * 1024 * 1024 {
            return Err(Error::Unavailable);
        }
        let parent = self.path.parent().ok_or(Error::Unavailable)?;
        let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        file.write_all(&bytes)
            .and_then(|_| file.as_file().sync_all())
            .map_err(|_| Error::Unavailable)?;
        file.persist(&self.path).map_err(|_| Error::Unknown)?;
        std::fs::File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)
    }
    pub fn find(&self, actor: PrincipalId, command: &Command) -> Result<Option<Outcome>, Error> {
        if let Some(r) = self
            .records
            .iter()
            .find(|r| r.actor == actor && r.command.request_id == command.request_id)
        {
            if r.command != *command {
                return Err(Error::Conflict);
            }
            return Ok(Some(r.outcome.clone()));
        }
        Ok(None)
    }
    pub fn active(&self, actor: PrincipalId) -> Option<(Command, Outcome)> {
        self.records
            .iter()
            .rev()
            .find(|r| {
                r.actor == actor && matches!(r.outcome.state, State::Running | State::Unknown)
            })
            .map(|r| (r.command.clone(), r.outcome.clone()))
    }
    pub fn observed(&mut self, actor: PrincipalId, outcome: Outcome) -> Result<(), Error> {
        let r = self
            .records
            .iter_mut()
            .find(|r| r.actor == actor && r.command.request_id == outcome.request_id)
            .ok_or(Error::Unavailable)?;
        r.outcome = outcome;
        self.persist()
    }
    pub fn begin(&mut self, actor: PrincipalId, command: Command) -> Result<Outcome, Error> {
        // Never evict unknown/running identities. Completed receipts have an explicit bound.
        if self
            .records
            .iter()
            .any(|r| matches!(r.outcome.state, State::Running | State::Unknown))
        {
            return Err(Error::Busy);
        }
        if self.records.len() >= 4096 {
            return Err(Error::Busy);
        }
        self.generation = self.generation.checked_add(1).ok_or(Error::Unavailable)?;
        let outcome = Outcome {
            request_id: command.request_id,
            generation: self.generation,
            state: State::Running,
            results: command
                .domains
                .iter()
                .map(|d| DomainResult {
                    domain: *d,
                    state: State::Running,
                    reason: None,
                })
                .collect(),
        };
        self.records.push(Record {
            actor,
            command,
            outcome: outcome.clone(),
        });
        if let Err(error) = self.persist() {
            let r = self.records.last_mut().ok_or(Error::Unavailable)?;
            r.outcome.state = State::Unknown;
            for result in &mut r.outcome.results {
                result.state = State::Unknown;
            }
            return Err(error);
        }
        Ok(outcome)
    }
    pub fn progress(
        &mut self,
        actor: PrincipalId,
        id: RequestId,
        domain: Domain,
        state: State,
        reason: Option<String>,
    ) -> Result<(), Error> {
        let r = self
            .records
            .iter_mut()
            .find(|r| r.actor == actor && r.command.request_id == id)
            .ok_or(Error::Unavailable)?;
        let result = r
            .outcome
            .results
            .iter_mut()
            .find(|r| r.domain == domain)
            .ok_or(Error::Invalid)?;
        result.state = state;
        result.reason = reason;
        self.persist()
    }
    pub fn finish(&mut self, actor: PrincipalId, id: RequestId) -> Result<(), Error> {
        let r = self
            .records
            .iter_mut()
            .find(|r| r.actor == actor && r.command.request_id == id)
            .ok_or(Error::Unavailable)?;
        r.outcome.state = if r.outcome.results.iter().any(|r| r.state == State::Running) {
            State::Unknown
        } else if r.outcome.results.iter().any(|r| r.state == State::Failed) {
            State::Failed
        } else if r.outcome.results.iter().any(|r| r.state == State::Deferred) {
            State::Deferred
        } else {
            State::Applied
        };
        self.persist()
    }
}
