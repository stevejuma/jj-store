use super::*;
use jj_store_capture::codex_transcript::{CodexBatchRecovery, CodexRecoveryPoll};

pub(super) struct PendingRecovery {
    entry: PendingCaptureV1,
    probe: CodexBatchRecovery,
}
impl CodexCaptureSource {
    pub(super) async fn recover_step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<Option<CaptureSourceStep>> {
        let mut entries: Vec<_> = context
            .spool
            .pending()
            .map_err(|_| Error::Service("capture pending source unavailable"))?
            .into_iter()
            .filter(|entry| {
                entry.batch.integration == self.profile.integration
                    && entry.batch.source_instance_id == self.reader.source_instance_id()
            })
            .collect();
        entries.sort_by_key(|entry| {
            (
                entry.cursor.generation,
                entry.batch.observations[0].source_sequence,
                entry.batch.batch_id,
            )
        });
        let Some(entry) = entries.into_iter().next() else {
            self.recovery = None;
            return Ok(None);
        };
        self.recover_entry_step(entry, context, true)
            .await
            .map(Some)
    }
    pub(super) async fn recover_entry_step(
        &mut self,
        entry: PendingCaptureV1,
        context: &mut CaptureStepContext<'_>,
        serial: bool,
    ) -> Result<CaptureSourceStep> {
        self.recover_entry_guarded(entry, context, serial, &|| Ok(()))
            .await
    }
    pub(super) async fn recover_entry_guarded(
        &mut self,
        entry: PendingCaptureV1,
        context: &mut CaptureStepContext<'_>,
        serial: bool,
        check: &(dyn Fn() -> Result<()> + Sync),
    ) -> Result<CaptureSourceStep> {
        check()?;
        self.check_spool(context.spool)?;
        if self.recovery.as_ref().is_none_or(|old| old.entry != entry) {
            let probe = self
                .reader
                .recover_batch(&entry)
                .map_err(|_| Error::Service("capture queued proof unavailable"))?;
            self.recovery = Some(PendingRecovery { entry, probe });
        }
        let recovery = self.recovery.as_mut().expect("queued recovery prepared");
        if !self
            .reader
            .revalidate()
            .map_err(|_| Error::Service("capture queued source unavailable"))?
        {
            return Ok(CaptureSourceStep::Withheld);
        }
        let directory = match recovery
            .probe
            .poll()
            .map_err(|_| Error::Service("capture queued proof unavailable"))?
        {
            CodexRecoveryPoll::Verifying => return Ok(CaptureSourceStep::Verifying),
            CodexRecoveryPoll::ChangedDuringRead => {
                return Ok(CaptureSourceStep::ChangedDuringRead);
            }
            CodexRecoveryPoll::Ready(directory) => directory,
        };
        let proof = if let Some(path) = directory.directory() {
            match context
                .admission
                .inspect(
                    path.to_str()
                        .ok_or(Error::Service("capture queued directory unavailable"))?,
                )
                .await
            {
                Ok(proof) if proof.workspace.repository == recovery.entry.repository => Some(proof),
                Ok(_)
                | Err(
                    crate::capture_admission::Error::Unbound
                    | crate::capture_admission::Error::Limit,
                ) => None,
                Err(_) => return Ok(CaptureSourceStep::Withheld),
            }
        } else {
            None
        };
        let proof = match proof {
            Some(proof) => proof,
            None if directory.omission_only() => {
                let candidates = [
                    directory.omission_anchor().and_then(|path| path.to_str()),
                    self.reader.directory().to_str(),
                    Some(self.anchor.directory.as_str()),
                ];
                let mut matched = None;
                for fallback in candidates.into_iter().flatten() {
                    match context.admission.inspect(fallback).await {
                        Ok(proof) if proof.workspace.repository == recovery.entry.repository => {
                            matched = Some(proof);
                            break;
                        }
                        Ok(_)
                        | Err(
                            crate::capture_admission::Error::Unbound
                            | crate::capture_admission::Error::Limit,
                        ) => {}
                        Err(_) => return Ok(CaptureSourceStep::Withheld),
                    }
                }
                let Some(proof) = matched else {
                    return Ok(CaptureSourceStep::Withheld);
                };
                proof
            }
            None => return Ok(CaptureSourceStep::Withheld),
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        if !recovery
            .probe
            .revalidate()
            .map_err(|_| Error::Service("capture queued source unavailable"))?
            || !self
                .reader
                .revalidate()
                .map_err(|_| Error::Service("capture queued source unavailable"))?
        {
            return Ok(CaptureSourceStep::Withheld);
        }
        let entry = recovery.entry.clone();
        self.check_spool(context.spool)?;
        check()?;
        let publisher = context
            .authority
            .publisher(context.host, context.admission, &proof)
            .with_guard(check);
        let report = drain_capture_spool_entry(context.spool, &entry, &publisher)
            .await
            .map_err(|error| {
                Error::capture_spool(error, "capture recovered publication unavailable")
            })?;
        check()?;
        if !report.outcome_unknown {
            self.recovery = None;
            if serial {
                self.anchor = proof;
                self.needs_resume = true;
            }
        }
        Ok(CaptureSourceStep::Published(report))
    }
}
