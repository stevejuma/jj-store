use crate::{
    disk::{Generation, Owner},
    projection::Serving,
    *,
};
use jj_store_adapters::{Authorizer, Checkpoint};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use tantivy::{
    Term,
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, TermQuery},
    schema::{IndexRecordOption, TantivyDocument, Value as _},
};
use tokio::sync::Semaphore;

#[derive(Clone, Copy)]
pub struct SearchLimits {
    pub max_offset: usize,
    pub max_page: usize,
    pub output_bytes: usize,
    pub timeout: Duration,
}
impl Default for SearchLimits {
    fn default() -> Self {
        Self {
            max_offset: 10_000,
            max_page: 100,
            output_bytes: 1024 * 1024,
            timeout: Duration::from_secs(5),
        }
    }
}
#[derive(Clone)]
pub struct Search {
    serving: Arc<RwLock<Serving>>,
    _owner: Arc<Owner>,
    slots: Arc<Semaphore>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub durable: Option<Checkpoint>,
    pub visible: Option<Checkpoint>,
    pub degraded: bool,
}
impl Search {
    pub(crate) fn new(serving: Arc<RwLock<Serving>>, owner: Arc<Owner>) -> Self {
        Self {
            serving,
            _owner: owner,
            slots: Arc::new(Semaphore::new(4)),
        }
    }
    pub async fn status(&self, access: &dyn Authorizer) -> Result<Status> {
        let epoch = access.authorize().await?;
        let status = {
            let state = self.serving.read().unwrap();
            Status {
                durable: state.durable.clone(),
                visible: state.visible.as_ref().map(|g| g.checkpoint.clone()),
                degraded: state.degraded,
            }
        };
        for checkpoint in [&status.durable, &status.visible].into_iter().flatten() {
            if checkpoint.source.repository != access.repository()
                || checkpoint.source.epoch != epoch
            {
                return Err(Error::Denied);
            }
        }
        if access.authorize().await? != epoch {
            return Err(Error::Denied);
        }
        Ok(status)
    }
    pub async fn pin(&self, access: &dyn Authorizer) -> Result<PinnedReader> {
        let epoch = access.authorize().await?;
        let generation = self
            .serving
            .read()
            .unwrap()
            .visible
            .clone()
            .ok_or(Error::Unavailable)?;
        if generation.payload.source.repository != access.repository()
            || generation.payload.source.epoch != epoch
        {
            return Err(Error::Denied);
        }
        if access.authorize().await? != epoch {
            return Err(Error::Denied);
        }
        Ok(PinnedReader {
            generation,
            slots: self.slots.clone(),
        })
    }
}
#[derive(Clone)]
pub struct PinnedReader {
    generation: Arc<Generation>,
    slots: Arc<Semaphore>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Page {
    pub checkpoint: Checkpoint,
    pub scopes: BTreeMap<String, ScopeSnapshot>,
    pub hits: Vec<Document>,
    pub next_offset: Option<usize>,
}
impl PinnedReader {
    /// Each page must reauthorize; possession of this handle is not permission.
    pub async fn query(
        &self,
        access: &dyn Authorizer,
        query: &str,
        offset: usize,
        limit: usize,
        limits: SearchLimits,
    ) -> Result<Page> {
        if limits.max_offset > 100_000
            || limits.max_page > 1000
            || limits.output_bytes > 8 * 1024 * 1024
            || limits.timeout.is_zero()
            || limits.timeout > Duration::from_secs(30)
            || offset > limits.max_offset
            || limit == 0
            || limit > limits.max_page
            || query.len() > 1024
            || query.trim().is_empty()
        {
            return Err(Error::Limit);
        }
        let expected = &self.generation.payload.source;
        if access.repository() != expected.repository || access.authorize().await? != expected.epoch
        {
            return Err(Error::Denied);
        }
        let permit = self
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        let generation = self.generation.clone();
        let query = query.to_owned();
        let result = tokio::time::timeout(
            limits.timeout,
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                let searcher = generation.reader.searcher();
                let fields = &generation.fields;
                let query = literal_query(&generation, &query)?;
                let docs = tv(searcher.search(
                    query.as_ref(),
                    &TopDocs::with_limit(limit + 1)
                        .and_offset(offset)
                        .order_by_score(),
                ))?;
                let next_offset = (docs.len() > limit).then_some(offset + limit);
                let mut hits = Vec::new();
                let mut bytes = 0usize;
                for (_, address) in docs.into_iter().take(limit) {
                    let stored: TantivyDocument = tv(searcher.doc(address))?;
                    let value = stored
                        .get_first(fields.stored)
                        .and_then(|v| v.as_str())
                        .ok_or(Error::Corrupt)?;
                    bytes = bytes.checked_add(value.len()).ok_or(Error::Limit)?;
                    if bytes > limits.output_bytes {
                        return Err(Error::Limit);
                    }
                    let mut document: Document =
                        serde_json::from_str(value).map_err(|_| Error::Corrupt)?;
                    if let DocumentKey::File { scope, .. } = &document.key {
                        let scope = generation.payload.scopes.get(scope).ok_or(Error::Corrupt)?;
                        if scope.state != ScopeState::Ready {
                            return Err(Error::Corrupt);
                        }
                        document.operation = scope.operation;
                        document.commit = scope.commits.first().copied();
                    }
                    hits.push(document);
                }
                let page = Page {
                    checkpoint: generation.checkpoint.clone(),
                    scopes: generation.payload.scopes.clone(),
                    hits,
                    next_offset,
                };
                bounded_json(&page, limits.output_bytes)?;
                Ok(page)
            }),
        )
        .await
        .map_err(|_| Error::Limit)?
        .map_err(|_| Error::Unavailable)?;
        if access.authorize().await? != expected.epoch {
            return Err(Error::Denied);
        }
        result
    }
}

fn literal_query(generation: &Generation, input: &str) -> Result<Box<dyn Query>> {
    let fields = &generation.fields;
    let term = |field, text: &str| -> Box<dyn Query> {
        Box::new(TermQuery::new(
            Term::from_field_text(field, text),
            IndexRecordOption::Basic,
        ))
    };
    let input = input.trim();
    if matches!(input.len(), 32 | 64 | 128)
        && input
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        let mut clauses = vec![(Occur::Should, term(fields.id, input))];
        for (name, scope) in &generation.payload.scopes {
            if scope.state == ScopeState::Ready
                && (scope.operation.is_some_and(|id| id.to_string() == input)
                    || scope.commits.iter().any(|id| id.to_string() == input))
            {
                clauses.push((Occur::Should, term(fields.scope, name)));
            }
        }
        return Ok(Box::new(BooleanQuery::new(clauses)));
    }
    let index = generation.reader.searcher().index().clone();
    let mut tokenizer = index.tokenizers().get("default").ok_or(Error::Corrupt)?;
    let mut stream = tokenizer.token_stream(input);
    let mut clauses = Vec::new();
    while stream.advance() {
        if clauses.len() >= 32 {
            return Err(Error::Limit);
        }
        let token = &stream.token().text;
        let fields = [fields.title, fields.body, fields.text, fields.path];
        clauses.push((
            Occur::Must,
            Box::new(BooleanQuery::new(
                fields
                    .into_iter()
                    .map(|field| (Occur::Should, term(field, token)))
                    .collect(),
            )) as Box<dyn Query>,
        ));
    }
    if clauses.is_empty() {
        return Err(Error::Invalid);
    }
    Ok(Box::new(BooleanQuery::new(clauses)))
}
