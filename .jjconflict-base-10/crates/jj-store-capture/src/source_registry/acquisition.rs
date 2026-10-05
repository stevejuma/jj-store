use super::import::ImportAdmission;
use super::*;

pub(super) enum Admission<'a> {
    Registration,
    Active(&'a ActiveCodexSource),
    Resume,
    Import(ImportAdmission<'a>),
}
impl Admission<'_> {
    fn revalidate(&mut self, qualified: &QualifiedCodexSource) -> CaptureResult<()> {
        match self {
            Self::Active(active) => active.revalidate()?,
            Self::Import(proof) => proof.revalidate(qualified)?,
            Self::Registration | Self::Resume => {}
        }
        qualified.revalidate()
    }
    fn marker(&self, qualified: &QualifiedCodexSource) -> Option<[u8; 32]> {
        match self {
            Self::Active(_) => Some(keys(qualified).2),
            Self::Import(_) => Some(import_key(qualified)),
            Self::Registration | Self::Resume => None,
        }
    }
}
impl SourceRegistry {
    pub(super) fn acquire_checked(
        &self,
        qualified: QualifiedCodexSource,
        mut admission: Admission<'_>,
    ) -> CaptureResult<RegisteredCodexSource> {
        let _turn = self.acquisitions.acquire()?;
        qualified.revalidate()?;
        self.directory.revalidate()?;
        let gate = self.directory.lock("registry.lock")?;
        self.directory.recover()?;
        let mut catalog: Catalog = self
            .directory
            .load()?
            .ok_or("source registry catalog unavailable")?;
        catalog.validate(self.installation, self.capacity)?;
        let (location, physical, _) = keys(&qualified);
        if matches!(admission, Admission::Resume)
            && admitted_source(&catalog, &qualified, keys(&qualified).2)?.is_none()
        {
            return Err("source session admission changed".into());
        }
        // Explicit durable consent precedes even transient source/alias allocation.
        if matches!(admission, Admission::Import(_)) {
            admission.revalidate(&qualified)?;
        }
        let source =
            catalog.register(location, qualified.header.witness, physical, self.capacity)?;
        let physical_lease = self
            .directory
            .lock(&format!("physical-{}.lock", hex(&physical)))?;
        let source_lease = self.directory.lock(&format!("source-{source}.lock"))?;
        admission.revalidate(&qualified)?;
        if let Some(marker) = admission.marker(&qualified) {
            catalog.admit(source, marker, self.capacity)?;
        }
        physical_lease.revalidate()?;
        source_lease.revalidate()?;
        gate.revalidate()?;
        self.directory.persist(&catalog)?;
        if matches!(admission, Admission::Import(_)) {
            admission.revalidate(&qualified)?;
        }
        physical_lease.revalidate()?;
        source_lease.revalidate()?;
        gate.revalidate()?;
        self.directory.revalidate()?;
        Ok(RegisteredCodexSource {
            qualified,
            source,
            physical_lease,
            source_lease,
        })
    }
}
