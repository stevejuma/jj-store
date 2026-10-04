//! Offers accepted intent only; native admission and body work remain source-owned.
use super::*;
use crate::capture_worker::executor::OwnedWork;

#[derive(Clone)]
pub struct CaptureImportRecovery {
    manager: Manager,
    after: u64,
}
impl Manager {
    /// Retained metadata capability only. Creation performs no filesystem work.
    pub fn import_recovery(&self) -> Option<CaptureImportRecovery> {
        self.0.binding.as_ref()?.imports.as_ref()?;
        Some(CaptureImportRecovery {
            manager: self.clone(),
            after: 0,
        })
    }
}
impl CaptureImportRecovery {
    fn next(
        &mut self,
        permit: tokio::sync::OwnedMutexGuard<()>,
    ) -> Result<Option<CaptureImportDispatch>, Error> {
        let manager = &self.manager;
        let actor = manager.0.authority.actor();
        if !manager.0.authority.admit(actor)? {
            return Err(Error::Disabled);
        }
        let binding = manager.binding()?;
        let runtime = binding.runtime.get().ok_or(Error::Unavailable)?;
        runtime.admit_import().map_err(|_| Error::Unavailable)?;
        let resources = manager.import_resources()?;
        resources.check()?;
        let candidate = binding
            .journal
            .read_then(|policy, check| {
                check().map_err(report::failure)?;
                if policy.generation != binding.generation || policy.paused {
                    return Err(Error::Unavailable);
                }
                let sources = runtime.source_snapshot().map_err(|_| Error::Unavailable)?;
                let receipts = resources
                    .journal
                    .entries()
                    .map_err(import_policy::import_failure)?;
                let candidate = receipts.into_iter().find(|receipt| {
                    receipt.ordinal > self.after
                        && !sources.sources.iter().any(|source| {
                            source.import_request
                                == Some((receipt.intent.principal, receipt.intent.request))
                        })
                });
                check().map_err(report::failure)?;
                if !manager.0.authority.admit(actor)? {
                    return Err(Error::Disabled);
                }
                resources.check()?;
                Ok(candidate)
            })
            .map_err(report::failure)??;
        let Some(receipt) = candidate else {
            self.after = 0;
            return Ok(None);
        };
        self.after = receipt.ordinal;
        let policy =
            manager.begin_import_with_permit(receipt.intent, resources.journal.clone(), permit)?;
        if policy.selected().is_none() {
            return Err(Error::Unavailable);
        }
        policy.into_dispatch().map(Some)
    }
}
#[async_trait::async_trait]
impl OwnedWork for CaptureImportRecovery {
    type Command = ();
    type Output = Result<Option<CaptureImportDispatch>, Error>;
    async fn step(&mut self, _: ()) -> Self::Output {
        // Queue behind inspection instead of losing every polling race. The owned
        // worker's cancellation/deadline governs this await and retains its lane.
        let permit = self.manager.0.gate.clone().lock_owned().await;
        self.next(permit)
    }
}
