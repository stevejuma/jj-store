//! Current complete-record context only; no eligibility, leases, cursors or publication.
use crate::{
    CaptureResult,
    codex_transcript::{
        CodexDirectoryContext, MAX_VERIFY_BYTES,
        context::{ContextReconstruction, DirectoryHistory},
        file,
    },
    source_registry::QualifiedCodexSource,
};
use std::{
    io::{Read as _, Seek as _},
    path::{Path, PathBuf},
};

// Raw context paths and history intentionally have no Debug or serialization.
#[derive(Clone)]
pub struct CodexResumeDirectory {
    context: CodexDirectoryContext,
    prior: Option<PathBuf>,
}
impl CodexResumeDirectory {
    pub fn directory(&self) -> Option<&Path> {
        self.context.directory()
    }
    /// Independently admit this anchor; it cannot authorize batch body evidence.
    pub fn prior_directory(&self) -> Option<&Path> {
        self.prior.as_deref()
    }
}
pub enum CodexResumePoll {
    Verifying,
    ChangedDuringRead,
    Ready(CodexResumeDirectory),
}
struct Scan {
    offset: u64,
    context: ContextReconstruction,
    prior: Option<PathBuf>,
}
pub struct CodexResumeProbe {
    qualified: QualifiedCodexSource,
    stamp: file::Stamp,
    scan: Scan,
    ready: Option<CodexResumeDirectory>,
}
impl CodexResumeProbe {
    /// A stable EOF proof retains context and metadata, with no transcript allocation.
    pub fn is_compact(&self) -> bool {
        self.ready.is_some()
    }
    pub fn new(qualified: QualifiedCodexSource) -> CaptureResult<Self> {
        qualified.revalidate()?;
        let (_, header, stamp) = file::inspect_bound(
            qualified.profile,
            qualified.candidate.path(),
            Some(qualified.candidate.identity()),
        )?;
        if header.witness != qualified.header.witness {
            return Err("Codex resume header changed".into());
        }
        qualified.revalidate()?;
        let scan = Scan::new(&header.directory);
        Ok(Self {
            qualified,
            stamp,
            scan,
            ready: None,
        })
    }
    /// Inspect at most 4 MiB of this stable snapshot per call; never emit bodies.
    pub fn poll(&mut self) -> CaptureResult<CodexResumePoll> {
        self.qualified.revalidate()?;
        let (mut file, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        if header.witness != self.qualified.header.witness {
            return Err("Codex resume header changed".into());
        }
        if self.stamp != stamp {
            self.stamp = stamp.clone();
            self.scan = Scan::new(&header.directory);
            self.ready = None;
        }
        if let Some(context) = &self.ready {
            self.qualified.revalidate()?;
            return Ok(CodexResumePoll::Ready(context.clone()));
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
                return Err("Codex resume prefix unavailable".into());
            }
            self.scan
                .context
                .feed_observed(&bytes[..amount], |context| {
                    if let Some(directory) = context.directory() {
                        self.scan.prior = Some(directory.to_owned());
                    }
                });
            self.scan.offset += amount as u64;
            budget -= amount as u64;
        }
        if !file::unchanged(&file, self.qualified.candidate.path(), &stamp)? {
            self.scan = Scan::new(&header.directory);
            self.ready = None;
            return Ok(CodexResumePoll::ChangedDuringRead);
        }
        self.qualified.revalidate()?;
        if self.scan.offset < stamp.len {
            return Ok(CodexResumePoll::Verifying);
        }
        // DirectoryHistory observes only complete records; an EOF partial cannot
        // alter this context or acknowledge any bytes.
        let context = CodexResumeDirectory {
            context: self.scan.context.history.context.clone(),
            prior: self.scan.prior.clone(),
        };
        self.ready = Some(context.clone());
        self.scan.context.release_eof_bytes();
        Ok(CodexResumePoll::Ready(context))
    }
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
}
impl Scan {
    fn new(header: &Path) -> Self {
        Self {
            offset: 0,
            context: ContextReconstruction::new(header),
            prior: DirectoryHistory::new(header)
                .context
                .directory()
                .map(Path::to_owned),
        }
    }
}
