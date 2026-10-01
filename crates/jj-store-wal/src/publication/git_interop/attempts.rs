//! Frozen publication intents and causal external evidence. Never executes Git.
use super::*;
use jj_store_model::{OperationId, PrincipalId, RequestId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitPushRefV1 {
    /// Only a known OID or known absence is representable. Unknown is not pushable.
    pub expected: Option<String>,
    pub desired: GitRefV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitPushIntentV1 {
    pub id: RequestId,
    pub executor: PrincipalId,
    pub remote: RequestId,
    pub binding: RecordDigest,
    pub operation: OperationId,
    pub observations: BTreeSet<RequestId>,
    pub refs: BTreeMap<String, GitPushRefV1>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPushRefOutcomeV1 {
    Acknowledged,
    ObservedSatisfied,
    Rejected,
    Unknown,
}
impl GitPushRefOutcomeV1 {
    pub fn satisfied(self) -> bool {
        matches!(self, Self::Acknowledged | Self::ObservedSatisfied)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum GitPushEventKindV1 {
    TransportStarted,
    ResultRecorded {
        refs: BTreeMap<String, GitPushRefOutcomeV1>,
    },
    Applied {
        observation: RequestId,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitPushEventV1 {
    pub id: RequestId,
    pub attempt: RequestId,
    pub predecessors: BTreeSet<RequestId>,
    pub kind: GitPushEventKindV1,
}
impl GitPushIntentV1 {
    pub fn validate(&self) -> Result<()> {
        if self.refs.is_empty()
            || self.refs.len() > 64
            || self.observations.is_empty()
            || self.observations.len() > 256
            || self.refs.iter().any(|(name, value)| {
                !name.starts_with("refs/heads/")
                    || !valid_git_ref(name)
                    || value.expected.as_deref().is_some_and(|v| !valid_git_oid(v))
            })
        {
            return Err(Error::Corrupt("Git push intent"));
        }
        GitObservationV1 {
            id: self.id,
            remote: self.remote,
            binding: self.binding,
            predecessors: BTreeSet::new(),
            operation: self.operation,
            complete_heads: false,
            refs: self
                .refs
                .iter()
                .map(|(name, value)| (name.clone(), value.desired.clone()))
                .collect(),
        }
        .validate()
    }
}
impl GitPushEventV1 {
    pub fn validate(&self) -> Result<()> {
        if self.predecessors.len() > 256 || self.predecessors.contains(&self.id) {
            return Err(Error::Corrupt("Git push event predecessors"));
        }
        match &self.kind {
            GitPushEventKindV1::TransportStarted if !self.predecessors.is_empty() => {
                Err(Error::Corrupt("Git push start predecessors"))
            }
            GitPushEventKindV1::Applied { .. } if self.predecessors.is_empty() => {
                Err(Error::Corrupt("Git push apply predecessors"))
            }
            GitPushEventKindV1::ResultRecorded { refs }
                if refs.is_empty()
                    || refs.len() > 64
                    || refs
                        .keys()
                        .any(|v| !v.starts_with("refs/heads/") || !valid_git_ref(v)) =>
            {
                Err(Error::Corrupt("Git push results"))
            }
            _ => Ok(()),
        }
    }
}
impl GitStoreSnapshot {
    pub fn push_intent(&self, id: RequestId) -> Option<&GitPushIntentV1> {
        self.values.values().find_map(|v| match v {
            GitRecordV1::PushIntent(v) if v.id == id => Some(v),
            _ => None,
        })
    }
    pub fn push_events(&self, id: RequestId) -> Vec<&GitPushEventV1> {
        let events: Vec<_> = self
            .values
            .values()
            .filter_map(|v| match v {
                GitRecordV1::PushEvent(v) if v.attempt == id => Some(v),
                _ => None,
            })
            .collect();
        let replaced: BTreeSet<_> = events
            .iter()
            .flat_map(|v| v.predecessors.iter().copied())
            .collect();
        events
            .into_iter()
            .filter(|v| !replaced.contains(&v.id))
            .collect()
    }
    pub fn push_status(&self, id: RequestId) -> &'static str {
        match self.push_events(id).as_slice() {
            [] => "prepared",
            [event] => match &event.kind {
                GitPushEventKindV1::TransportStarted => "unknown",
                GitPushEventKindV1::Applied { .. } => "applied",
                GitPushEventKindV1::ResultRecorded { refs }
                    if refs.values().all(|v| v.satisfied()) =>
                {
                    "apply_pending"
                }
                GitPushEventKindV1::ResultRecorded { refs }
                    if refs.values().all(|v| *v == GitPushRefOutcomeV1::Rejected) =>
                {
                    "rejected"
                }
                _ => "unknown",
            },
            _ => "conflicted",
        }
    }
}
pub(super) fn validate_attempts(snapshot: &GitStoreSnapshot) -> Result<()> {
    let values = &snapshot.values;
    let observations: BTreeMap<_, _> = values
        .values()
        .filter_map(|v| match v {
            GitRecordV1::Observation(v) => Some((v.id, v)),
            _ => None,
        })
        .collect();
    let intents: BTreeMap<_, _> = values
        .values()
        .filter_map(|v| match v {
            GitRecordV1::PushIntent(v) => Some((v.id, v)),
            _ => None,
        })
        .collect();
    let events: BTreeMap<_, _> = values
        .values()
        .filter_map(|v| match v {
            GitRecordV1::PushEvent(v) => Some((v.id, v)),
            _ => None,
        })
        .collect();
    for intent in intents.values() {
        let mut allowed = BTreeSet::new();
        let mut pending: Vec<_> = intent.observations.iter().copied().collect();
        while let Some(id) = pending.pop() {
            if !allowed.insert(id) {
                continue;
            }
            let observation = observations
                .get(&id)
                .ok_or(Error::Corrupt("Git push observation missing"))?;
            if observation.remote != intent.remote || observation.binding != intent.binding {
                return Err(Error::Corrupt("Git push observation destination"));
            }
            pending.extend(observation.predecessors.iter().copied());
        }
        let base: BTreeMap<_, _> = allowed.iter().map(|id| (*id, observations[id])).collect();
        for (reference, value) in &intent.refs {
            if super::facts::observed_expectation(&base, reference) != Some(value.expected.clone())
            {
                return Err(Error::Corrupt("Git push expectation evidence"));
            }
            if let Some(native) = value.desired.native {
                let representation = GitRecordV1::Representation(GitRepresentationV1 {
                    native_commit: native,
                    git_commit: value.desired.commit_oid.clone().unwrap(),
                    profile: GIT_PROFILE.into(),
                    imported_exact: false,
                });
                if !values.contains_key(&representation.key()?) {
                    return Err(Error::Corrupt("Git push representation missing"));
                }
            }
        }
    }
    for event in events.values() {
        let intent = intents
            .get(&event.attempt)
            .ok_or(Error::Corrupt("Git push intent missing"))?;
        if event
            .predecessors
            .iter()
            .any(|id| events.get(id).is_none_or(|v| v.attempt != event.attempt))
        {
            return Err(Error::Corrupt("Git push event causal closure"));
        }
        match &event.kind {
            GitPushEventKindV1::TransportStarted => {}
            GitPushEventKindV1::ResultRecorded { refs } => {
                if !refs.keys().eq(intent.refs.keys()) {
                    return Err(Error::Corrupt("Git push per-ref result scope"));
                }
                if event.predecessors.is_empty()
                    && refs
                        .values()
                        .any(|v| *v != GitPushRefOutcomeV1::ObservedSatisfied)
                {
                    return Err(Error::Corrupt("Git push result without transport"));
                }
                if event
                    .predecessors
                    .iter()
                    .any(|id| matches!(events[id].kind, GitPushEventKindV1::Applied { .. }))
                {
                    return Err(Error::Corrupt("Git push result after apply"));
                }
            }
            GitPushEventKindV1::Applied { observation } => {
                let observed = observations
                    .get(observation)
                    .ok_or(Error::Corrupt("Git push applied observation missing"))?;
                if observed.remote!=intent.remote || observed.binding!=intent.binding || observed.complete_heads || observed.refs != intent.refs.iter().map(|(name,value)| (name.clone(),value.desired.clone())).collect() || event.predecessors.iter().any(|id| !matches!(&events[id].kind,GitPushEventKindV1::ResultRecorded{refs} if refs.values().all(|v| v.satisfied()))) {return Err(Error::Corrupt("Git push applied evidence"));}
            }
        }
    }
    let mut counts: BTreeMap<_, _> = events
        .iter()
        .map(|(id, event)| (*id, event.predecessors.len()))
        .collect();
    let mut children: BTreeMap<RequestId, Vec<RequestId>> = BTreeMap::new();
    for event in events.values() {
        for parent in &event.predecessors {
            children.entry(*parent).or_default().push(event.id);
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
                .ok_or(Error::Corrupt("Git push causal child"))?;
            *count -= 1;
            if *count == 0 {
                ready.push(*child);
            }
        }
    }
    if processed != events.len() {
        return Err(Error::Corrupt("Git push event cycle"));
    }
    Ok(())
}
