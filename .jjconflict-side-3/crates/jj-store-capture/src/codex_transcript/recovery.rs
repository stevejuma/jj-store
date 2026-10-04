//! Bounded read-only reconstruction of one exact sanitized queued range.
use super::{context::ContextReconstruction, file, *};
use crate::{agent_capture::PendingCaptureV1, codex_discovery::CodexSourceCandidate};
use std::{
    collections::VecDeque,
    io::{Read as _, Seek as _},
};
mod identity;
use identity::Target;

// Device paths and context history intentionally have no Debug/serialization.
#[derive(Clone)]
pub struct RecoveredCodexDirectory {
    context: CodexDirectoryContext,
    prior: Option<PathBuf>,
    omission_only: bool,
}
impl RecoveredCodexDirectory {
    pub fn directory(&self) -> Option<&Path> {
        self.context.directory()
    }
    /// This candidate grants no body admission or repository authorization.
    pub fn omission_anchor(&self) -> Option<&Path> {
        self.omission_only
            .then_some(self.prior.as_deref())
            .flatten()
    }
    pub fn omission_only(&self) -> bool {
        self.omission_only
    }
}
pub enum CodexRecoveryPoll {
    Verifying,
    ChangedDuringRead,
    Ready(RecoveredCodexDirectory),
}
struct RecordContext {
    context: CodexDirectoryContext,
    prior: Option<PathBuf>,
}
struct Reconstruction {
    stamp: Stamp,
    offset: u64,
    hash: Sha256,
    context: ContextReconstruction,
    last_known: Option<PathBuf>,
    recent: VecDeque<RecordContext>,
    count: u64,
}
pub struct CodexBatchRecovery {
    candidate: CodexSourceCandidate,
    profile: CaptureProfile,
    header: Header,
    target: Target,
    scan: Option<Reconstruction>,
    ready: Option<(Stamp, RecoveredCodexDirectory)>,
}
impl CodexBatchRecovery {
    pub(crate) fn open(
        candidate: CodexSourceCandidate,
        profile: CaptureProfile,
        source: SourceInstanceId,
        entry: &PendingCaptureV1,
    ) -> CaptureResult<Self> {
        if !candidate.revalidate()? {
            return Err("queued Codex source unavailable".into());
        }
        let (_, header, _) =
            file::inspect_bound(profile, candidate.path(), Some(candidate.identity()))?;
        let target = identity::target(entry, profile, source, &header)?;
        if !candidate.revalidate()? {
            return Err("queued Codex source unavailable".into());
        }
        Ok(Self {
            candidate,
            profile,
            header,
            target,
            scan: None,
            ready: None,
        })
    }
    /// At most 4 MiB of prefix work; no body records are returned or persisted.
    pub fn poll(&mut self) -> CaptureResult<CodexRecoveryPoll> {
        if !self.candidate.revalidate()? {
            return Err("queued Codex source unavailable".into());
        }
        let (mut file, header, stamp) = file::inspect_bound(
            self.profile,
            self.candidate.path(),
            Some(self.candidate.identity()),
        )?;
        if header.witness != self.header.witness || stamp.len < self.target.offset {
            return Err("queued Codex generation unavailable".into());
        }
        if let Some((verified, context)) = &self.ready
            && *verified == stamp
            && self.candidate.revalidate()?
        {
            return Ok(CodexRecoveryPoll::Ready(context.clone()));
        }
        self.ready = None;
        if self.scan.as_ref().is_none_or(|scan| scan.stamp != stamp) {
            self.scan = Some(Reconstruction {
                stamp: stamp.clone(),
                offset: 0,
                hash: Sha256::new(),
                context: ContextReconstruction::new(&header.directory),
                last_known: DirectoryHistory::new(&header.directory)
                    .context
                    .directory()
                    .map(Path::to_owned),
                recent: VecDeque::new(),
                count: 0,
            });
        }
        let scan = self.scan.as_mut().expect("reconstruction initialized");
        file.seek(std::io::SeekFrom::Start(scan.offset))?;
        let mut buffer = [0u8; 64 * 1024];
        let mut budget = MAX_VERIFY_BYTES as u64;
        while budget > 0 && scan.offset < self.target.offset {
            let limit = budget
                .min(self.target.offset - scan.offset)
                .min(buffer.len() as u64) as usize;
            let amount = file.read(&mut buffer[..limit])?;
            if amount == 0 {
                return Err("queued Codex prefix unavailable".into());
            }
            scan.hash.update(&buffer[..amount]);
            scan.context.feed_observed(&buffer[..amount], |context| {
                scan.recent.push_back(RecordContext {
                    context: context.clone(),
                    prior: scan.last_known.clone(),
                });
                if scan.recent.len() > self.target.records {
                    scan.recent.pop_front();
                }
                if let Some(directory) = context.directory() {
                    scan.last_known = Some(directory.to_owned());
                }
                scan.count += 1;
            });
            scan.offset += amount as u64;
            budget -= amount as u64;
        }
        if !file::unchanged(&file, self.candidate.path(), &stamp)?
            || !self.candidate.revalidate()?
        {
            self.scan = None;
            return Ok(CodexRecoveryPoll::ChangedDuringRead);
        }
        if scan.offset < self.target.offset {
            return Ok(CodexRecoveryPoll::Verifying);
        }
        if !scan.context.complete()
            || scan.recent.len() != self.target.records
            || scan.count > self.target.last_sequence
            || <[u8; 32]>::from(scan.hash.clone().finalize()) != self.target.prefix
        {
            return Err("queued Codex prefix or record range mismatch".into());
        }
        let first = scan.recent.front().expect("validated nonempty range");
        if scan.recent.iter().any(|row| row.context != first.context) {
            return Err("queued Codex range has mixed directory contexts".into());
        }
        let context = RecoveredCodexDirectory {
            context: first.context.clone(),
            prior: first.prior.clone(),
            omission_only: self.target.omission_only,
        };
        self.ready = Some((stamp, context.clone()));
        self.scan = None;
        Ok(CodexRecoveryPoll::Ready(context))
    }
    /// Check that the exact verified snapshot still holds immediately before use.
    pub fn revalidate(&self) -> CaptureResult<bool> {
        let Some((verified, _)) = &self.ready else {
            return Ok(false);
        };
        if !self.candidate.revalidate()? {
            return Ok(false);
        }
        let (_, header, stamp) = file::inspect_bound(
            self.profile,
            self.candidate.path(),
            Some(self.candidate.identity()),
        )?;
        Ok(header.witness == self.header.witness
            && stamp == *verified
            && self.candidate.revalidate()?)
    }
}
