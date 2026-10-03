//! Public commands schedule retained work, then observe durable acceptance separately.
use super::*;
use crate::capture_import_state::CaptureImportIntent;
use jj_store_model::RequestId;
use std::time::Duration;

impl Manager {
    pub(super) fn import_resources(&self) -> Result<&Arc<CaptureImportManagement>, Error> {
        let binding = self.binding()?;
        let resources = binding.imports.as_ref().ok_or(Error::Unavailable)?;
        let (path, installation) = self.0.authority.import_binding()?;
        if !resources
            .journal
            .belongs_to(&path, installation, binding.generation)
        {
            return Err(Error::Unavailable);
        }
        Ok(resources)
    }
    pub(super) async fn read_import_status(
        &self,
        actor: PrincipalId,
        query: CaptureImportQuery,
    ) -> Result<CaptureImportStatus, Error> {
        if query.request_id.as_bytes() == &[0; 16] {
            return Err(Error::Invalid);
        }
        let this = self.clone();
        let permit = self
            .0
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            if !this.0.authority.admit(actor)? {
                return Err(Error::Disabled);
            }
            let binding = this.binding()?;
            let resources = this.import_resources()?;
            resources.check()?;
            binding
                .journal
                .read_then(|policy, check| {
                    if policy.generation != binding.generation {
                        return Err(Error::Unavailable);
                    }
                    check().map_err(report::failure)?;
                    let snapshot = binding.runtime.get().and_then(|r| r.source_snapshot().ok());
                    let status = resources
                        .journal
                        .inspect_request(query.request_id, |selected| {
                            import_progress::project(
                                actor,
                                query.request_id,
                                policy.paused,
                                snapshot.as_deref(),
                                resources,
                                selected,
                            )
                        })
                        .map_err(import_policy::import_failure)?;
                    check().map_err(report::failure)?;
                    if !this.0.authority.admit(actor)? {
                        return Err(Error::Disabled);
                    }
                    resources.check()?;
                    Ok(status)
                })
                .map_err(report::failure)?
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
    pub(super) async fn request_import(
        &self,
        actor: PrincipalId,
        request: CaptureImportCommand,
    ) -> Result<CaptureImportOutcome, Error> {
        let generation: [u8; 16] = hex::decode(&request.generation)
            .ok()
            .and_then(|b| b.try_into().ok())
            .ok_or(Error::Invalid)?;
        if request.generation != hex::encode(generation) {
            return Err(Error::Invalid);
        }
        let intent = CaptureImportIntent {
            version: request.version,
            generation,
            principal: actor,
            request: request.request_id,
            expected_revision: request.expected_revision,
            selector: request.selector.clone(),
        };
        intent.validate().map_err(import_policy::import_failure)?;
        let this = self.clone();
        let prepared = tokio::task::spawn_blocking(move || {
            if !this.0.authority.admit(actor)? {
                return Err(Error::Disabled);
            }
            let journal = this.import_resources()?.journal.clone();
            let policy = this.begin_import(intent, journal)?;
            if policy.selected().is_some() {
                return Ok(None);
            }
            policy.into_dispatch().map(Some)
        })
        .await
        .map_err(|_| Error::Unknown)??;
        let replayed = prepared.is_none();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        if let Some(dispatch) = prepared {
            let runtime = self.binding()?.runtime.get().ok_or(Error::Unavailable)?;
            tokio::time::timeout_at(deadline, runtime.schedule_import(dispatch))
                .await
                .map_err(|_| Error::Unknown)?
                .map_err(|_| Error::Unknown)?;
        }
        // The source owner survives caller cancellation. Polling never enrolls or executes it.
        let status = self
            .await_acceptance(actor, request.request_id, deadline)
            .await?;
        let receipt = status.receipt.ok_or(Error::Unknown)?;
        if receipt.command != request {
            return Err(Error::Conflict);
        }
        Ok(CaptureImportOutcome {
            version: 1,
            replayed,
            receipt,
            progress: status.progress,
        })
    }
    async fn await_acceptance(
        &self,
        actor: PrincipalId,
        request_id: RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<CaptureImportStatus, Error> {
        tokio::time::timeout_at(deadline, async {
            loop {
                match self
                    .read_import_status(actor, CaptureImportQuery { request_id })
                    .await
                {
                    Ok(status) if status.receipt.is_some() => return Ok(status),
                    Ok(_) | Err(Error::Busy | Error::Unavailable) => {}
                    Err(_) => return Err(Error::Unknown),
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .map_err(|_| Error::Unknown)?
    }
}
