//! Immutable logical configuration and causal observations; no executable URLs.
use super::*;
use jj_store_model::{OperationId, RequestId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitRemoteV1 {
    pub remote: RequestId,
    pub event: RequestId,
    pub predecessors: BTreeSet<RequestId>,
    pub name: String,
    pub active: bool,
}
impl GitRemoteV1 {
    pub fn validate(&self) -> Result<()> {
        if !valid_remote_name(&self.name)
            || self.predecessors.len() > 256
            || self.predecessors.contains(&self.event)
        {
            return Err(Error::Corrupt("Git remote event"));
        }
        Ok(())
    }
}
pub fn valid_remote_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name != "git"
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        && !name.starts_with('-')
        && name != "."
        && name != ".."
}
pub fn valid_git_ref(name: &str) -> bool {
    (name.starts_with("refs/heads/") || name.starts_with("refs/tags/"))
        && name.len() <= 1024
        && !name.ends_with('/')
        && !name.ends_with('.')
        && !name.contains("..")
        && !name.contains("@{")
        && name
            .bytes()
            .all(|b| b > 32 && b < 127 && !b"~^:?*[\\".contains(&b))
        && name
            .split('/')
            .all(|v| !v.is_empty() && !v.starts_with('.') && !v.ends_with(".lock"))
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitRefV1 {
    /// Original wire ref target, including an annotated tag object.
    pub oid: Option<String>,
    pub commit_oid: Option<String>,
    pub native: Option<CommitId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitObservationV1 {
    pub id: RequestId,
    pub remote: RequestId,
    pub binding: RecordDigest,
    pub predecessors: BTreeSet<RequestId>,
    pub operation: OperationId,
    pub complete_heads: bool,
    pub refs: BTreeMap<String, GitRefV1>,
}
impl GitObservationV1 {
    pub fn validate(&self) -> Result<()> {
        if self.predecessors.len() > 256
            || self.predecessors.contains(&self.id)
            || self.refs.len() > 4096
            || self.refs.iter().any(|(name, value)| {
                !valid_git_ref(name)
                    || value.oid.as_ref().is_some_and(|v| !valid_git_oid(v))
                    || value.commit_oid.as_ref().is_some_and(|v| !valid_git_oid(v))
                    || (value.oid.is_some() != value.native.is_some())
                    || (value.oid.is_some() != value.commit_oid.is_some())
                    || (name.starts_with("refs/heads/") && value.oid != value.commit_oid)
            })
        {
            return Err(Error::Corrupt("Git observation"));
        }
        Ok(())
    }
}
impl GitStoreSnapshot {
    pub fn remote_versions(&self, remote: RequestId) -> Vec<&GitRemoteV1> {
        let events: Vec<_> = self
            .values
            .values()
            .filter_map(|v| match v {
                GitRecordV1::Remote(v) if v.remote == remote => Some(v),
                _ => None,
            })
            .collect();
        let replaced: BTreeSet<_> = events
            .iter()
            .flat_map(|v| v.predecessors.iter().copied())
            .collect();
        events
            .into_iter()
            .filter(|v| !replaced.contains(&v.event))
            .collect()
    }
    pub fn remotes(&self) -> Result<BTreeMap<String, RequestId>> {
        let ids: BTreeSet<_> = self
            .values
            .values()
            .filter_map(|v| match v {
                GitRecordV1::Remote(v) => Some(v.remote),
                _ => None,
            })
            .collect();
        let mut selected = BTreeMap::new();
        for id in ids {
            let versions = self.remote_versions(id);
            let [version] = versions.as_slice() else {
                return Err(Error::Stale);
            };
            if version.active && selected.insert(version.name.clone(), id).is_some() {
                return Err(Error::Stale);
            }
        }
        Ok(selected)
    }
    /// All maximal observations for this destination. Causally superseded facts
    /// stay durable, and incomparable expectations remain distinct.
    pub fn observations(&self, remote: RequestId, binding: RecordDigest) -> Vec<&GitObservationV1> {
        let observations: Vec<_> = self
            .values
            .values()
            .filter_map(|v| match v {
                GitRecordV1::Observation(v) if v.remote == remote && v.binding == binding => {
                    Some(v)
                }
                _ => None,
            })
            .collect();
        let replaced: BTreeSet<_> = observations
            .iter()
            .flat_map(|v| v.predecessors.iter().copied())
            .collect();
        observations
            .into_iter()
            .filter(|v| !replaced.contains(&v.id))
            .collect()
    }
    /// None is unknown or conflicting; Some(None) is known absent.
    pub fn expectation(
        &self,
        remote: RequestId,
        binding: RecordDigest,
        reference: &str,
    ) -> Option<Option<String>> {
        let observations: BTreeMap<_, _> = self
            .values
            .values()
            .filter_map(|v| match v {
                GitRecordV1::Observation(v) if v.remote == remote && v.binding == binding => {
                    Some((v.id, v))
                }
                _ => None,
            })
            .collect();
        let covering: BTreeMap<_, _> = observations
            .iter()
            .filter(|(_, v)| {
                v.refs.contains_key(reference)
                    || (v.complete_heads && reference.starts_with("refs/heads/"))
            })
            .map(|(id, v)| (*id, *v))
            .collect();
        let mut superseded = BTreeSet::new();
        let mut pending: Vec<_> = covering
            .values()
            .flat_map(|v| v.predecessors.iter().copied())
            .collect();
        while let Some(id) = pending.pop() {
            if superseded.insert(id)
                && let Some(previous) = observations.get(&id)
            {
                pending.extend(previous.predecessors.iter().copied());
            }
        }
        let mut expected: BTreeSet<_> = covering
            .into_iter()
            .filter(|(id, _)| !superseded.contains(id))
            .map(|(_, v)| v.refs.get(reference).and_then(|v| v.oid.clone()))
            .collect();
        if expected.len() == 1 {
            expected.pop_first()
        } else {
            None
        }
    }
}
pub(super) fn validate_closure(values: &BTreeMap<RecordDigest, GitRecordV1>) -> Result<()> {
    let remotes: BTreeMap<_, _> = values
        .values()
        .filter_map(|v| match v {
            GitRecordV1::Remote(v) => Some((v.event, v)),
            _ => None,
        })
        .collect();
    let observations: BTreeMap<_, _> = values
        .values()
        .filter_map(|v| match v {
            GitRecordV1::Observation(v) => Some((v.id, v)),
            _ => None,
        })
        .collect();
    let remote_ids: BTreeSet<_> = remotes.values().map(|v| v.remote).collect();
    for remote in remotes.values() {
        if remote
            .predecessors
            .iter()
            .any(|id| remotes.get(id).is_none_or(|v| v.remote != remote.remote))
            || (remote.predecessors.is_empty() && remote.event != remote.remote)
        {
            return Err(Error::Corrupt("Git remote causal closure"));
        }
    }
    for observation in observations.values() {
        if !remote_ids.contains(&observation.remote)
            || observation.predecessors.iter().any(|id| {
                observations.get(id).is_none_or(|v| {
                    v.remote != observation.remote || v.binding != observation.binding
                })
            })
        {
            return Err(Error::Corrupt("Git observation causal closure"));
        }
        for value in observation.refs.values().filter(|v| v.oid.is_some()) {
            let representation = GitRecordV1::Representation(GitRepresentationV1 {
                native_commit: value.native.unwrap(),
                git_commit: value.commit_oid.clone().unwrap(),
                profile: GIT_PROFILE.into(),
                imported_exact: true,
            });
            if !values.contains_key(&representation.key()?)
                || !values.values().any(
                    |v| matches!(v, GitRecordV1::Object(o) if Some(&o.oid) == value.oid.as_ref()),
                )
            {
                return Err(Error::Corrupt("Git observed representation closure"));
            }
        }
    }
    // An iterative topological removal detects cycles without recursive stack use.
    for graph in [
        remotes
            .iter()
            .map(|(id, v)| (*id, v.predecessors.clone()))
            .collect::<BTreeMap<_, _>>(),
        observations
            .iter()
            .map(|(id, v)| (*id, v.predecessors.clone()))
            .collect(),
    ] {
        let mut counts: BTreeMap<_, _> = graph
            .iter()
            .map(|(id, parents)| (*id, parents.len()))
            .collect();
        let mut children: BTreeMap<RequestId, Vec<RequestId>> = BTreeMap::new();
        for (id, parents) in &graph {
            for parent in parents {
                children.entry(*parent).or_default().push(*id);
            }
        }
        let mut ready: Vec<_> = counts
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(id, _)| *id)
            .collect();
        let mut processed = 0;
        while let Some(id) = ready.pop() {
            processed += 1;
            for child in children.get(&id).into_iter().flatten() {
                let count = counts
                    .get_mut(child)
                    .ok_or(Error::Corrupt("Git causal child"))?;
                *count -= 1;
                if *count == 0 {
                    ready.push(*child);
                }
            }
        }
        if processed != graph.len() {
            return Err(Error::Corrupt("Git fact causal cycle"));
        }
    }
    Ok(())
}
