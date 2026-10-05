//! Private retained preparations and historical outcomes, never code authority.
//! Callers authenticate and verify provider evidence; this journal neither
//! retains provider objects nor publishes heads or touches checkout files.
use crate::capture_journal_files as files;
use jj_store_model as m;
use jj_store_service::stack_workspace as s;
use serde::{Deserialize, Serialize};
use std::path::Path;
mod catalog;
mod checkout;
mod execution;
mod transition;
use catalog::Catalog;
pub use checkout::SavedCheckoutV1;
pub use execution::{LocalExecutionV1, preparation_key};

pub const MAX_RECORDS: usize = 128;
pub const MAX_BYTES: usize = 8 * 1024 * 1024;
const LAYOUT: files::Layout = files::Layout {
    lock: "stacks.lock",
    temporary: ".stack-command-",
    max_bytes: MAX_BYTES,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Profile {
    GitSha1Jj045,
    Native,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    pub installation: [u8; 16],
    pub repository: m::RepositoryId,
    pub physical_workspace: s::PhysicalWorkspaceId,
    pub jj_workspace: String,
    pub profile: Profile,
}
impl Scope {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.installation == [0; 16]
            || self.repository.as_bytes() == &[0; 16]
            || self.physical_workspace.0.as_bytes() == &[0; 32]
            || self.jj_workspace.trim().is_empty()
            || self.jj_workspace.len() > 256
            || self.jj_workspace.chars().any(char::is_control)
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    fn matches(&self, selection: &s::Selection) -> bool {
        self.repository == selection.repository
            && self.physical_workspace == selection.physical_workspace
            && self.jj_workspace == selection.jj_workspace
            && matches!(
                (self.profile, &selection.code),
                (Profile::GitSha1Jj045, s::CodeSelection::GitSha1Jj045 { .. })
                    | (Profile::Native, s::CodeSelection::Native { .. })
            )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid stack journal input")]
    Invalid,
    #[error("stack journal unavailable")]
    Unavailable,
    #[error("stack journal scope differs")]
    ScopeMismatch,
    #[error("stack request identity conflicts")]
    IntentMismatch,
    #[error("stack request is not retained")]
    NotFound,
    #[error("stack receipt state changed")]
    StaleState,
    #[error("invalid stack receipt transition")]
    InvalidTransition,
    #[error("another stack publication or checkout is unresolved")]
    Pending,
    #[error("stack journal capacity exhausted")]
    Capacity,
    #[error("stack journal selection outcome unknown")]
    OutcomeUnknown,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<files::Error> for Error {
    fn from(error: files::Error) -> Self {
        match error {
            files::Error::Unavailable => Self::Unavailable,
            files::Error::Capacity => Self::Capacity,
            files::Error::OutcomeUnknown => Self::OutcomeUnknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub principal: m::PrincipalId,
    pub preview: s::Preview,
    pub receipt: s::Receipt,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<LocalExecutionV1>,
}
impl Record {
    fn validate(&self, scope: &Scope) -> Result<()> {
        if self.principal.as_bytes() == &[0; 16]
            || self.preview.plan.request.request_id.as_bytes() == &[0; 16]
            || !scope.matches(&self.preview.plan.request.selection)
        {
            return Err(Error::Invalid);
        }
        if let Some(execution) = &self.execution {
            execution.validate(self.principal, &self.preview)?;
            if execution.checkout.is_some()
                && !matches!(
                    self.receipt.state,
                    s::ReceiptState::Published { .. } | s::ReceiptState::Retired { .. }
                )
            {
                return Err(Error::Invalid);
            }
            if let s::ReceiptState::Published {
                checkout: s::CheckoutState::Verified { fingerprint },
                ..
            } = &self.receipt.state
                && execution
                    .checkout
                    .as_ref()
                    .is_some_and(|saved| saved.witness.physical() != *fingerprint)
            {
                return Err(Error::Invalid);
            }
        }
        self.receipt
            .validate_prepared(&self.preview)
            .map_err(|_| Error::Invalid)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection {
    pub preview: s::Preview,
    pub receipt: s::Receipt,
    pub replayed: bool,
}
impl Selection {
    fn from_record(record: &Record, replayed: bool) -> Self {
        Self {
            preview: record.preview.clone(),
            receipt: record.receipt.clone(),
            replayed,
        }
    }
}
/// A physical private-state handle. Scope is supplied by server composition,
/// never inferred from an inbound folder path or client claim.
#[derive(Clone)]
pub struct StackCommandJournal {
    root: files::Root,
    scope: Scope,
}
impl StackCommandJournal {
    pub(crate) fn scope(&self) -> &Scope {
        &self.scope
    }
    pub fn check_scope(&self, selection: &s::Selection) -> Result<()> {
        if self.scope.matches(selection) {
            Ok(())
        } else {
            Err(Error::ScopeMismatch)
        }
    }
    pub fn create(path: &Path, scope: Scope) -> Result<Self> {
        scope.validate()?;
        let root = files::Root::create(path, LAYOUT)?;
        let gate = root.gate()?;
        root.persist(&gate, &Catalog::new(scope.clone()).encode()?, None)?;
        drop(gate);
        Ok(Self { root, scope })
    }
    /// Read-only open; unavailable state is never repaired or initialized.
    pub fn open(path: &Path, scope: Scope) -> Result<Self> {
        scope.validate()?;
        let journal = Self {
            root: files::Root::open(path, LAYOUT)?,
            scope,
        };
        let gate = journal.root.gate()?;
        journal.read(&gate)?;
        Ok(journal)
    }
    fn read(&self, gate: &files::Gate) -> Result<(Catalog, files::Stamp)> {
        let (bytes, stamp) = self.root.read(gate)?;
        Ok((Catalog::decode(&bytes, &self.scope)?, stamp))
    }
    /// Retains the complete original server preparation. Exact replay does not
    /// recheck live generations or reset a historical publication/retirement.
    pub fn record_prepared(
        &self,
        principal: m::PrincipalId,
        preview: &s::Preview,
    ) -> Result<Selection> {
        self.record(principal, preview, None)
    }
    pub fn record_local_prepared(
        &self,
        principal: m::PrincipalId,
        preview: &s::Preview,
        execution: &LocalExecutionV1,
    ) -> Result<Selection> {
        self.record(principal, preview, Some(execution.clone()))
    }
    fn record(
        &self,
        principal: m::PrincipalId,
        preview: &s::Preview,
        execution: Option<LocalExecutionV1>,
    ) -> Result<Selection> {
        preview.validate().map_err(|_| Error::Invalid)?;
        if !self.scope.matches(&preview.plan.request.selection) {
            return Err(Error::ScopeMismatch);
        }
        let request = &preview.plan.request;
        let record = Record {
            principal,
            preview: preview.clone(),
            execution,
            receipt: s::Receipt {
                version: 1,
                request_id: request.request_id,
                intent_digest: request.intent_digest,
                selection: request.selection.clone(),
                prepared: Some(preview.prepared),
                state: s::ReceiptState::Prepared,
            },
        };
        record.validate(&self.scope)?;
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        if let Some(retained) = catalog
            .records
            .iter()
            .find(|r| r.receipt.request_id == request.request_id)
        {
            if retained.principal != principal
                || retained.preview != *preview
                || retained.execution != record.execution
            {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(retained, true));
        }
        if catalog.records.len() == MAX_RECORDS {
            return Err(Error::Capacity);
        }
        catalog.records.push(record.clone());
        catalog.include_execution();
        self.root
            .persist(&gate, &catalog.encode_for_admission()?, Some(stamp))?;
        Ok(Selection::from_record(&record, false))
    }
    /// Bind original identity before any caller's live-state validation.
    pub fn status(
        &self,
        principal: m::PrincipalId,
        request: m::RequestId,
        digest: m::IntentDigest,
    ) -> Result<Option<Record>> {
        let gate = self.root.gate()?;
        let (catalog, stamp) = self.read(&gate)?;
        let selected = catalog.lookup(principal, request, digest)?.cloned();
        if selected.is_some() {
            self.root.sync_selected(&gate, stamp)?;
        }
        Ok(selected)
    }
    /// Complete bounded discovery for the authenticated actor; no credentials,
    /// new preparations or current-state substitution.
    pub fn pending(&self, principal: m::PrincipalId) -> Result<Vec<Record>> {
        if principal.as_bytes() == &[0; 16] {
            return Err(Error::Invalid);
        }
        let gate = self.root.gate()?;
        let (catalog, _) = self.read(&gate)?;
        Ok(catalog
            .records
            .into_iter()
            .filter(|r| {
                r.principal == principal
                    && matches!(
                        r.receipt.state,
                        s::ReceiptState::Prepared
                            | s::ReceiptState::PublicationUnknown
                            | s::ReceiptState::Published {
                                checkout: s::CheckoutState::Required
                                    | s::CheckoutState::Blocked { .. },
                                ..
                            }
                    )
            })
            .collect())
    }
}

#[cfg(all(test, unix))]
mod tests;
