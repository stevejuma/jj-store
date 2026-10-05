//! Memory-only exact API history preparation; no source enrollment or durable writes.
use crate::{
    Error, Result,
    capture_import_state::CaptureImportSourceBinding,
    opencode_read_api::{OpenCodeMessageAcquisition, OpenCodeReadApi},
};
use jj_store_capture::{
    opencode_checkpoint::{initial_message_prefix, next_message_prefix},
    opencode_snapshot::OpenCodeSessionSnapshot,
    profile::CaptureProfile,
};
use sha2::{Digest as _, Sha256};

pub struct OpenCodeImportProbe {
    api: OpenCodeReadApi,
    session: OpenCodeSessionSnapshot,
    acquisition: OpenCodeMessageAcquisition,
    binding: CaptureImportSourceBinding,
    selected: Option<CaptureImportSourceBinding>,
    count: u64,
    prefix: [u8; 32],
    ready: bool,
    failed: bool,
}
impl OpenCodeImportProbe {
    /// Idle metadata supplies no activity or source admission.
    pub async fn select(api: OpenCodeReadApi, native_session: &str) -> Result<Self> {
        Self::prepare(api, native_session, None).await
    }
    /// Accepted intent never extends to appended messages or replacement bindings.
    pub async fn reconcile(
        api: OpenCodeReadApi,
        native_session: &str,
        accepted: CaptureImportSourceBinding,
    ) -> Result<Self> {
        Self::prepare(api, native_session, Some(accepted)).await
    }
    async fn prepare(
        api: OpenCodeReadApi,
        native_session: &str,
        accepted: Option<CaptureImportSourceBinding>,
    ) -> Result<Self> {
        let observed = api.observe(native_session).await?;
        let session = observed.session().clone();
        let witness = api.source_binding(&session)?.generation_digest();
        let binding = CaptureImportSourceBinding {
            profile: CaptureProfile::OpencodeReadOnlyApiV1,
            product_version: CaptureProfile::OpencodeReadOnlyApiV1
                .product_version()
                .into(),
            physical_digest: digest(b"endpoint/v1", &[&api.endpoint_digest()]),
            header_digest: witness,
            session_digest: digest(b"session/v1", &[native_session.as_bytes()]),
            prefix_digest: initial_message_prefix(witness),
            complete_byte_offset: 0,
            complete_records: 0,
        };
        if let Some(selected) = &accepted
            && (selected.profile != binding.profile
                || selected.product_version != binding.product_version
                || selected.physical_digest != binding.physical_digest
                || selected.header_digest != binding.header_digest
                || selected.session_digest != binding.session_digest
                || selected.complete_byte_offset != 0
                || selected.complete_records > 131072
                || selected.prefix_digest == [0; 32]
                || (selected.complete_records == 0
                    && selected.prefix_digest != binding.prefix_digest))
        {
            return Err(Error::Service("OpenCode accepted import binding changed"));
        }
        Ok(Self {
            acquisition: api.messages(&session),
            prefix: binding.prefix_digest,
            api,
            session,
            binding,
            selected: accepted,
            count: 0,
            ready: false,
            failed: false,
        })
    }
    /// One bounded page per step. Raw records are dropped before returning.
    pub async fn step(&mut self) -> Result<bool> {
        if self.failed {
            return Err(Error::Service("OpenCode import preparation unavailable"));
        }
        if self.ready {
            return Ok(true);
        }
        let result = self.advance().await;
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    async fn advance(&mut self) -> Result<bool> {
        let page = self
            .acquisition
            .next_page()
            .await?
            .ok_or(Error::Service("OpenCode import page unavailable"))?;
        for record in &page.records {
            if self
                .selected
                .as_ref()
                .is_some_and(|s| self.count == s.complete_records)
            {
                break;
            }
            self.count = self
                .count
                .checked_add(1)
                .filter(|n| *n <= 131072)
                .ok_or(Error::Service("OpenCode import record budget exhausted"))?;
            self.prefix = next_message_prefix(self.prefix, record)
                .map_err(|_| Error::Service("OpenCode import prefix unavailable"))?;
        }
        if let Some(selected) = &self.selected {
            if self.count == selected.complete_records {
                if self.prefix != selected.prefix_digest {
                    return Err(Error::Service("OpenCode accepted import prefix changed"));
                }
                self.ready = true;
            } else if self.acquisition.finished() {
                return Err(Error::Service("OpenCode accepted import prefix truncated"));
            }
        } else if self.acquisition.finished() {
            let mut selected = self.binding.clone();
            selected.complete_records = self.count;
            selected.prefix_digest = self.prefix;
            self.selected = Some(selected);
            self.count = 0;
            self.prefix = self.binding.prefix_digest;
            self.acquisition = self.api.messages(&self.session);
        }
        Ok(self.ready)
    }
    pub fn boundary(&self) -> Result<CaptureImportSourceBinding> {
        if !self.ready || self.failed {
            return Err(Error::Service("OpenCode import boundary unavailable"));
        }
        self.selected
            .clone()
            .ok_or(Error::Service("OpenCode import boundary unavailable"))
    }
}
fn digest(domain: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/opencode-import/");
    hash.update(domain);
    hash.update([0]);
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    hash.finalize().into()
}
