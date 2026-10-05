use super::*;
use sha2::{Digest as _, Sha256};

/// Exact installation-private staged after-state. No client can issue a binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombinationBindingV1 {
    version: u32,
    id: m::RecordDigest,
    before_revision: u64,
    selection: s::CombinationSelection,
}
impl CombinationBindingV1 {
    pub fn id(&self) -> m::RecordDigest {
        self.id
    }
    pub fn selection(&self) -> &s::CombinationSelection {
        &self.selection
    }
    pub(crate) fn validate(&self) -> Result<()> {
        if self.version != 1
            || self.id.as_bytes() == &[0; 32]
            || self.before_revision == u64::MAX
            || self.selection.generation == 0
            || self.selection.id.0.as_bytes() == &[0; 16]
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub(crate) fn proposal(
        scope: &j::Scope,
        principal: m::PrincipalId,
        preview: &s::Preview,
        before_revision: u64,
        after: &Snapshot,
        retained: &StackRetention,
    ) -> Result<Self> {
        let retention = validation::retained(scope, principal, preview, retained)?;
        Self::digest(
            scope,
            principal,
            preview,
            before_revision,
            after,
            &retention,
        )
    }
    pub(super) fn from_record(
        scope: &j::Scope,
        principal: m::PrincipalId,
        r: &Record,
    ) -> Result<Self> {
        Self::digest(
            scope,
            principal,
            &r.preview,
            r.before_revision,
            &r.after,
            &r.retention,
        )
    }
    fn digest(
        scope: &j::Scope,
        principal: m::PrincipalId,
        preview: &s::Preview,
        before_revision: u64,
        after: &Snapshot,
        retention: &RetainedCode,
    ) -> Result<Self> {
        let bytes =
            serde_json::to_vec(&(scope, principal, preview, before_revision, after, retention))
                .map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        let mut digest = Sha256::new();
        digest.update(b"jj-store:stack-combination-binding:v1\0");
        digest.update(bytes);
        let value = Self {
            version: 1,
            id: m::RecordDigest::new(digest.finalize().into()),
            before_revision,
            selection: preview.plan.combination.clone(),
        };
        value.validate()?;
        Ok(value)
    }
}
impl CombinationStore {
    pub(crate) fn check_scope(&self, scope: &j::Scope, principal: m::PrincipalId) -> Result<()> {
        if self.scope != *scope || self.principal != principal {
            return Err(Error::Forbidden);
        }
        Ok(())
    }
    /// Original-only lookup and sync; never reconstruct a selected missing body.
    pub(crate) fn binding(&self, record: &j::Record) -> Result<CombinationBindingV1> {
        self.check_scope_for_record(record)?;
        let gate = self.root.gate()?;
        let (catalog, stamp) = self.read(&gate)?;
        let staged = catalog
            .lookup(&record.preview.plan.request)?
            .ok_or(Error::Unavailable)?;
        check_record(staged, record)?;
        let binding = CombinationBindingV1::from_record(&self.scope, self.principal, staged)?;
        self.root.sync_selected(&gate, stamp)?;
        Ok(binding)
    }
    pub(crate) fn binding_proposal(
        &self,
        record: &j::Record,
        expected_revision: u64,
        after: &Snapshot,
        retained: &StackRetention,
    ) -> Result<CombinationBindingV1> {
        self.check_scope_for_record(record)?;
        let retention =
            validation::retained(&self.scope, self.principal, &record.preview, retained)?;
        let gate = self.root.gate()?;
        let (catalog, stamp) = self.read(&gate)?;
        let staged = catalog
            .lookup(&record.preview.plan.request)?
            .ok_or(Error::Unavailable)?;
        check_record(staged, record)?;
        if staged.before_revision != expected_revision
            || staged.after != *after
            || staged.retention != retention
        {
            return Err(Error::IntentMismatch);
        }
        let binding = CombinationBindingV1::from_record(&self.scope, self.principal, staged)?;
        self.root.sync_selected(&gate, stamp)?;
        Ok(binding)
    }
    fn check_scope_for_record(&self, record: &j::Record) -> Result<()> {
        if record.principal != self.principal {
            return Err(Error::Forbidden);
        }
        Ok(())
    }
    /// Called only while the original journal gate is held, after fresh sealed
    /// native checkout evidence is checked. It does not acquire the journal.
    pub(crate) fn select_completed(&self, record: &j::Record) -> Result<()> {
        self.check_scope_for_record(record)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        let saved = execution.combination.as_ref().ok_or(Error::Invalid)?;
        if execution.completion.is_none() {
            return Err(Error::Pending);
        }
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let staged = catalog
            .lookup(&record.preview.plan.request)?
            .ok_or(Error::Unavailable)?;
        check_record(staged, record)?;
        if CombinationBindingV1::from_record(&self.scope, self.principal, staged)? != *saved {
            return Err(Error::IntentMismatch);
        }
        match staged.phase {
            Phase::Selected => return Ok(self.root.sync_selected(&gate, stamp)?),
            Phase::Rejected => return Err(Error::Stale),
            Phase::Pending => (),
        }
        if catalog.selected.revision != staged.before_revision {
            return Err(Error::Stale);
        }
        let index = catalog
            .records
            .iter()
            .position(|r| r.preview.plan.request.request_id == record.receipt.request_id)
            .ok_or(Error::Unavailable)?;
        catalog.selected = catalog.records[index].after.clone();
        catalog.records[index].phase = Phase::Selected;
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(())
    }
}
fn check_record(staged: &Record, record: &j::Record) -> Result<()> {
    let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
    if staged.preview != record.preview
        || staged.retention.key != execution.key
        || Some(staged.retention.commit) != execution.retention
        || staged.retention.archive != execution.archive
    {
        return Err(Error::IntentMismatch);
    }
    Ok(())
}
