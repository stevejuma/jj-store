//! API channels share the existing private catalog, capacity and exclusive leases.
use super::{
    CaptureResult, Catalog, SourceInstanceId, SourceRegistry, digest, files::PrivateLock, hex,
};
use crate::opencode_snapshot::{OpenCodeActivity, OpenCodeSessionSnapshot, PRODUCT_VERSION};

/// A caller supplies the canonical, credential-free endpoint digest after API qualification.
/// This binding is not directory, activity or publication authority.
#[derive(Clone)]
pub struct OpenCodeSourceBinding {
    endpoint: [u8; 32],
    native_session: String,
    location: [u8; 32],
    physical: [u8; 32],
    witness: [u8; 32],
    marker: [u8; 32],
}
impl OpenCodeSourceBinding {
    pub fn new(endpoint: [u8; 32], session: &OpenCodeSessionSnapshot) -> CaptureResult<Self> {
        if endpoint == [0; 32] {
            return Err("OpenCode endpoint identity unavailable".into());
        }
        let native = session.native_session_key().as_bytes();
        let directory = session.directory().as_os_str().as_encoded_bytes();
        let witness = digest(
            b"opencode-session-witness/v1",
            &[&endpoint, native, directory, PRODUCT_VERSION.as_bytes()],
        );
        Ok(Self {
            endpoint,
            native_session: session.native_session_key().into(),
            location: digest(b"opencode-api-location/v1", &[&endpoint, native]),
            physical: digest(b"opencode-api-lease/v1", &[&endpoint, native]),
            witness,
            marker: digest(b"opencode-admitted-session/v1", &[&witness]),
        })
    }
    pub fn generation_digest(&self) -> [u8; 32] {
        self.witness
    }
}
/// Retains channel and endpoint/session ownership; native paths and bodies stay memory-only.
pub struct LeasedOpenCodeSource {
    binding: OpenCodeSourceBinding,
    source: SourceInstanceId,
    physical_lease: PrivateLock,
    source_lease: PrivateLock,
}
impl LeasedOpenCodeSource {
    pub fn source_instance_id(&self) -> SourceInstanceId {
        self.source
    }
    pub fn binding(&self) -> &OpenCodeSourceBinding {
        &self.binding
    }
    pub fn revalidate(&self) -> CaptureResult<()> {
        self.physical_lease.validate_peer(&self.source_lease)
    }
}
impl SourceRegistry {
    /// Discovery metadata only: does not allocate, admit, lease or authorize a source.
    pub fn admitted_opencode_sessions(&self, endpoint: [u8; 32]) -> CaptureResult<Vec<String>> {
        if endpoint == [0; 32] {
            return Err("OpenCode endpoint identity unavailable".into());
        }
        self.directory.validate_contents()?;
        let catalog: Catalog = self
            .directory
            .load()?
            .ok_or("source registry catalog unavailable")?;
        catalog.validate(self.installation, self.capacity)?;
        let sessions = catalog.opencode_sessions(endpoint);
        self.directory.revalidate()?;
        Ok(sessions)
    }
    /// Read-only lookup: neither registration nor existence admits inactive history.
    pub fn lookup_opencode_admitted(
        &self,
        binding: &OpenCodeSourceBinding,
    ) -> CaptureResult<Option<SourceInstanceId>> {
        self.directory.validate_contents()?;
        let catalog: Catalog = self
            .directory
            .load()?
            .ok_or("source registry catalog unavailable")?;
        catalog.validate(self.installation, self.capacity)?;
        let found = catalog.admitted_source(
            binding.location,
            binding.witness,
            binding.physical,
            binding.marker,
        )?;
        self.directory.revalidate()?;
        Ok(found)
    }
    /// Caller must first verify current API activity, directory/catalog and native Write.
    pub fn acquire_opencode_active(
        &self,
        binding: OpenCodeSourceBinding,
        activity: OpenCodeActivity,
    ) -> CaptureResult<LeasedOpenCodeSource> {
        if !activity.active() {
            return Err("OpenCode explicit activity unavailable".into());
        }
        self.acquire_opencode(binding, true)?
            .ok_or_else(|| "OpenCode source admission unavailable".into())
    }
    pub fn acquire_opencode_admitted(
        &self,
        binding: OpenCodeSourceBinding,
    ) -> CaptureResult<Option<LeasedOpenCodeSource>> {
        self.acquire_opencode(binding, false)
    }
    fn acquire_opencode(
        &self,
        binding: OpenCodeSourceBinding,
        active: bool,
    ) -> CaptureResult<Option<LeasedOpenCodeSource>> {
        let _turn = self.acquisitions.acquire()?;
        self.directory.revalidate()?;
        let gate = self.directory.lock("registry.lock")?;
        self.directory.recover()?;
        let mut catalog: Catalog = self
            .directory
            .load()?
            .ok_or("source registry catalog unavailable")?;
        catalog.validate(self.installation, self.capacity)?;
        if !active
            && catalog
                .admitted_source(
                    binding.location,
                    binding.witness,
                    binding.physical,
                    binding.marker,
                )?
                .is_none()
        {
            return Ok(None);
        }
        let source = catalog.register(
            binding.location,
            binding.witness,
            binding.physical,
            self.capacity,
        )?;
        let physical_lease = self
            .directory
            .lock(&format!("physical-{}.lock", hex(&binding.physical)))?;
        let source_lease = self.directory.lock(&format!("source-{source}.lock"))?;
        if active {
            catalog.admit(source, binding.marker, self.capacity)?;
        }
        catalog.remember_opencode(
            source,
            binding.endpoint,
            &binding.native_session,
            binding.witness,
        )?;
        physical_lease.validate_peer(&source_lease)?;
        gate.revalidate()?;
        self.directory.persist(&catalog)?;
        physical_lease.validate_peer(&source_lease)?;
        gate.revalidate()?;
        Ok(Some(LeasedOpenCodeSource {
            binding,
            source,
            physical_lease,
            source_lease,
        }))
    }
}
