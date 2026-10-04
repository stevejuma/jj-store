//! Bounded complete-record polling and acknowledged-prefix verification.
use super::*;
use std::{
    fs::File,
    io::{Read as _, Seek as _},
};
impl CodexTranscriptReader {
    pub fn poll(&mut self) -> CaptureResult<TranscriptPoll> {
        if let Some(pending) = &self.pending {
            return Ok(TranscriptPoll::Ready(Box::new(pending.batch.clone())));
        }
        let (mut file, header, stamp) =
            file::inspect_bound(self.profile, &self.path, self.physical_identity)?;
        if header.witness != self.header.witness || stamp.len < self.offset {
            self.header = header;
            self.new_generation()?;
            self.stamp = stamp.clone();
        } else if self.offset > 0
            && (self.verification.as_ref().is_some_and(|v| v.stamp != stamp)
                || (stamp != self.stamp && self.verification.is_none()))
        {
            self.start_verification(stamp.clone());
        }
        if self.verification.is_some() {
            let complete = self.verify(&mut file)?;
            if !file::unchanged(&file, &self.path, &stamp)? {
                if self.offset > 0 {
                    self.start_verification(stamp);
                }
                return Ok(TranscriptPoll::ChangedDuringRead);
            }
            if !complete {
                return Ok(TranscriptPoll::Verifying);
            }
            self.stamp = stamp.clone();
        }
        // A delayed partial write or rewrite cannot splice old partial bytes into
        // newly read bytes. Only complete, acknowledged prefixes survive it.
        if self.scan.as_ref().is_some_and(|scan| scan.stamp != stamp) {
            self.scan = None;
        }
        let scan = self.scan.get_or_insert_with(|| Scan {
            stamp: stamp.clone(),
            offset: self.offset,
            hash: self.hash.clone(),
            line: Vec::new(),
            oversized: false,
            history: self.history.clone(),
        });
        file.seek(std::io::SeekFrom::Start(scan.offset))?;
        let mut budget = MAX_SCAN_BYTES;
        let mut records = Vec::new();
        let mut last_complete = None;
        let mut directory_context = None;
        let mut buffer = [0u8; 64 * 1024];
        'read: while budget > 0 && records.len() < MAX_BATCH_RECORDS {
            let limit = budget.min(buffer.len());
            let amount = file.read(&mut buffer[..limit])?;
            if amount == 0 {
                break;
            }
            budget -= amount;
            let mut consumed = 0;
            while consumed < amount {
                let rest = &buffer[consumed..amount];
                let length = rest
                    .iter()
                    .position(|b| *b == b'\n')
                    .map_or(rest.len(), |i| i + 1);
                let part = &rest[..length];
                scan.hash.update(part);
                scan.offset = scan
                    .offset
                    .checked_add(length as u64)
                    .ok_or("transcript offset limit")?;
                if !scan.oversized {
                    if scan.line.len() + part.len() > MAX_RECORD_BYTES {
                        scan.line.clear();
                        scan.oversized = true;
                    } else {
                        scan.line.extend_from_slice(part);
                    }
                }
                consumed += length;
                if part.last() == Some(&b'\n') {
                    let sequence = self
                        .sequence
                        .checked_add(records.len() as u64 + 1)
                        .ok_or("transcript sequence limit")?;
                    let bytes = if scan.oversized {
                        None
                    } else {
                        Some(std::mem::take(&mut scan.line))
                    };
                    scan.history.observe(bytes.as_deref());
                    if directory_context
                        .as_ref()
                        .is_some_and(|old| old != &scan.history.context)
                    {
                        break 'read;
                    }
                    directory_context.get_or_insert_with(|| scan.history.context.clone());
                    records.push(RawTranscriptRecord {
                        sequence,
                        bytes,
                        record_key: self
                            .profile
                            .is_cursor()
                            .then(|| scan.hash.clone().finalize().into()),
                    });
                    scan.oversized = false;
                    last_complete = Some((scan.offset, scan.hash.clone(), scan.history.clone()));
                    if records.len() >= MAX_BATCH_RECORDS {
                        break 'read;
                    }
                }
            }
        }
        if !file::unchanged(&file, &self.path, &stamp)? {
            self.scan = None;
            return Ok(TranscriptPoll::ChangedDuringRead);
        }
        let Some((offset, hash, history)) = last_complete else {
            return Ok(TranscriptPoll::Waiting);
        };
        let prefix: [u8; 32] = hash.clone().finalize().into();
        if self.generation < self.generation_floor {
            self.continue_generation()?;
        }
        let mut id = Sha256::new_with_prefix(b"jj-store/codex-transcript-batch/v1\0");
        id.update(self.source.as_bytes());
        id.update(self.generation.to_be_bytes());
        id.update(self.header.witness);
        id.update(offset.to_be_bytes());
        id.update(prefix);
        let digest = id.finalize();
        let batch = CodexTranscriptBatch {
            batch_id: CaptureBatchId::new(digest[..16].try_into().expect("digest length")),
            cursor: CaptureCursorV1 {
                schema_version: 1,
                integration: self.profile.integration(),
                source_instance_id: self.source,
                generation: self.generation,
                parser_version: self.parser(),
                native_session_key: Some(self.header.session.clone()),
                position: SourcePositionV1::JsonLines {
                    complete_byte_offset: offset,
                    prefix_sha256: prefix,
                },
            },
            metadata: ReaderCaptureMetadataV1 {
                schema_version: 1,
                generation_digest: self.header.witness,
                opencode_snapshot: None,
            },
            records: Arc::new(records),
            directory_context: Arc::new(directory_context.expect("complete contextual record")),
        };
        self.pending = Some(Pending {
            last_sequence: batch.records.last().expect("complete record").sequence,
            batch: batch.clone(),
            hash,
            stamp,
            history,
        });
        self.scan = None;
        Ok(TranscriptPoll::Ready(Box::new(batch)))
    }
    fn verify(&mut self, file: &mut File) -> CaptureResult<bool> {
        let state = self.verification.as_mut().expect("verification state");
        file.seek(std::io::SeekFrom::Start(state.offset))?;
        let mut buffer = [0u8; 64 * 1024];
        let mut budget = MAX_VERIFY_BYTES as u64;
        while budget > 0 && state.offset < self.offset {
            let limit = budget
                .min(self.offset - state.offset)
                .min(buffer.len() as u64) as usize;
            let amount = file.read(&mut buffer[..limit])?;
            if amount == 0 {
                self.new_generation()?;
                return Ok(true);
            }
            state.hash.update(&buffer[..amount]);
            state.context.feed(&buffer[..amount]);
            state.offset += amount as u64;
            budget -= amount as u64;
        }
        if state.offset < self.offset {
            return Ok(false);
        }
        if <[u8; 32]>::from(state.hash.clone().finalize()) != state.expected
            || !state.context.complete()
        {
            self.new_generation()?;
        } else {
            self.hash = state.hash.clone();
            self.history = state.context.history.clone();
            self.verification = None;
        }
        Ok(true)
    }
}
