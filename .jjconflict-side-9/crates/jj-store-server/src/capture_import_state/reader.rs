use super::*;
use jj_store_service::collaboration::AgentSourceInstanceIdentity as SourceInstanceId;

/// Private exact reader association; never part of the immutable accepted receipt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportReaderBinding {
    pub ordinal: u64,
    pub source: SourceInstanceId,
    pub generation: u64,
    #[serde(with = "super::hex_codec::hex32")]
    pub generation_digest: [u8; 32],
    #[serde(default)]
    pub prefix_verified: bool,
}
impl CaptureImportReaderBinding {
    pub(super) fn validate(&self, receipt: &CaptureImportReceipt) -> Result<()> {
        use jj_store_capture::profile::CaptureProfile;
        if self.ordinal != receipt.ordinal
            || self.generation == 0
            || self.source.as_bytes() == &[0; 16]
            || !matches!(
                receipt.source.profile,
                CaptureProfile::CodexCliJsonlV1 | CaptureProfile::CodexDesktopJsonlV1
            )
            || !receipt
                .source
                .codex()
                .matches_generation_digest(self.generation_digest)
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    fn continues(&self, original: &Self) -> bool {
        self.ordinal == original.ordinal
            && self.source == original.source
            && self.generation_digest == original.generation_digest
            && self.generation >= original.generation
    }
}
impl CaptureImportSelected {
    pub(crate) fn bind_reader(
        &mut self,
        offered: CaptureImportReaderBinding,
    ) -> Result<CaptureImportReaderBinding> {
        self.check()?;
        offered.validate(&self.selection().receipt)?;
        let (bytes, stamp) = self.journal.root.read(&self.gate)?;
        let mut catalog = self.journal.decode(&bytes)?;
        let selected_binding = if let Some(original) = catalog
            .readers
            .iter_mut()
            .find(|r| r.ordinal == offered.ordinal)
        {
            if !offered.continues(original) {
                return Err(Error::Conflict);
            }
            if original.prefix_verified || !offered.prefix_verified {
                self.journal.root.sync_selected(&self.gate, stamp)?;
                return Ok(original.clone());
            }
            original.prefix_verified = true;
            original.clone()
        } else {
            if catalog.readers.len() == MAX_RECEIPTS {
                return Err(Error::Capacity);
            }
            catalog.readers.push(offered.clone());
            offered
        };
        self.journal
            .root
            .persist(&self.gate, &catalog.encode()?, Some(stamp))?;
        let (bytes, stamp) = self
            .journal
            .root
            .read(&self.gate)
            .map_err(|_| Error::OutcomeUnknown)?;
        if self
            .journal
            .decode(&bytes)
            .map_err(|_| Error::OutcomeUnknown)?
            .readers
            .iter()
            .find(|r| r.ordinal == selected_binding.ordinal)
            != Some(&selected_binding)
        {
            return Err(Error::OutcomeUnknown);
        }
        // Atomic replacement is selected by this same held gate. Preserve exact checking.
        self.stamp = stamp;
        self.check().map_err(|_| Error::OutcomeUnknown)?;
        Ok(selected_binding)
    }
}
impl CaptureImportJournal {
    pub(crate) fn inspect_request<T>(
        &self,
        request: RequestId,
        apply: impl FnOnce(Option<(&CaptureImportReceipt, Option<&CaptureImportReaderBinding>)>) -> T,
    ) -> Result<T> {
        let gate = self.root.gate()?;
        let (bytes, stamp) = self.root.read(&gate)?;
        let catalog = self.decode(&bytes)?;
        let selected = catalog
            .receipts
            .iter()
            .find(|r| r.intent.request == request)
            .map(|r| (r, catalog.readers.iter().find(|b| b.ordinal == r.ordinal)));
        let result = apply(selected);
        let (_, after) = self.root.read(&gate)?;
        if stamp != after {
            return Err(Error::Unavailable);
        }
        Ok(result)
    }
    /// Private metadata only; no source, transcript, lease or worker inspection.
    pub fn reader_binding(
        &self,
        intent: &CaptureImportIntent,
    ) -> Result<Option<CaptureImportReaderBinding>> {
        self.validate(intent)?;
        let gate = self.root.gate()?;
        let (bytes, _) = self.root.read(&gate)?;
        let catalog = self.decode(&bytes)?;
        let Some(receipt) = catalog.find(intent)? else {
            return Ok(None);
        };
        let ordinal = receipt.ordinal;
        Ok(catalog.readers.into_iter().find(|r| r.ordinal == ordinal))
    }
}
