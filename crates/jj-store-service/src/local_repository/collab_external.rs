//! Complete retained source closures; source stores keep their original identities.
use super::{
    LocalError, LocalRepository, LocalResult,
    agent_evidence::{EvidenceEntry, EvidenceIo},
    agent_io::AgentIo,
    records::GitRecords,
    review_io::ReviewIo,
};
use jj_collab::{
    CollabGraphV1, CollabObjectRef, CollabObjectV1, ThreadCodeAnchorV1, ThreadLinkTargetV1,
};
use jj_store_jj::local_metadata::{GitOid, RetainedKind, Snapshot};
use jj_store_model::RepositoryId;
use jj_store_wal::{
    publication::{AgentObjectIo, ReviewObjectIo},
    records::{RecordKind, RecordRef},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExternalV1 {
    pub agent: BTreeMap<String, GitOid>,
    pub review: BTreeMap<String, GitOid>,
    pub evidence: Vec<EvidenceEntry>,
}
impl ExternalV1 {
    /// Bound captures reference a newly staged session, before the Agent root
    /// is selected. Admit its immutable inventory explicitly for closure audit.
    pub(crate) fn retain_staged_agent(
        &mut self,
        agent: &super::LocalAgentStore,
    ) -> LocalResult<()> {
        merge(&mut self.agent, agent.io.store.inventory())?;
        let mut evidence: BTreeMap<_, _> = self
            .evidence
            .iter()
            .map(|e| (e.reference.capsule, e.clone()))
            .collect();
        for value in agent.io.evidence.inventory() {
            if let Some(old) = evidence.insert(value.reference.capsule, value.clone())
                && (old.reference != value.reference || old.blob != value.blob)
            {
                return Err(LocalError::Corrupt);
            }
        }
        self.evidence = evidence.into_values().collect();
        Ok(())
    }
    pub fn retained(
        &self,
        repo: &LocalRepository,
        pin: Option<&Snapshot>,
    ) -> LocalResult<BTreeMap<GitOid, RetainedKind>> {
        let mut result: BTreeMap<_, _> = self
            .agent
            .values()
            .chain(self.review.values())
            .map(|oid| (*oid, RetainedKind::Blob))
            .collect();
        result
            .extend(EvidenceIo::new(repo.metadata.clone(), pin, self.evidence.clone())?.retained());
        Ok(result)
    }
    pub fn check_retained(
        &self,
        repo: &LocalRepository,
        pin: Option<&Snapshot>,
    ) -> LocalResult<()> {
        for blob in self.agent.values().chain(self.review.values()) {
            repo.metadata
                .read_retained_blob(pin.ok_or(LocalError::Corrupt)?, *blob)?;
        }
        EvidenceIo::new(repo.metadata.clone(), pin, self.evidence.clone())?;
        Ok(())
    }
    pub async fn complete(
        &self,
        repo: &LocalRepository,
        pin: Option<&Snapshot>,
        repository: RepositoryId,
        graph: &CollabGraphV1,
        closure: &BTreeSet<CollabObjectRef>,
        extend: bool,
    ) -> LocalResult<Self> {
        let mut agent = self.agent.clone();
        let mut review = self.review.clone();
        let mut evidence: BTreeMap<_, _> = self
            .evidence
            .iter()
            .map(|e| (e.reference.capsule, e.clone()))
            .collect();
        if extend {
            let a = repo.agent_store_at(pin, repository).await?;
            let r = repo.review_store_at(pin, repository).await?;
            merge(&mut agent, a.io.store.inventory())?;
            merge(&mut review, r.io.store.inventory())?;
            for e in a.io.evidence.inventory().into_iter().chain(
                r.io.evidence
                    .as_ref()
                    .ok_or(LocalError::Corrupt)?
                    .inventory(),
            ) {
                if let Some(old) = evidence.insert(e.reference.capsule, e.clone())
                    && (old.reference != e.reference || old.blob != e.blob)
                {
                    return Err(LocalError::Corrupt);
                }
            }
        }
        let entries: Vec<_> = evidence.into_values().collect();
        let a = AgentIo::new(
            Arc::new(GitRecords::new(
                repository,
                repo.metadata.clone(),
                agent.clone(),
            )?),
            EvidenceIo::new(repo.metadata.clone(), pin, entries.clone())?,
        );
        let r = ReviewIo::with_evidence(
            Arc::new(GitRecords::new(
                repository,
                repo.metadata.clone(),
                review.clone(),
            )?),
            EvidenceIo::new(repo.metadata.clone(), pin, entries.clone())?,
        );
        let mut pending_agent = BTreeSet::new();
        let mut pending_review = BTreeSet::new();
        let mut required_evidence = BTreeSet::new();
        for reference in closure {
            let value = graph.get(*reference)?;
            if !value.native_dependencies().is_empty() {
                return Err(LocalError::Unsupported);
            }
            pending_agent.extend(value.agent_dependencies());
            pending_review.extend(value.review_dependencies());
            required_evidence.extend(value.code_evidence());
            let anchor = match value {
                CollabObjectV1::ThreadVersion(v) => v.baseline.as_ref(),
                CollabObjectV1::ThreadLink(v) => match &v.target {
                    ThreadLinkTargetV1::Code { anchor, .. } => Some(anchor),
                    _ => None,
                },
                _ => None,
            };
            if let Some(ThreadCodeAnchorV1::Portable(anchor)) = anchor {
                for commit in &anchor.commits {
                    a.evidence.commit(&commit.evidence, &commit.commit)?;
                }
            }
        }
        let mut agent_seen = BTreeSet::new();
        let mut review_seen = BTreeSet::new();
        while let Some(reference) = pending_review.pop_first() {
            if !review_seen.insert(reference) {
                continue;
            }
            if review_seen.len() > 16_384 {
                return Err(LocalError::Limit);
            }
            let value = r.read_review_object(reference).await?;
            required_evidence.extend(value.code_evidence().map_err(|_| LocalError::Corrupt)?);
            pending_review.extend(
                value
                    .review_dependencies()
                    .map_err(|_| LocalError::Corrupt)?,
            );
            pending_agent.extend(
                value
                    .agent_dependencies()
                    .map_err(|_| LocalError::Corrupt)?,
            );
        }
        while let Some(reference) = pending_agent.pop_first() {
            if !agent_seen.insert(reference) {
                continue;
            }
            if agent_seen.len() > 16_384 {
                return Err(LocalError::Limit);
            }
            let value = a.read_agent_object(reference).await?;
            required_evidence.extend(value.code_evidence().map_err(|_| LocalError::Corrupt)?);
            pending_agent.extend(value.dependencies().map_err(|_| LocalError::Corrupt)?);
        }
        let agent = select_records(
            repository,
            RecordKind::AgentObject,
            agent_seen.iter().map(|r| *r.id.as_bytes()),
            &agent,
        )?;
        let review = select_records(
            repository,
            RecordKind::ReviewObject,
            review_seen.iter().map(|r| *r.id.as_bytes()),
            &review,
        )?;
        for reference in &required_evidence {
            a.evidence.read(reference)?;
        }
        let evidence: Vec<_> = entries
            .into_iter()
            .filter(|e| required_evidence.contains(&e.reference))
            .collect();
        if required_evidence.len() != evidence.len() {
            return Err(LocalError::Corrupt);
        }
        let result = Self {
            agent,
            review,
            evidence,
        };
        if !extend
            && serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)?
                != serde_json::to_vec(self).map_err(|_| LocalError::Corrupt)?
        {
            return Err(LocalError::Corrupt);
        }
        Ok(result)
    }
}
fn merge(
    target: &mut BTreeMap<String, GitOid>,
    source: BTreeMap<String, GitOid>,
) -> LocalResult<()> {
    for (key, value) in source {
        if target.insert(key, value).is_some_and(|old| old != value) {
            return Err(LocalError::Corrupt);
        }
    }
    Ok(())
}
fn select_records(
    repository: RepositoryId,
    kind: RecordKind,
    ids: impl Iterator<Item = [u8; 32]>,
    source: &BTreeMap<String, GitOid>,
) -> LocalResult<BTreeMap<String, GitOid>> {
    ids.map(|id| {
        let key = RecordRef {
            kind,
            digest: jj_store_model::RecordDigest::new(id),
            attempt: None,
        }
        .key(repository)?
        .as_str()
        .to_owned();
        Ok((key.clone(), *source.get(&key).ok_or(LocalError::Corrupt)?))
    })
    .collect()
}
