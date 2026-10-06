//! Operation-owned topology, independent of paginated presentation history.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopologyHead")]
pub struct Head {
    pub names: Vec<String>,
    pub commit: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopologyLane")]
pub struct Lane {
    /// Newest first. Multiple names on one tip are aliases, not extra segments.
    pub heads: Vec<Head>,
    pub applied: Vec<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopology")]
pub struct Topology {
    pub lanes: Vec<Lane>,
    pub integrated: Vec<String>,
    pub complete: bool,
}

pub(super) async fn read(repo: &ReadonlyRepo, saved: &SavedWorkspace) -> Result<Topology> {
    let mut result = Topology {
        complete: true,
        ..Default::default()
    };
    let Ok(target) = resolve(repo, &saved.target) else {
        result.complete = false;
        return Ok(result);
    };
    let mut heads: Vec<Head> = vec![];
    for name in &saved.applied {
        let Ok(commit) = resolve(repo, name) else {
            // Missing/conflicted membership remains individually recoverable.
            heads.push(Head {
                names: vec![name.clone()],
                commit: String::new(),
            });
            continue;
        };
        if commit.id() != target.id()
            && repo
                .index()
                .is_ancestor(commit.id(), target.id())
                .await
                .map_err(jj)?
        {
            result.integrated.push(name.clone());
        } else if commit.id() != target.id()
            && let Some(head) = heads.iter_mut().find(|h| h.commit == commit.id().hex())
        {
            head.names.push(name.clone());
        } else {
            // Distinct empty branches at the target are independent destinations.
            heads.push(Head {
                names: vec![name.clone()],
                commit: commit.id().hex(),
            });
        }
    }
    let positions: BTreeMap<_, _> = heads
        .iter()
        .enumerate()
        .filter(|(_, h)| !h.commit.is_empty() && h.commit != target.id().hex())
        .map(|(i, h)| (h.commit.clone(), i))
        .collect();
    let mut paths = Vec::new();
    let mut cache = BTreeMap::new();
    let mut lower = BTreeSet::new();
    let mut work = 0;
    for (index, head) in heads.iter().enumerate() {
        let mut path = vec![index];
        if !head.commit.is_empty() && head.commit != target.id().hex() {
            let mut current = resolve(repo, &head.names[0])?;
            let mut seen = BTreeSet::new();
            loop {
                work += 1;
                if work > 50_000 {
                    result.complete = false;
                    return Ok(result);
                }
                if current.id() == target.id()
                    || current.parent_ids().len() != 1
                    || !seen.insert(current.id().clone())
                {
                    break;
                }
                let parent = current.parent_ids()[0].clone();
                if repo
                    .index()
                    .is_ancestor(&parent, target.id())
                    .await
                    .map_err(jj)?
                {
                    break;
                }
                if let Some(&position) = positions.get(&parent.hex()) {
                    path.push(position);
                    lower.insert(position);
                }
                if !cache.contains_key(&parent) {
                    if cache.len() >= 50_000 {
                        result.complete = false;
                        result.lanes.clear();
                        return Ok(result);
                    }
                    cache.insert(
                        parent.clone(),
                        repo.store().get_commit(&parent).map_err(jj)?,
                    );
                }
                current = cache[&parent].clone();
            }
        }
        paths.push(path);
    }
    // A shared applied base is not a linear dependency of either sibling.
    // Keep it as its own lane instead of duplicating ownership/actions.
    let mut children = BTreeMap::<usize, BTreeSet<usize>>::new();
    for path in &paths {
        for pair in path.windows(2) {
            children.entry(pair[1]).or_default().insert(pair[0]);
        }
    }
    let forks: BTreeSet<_> = children
        .iter()
        .filter(|(_, children)| children.len() > 1)
        .map(|(&index, _)| index)
        .collect();
    lower.retain(|index| !forks.contains(index));
    for (index, mut path) in paths.into_iter().enumerate() {
        if lower.contains(&index) {
            continue;
        }
        if let Some(boundary) = path.iter().skip(1).position(|index| forks.contains(index)) {
            path.truncate(boundary + 1);
        }
        let lane_heads: Vec<_> = path.into_iter().map(|i| heads[i].clone()).collect();
        result.lanes.push(Lane {
            applied: lane_heads.iter().flat_map(|h| h.names.clone()).collect(),
            heads: lane_heads,
        });
    }
    // Preserve saved membership ordering, even if a lower branch was applied first.
    result.lanes.sort_by_key(|lane| {
        saved
            .applied
            .iter()
            .position(|name| lane.applied.contains(name))
            .unwrap_or(usize::MAX)
    });
    Ok(result)
}

pub(super) fn validate_unapply(config: &Config, branches: &[String]) -> Result<()> {
    let saved = config.active().ok_or(Error::Invalid)?;
    if branches.is_empty()
        || branches.len() > 4096
        || branches.iter().collect::<BTreeSet<_>>().len() != branches.len()
        || branches.iter().any(|b| !saved.applied.contains(b))
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
