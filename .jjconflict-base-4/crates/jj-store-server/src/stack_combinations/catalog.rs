use super::*;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    scope: j::Scope,
    principal: m::PrincipalId,
    pub selected: Snapshot,
    pub records: Vec<Record>,
}
impl Catalog {
    pub fn new(scope: j::Scope, principal: m::PrincipalId) -> Self {
        Self {
            version: 1,
            scope,
            principal,
            selected: Snapshot::empty(),
            records: vec![],
        }
    }
    pub fn decode(bytes: &[u8], scope: &j::Scope, principal: m::PrincipalId) -> Result<Self> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| Error::Unavailable)?;
        if value.scope != *scope || value.principal != principal {
            return Err(Error::Forbidden);
        }
        value.validate().map_err(|_| Error::Unavailable)?;
        Ok(value)
    }
    fn validate(&self) -> Result<()> {
        validation::scope(&self.scope, self.principal)?;
        if self.version != 1 || self.records.len() > MAX_RECORDS {
            return Err(Error::Invalid);
        }
        validation::snapshot(&self.scope, self.principal, &self.selected)?;
        let mut requests = BTreeSet::new();
        let mut selected = Snapshot::empty();
        let mut pending = false;
        for r in &self.records {
            r.preview.validate().map_err(|_| Error::Invalid)?;
            validation::snapshot(&self.scope, self.principal, &r.after)?;
            if !requests.insert(r.preview.plan.request.request_id)
                || pending
                || r.before_revision != selected.revision
                || r.after.revision != selected.revision.checked_add(1).ok_or(Error::Capacity)?
                || r.retention.key != j::preparation_key(self.principal, &r.preview)
                || r.retention.archive.as_bytes() == &[0; 32]
            {
                return Err(Error::Invalid);
            }
            validation::transition(&self.scope, self.principal, &selected, r)?;
            match r.phase {
                Phase::Selected => selected = r.after.clone(),
                Phase::Rejected => (),
                Phase::Pending => pending = true,
            }
        }
        if selected != self.selected {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        Ok(bytes)
    }
    pub fn lookup(&self, request: &s::PrepareRequest) -> Result<Option<&Record>> {
        let record = self
            .records
            .iter()
            .find(|r| r.preview.plan.request.request_id == request.request_id);
        if record.is_some_and(|r| r.preview.plan.request != *request) {
            return Err(Error::IntentMismatch);
        }
        Ok(record)
    }
}
