use crate::{
    disk::{Generation, Owner},
    *,
};
use jj_store_adapters::{Batch, Checkpoint, Context, Identity, Projection, Snapshot};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, RwLock, atomic::Ordering},
};

/// Native extraction belongs behind an owned service interface. Implementations
/// read only the supplied selected snapshot and retain its bounded authorization.
#[async_trait::async_trait]
pub trait Extractor: Send + Sync {
    /// Configured extractors bind persisted checkpoints to the inputs they use.
    fn validate_identity(&self, identity: &Identity) -> Result<()> {
        identity.validate()
    }
    async fn bootstrap(&mut self, snapshot: &Snapshot, context: &Context) -> Result<Update>;
    async fn update(&mut self, batch: &Batch, context: &Context) -> Result<Update>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    BeforeCommit,
    AfterCommit,
    BeforeReader,
    AfterReader,
}
/// One-shot deterministic crash-boundary injection; no wire exposure.
#[derive(Default)]
pub struct Faults(Mutex<Option<Fault>>);
impl Faults {
    pub fn once(&self, fault: Fault) {
        *self.0.lock().unwrap() = Some(fault);
    }
    pub(crate) fn check(&self, point: Fault) -> Result<()> {
        let mut fault = self.0.lock().unwrap();
        if *fault == Some(point) {
            *fault = None;
            return Err(Error::Projection);
        }
        Ok(())
    }
}
#[derive(Default)]
pub(crate) struct Serving {
    pub visible: Option<Arc<Generation>>,
    pub durable: Option<Checkpoint>,
    pub degraded: bool,
}
pub struct TantivyProjection<E> {
    extractor: E,
    owner: Arc<Owner>,
    limits: Limits,
    serving: Arc<RwLock<Serving>>,
    faults: Arc<Faults>,
}
impl<E: Extractor> TantivyProjection<E> {
    pub fn open(root: PathBuf, extractor: E, limits: Limits) -> Result<(Self, Search)> {
        limits.validate()?;
        let owner = Owner::acquire(root)?;
        let serving = Arc::new(RwLock::new(Serving::default()));
        let search = Search::new(serving.clone(), owner.clone());
        Ok((
            Self {
                extractor,
                owner,
                limits,
                serving,
                faults: Arc::default(),
            },
            search,
        ))
    }
    pub fn faults(&self) -> Arc<Faults> {
        self.faults.clone()
    }
    fn failed(&self) {
        self.serving.write().unwrap().degraded = true;
    }
    async fn recover_inner(&self, context: Context) -> Result<Option<Arc<Generation>>> {
        let owner = self.owner.clone();
        let limits = self.limits;
        tokio::task::spawn_blocking(move || disk::recover(owner, limits, &context))
            .await
            .map_err(|_| Error::Unavailable)?
    }
    fn activate(&self, generation: Arc<Generation>) -> Result<()> {
        self.serving.write().unwrap().durable = Some(generation.checkpoint.clone());
        let mut state = self.serving.write().unwrap();
        if state
            .visible
            .as_ref()
            .is_none_or(|g| g.checkpoint != generation.checkpoint)
        {
            self.faults.check(Fault::BeforeReader)?;
            if let Some(old) = state.visible.replace(generation) {
                old.retired.store(true, Ordering::SeqCst);
            }
        }
        state.degraded = false;
        Ok(())
    }
    async fn write(
        &self,
        previous: Option<Arc<Generation>>,
        snapshot: &Snapshot,
        identity: &Identity,
        update: Update,
        context: Context,
    ) -> Result<Checkpoint> {
        let owner = self.owner.clone();
        let limits = self.limits;
        let identity = identity.clone();
        let source = snapshot.position.clone();
        let faults = self.faults.clone();
        let generation = tokio::task::spawn_blocking(move || {
            disk::commit(
                owner,
                limits,
                &context,
                disk::CommitInput {
                    previous,
                    identity,
                    source,
                    update,
                },
                &faults,
            )
        })
        .await
        .map_err(|_| Error::Unavailable)??;
        let checkpoint = generation.checkpoint.clone();
        self.activate(generation)?;
        self.faults.check(Fault::AfterReader)?;
        Ok(checkpoint)
    }
}
#[async_trait::async_trait]
impl<E: Extractor> Projection for TantivyProjection<E> {
    fn validate_identity(&self, identity: &Identity) -> Result<()> {
        if identity.implementation != IMPLEMENTATION || identity.schema != SCHEMA_VERSION {
            return Err(Error::Invalid);
        }
        self.extractor.validate_identity(identity)
    }
    fn supports_snapshots(&self) -> bool {
        true
    }
    async fn restore_snapshot(
        &mut self,
        snapshot: &Snapshot,
        identity: &Identity,
        context: Context,
    ) -> Result<Option<Checkpoint>> {
        let result =
            snapshots::restore(self.owner.clone(), self.limits, snapshot, identity, context).await;
        let generation = match result {
            Ok(generation) => generation,
            Err(Error::Invalid | Error::Corrupt | Error::Unavailable | Error::Projection) => {
                return Ok(None);
            }
            Err(error) => return Err(error),
        };
        let Some(generation) = generation else {
            return Ok(None);
        };
        let checkpoint = generation.checkpoint.clone();
        self.activate(generation)?;
        Ok(Some(checkpoint))
    }
    async fn publish_snapshot(
        &mut self,
        snapshot: &Snapshot,
        checkpoint: &Checkpoint,
        identity: &Identity,
        context: Context,
    ) -> Result<()> {
        snapshots::publish(
            self.owner.clone(),
            self.limits,
            snapshot,
            checkpoint,
            identity,
            context,
        )
        .await
    }
    async fn recover(&mut self, context: Context) -> Result<Option<Checkpoint>> {
        let result = async {
            let recovered = self.recover_inner(context).await?;
            let compatible = match recovered {
                Some(generation) => match self.validate_identity(&generation.checkpoint.identity) {
                    Ok(()) => Some(generation),
                    Err(Error::Invalid) => None,
                    Err(error) => return Err(error),
                },
                None => None,
            };
            if let Some(generation) = compatible {
                let checkpoint = generation.checkpoint.clone();
                self.activate(generation)?;
                Ok(Some(checkpoint))
            } else {
                let mut state = self.serving.write().unwrap();
                state.durable = None;
                // Keep any pinned readers alive, but prevent new requests from
                // treating a missing artifact as a recovered serving index.
                if let Some(old) = state.visible.take() {
                    old.retired.store(true, Ordering::SeqCst);
                }
                Ok(None)
            }
        }
        .await;
        if result.is_err() {
            self.failed();
        }
        result
    }
    async fn bootstrap(
        &mut self,
        snapshot: Snapshot,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint> {
        let result = async {
            self.validate_identity(identity)?;
            let update = self.extractor.bootstrap(&snapshot, &context).await?;
            if !update.delete.is_empty() {
                return Err(Error::Invalid);
            }
            self.write(None, &snapshot, identity, update, context).await
        }
        .await;
        if result.is_err() {
            self.failed();
        }
        result
    }
    async fn apply(
        &mut self,
        batch: Batch,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint> {
        let result = async {
            self.validate_identity(identity)?;
            let previous = self
                .recover_inner(context.clone())
                .await?
                .ok_or(Error::Corrupt)?;
            if previous.checkpoint.identity != *identity || previous.payload.source != batch.before
            {
                return Err(Error::Corrupt);
            }
            let update = self.extractor.update(&batch, &context).await?;
            self.write(Some(previous), &batch.snapshot, identity, update, context)
                .await
        }
        .await;
        if result.is_err() {
            self.failed();
        }
        result
    }
}
