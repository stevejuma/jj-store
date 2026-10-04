//! Verify a retained reader checkpoint against current accepted bytes before reuse.
use super::*;
use crate::reader_progress::ReaderProgressV1;
use jj_agent::{IntegrationId, SourcePositionV1};
use sha2::{Digest as _, Sha256};

pub(super) struct ReaderCheck {
    end: u64,
    expected: [u8; 32],
    offset: u64,
    hash: Sha256,
    verified: bool,
}
impl CodexImportProbe {
    /// Shares this poll's 4-MiB byte allowance; retains only a cursor digest accumulator.
    /// False means another owned step is needed. Mismatch never resets old history.
    pub fn verify_reader_progress(
        &mut self,
        previous: Option<&ReaderProgressV1>,
    ) -> CaptureResult<bool> {
        if !self.revalidate()? {
            return Err("import reader snapshot unavailable".into());
        }
        let Some(previous) = previous else {
            self.reader = None;
            return Ok(true);
        };
        previous.validate()?;
        if previous.cursor.integration != IntegrationId::Codex
            || previous.metadata.generation_digest != self.qualified.generation_digest()
        {
            return Err("import reader generation differs".into());
        }
        let SourcePositionV1::JsonLines {
            complete_byte_offset: end,
            prefix_sha256: expected,
        } = previous.cursor.position
        else {
            return Err("import reader position differs".into());
        };
        if end > self.stamp.len {
            return Err("import retained prefix truncated".into());
        }
        if self.reader.as_ref().is_none_or(|r| r.offset > end) {
            self.reader = Some(ReaderCheck {
                end,
                expected,
                offset: 0,
                hash: Sha256::new(),
                verified: false,
            });
        }
        let reader = self.reader.as_mut().expect("retained reader verification");
        if reader.end != end || reader.expected != expected {
            // The probe clears this accumulator on every snapshot change. A
            // later cursor on the same snapshot can extend the verified bytes.
            reader.end = end;
            reader.expected = expected;
            reader.verified = false;
        }
        if reader.verified {
            return Ok(true);
        }
        let (mut file, header, stamp) = file::inspect_bound(
            self.qualified.profile,
            self.qualified.candidate.path(),
            Some(self.qualified.candidate.identity()),
        )?;
        if header.witness != self.qualified.header.witness || stamp != self.stamp {
            return Err("import reader snapshot changed".into());
        }
        file.seek(std::io::SeekFrom::Start(reader.offset))?;
        let mut bytes = [0; 64 * 1024];
        while self.remaining_bytes > 0 && reader.offset < end {
            let limit = self
                .remaining_bytes
                .min(end - reader.offset)
                .min(bytes.len() as u64) as usize;
            let amount = file.read(&mut bytes[..limit])?;
            if amount == 0 {
                return Err("import reader prefix unavailable".into());
            }
            reader.hash.update(&bytes[..amount]);
            reader.offset += amount as u64;
            self.remaining_bytes -= amount as u64;
        }
        if !file::unchanged(&file, self.qualified.candidate.path(), &stamp)? {
            return Err("import reader snapshot changed".into());
        }
        if reader.offset < end {
            return Ok(false);
        }
        let digest: [u8; 32] = reader.hash.clone().finalize().into();
        if digest != expected {
            return Err("import retained prefix changed".into());
        }
        reader.verified = true;
        Ok(true)
    }
}
