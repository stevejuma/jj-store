//! Read-only activity since an armed boundary; no source allocation or durability.
use crate::{
    CaptureResult,
    codex_transcript::{MAX_VERIFY_BYTES, file},
    source_registry::QualifiedCodexSource,
};
use sha2::{Digest as _, Sha256};
use std::{
    io::{Read as _, Seek as _},
    path::{Path, PathBuf},
};
mod scan;
use scan::{Checkpoint, Scan};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityPoll {
    Waiting,
    Scanning,
    Verifying,
    Active,
}

/// An unforgeable current activity snapshot. Device paths/bodies have no encoding.
#[derive(Clone)]
pub struct ActiveCodexSource {
    pub(crate) qualified: QualifiedCodexSource,
    directory: PathBuf,
    stamp: file::Stamp,
}
impl ActiveCodexSource {
    /// Qualification alone grants no activity, source lease or directory authority.
    pub fn qualified(&self) -> &QualifiedCodexSource {
        &self.qualified
    }
    pub fn profile(&self) -> crate::profile::CaptureProfile {
        self.qualified.profile()
    }
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    pub fn native_session_key(&self) -> &str {
        self.qualified.native_session_key()
    }
    pub fn revalidate(&self) -> CaptureResult<()> {
        self.qualified.revalidate()?;
        let (_, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        if header.witness != self.qualified.header.witness || stamp != self.stamp {
            return Err("Codex activity snapshot changed".into());
        }
        self.qualified.revalidate()
    }
}
struct Verification {
    offset: u64,
    hash: Sha256,
}

pub struct CodexActivityProbe {
    qualified: QualifiedCodexSource,
    cutoff: u64,
    stamp: file::Stamp,
    scan: Scan,
    verification: Option<Verification>,
    ready: bool,
    unavailable: bool,
    initialized: bool,
    eof: bool,
}
impl CodexActivityProbe {
    pub fn arm(qualified: QualifiedCodexSource) -> CaptureResult<Self> {
        qualified.revalidate()?;
        let (_, header, stamp) = file::inspect_bound(
            qualified.profile,
            qualified.candidate.path(),
            Some(qualified.candidate.identity()),
        )?;
        if header.witness != qualified.header.witness {
            return Err("Codex activity header changed".into());
        }
        qualified.revalidate()?;
        Ok(Self {
            scan: Scan::new(qualified.directory()),
            cutoff: stamp.len,
            stamp,
            qualified,
            verification: None,
            ready: false,
            unavailable: false,
            initialized: false,
            eof: false,
        })
    }
    pub fn poll(&mut self) -> CaptureResult<ActivityPoll> {
        if self.unavailable {
            return Err("Codex activity probe unavailable; rearm required".into());
        }
        self.ready = false;
        self.qualified.revalidate()?;
        let (mut file, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        if header.witness != self.qualified.header.witness
            || stamp.len < self.cutoff
            || stamp.len < self.scan.complete.offset
        {
            self.unavailable = true;
            return Err("Codex activity source changed; rearm required".into());
        }
        if stamp != self.stamp {
            self.eof = false;
            // No baseline prefix proof exists yet. A changing initial snapshot
            // is conservatively historical; it cannot assert new activity.
            if !self.initialized {
                self.cutoff = stamp.len;
                self.scan = Scan::new(self.qualified.directory());
            }
            self.scan.discard_partial();
            self.verification = (self.scan.complete.offset > 0).then_some(Verification {
                offset: 0,
                hash: Sha256::new(),
            });
            self.stamp = stamp.clone();
        }
        if self.eof {
            self.ready = true;
            return Ok(self.result_at_eof());
        }
        if let Some(state) = &mut self.verification {
            file.seek(std::io::SeekFrom::Start(state.offset))?;
            let mut budget = MAX_VERIFY_BYTES as u64;
            let mut bytes = [0; 64 * 1024];
            while budget > 0 && state.offset < self.scan.complete.offset {
                let limit = budget
                    .min(self.scan.complete.offset - state.offset)
                    .min(bytes.len() as u64) as usize;
                let amount = file.read(&mut bytes[..limit])?;
                if amount == 0 {
                    self.unavailable = true;
                    return Err("Codex activity prefix unavailable".into());
                }
                state.hash.update(&bytes[..amount]);
                state.offset += amount as u64;
                budget -= amount as u64;
            }
            if !file::unchanged(&file, self.qualified.candidate.path(), &stamp)? {
                self.verification = Some(Verification {
                    offset: 0,
                    hash: Sha256::new(),
                });
                return Ok(ActivityPoll::Verifying);
            }
            if state.offset < self.scan.complete.offset {
                return Ok(ActivityPoll::Verifying);
            }
            if <[u8; 32]>::from(state.hash.clone().finalize()) != self.scan.complete.digest() {
                self.unavailable = true;
                return Err("Codex activity prefix changed; rearm required".into());
            }
            self.verification = None;
        }
        let before: Checkpoint = self.scan.complete.clone();
        let eof = self.scan.read(&mut file, self.cutoff)?;
        if !file::unchanged(&file, self.qualified.candidate.path(), &stamp)? {
            self.scan.complete = before;
            self.scan.discard_partial();
            return Ok(ActivityPoll::Verifying);
        }
        self.qualified.revalidate()?;
        self.initialized |= self.scan.reached(self.cutoff);
        self.eof = eof || self.scan.reached(stamp.len);
        if self.eof {
            self.scan.release_eof_bytes();
        }
        self.ready = true;
        Ok(if self.scan.complete.active.is_some() {
            ActivityPoll::Active
        } else if eof {
            ActivityPoll::Waiting
        } else {
            ActivityPoll::Scanning
        })
    }
    /// Metadata/header only. A changed snapshot requires a separately reserved
    /// body frame; this method never allocates or reads partial transcript rows.
    pub fn poll_idle(&mut self) -> CaptureResult<Option<ActivityPoll>> {
        self.ready = false;
        if self.unavailable || !self.eof {
            return Ok(None);
        }
        self.qualified.revalidate()?;
        let (_, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        if header.witness != self.qualified.header.witness {
            return Err("Codex activity header changed".into());
        }
        if stamp != self.stamp {
            return Ok(None);
        }
        self.qualified.revalidate()?;
        self.ready = true;
        Ok(Some(self.result_at_eof()))
    }
    pub fn is_compact(&self) -> bool {
        self.eof
    }
    /// Definitively invalidated prefix/truncation state needs a new historical
    /// boundary. Retryable qualification and I/O errors do not set this flag.
    pub fn requires_rearm(&self) -> bool {
        self.unavailable
    }
    fn result_at_eof(&self) -> ActivityPoll {
        if self.scan.complete.active.is_some() {
            ActivityPoll::Active
        } else {
            ActivityPoll::Waiting
        }
    }
    /// Does not consume the probe: append races can be verified and retried.
    pub fn proof(&self) -> CaptureResult<Option<ActiveCodexSource>> {
        if !self.ready || self.unavailable {
            return Ok(None);
        }
        let Some(directory) = &self.scan.complete.active else {
            return Ok(None);
        };
        let proof = ActiveCodexSource {
            qualified: self.qualified.clone(),
            directory: directory.clone(),
            stamp: self.stamp.clone(),
        };
        proof.revalidate()?;
        Ok(Some(proof))
    }
}
