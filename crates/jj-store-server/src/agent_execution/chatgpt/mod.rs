//! Device-owned subscription authentication, isolated from ACP and repository authority.
use super::*;
use jj_store_service::agent_execution::chatgpt as contract;
use serde::{Deserialize, Serialize};
mod accounts;
mod attempts;
mod authorization;
pub(super) mod broker;
mod callback;
mod io;
mod oauth;
#[cfg(test)]
mod tests;
mod vault;
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Record {
    pub view: contract::Account,
    pub client_id: String,
    pub subject: String,
    pub generation: u64,
}
pub(super) struct Cached {
    tokens: oauth::Tokens,
    expires: u64,
    generation: u64,
}
pub(super) struct Pending {
    view: contract::Attempt,
    task: Option<tokio::task::AbortHandle>,
}
#[derive(Default)]
pub(super) struct Runtime {
    #[cfg(test)]
    fixture: Mutex<Option<Arc<tests::FixtureIo>>>,
    redactions: Mutex<Vec<String>>,
    attempts: Mutex<BTreeMap<String, Pending>>,
    slots: Mutex<BTreeMap<String, Arc<tokio::sync::Mutex<Option<Cached>>>>>,
    leases: Mutex<BTreeMap<String, broker::Lease>>,
    broker: tokio::sync::Mutex<Option<PathBuf>>,
}
impl Runtime {
    fn slot(&self, id: &str) -> Result<Arc<tokio::sync::Mutex<Option<Cached>>>> {
        let mut slots = self.slots.lock().map_err(|_| Error::Unavailable)?;
        if slots.len() >= 64 && !slots.contains_key(id) {
            return Err(Error::Limit);
        }
        Ok(slots.entry(id.into()).or_default().clone())
    }
    pub(super) fn revoke_worker(&self, worker: &str, generation: Option<u64>) {
        if let Ok(mut leases) = self.leases.lock() {
            leases
                .retain(|_, v| v.worker != worker || generation.is_some_and(|g| g != v.generation));
        }
    }
    pub(super) fn shutdown(&self) {
        if let Ok(mut attempts) = self.attempts.lock() {
            for attempt in attempts.values_mut() {
                if let Some(task) = attempt.task.take() {
                    task.abort();
                }
            }
        }
        if let Ok(mut leases) = self.leases.lock() {
            leases.clear();
        }
        if let Ok(mut slots) = self.slots.lock() {
            slots.clear();
        }
    }
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
impl Supervisor {
    pub(super) fn chatgpt_view(&self) -> Result<contract::Status> {
        let accounts = self.read(|s| {
            Ok(s.chatgpt_accounts
                .values()
                .map(|a| a.view.clone())
                .collect())
        })?;
        let mut attempts =
            self.read(|s| Ok(s.chatgpt_notices.values().cloned().collect::<Vec<_>>()))?;
        attempts.extend(
            self.0
                .chatgpt
                .attempts
                .lock()
                .map_err(|_| Error::Unavailable)?
                .values()
                .map(|a| a.view.clone())
                .take(32 - attempts.len().min(32))
                .collect::<Vec<_>>(),
        );
        let old = self.read(|s| {
            Ok(s.chatgpt_requests
                .values()
                .rev()
                .map(|(_, a)| a.clone())
                .collect::<Vec<_>>())
        })?;
        for a in old {
            if attempts.len() >= 32 {
                break;
            }
            if !attempts.iter().any(|p| p.id == a.id) {
                attempts.push(a);
            }
        }
        Ok(contract::Status {
            supported: cfg!(target_os = "macos"),
            accounts,
            attempts,
        })
    }
    pub(super) fn chatgpt_record(&self, id: &str) -> Result<Record> {
        self.read(|s| s.chatgpt_accounts.get(id).cloned().ok_or(Error::Invalid))
    }
    pub(super) fn oauth_redactions(&self) -> Result<Vec<String>> {
        Ok(self
            .0
            .chatgpt
            .redactions
            .lock()
            .map_err(|_| Error::Unavailable)?
            .clone())
    }
    fn register_oauth_secrets(&self, tokens: &oauth::Tokens) -> Result<()> {
        let values = [
            tokens.access_token.clone(),
            tokens.refresh_token.clone(),
            tokens.id_token.clone(),
        ];
        let mut redactions = self
            .0
            .chatgpt
            .redactions
            .lock()
            .map_err(|_| Error::Unavailable)?;
        for value in &values {
            if !redactions.contains(value) {
                if redactions.len() >= 4000 {
                    return Err(Error::Limit);
                }
                redactions.push(value.clone());
            }
        }
        let mut slot = self.0.sanitizer.lock().map_err(|_| Error::Unavailable)?;
        *slot = Some(
            slot.as_ref()
                .ok_or(Error::Busy)?
                .for_oauth_secrets(&values)
                .map_err(|_| Error::Invalid)?,
        );
        Ok(())
    }
}
