use crate::{Result, cache, local};
use std::sync::Arc;

/// Derived from the actual native caller; never accepted as renderer arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    pub window: String,
    pub url: String,
}
#[async_trait::async_trait]
pub trait Admission: Send + Sync + 'static {
    /// Validate caller origin, live vault token, physical identity and generation.
    /// Access must recheck current authority at entry and completion, not only here.
    async fn admit(&self, caller: &Context, vault: &str) -> Result<Arc<dyn local::Access>>;
}

#[derive(Clone, Copy)]
pub struct Config {
    pub max_handles: usize,
    pub concurrency: usize,
    pub reads: local::Limits,
    pub preparation: local::preparation::Limits,
    pub cache: cache::Policy,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            max_handles: 32,
            concurrency: 2,
            reads: local::Limits::default(),
            preparation: local::preparation::Limits::default(),
            cache: cache::Policy::default(),
        }
    }
}
