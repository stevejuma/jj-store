use super::*;

#[derive(Serialize, JsonSchema)]
#[schemars(rename = "ApplicationFreshnessFrame")]
pub(super) struct Frame {
    pub(super) id: String,
    pub(super) event: Event,
}
pub(super) enum Delivery {
    Event(Frame),
    Heartbeat,
}
pub(super) struct Subscription {
    state: Arc<State>,
    auth: authorization::Authorization,
    leases: BTreeMap<Option<m::RepositoryId>, monitors::Lease>,
    updates: tokio::sync::watch::Receiver<u64>,
    revision: u64,
    pending: VecDeque<Frame>,
    closed: bool,
    heartbeat: Instant,
}
impl Subscription {
    async fn synchronize(&mut self) -> Result<()> {
        let repositories = self.auth.repositories(&self.state).await?;
        self.leases.retain(|repo, _| {
            repo.is_none() || repositories.iter().any(|(id, _)| Some(*id) == *repo)
        });
        for (repository, local) in repositories {
            if !self.leases.contains_key(&Some(repository)) {
                let lease = self
                    .state
                    .event_monitors
                    .acquire(&self.state, Some(repository), local)
                    .await?;
                self.leases.insert(Some(repository), lease);
            }
        }
        Ok(())
    }
    fn witnesses(&self) -> Vec<Change> {
        self.state
            .freshness
            .snapshot()
            .1
            .into_iter()
            .filter(|change| self.auth.visible(change))
            .collect()
    }
    fn frame(&self, revision: u64, _name: &str, event: Event) -> Result<Frame> {
        Ok(Frame {
            id: self.auth.cursor(&self.state, revision),
            event,
        })
    }
    async fn load(&mut self) -> Result<()> {
        let Some(records) = self.state.freshness.after(self.revision) else {
            self.revision = self.state.freshness.snapshot().0;
            self.pending.push_back(self.frame(
                self.revision,
                "reset",
                Event::Reset {
                    version: 1,
                    reason: "replay_window",
                    witnesses: self.witnesses(),
                },
            )?);
            self.closed = true;
            return Ok(());
        };
        if records
            .iter()
            .any(|(_, c)| matches!(c.domain, Domain::Discovery | Domain::Workspaces))
        {
            self.synchronize().await?;
        }
        for (revision, change) in records {
            self.revision = revision;
            if self.auth.visible(&change) {
                self.auth.authorize(&self.state, change.repository).await?;
                self.pending.push_back(self.frame(
                    revision,
                    "changed",
                    Event::Changed { version: 1, change },
                )?);
            }
        }
        Ok(())
    }
}

impl Subscription {
    pub(super) async fn open(
        state: Arc<State>,
        headers: &HeaderMap,
        after: Option<&str>,
    ) -> Result<Self> {
        let auth = authorization::Authorization::new(&state, headers)?;
        // Install the listener before any capture so a racing publication stays replayable.
        let updates = state.freshness.subscribe();
        let mut subscription = Subscription {
            state,
            auth,
            leases: BTreeMap::new(),
            updates,
            revision: 0,
            pending: VecDeque::new(),
            closed: false,
            heartbeat: Instant::now() + Duration::from_secs(15),
        };
        subscription.synchronize().await?;
        let lease = subscription
            .state
            .event_monitors
            .acquire(&subscription.state, None, false)
            .await?;
        subscription.leases.insert(None, lease);
        subscription.synchronize().await?;
        let current = subscription.state.freshness.snapshot().0;
        match after
            .map(|cursor| subscription.auth.resume(&subscription.state, cursor))
            .transpose()?
        {
            Some(Some(revision)) => {
                subscription.revision = revision;
                subscription.load().await?;
            }
            Some(None) => {
                subscription.revision = current;
                subscription.pending.push_back(subscription.frame(
                    current,
                    "reset",
                    Event::Reset {
                        version: 1,
                        reason: "generation_or_scope",
                        witnesses: subscription.witnesses(),
                    },
                )?);
                subscription.closed = true;
            }
            None => subscription.revision = current,
        }
        if !subscription.closed {
            subscription.pending.push_back(subscription.frame(
                subscription.revision,
                "ready",
                Event::Ready {
                    version: 1,
                    witnesses: subscription.witnesses(),
                },
            )?);
        }
        Ok(subscription)
    }
    pub(super) async fn next(&mut self) -> Result<Option<Delivery>> {
        loop {
            if !self.closed && Instant::now() >= self.heartbeat {
                self.auth.authorize(&self.state, None).await?;
                self.heartbeat = Instant::now() + Duration::from_secs(15);
                return Ok(Some(Delivery::Heartbeat));
            }
            if let Some(frame) = self.pending.pop_front() {
                self.auth.authorize(&self.state, None).await?;
                return Ok(Some(Delivery::Event(frame)));
            }
            if self.closed {
                return Ok(None);
            }
            tokio::select! {
                result = self.updates.changed() => {
                    result.map_err(|_| Error::unavailable())?;
                    self.load().await?;
                }
                _ = tokio::time::sleep_until(self.heartbeat) => {}
            }
        }
    }
}
