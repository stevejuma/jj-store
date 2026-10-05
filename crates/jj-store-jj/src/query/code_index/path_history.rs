//! Bounded JJ file-history batches. Retain IDs only, never readers or authority.
use super::*;
use futures::StreamExt as _;
use jj_lib::{
    backend::CommitId,
    fileset::FilesetExpression,
    index::Index as _,
    repo_path::RepoPathBuf,
    revset::{
        ResolvedExpression as Expr, ResolvedPredicateExpression as Predicate, RevsetFilterPredicate,
    },
};
use std::{collections::BTreeSet, sync::Mutex, time::Instant};

pub(crate) const BATCH: usize = 64;
// A wide directory must not multiply a full ancestry batch by every row.
const PATH_EVALUATIONS: usize = 1024;
const FRONTIER: usize = 4096;
const WINDOWS: usize = 16;
#[derive(Clone, PartialEq, Eq)]
struct Key {
    commit: CommitId,
    paths: Vec<RepoPathBuf>,
}
#[derive(Clone)]
struct State {
    frontier: Vec<CommitId>,
    results: Vec<Option<CommitId>>,
}
struct Entry {
    key: Key,
    state: tokio::sync::Mutex<State>,
    _memory: memory::Lease,
}
#[derive(Default)]
pub(crate) struct History(Mutex<Vec<(Instant, Arc<Entry>)>>);
pub(crate) enum Change {
    Found(CommitId),
    Pending,
    Missing,
}
impl History {
    fn entry(&self, commit: &CommitId, paths: &[RepoPathBuf]) -> Result<Arc<Entry>> {
        let mut entries = self.0.lock().unwrap();
        if let Some((used, entry)) = entries
            .iter_mut()
            .find(|(_, e)| e.key.commit == *commit && e.key.paths == paths)
        {
            *used = Instant::now();
            return Ok(entry.clone());
        }
        if entries.len() >= WINDOWS {
            let oldest = entries
                .iter()
                .enumerate()
                .filter(|(_, (_, e))| Arc::strong_count(e) == 1)
                .min_by_key(|(_, (used, _))| *used)
                .map(|(i, _)| i)
                .ok_or(Error::Busy)?;
            entries.swap_remove(oldest);
        }
        // Includes retained strings, frontier/row IDs, and a transaction's copy.
        let bytes = 4 * paths
            .iter()
            .map(|p| p.as_internal_file_string().len() + 256)
            .sum::<usize>()
            + 4 * FRONTIER * 128;
        let lease = memory::Lease::reserve(bytes as u64)?;
        let entry = Arc::new(Entry {
            key: Key {
                commit: commit.clone(),
                paths: paths.to_vec(),
            },
            state: tokio::sync::Mutex::new(State {
                frontier: vec![commit.clone()],
                results: vec![None; paths.len()],
            }),
            _memory: lease,
        });
        entries.push((Instant::now(), entry.clone()));
        Ok(entry)
    }
    pub async fn read(
        &self,
        index: &DefaultReadonlyIndex,
        store: &Arc<Store>,
        commit: CommitId,
        paths: Vec<RepoPathBuf>,
        mut charge: impl FnMut(usize) -> Result<()>,
    ) -> Result<Vec<Change>> {
        charge(paths.len())?;
        if paths.is_empty() {
            return Ok(vec![]);
        }
        if paths.len() > PATH_EVALUATIONS {
            return Err(Error::Limit);
        }
        if !index
            .has_id(&commit)
            .await
            .map_err(|_| Error::Unavailable)?
        {
            return Err(Error::NotCommitted);
        }
        let entry = self.entry(&commit, &paths)?;
        let mut saved = entry.state.try_lock().map_err(|_| Error::Busy)?;
        let mut state = saved.clone();
        if !state.frontier.is_empty() && state.results.iter().any(Option::is_none) {
            let ancestors = Expr::Ancestors {
                heads: Box::new(Expr::Commits(state.frontier.clone())),
                generation: 0..u64::MAX,
                parents_range: 0..u32::MAX,
            };
            let set = index
                .evaluate_revset(&ancestors, store)
                .map_err(|_| Error::Unavailable)?;
            let mut stream = set.stream();
            let mut candidates = Vec::new();
            let unresolved = state.results.iter().filter(|id| id.is_none()).count();
            let prefilter = unresolved > 1 && unresolved < PATH_EVALUATIONS;
            let evaluations = unresolved + usize::from(prefilter);
            let batch = BATCH.min((PATH_EVALUATIONS / evaluations).max(1));
            while candidates.len() < batch {
                charge(1)?;
                match stream.next().await {
                    Some(Ok(id)) => candidates.push(id),
                    Some(Err(_)) => return Err(Error::Unavailable),
                    None => break,
                }
            }
            let matching_candidates = if prefilter {
                charge(candidates.len())?;
                let expression = Expr::FilterWithin {
                    candidates: Box::new(Expr::Commits(candidates.clone())),
                    predicate: Predicate::Filter(RevsetFilterPredicate::File(
                        FilesetExpression::union_all(
                            paths
                                .iter()
                                .zip(&state.results)
                                .filter(|(_, result)| result.is_none())
                                .map(|(path, _)| FilesetExpression::prefix_path(path.clone()))
                                .collect(),
                        ),
                    )),
                };
                tokio::task::unconstrained(async {
                    let matching = index
                        .evaluate_revset(&expression, store)
                        .map_err(|_| Error::Unavailable)?;
                    let mut stream = matching.stream();
                    let mut ids = Vec::new();
                    while let Some(id) = stream.next().await {
                        charge(1)?;
                        ids.push(id.map_err(revset_error)?);
                    }
                    Ok::<_, Error>(ids)
                })
                .await?
            } else {
                candidates.clone()
            };
            for (row, path) in paths.iter().enumerate() {
                if state.results[row].is_some() || matching_candidates.is_empty() {
                    continue;
                }
                charge(matching_candidates.len())?;
                let expression = Expr::FilterWithin {
                    candidates: Box::new(Expr::Commits(matching_candidates.clone())),
                    predicate: Predicate::Filter(RevsetFilterPredicate::File(
                        FilesetExpression::prefix_path(path.clone()),
                    )),
                };
                // JJ's file predicate uses pollster for scoped backend reads.
                // A scheduler-only cooperative yield cannot progress there;
                // owned work, byte, cancellation and deadline limits still apply.
                state.results[row] = tokio::task::unconstrained(async {
                    let matching = index
                        .evaluate_revset(&expression, store)
                        .map_err(|_| Error::Unavailable)?;
                    matching
                        .stream()
                        .next()
                        .await
                        .transpose()
                        .map_err(revset_error)
                })
                .await?;
                charge(1)?;
            }
            // JJ's topological order makes the unvisited parents a sufficient
            // checkpoint. No replay of the already visited ancestry is needed.
            let consumed: BTreeSet<_> = candidates.iter().cloned().collect();
            let mut frontier: BTreeSet<_> = state
                .frontier
                .into_iter()
                .filter(|id| !consumed.contains(id))
                .collect();
            let parents = Expr::Ancestors {
                heads: Box::new(Expr::Commits(candidates)),
                generation: 1..2,
                parents_range: 0..u32::MAX,
            };
            let set = index
                .evaluate_revset(&parents, store)
                .map_err(|_| Error::Unavailable)?;
            let mut stream = set.stream();
            while let Some(id) = stream.next().await {
                charge(1)?;
                let id = id.map_err(|_| Error::Unavailable)?;
                if !consumed.contains(&id) {
                    frontier.insert(id);
                }
                if frontier.len() > FRONTIER {
                    return Err(Error::Limit);
                }
            }
            state.frontier = frontier.into_iter().collect();
            if state.results.iter().all(Option::is_some) {
                state.frontier.clear();
            }
        }
        charge(1)?;
        // Failed/canceled batches leave the previous checkpoint intact.
        *saved = state;
        Ok(saved
            .results
            .iter()
            .map(|id| match id {
                Some(id) => Change::Found(id.clone()),
                None if saved.frontier.is_empty() => Change::Missing,
                None => Change::Pending,
            })
            .collect())
    }
}

fn revset_error(error: jj_lib::revset::RevsetEvaluationError) -> Error {
    match error.into_backend_error() {
        jj_lib::backend::BackendError::Other(source) => source
            .downcast::<Error>()
            .map(|error| *error)
            .unwrap_or(Error::Unavailable),
        jj_lib::backend::BackendError::ObjectNotFound { .. } => Error::Incomplete,
        _ => Error::Unavailable,
    }
}
