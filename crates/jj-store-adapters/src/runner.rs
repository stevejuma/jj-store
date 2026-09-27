use crate::{context::Lease, *};
use async_trait::async_trait;
use std::{path::Path, sync::Arc};
use tokio::sync::{Semaphore, watch};

/// Implementations own transactional destination durability and replay safety.
/// Recover verifies actual local artifacts, rather than a remote cursor/cache.
/// Blocking workers must keep a Context clone until they exit, including after
/// timeout/cancellation, so they retain admission and the local writer lease.
#[async_trait]
pub trait Projection: Send {
    /// Reject incompatible implementation/configuration before any runner I/O.
    fn validate_identity(&self, identity: &Identity) -> Result<()> {
        identity.validate()
    }
    async fn recover(&mut self, context: Context) -> Result<Option<Checkpoint>>;
    async fn bootstrap(
        &mut self,
        snapshot: Snapshot,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint>;
    async fn apply(
        &mut self,
        batch: Batch,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint>;
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Health {
    pub repository_position: Option<Position>,
    pub applied: Option<Checkpoint>,
    pub remote: Option<Checkpoint>,
    pub lag: Option<u64>,
    pub degraded: bool,
    pub last_error: Option<Error>,
}
pub struct Runner<P> {
    projection: P,
    source: Source,
    identity: Identity,
    limits: Limits,
    lease: Arc<Lease>,
    slots: Arc<Semaphore>,
    health: watch::Sender<Health>,
}
impl<P: Projection> Runner<P> {
    pub fn new(
        source: Source,
        identity: Identity,
        projection: P,
        owner_root: &Path,
        limits: Limits,
    ) -> Result<Self> {
        identity.validate()?;
        projection.validate_identity(&identity)?;
        limits.validate()?;
        let lease = Lease::acquire(owner_root, source.repository(), &identity.name)?;
        Ok(Self {
            source,
            identity,
            projection,
            limits,
            lease,
            slots: Arc::new(Semaphore::new(1)),
            health: watch::channel(Health::default()).0,
        })
    }
    /// Wait for work retained by canceled/timed-out callbacks to release its
    /// context. The caller stops run/step first and supplies a shutdown deadline.
    pub async fn wait_idle(&self) -> Result<()> {
        let _permit = self.slots.acquire().await.map_err(|_| Error::Unavailable)?;
        Ok(())
    }
    pub fn subscribe(&self) -> watch::Receiver<Health> {
        self.health.subscribe()
    }
    pub fn health(&self) -> Health {
        self.health.borrow().clone()
    }
    pub async fn step(&mut self, cancel: CancellationToken) -> Result<Health> {
        let context = Context::new(self.lease.clone(), self.slots.clone(), self.limits, cancel);
        let result = match context {
            Ok(context) => context.until(self.step_inner(&context)).await,
            Err(e) => Err(e),
        };
        self.health.send_modify(|health| {
            health.degraded = result.is_err();
            health.last_error = result.as_ref().err().cloned();
        });
        result.map(|()| self.health())
    }
    async fn step_inner(&mut self, context: &Context) -> Result<()> {
        let source = self.source.capture(context).await?;
        let current = source.position();
        self.health.send_modify(|health| {
            health.repository_position = Some(current.clone());
            health.lag = health
                .applied
                .as_ref()
                .filter(|a| a.source.epoch == current.epoch)
                .map(|a| current.position.0.saturating_sub(a.source.position.0));
        });
        let remote = progress::read(&self.source, &self.identity, context, current.epoch).await?;
        if let Some(checkpoint) = &remote.checkpoint
            && checkpoint.source.epoch == current.epoch
        {
            source.validate_position(&checkpoint.source).await?;
        }
        self.health
            .send_modify(|health| health.remote = remote.checkpoint.clone());
        let recovered = self.projection.recover(context.clone()).await?;
        let mut local = None;
        if let Some(checkpoint) = recovered {
            checkpoint.validate()?;
            if checkpoint.source.repository != current.repository {
                return Err(Error::Corrupt);
            }
            if checkpoint.identity == self.identity && checkpoint.source.epoch == current.epoch {
                source.validate_position(&checkpoint.source).await?;
                local = Some(checkpoint);
            }
        }
        self.set_applied(local.as_ref(), &current);
        let (target, checkpoint) = if let Some(local) = local {
            if let Some(batch) = source.batch(&local.source).await? {
                let target = batch.snapshot.position.clone();
                let checkpoint = self
                    .projection
                    .apply(batch, &self.identity, context.clone())
                    .await?;
                (target, checkpoint)
            } else {
                (local.source.clone(), local)
            }
        } else {
            let snapshot = source.bootstrap().await?;
            let target = snapshot.position.clone();
            let checkpoint = self
                .projection
                .bootstrap(snapshot, &self.identity, context.clone())
                .await?;
            (target, checkpoint)
        };
        checkpoint.validate()?;
        if checkpoint.identity != self.identity || checkpoint.source != target {
            return Err(Error::Corrupt);
        }
        // A successful callback alone cannot advance the cursor: the projection
        // must already be able to recover the same durable local artifact.
        if self.projection.recover(context.clone()).await?.as_ref() != Some(&checkpoint) {
            return Err(Error::Corrupt);
        }
        source.store.authorize().await?;
        self.set_applied(Some(&checkpoint), &current);
        let selected = progress::persist(&self.source, &remote, &checkpoint, context).await?;
        let final_remote =
            progress::read(&self.source, &self.identity, context, current.epoch).await?;
        if final_remote.checkpoint.as_ref() != Some(&selected) {
            return Err(Error::CursorConflict);
        }
        self.health
            .send_modify(|health| health.remote = final_remote.checkpoint);
        Ok(())
    }
    fn set_applied(&self, applied: Option<&Checkpoint>, current: &Position) {
        self.health.send_modify(|health| {
            health.applied = applied.cloned();
            health.lag = applied.map(|a| current.position.0.saturating_sub(a.source.position.0));
        });
    }
    /// Poll selected authority even without wake hints. Poison records are
    /// retried with bounded backoff; repository publication is never called.
    pub async fn run(&mut self, cancel: CancellationToken) -> Result<()> {
        let mut delay = self.limits.poll;
        loop {
            if cancel.is_cancelled() {
                return Ok(());
            }
            match self.step(cancel.clone()).await {
                Ok(_) => delay = self.limits.poll,
                Err(Error::Canceled) => return Ok(()),
                Err(_) => delay = delay.saturating_mul(2).min(self.limits.max_backoff),
            }
            tokio::select! { biased;
                _=cancel.cancelled()=>return Ok(()),
                _=tokio::time::sleep(delay)=>{}
            }
        }
    }
}
