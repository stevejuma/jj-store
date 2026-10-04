//! Read-only bounded import preparation. No intent, eligibility, leases or progress.
use crate::{
    CaptureResult,
    codex_transcript::{MAX_VERIFY_BYTES, context::ContextReconstruction, file},
    source_registry::QualifiedCodexSource,
};
use std::{
    io::{Read as _, Seek as _},
    path::{Path, PathBuf},
};
mod binding;
mod reader;
mod scan;
pub use binding::CodexImportBinding;
use scan::Scan;

// Context contains device paths and intentionally has no Debug or serialization.
#[derive(Clone)]
pub struct CodexImportSnapshot {
    binding: CodexImportBinding,
    context: crate::codex_transcript::CodexDirectoryContext,
    prior: Option<PathBuf>,
}
impl CodexImportSnapshot {
    pub fn binding(&self) -> CodexImportBinding {
        self.binding
    }
    pub fn directory(&self) -> Option<&Path> {
        self.context.directory()
    }
    /// A separate anchor candidate; never authority for ambiguous batch bodies.
    pub fn prior_directory(&self) -> Option<&Path> {
        self.prior.as_deref()
    }
}
pub enum CodexImportPoll {
    Verifying,
    ChangedDuringRead,
    Ready(CodexImportSnapshot),
}
pub struct CodexImportProbe {
    qualified: QualifiedCodexSource,
    stamp: file::Stamp,
    identity: CodexImportBinding,
    expected: Option<CodexImportBinding>,
    scan: Scan,
    ready: Option<CodexImportSnapshot>,
    reader: Option<reader::ReaderCheck>,
    remaining_bytes: u64,
}
impl CodexImportProbe {
    /// Caller owns authorized preparation and its bounded reader frame separately.
    pub fn select(qualified: QualifiedCodexSource) -> CaptureResult<Self> {
        Self::new(qualified, None)
    }
    /// Retains accepted metadata exactly; a replacement cannot inherit the selection.
    pub fn reconcile(
        qualified: QualifiedCodexSource,
        expected: CodexImportBinding,
    ) -> CaptureResult<Self> {
        expected.validate()?;
        Self::new(qualified, Some(expected))
    }
    fn new(
        qualified: QualifiedCodexSource,
        expected: Option<CodexImportBinding>,
    ) -> CaptureResult<Self> {
        qualified.revalidate()?;
        let (_, header, stamp) = file::inspect_bound(
            qualified.profile,
            qualified.candidate.path(),
            Some(qualified.candidate.identity()),
        )?;
        if header.witness != qualified.header.witness {
            return Err("Codex import header changed".into());
        }
        let identity = binding::identity(&qualified);
        if let Some(selected) = expected
            && (!identity.same_source(&selected) || selected.complete_byte_offset > stamp.len)
        {
            return Err("Codex import selected source unavailable".into());
        }
        qualified.revalidate()?;
        let scan = Scan::new(&header.directory, expected);
        Ok(Self {
            qualified,
            stamp,
            identity,
            expected,
            scan,
            ready: None,
            reader: None,
            remaining_bytes: 0,
        })
    }
    pub fn is_compact(&self) -> bool {
        self.ready.is_some()
    }
    /// Retain exact selected bytes across later bounded checkpoint steps and appends.
    pub fn retain_accepted(&mut self, accepted: CodexImportBinding) -> CaptureResult<()> {
        if self.ready.as_ref().map(|s| s.binding) != Some(accepted) || !self.revalidate()? {
            return Err("import accepted snapshot differs".into());
        }
        self.expected = Some(accepted);
        Ok(())
    }
    /// Scan at most 4 MiB of prefix per step, plus fixed bounded header checks.
    /// Partial EOF grants no accepted bytes or records.
    pub fn poll(&mut self) -> CaptureResult<CodexImportPoll> {
        self.remaining_bytes = MAX_VERIFY_BYTES as u64;
        self.qualified.revalidate()?;
        let (mut file, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        if header.witness != self.qualified.header.witness {
            return Err("Codex import header changed".into());
        }
        if self
            .expected
            .is_some_and(|selected| selected.complete_byte_offset > stamp.len)
        {
            return Err("Codex import selected prefix truncated".into());
        }
        if self.stamp != stamp {
            self.stamp = stamp.clone();
            self.scan = Scan::new(&header.directory, self.expected);
            self.ready = None;
            self.reader = None;
            return Ok(CodexImportPoll::ChangedDuringRead);
        }
        if let Some(snapshot) = &self.ready {
            self.qualified.revalidate()?;
            return Ok(CodexImportPoll::Ready(snapshot.clone()));
        }
        file.seek(std::io::SeekFrom::Start(self.scan.offset))?;
        let mut budget = MAX_VERIFY_BYTES as u64;
        let mut bytes = [0; 64 * 1024];
        while budget > 0 && self.scan.offset < stamp.len {
            let limit = budget
                .min(stamp.len - self.scan.offset)
                .min(bytes.len() as u64) as usize;
            let amount = file.read(&mut bytes[..limit])?;
            if amount == 0 {
                return Err("Codex import prefix unavailable".into());
            }
            self.scan.feed(&bytes[..amount]);
            budget -= amount as u64;
        }
        self.remaining_bytes = budget;
        if !file::unchanged(&file, self.qualified.candidate.path(), &stamp)? {
            self.scan = Scan::new(&header.directory, self.expected);
            self.ready = None;
            self.reader = None;
            return Ok(CodexImportPoll::ChangedDuringRead);
        }
        self.qualified.revalidate()?;
        if self.scan.offset < stamp.len {
            return Ok(CodexImportPoll::Verifying);
        }
        let binding = self.scan.binding(self.identity);
        binding.validate()?;
        if self.expected.is_some_and(|selected| selected != binding) {
            return Err("Codex import selected prefix changed".into());
        }
        let snapshot = CodexImportSnapshot {
            binding,
            context: self.scan.context.history.context.clone(),
            prior: self.scan.prior.clone(),
        };
        self.scan.context.release_eof_bytes();
        self.ready = Some(snapshot.clone());
        Ok(CodexImportPoll::Ready(snapshot))
    }
    /// Current stable snapshot only; this is not native authorization or a prefix re-scan.
    pub fn revalidate(&self) -> CaptureResult<bool> {
        if self.ready.is_none() {
            return Ok(false);
        }
        self.qualified.revalidate()?;
        let (_, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        Ok(header.witness == self.qualified.header.witness
            && stamp == self.stamp
            && self.qualified.revalidate().is_ok())
    }
    /// Registry-only checked selection. Offered metadata alone cannot enroll history.
    pub(crate) fn selected_source(
        &self,
        accepted: CodexImportBinding,
    ) -> CaptureResult<QualifiedCodexSource> {
        if self.ready.as_ref().map(|s| s.binding) != Some(accepted) || !self.revalidate()? {
            return Err("Codex import preparation no longer selects accepted history".into());
        }
        Ok(self.qualified.clone())
    }
}
