use super::*;
use std::collections::BTreeMap;

type Factory<T> = Box<dyn Fn(&StoreContext) -> Result<Arc<T>> + Send + Sync>;
#[derive(Clone, Debug, Default)]
pub struct StoreConfig {
    implementations: BTreeMap<StoreKind, String>,
}
impl StoreConfig {
    pub fn all(name: &str) -> Self {
        Self {
            implementations: StoreKind::ALL
                .into_iter()
                .map(|kind| (kind, name.to_owned()))
                .collect(),
        }
    }
    pub fn enable(&mut self, kind: StoreKind, name: &str) {
        self.implementations.insert(kind, name.to_owned());
    }
    pub fn disable(&mut self, kind: StoreKind) {
        self.implementations.remove(&kind);
    }
    pub fn enabled(&self) -> BTreeSet<StoreKind> {
        self.implementations.keys().copied().collect()
    }
}
#[derive(Default)]
pub struct StoreFactories {
    agent: BTreeMap<String, Factory<dyn AgentStore>>,
    work: BTreeMap<String, Factory<dyn WorkStore>>,
    review: BTreeMap<String, Factory<dyn ReviewStore>>,
    thread: BTreeMap<String, Factory<dyn ThreadStore>>,
    reaction: BTreeMap<String, Factory<dyn ReactionStore>>,
}
macro_rules! register {
    ($method:ident, $field:ident, $trait:ident) => {
        pub fn $method(
            &mut self,
            name: &str,
            factory: impl Fn(&StoreContext) -> Result<Arc<dyn $trait>> + Send + Sync + 'static,
        ) -> Result<()> {
            if name.is_empty() || self.$field.contains_key(name) {
                return Err(Error::Invalid("empty or duplicate store factory name"));
            }
            self.$field.insert(name.to_owned(), Box::new(factory));
            Ok(())
        }
    };
}
impl StoreFactories {
    pub fn empty() -> Self {
        Self::default()
    }
    register!(add_agent_store, agent, AgentStore);
    register!(add_work_store, work, WorkStore);
    register!(add_review_store, review, ReviewStore);
    register!(add_thread_store, thread, ThreadStore);
    register!(add_reaction_store, reaction, ReactionStore);
    pub fn loader(&self) -> StoreLoader<'_> {
        StoreLoader { factories: self }
    }
}
pub struct StoreLoader<'a> {
    factories: &'a StoreFactories,
}
#[derive(Default)]
pub struct RepositoryStores {
    agent: Option<Arc<dyn AgentStore>>,
    work: Option<Arc<dyn WorkStore>>,
    review: Option<Arc<dyn ReviewStore>>,
    thread: Option<Arc<dyn ThreadStore>>,
    reaction: Option<Arc<dyn ReactionStore>>,
}
impl StoreLoader<'_> {
    pub fn open(
        &self,
        config: &StoreConfig,
        mut context: StoreContext,
    ) -> Result<RepositoryStores> {
        context.enabled = config.enabled();
        context.participants = Arc::new(std::sync::OnceLock::new());
        // Resolve every name before invoking any factory.
        for (kind, name) in &config.implementations {
            let exists = match kind {
                StoreKind::Agent => self.factories.agent.contains_key(name),
                StoreKind::Work => self.factories.work.contains_key(name),
                StoreKind::Review => self.factories.review.contains_key(name),
                StoreKind::Thread => self.factories.thread.contains_key(name),
                StoreKind::Reaction => self.factories.reaction.contains_key(name),
            };
            if !exists {
                return Err(Error::Unsupported("unknown store factory"));
            }
        }
        let mut stores = RepositoryStores::default();
        for (kind, name) in &config.implementations {
            macro_rules! load {
                ($field:ident) => {{
                    let value = self.factories.$field[name](&context)?;
                    if value.scope() != &context.scope || value.name() != name {
                        return Err(Error::Invalid("store factory binding mismatch"));
                    }
                    stores.$field = Some(value);
                }};
            }
            match kind {
                StoreKind::Agent => load!(agent),
                StoreKind::Work => load!(work),
                StoreKind::Review => load!(review),
                StoreKind::Thread => load!(thread),
                StoreKind::Reaction => load!(reaction),
            }
        }
        let mut participants = BTreeMap::new();
        macro_rules! participant {
            ($kind:ident, $field:ident) => {
                if let Some(store) = &stores.$field {
                    participants.insert(StoreKind::$kind, store.publication_group());
                }
            };
        }
        participant!(Agent, agent);
        participant!(Work, work);
        participant!(Review, review);
        participant!(Thread, thread);
        participant!(Reaction, reaction);
        context
            .participants
            .set(participants)
            .map_err(|_| Error::Invalid("source participants already bound"))?;
        Ok(stores)
    }
}
impl RepositoryStores {
    pub fn agent(&self) -> Option<&Arc<dyn AgentStore>> {
        self.agent.as_ref()
    }
    pub fn work(&self) -> Option<&Arc<dyn WorkStore>> {
        self.work.as_ref()
    }
    pub fn review(&self) -> Option<&Arc<dyn ReviewStore>> {
        self.review.as_ref()
    }
    pub fn thread(&self) -> Option<&Arc<dyn ThreadStore>> {
        self.thread.as_ref()
    }
    pub fn reaction(&self) -> Option<&Arc<dyn ReactionStore>> {
        self.reaction.as_ref()
    }

