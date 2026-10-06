//! Operation-owned stack inventory, independent of membership and history pages.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopologyHead")]
pub struct Head {
    pub names: Vec<String>,
    pub commit: String,
    /// Exact lower bookmark or first excluded target ancestor; None at ambiguous merges.
    #[serde(default)]
    pub base_commit: Option<String>,
    #[serde(default)]
    pub shared: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopologyLane")]
pub struct Lane {
    /// Newest first. Names on one tip are aliases, not additional segments.
    pub heads: Vec<Head>,
    pub applied: Vec<String>,
    /// Explicit members removable without changing another active lane's membership.
    #[serde(default)]
    pub unapply: Vec<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksTopology")]
pub struct Topology {
    pub lanes: Vec<Lane>,
    pub integrated: Vec<String>,
    pub complete: bool,
    #[serde(default)]
    pub inventory: Vec<Lane>,
}

pub(super) async fn discover(
    repo: &ReadonlyRepo,
    target_name: &str,
    applied: &[String],
) -> Result<Topology> {
    let mut result = Topology {
        complete: true,
        ..Default::default()
    };
    let Ok(target) = resolve(repo, target_name) else {
        result.complete = false;
        return Ok(result);
    };
    let names: Vec<_> = repo
        .view()
        .local_bookmarks()
        .filter(|(name, _)| {
            name.as_str() != target_name && !name.as_str().starts_with(config::INTERNAL_PREFIX)
        })
        .map(|(name, _)| name.as_str().to_owned())
        .take(4097)
        .collect();
    if names.len() > 4096 {
        result.complete = false;
        return Ok(result);
    }
    let mut heads: Vec<Head> = vec![];
    for name in &names {
        let Ok(commit) = resolve(repo, name) else {
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
            heads.push(Head {
                names: vec![name.clone()],
                commit: commit.id().hex(),
                base_commit: None,
                shared: false,
            });
        }
    }
    // Preserve the chosen applied alias while giving unapplied aliases stable ordering.
    for head in &mut heads {
        head.names.sort_by_key(|name| {
            (
                applied.iter().position(|n| n == name).unwrap_or(usize::MAX),
                name.clone(),
            )
        });
    }
    let positions: BTreeMap<_, _> = heads
        .iter()
        .enumerate()
        .filter(|(_, h)| h.commit != target.id().hex())
        .map(|(i, h)| (h.commit.clone(), i))
        .collect();
    let mut paths = vec![];
    let mut cache = BTreeMap::new();
    let mut work = 0;
    for index in 0..heads.len() {
        let mut path = vec![index];
        let mut current = resolve(repo, &heads[index].names[0])?;
        if current.id() == target.id() {
            heads[index].base_commit = Some(target.id().hex());
        } else {
            loop {
                work += 1;
                if work > 50_000 {
                    result.complete = false;
                    return Ok(result);
                }
                if current.parent_ids().len() != 1 {
                    break;
                }
                let parent = current.parent_ids()[0].clone();
                if repo
                    .index()
                    .is_ancestor(&parent, target.id())
                    .await
                    .map_err(jj)?
                {
                    if path.len() == 1 {
                        heads[index].base_commit = Some(parent.hex());
                    }
                    break;
                }
                if let Some(&lower) = positions.get(&parent.hex()) {
                    if path.len() == 1 {
                        heads[index].base_commit = Some(parent.hex());
                    }
                    path.push(lower);
                }
                if !cache.contains_key(&parent) {
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
    let roots = |indices: Vec<usize>| {
        indices
            .iter()
            .copied()
            .filter(|i| !indices.iter().any(|j| i != j && paths[*j].contains(i)))
            .collect::<Vec<_>>()
    };
    let inventory_roots = roots((0..heads.len()).collect());
    let active_roots = roots(
        (0..heads.len())
            .filter(|i| heads[*i].names.iter().any(|n| applied.contains(n)))
            .collect(),
    );
    let lanes = |root_indices: &[usize]| {
        root_indices
            .iter()
            .map(|&index| {
                let path = &paths[index];
                let lane_heads = path
                    .iter()
                    .map(|&i| {
                        let mut h = heads[i].clone();
                        h.shared = inventory_roots
                            .iter()
                            .filter(|&&r| paths[r].contains(&i))
                            .count()
                            > 1;
                        h
                    })
                    .collect::<Vec<_>>();
                let members = applied
                    .iter()
                    .filter(|name| lane_heads.iter().any(|h| h.names.contains(name)))
                    .cloned()
                    .collect::<Vec<_>>();
                let unapply = members
                    .iter()
                    .filter(|name| {
                        !active_roots.iter().any(|&other| {
                            other != index
                                && paths[other].iter().any(|&i| heads[i].names.contains(name))
                        })
                    })
                    .cloned()
                    .collect();
                Lane {
                    heads: lane_heads,
                    applied: members,
                    unapply,
                }
            })
            .collect::<Vec<_>>()
    };
    result.inventory = lanes(&inventory_roots);
    result.lanes = lanes(&active_roots);
    for name in applied {
        if resolve(repo, name).is_err() {
            result.lanes.push(Lane {
                heads: vec![Head {
                    names: vec![name.clone()],
                    commit: String::new(),
                    base_commit: None,
                    shared: false,
                }],
                applied: vec![name.clone()],
                unapply: vec![name.clone()],
            });
        }
    }
    result.lanes.sort_by_key(|lane| {
        applied
            .iter()
            .position(|name| lane.applied.contains(name))
            .unwrap_or(usize::MAX)
    });
    Ok(result)
}

pub(super) async fn read(repo: &ReadonlyRepo, saved: &SavedWorkspace) -> Result<Topology> {
    discover(repo, &saved.target, &saved.applied).await
}

pub(super) async fn contains(
    repo: &ReadonlyRepo,
    saved: &SavedWorkspace,
    branch: &str,
) -> Result<bool> {
    if saved.applied.iter().any(|name| name == branch) {
        return Ok(true);
    }
    let topology = read(repo, saved).await?;
    Ok(topology.complete
        && topology.lanes.iter().any(|lane| {
            lane.heads
                .iter()
                .any(|head| head.names.iter().any(|name| name == branch))
        }))
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
