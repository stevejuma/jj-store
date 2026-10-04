//! Installation-private source channels. Registration grants no repository authority.
use crate::{CaptureResult, codex_activity::ActiveCodexSource, codex_discovery::PhysicalId};
pub use jj_agent::SourceInstanceId;
use sha2::{Digest as _, Sha256};
use std::path::Path;
mod acquisition;
mod catalog;
mod codex;
mod files;
mod import;
mod opencode;
mod parking;
use acquisition::Admission;
use catalog::Catalog;
pub use codex::{
    CodexSourceFingerprint, LeasedCodexTranscriptReader, QualifiedCodexSource,
    RegisteredCodexSource,
};
use files::PrivateDirectory;
pub use opencode::{LeasedOpenCodeSource, OpenCodeSourceBinding};
pub use parking::{
    ParkedCodexTranscriptReader, ReservedCodexPhysicalLease, validate_parked_progress,
};

pub struct SourceRegistry {
    directory: PrivateDirectory,
    installation: [u8; 16],
    capacity: usize,
    acquisitions: std::sync::Arc<crate::agent_capture_lock::ForkMutations>,
}
impl SourceRegistry {
    /// Read-only lookup before activity admission: never creates state or takes a lease.
    pub fn open_existing(
        path: &Path,
        installation: [u8; 16],
        capacity: usize,
    ) -> CaptureResult<Option<Self>> {
        if installation == [0; 16] || !(1..=4096).contains(&capacity) {
            return Err("invalid source registry configuration".into());
        }
        let Some(directory) = PrivateDirectory::existing(path)? else {
            return Ok(None);
        };
        directory.validate_contents()?;
        let catalog: Catalog = directory
            .load()?
            .ok_or("source registry catalog unavailable")?;
        catalog.validate(installation, capacity)?;
        directory.revalidate()?;
        Ok(Some(Self {
            directory,
            installation,
            capacity,
            acquisitions: Default::default(),
        }))
    }
    /// The caller supplies the existing installation identity, never a repository ID.
    /// Opening the private registry does not register any discovered source.
    pub fn open(path: &Path, installation: [u8; 16], capacity: usize) -> CaptureResult<Self> {
        if installation == [0; 16] || !(1..=4096).contains(&capacity) {
            return Err("invalid source registry configuration".into());
        }
        let directory = PrivateDirectory::open(path)?;
        let initialize = directory.is_empty()?;
        let gate = directory.lock("registry.lock")?;
        directory.recover()?;
        let existing: Option<Catalog> = directory.load()?;
        let new = existing.is_none();
        if new && !initialize {
            return Err("source registry catalog unavailable".into());
        }
        let catalog = existing.unwrap_or_else(|| Catalog::empty(installation));
        catalog.validate(installation, capacity)?;
        if new {
            gate.revalidate()?;
            directory.persist(&catalog)?;
        }
        gate.revalidate()?;
        directory.revalidate()?;
        Ok(Self {
            directory,
            installation,
            capacity,
            acquisitions: Default::default(),
        })
    }
    /// Call only after independent repository/activity admission. This capability
    /// neither performs nor substitutes for those checks or publication policy.
    pub fn acquire(&self, qualified: QualifiedCodexSource) -> CaptureResult<RegisteredCodexSource> {
        self.acquire_checked(qualified, Admission::Registration)
    }
    pub fn acquire_active(
        &self,
        active: ActiveCodexSource,
    ) -> CaptureResult<RegisteredCodexSource> {
        active.revalidate()?;
        self.acquire_checked(active.qualified.clone(), Admission::Active(&active))
    }
    /// Registration alone never resumes an inactive historical source.
    pub fn acquire_admitted(
        &self,
        qualified: QualifiedCodexSource,
    ) -> CaptureResult<Option<RegisteredCodexSource>> {
        if !self.previously_admitted(&qualified)? {
            return Ok(None);
        }
        self.acquire_checked(qualified, Admission::Resume).map(Some)
    }
    pub fn previously_admitted(&self, qualified: &QualifiedCodexSource) -> CaptureResult<bool> {
        Ok(self.lookup_admitted(qualified)?.is_some())
    }
    /// Read-only unambiguous channel lookup; no alias enrollment, IDs or leases.
    pub fn lookup_admitted(
        &self,
        qualified: &QualifiedCodexSource,
    ) -> CaptureResult<Option<SourceInstanceId>> {
        self.lookup_marker(qualified, keys(qualified).2)
    }
    fn lookup_marker(
        &self,
        qualified: &QualifiedCodexSource,
        marker: [u8; 32],
    ) -> CaptureResult<Option<SourceInstanceId>> {
        qualified.revalidate()?;
        self.directory.validate_contents()?;
        let catalog: Catalog = self
            .directory
            .load()?
            .ok_or("source registry catalog unavailable")?;
        catalog.validate(self.installation, self.capacity)?;
        let result = admitted_source(&catalog, qualified, marker)?;
        qualified.revalidate()?;
        self.directory.revalidate()?;
        Ok(result)
    }
}
fn admitted_source(
    catalog: &Catalog,
    qualified: &QualifiedCodexSource,
    marker: [u8; 32],
) -> CaptureResult<Option<SourceInstanceId>> {
    let (location, physical, _) = keys(qualified);
    catalog.admitted_source(location, qualified.header.witness, physical, marker)
}
fn import_key(qualified: &QualifiedCodexSource) -> [u8; 32] {
    digest(
        b"codex-import-admitted/v1",
        &[
            qualified.profile.product_version().as_bytes(),
            &keys(qualified).1,
            &qualified.header.witness,
            &keys(qualified).2,
        ],
    )
}
fn keys(qualified: &QualifiedCodexSource) -> ([u8; 32], [u8; 32], [u8; 32]) {
    (
        digest(
            if qualified.profile.is_cursor() {
                b"cursor-location/v1"
            } else {
                b"codex-location/v1"
            },
            &[qualified.candidate.path().as_os_str().as_encoded_bytes()],
        ),
        physical_key(qualified.candidate.identity()),
        if qualified.profile.is_cursor() {
            digest(
                b"cursor-admitted-session/v1",
                &[
                    qualified.native_session_key().as_bytes(),
                    &qualified.header.witness,
                ],
            )
        } else {
            digest(
                b"codex-admitted-session/v1",
                &[qualified
                    .native_session_key()
                    .to_ascii_lowercase()
                    .as_bytes()],
            )
        },
    )
}
fn digest(domain: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/source-registry/");
    hash.update(domain);
    for bytes in parts {
        hash.update((bytes.len() as u64).to_be_bytes());
        hash.update(bytes);
    }
    hash.finalize().into()
}
fn physical_key(id: PhysicalId) -> [u8; 32] {
    digest(
        b"physical-lease/v1",
        &[&id.device.to_be_bytes(), &id.inode.to_be_bytes()],
    )
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|v| format!("{v:02x}")).collect()
}