    pub fn require_shared_publication(&self, kinds: &[StoreKind]) -> Result<()> {
        let mut previous: Option<&dyn StoreBinding> = None;
        for kind in kinds {
            let binding: &dyn StoreBinding = match kind {
                StoreKind::Agent => self.agent.as_deref().map(|s| s as &dyn StoreBinding),
                StoreKind::Work => self.work.as_deref().map(|s| s as &dyn StoreBinding),
                StoreKind::Review => self.review.as_deref().map(|s| s as &dyn StoreBinding),
                StoreKind::Thread => self.thread.as_deref().map(|s| s as &dyn StoreBinding),
                StoreKind::Reaction => self.reaction.as_deref().map(|s| s as &dyn StoreBinding),
            }
            .ok_or(Error::Unsupported("store disabled"))?;
            if let Some(before) = previous {
                require_atomic_participants(before, binding)?;
            }
            previous = Some(binding);
        }
        Ok(())
    }
}

impl RepositoryStores {
    pub async fn execute_agent(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> Result<StoreOutcome<AgentResult>> {
        let agent = self
            .agent
            .as_ref()
            .ok_or(Error::Unsupported("AgentStore disabled"))?;
        if request.command.has_thread_binding() {
            if let Some(outcome) = agent.replay(request).await? {
                return Ok(outcome);
            }
            let thread = self
                .thread
                .as_ref()
                .ok_or(Error::Unsupported("ThreadStore disabled"))?;
            require_atomic_participants(agent.as_ref(), thread.as_ref())?;
            agent.execute_with_thread(request, thread.as_ref()).await
        } else {
            agent.execute(request).await
        }
    }
}
pub fn require_atomic_participants(a: &dyn StoreBinding, b: &dyn StoreBinding) -> Result<()> {
    a.scope().require_same(b.scope())?;
    match (a.publication_group(), b.publication_group()) {
        (Some(a), Some(b)) if Arc::ptr_eq(&a, &b) => Ok(()),
        _ => Err(Error::Unsupported(
            "stores do not share an atomic publication provider",
        )),
    }
}

impl RepositoryStores {
    pub async fn prepare_agent(
        &self,
        request: &StoreRequest<AgentCommand>,
    ) -> Result<StoreRequest<AgentCommand>> {
        let agent = self
            .agent
            .as_ref()
            .ok_or(Error::Unsupported("AgentStore disabled"))?;
        if request.command.has_thread_binding() {
            let thread = self
                .thread
                .as_ref()
                .ok_or(Error::Unsupported("ThreadStore disabled"))?;
            require_atomic_participants(agent.as_ref(), thread.as_ref())?;
            agent.prepare_with_thread(request, thread.as_ref()).await
        } else {
            agent.prepare(request).await
        }
    }
}
