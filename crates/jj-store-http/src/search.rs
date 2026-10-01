//! Transport parsing over repository-scoped, independently rebuildable projections.
use crate::{
    routes::{Empty, Input, input},
    *,
};
use jj_store_tantivy::paging;

/// A trusted serving composition supplies the projection and its live runner health.
pub struct SearchIndex {
    pub search: jj_store_tantivy::Search,
    pub health: tokio::sync::watch::Receiver<jj_store_adapters::Health>,
    pub limits: paging::Limits,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Query {
    q: String,
    #[serde(default)]
    mode: paging::Mode,
    at_op: Option<m::OperationId>,
    scope: Option<String>,
    #[serde(default = "page_size")]
    page_size: usize,
    minimum_position: Option<m::WalPosition>,
    #[serde(default)]
    wait_millis: u64,
    cursor: Option<String>,
}
fn page_size() -> usize {
    50
}
fn pager(state: &State, context: &Context) -> Result<Arc<paging::Pager>> {
    state
        .search
        .get(context.id("repo")?)
        .ok_or_else(|| Error::new(StatusCode::NOT_IMPLEMENTED, "search_not_configured"))
}
pub(crate) async fn search(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Query>,
) -> Result<Response> {
    let args = input(args)?;
    let pager = pager(&state, &context)?;
    let request = paging::Request {
        q: args.q,
        mode: args.mode,
        operation: args.at_op,
        scope: args.scope,
        page_size: args.page_size,
        minimum_position: args.minimum_position,
        wait_millis: args.wait_millis,
    };
    let page = if let Some(cursor) = args.cursor {
        pager.resume(context.session, &cursor, request).await?
    } else {
        pager.begin(context.session, request).await?
    };
    state.json(&page)
}
pub(crate) async fn search_status(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    args: Input<Empty>,
) -> Result<Response> {
    input(args)?;
    let status = pager(&state, &context)?.status(context.session).await?;
    state.json(&status)
}

#[derive(Clone)]
pub struct SearchRegistry {
    pagers: Arc<std::sync::RwLock<std::collections::BTreeMap<m::RepositoryId, Arc<paging::Pager>>>>,
    reader: q::content::ContentReader,
    max_json_bytes: usize,
}
impl SearchRegistry {
    pub(crate) fn new(
        pagers: std::collections::BTreeMap<m::RepositoryId, Arc<paging::Pager>>,
        reader: q::content::ContentReader,
        max_json_bytes: usize,
    ) -> Self {
        Self {
            pagers: Arc::new(std::sync::RwLock::new(pagers)),
            reader,
            max_json_bytes,
        }
    }
    fn get(&self, repo: m::RepositoryId) -> Option<Arc<paging::Pager>> {
        self.pagers.read().unwrap().get(&repo).cloned()
    }
    pub(crate) fn contains_key(&self, repo: &m::RepositoryId) -> bool {
        self.get(*repo).is_some_and(|p| p.configured())
    }
    /// Reuse the pager and its credential-free cursor pins across replacements.
    pub fn replace(
        &self,
        repo: m::RepositoryId,
        index: Option<SearchIndex>,
    ) -> std::result::Result<(), &'static str> {
        let mut pagers = self.pagers.write().unwrap();
        if let Some(pager) = pagers.get(&repo) {
            pager
                .replace(index.map(|i| paging::Backend {
                    search: i.search,
                    health: i.health,
                }))
                .map_err(|_| "search cursor state")?;
        } else if let Some(mut index) = index {
            if pagers.len() >= 128 {
                return Err("search repository limit");
            }
            index.limits.query.output_bytes =
                index.limits.query.output_bytes.min(self.max_json_bytes);
            let pager = paging::Pager::new(
                index.search,
                index.health,
                self.reader.clone(),
                index.limits,
                Arc::new(SystemClock),
            )
            .map_err(|_| "search limits")?;
            pagers.insert(repo, Arc::new(pager));
        }
        Ok(())
    }
}
