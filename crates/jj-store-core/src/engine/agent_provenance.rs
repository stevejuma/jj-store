//! Shared stale-safe provenance projection without publication capability.
use super::agent::agent_error;
use super::*;
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentRelationPageV1,
    AgentRelationSubjectV1,
};

impl AgentProjector<'_> {
    pub async fn stage_agent_provenance(
        &self,
        base: &dyn AgentSnapshotRead,
        expected: Option<AgentObjectRef>,
        object: &AgentObjectV1,
    ) -> Result<AgentDeltaV1> {
        object.validate().map_err(agent_error)?;
        let (catalog, key, session_id, supersedes, mut subjects) = match object {
            AgentObjectV1::Association(v) => (
                AgentCatalogKind::Associations,
                AgentCatalogKey::derive("association", v.association_id.as_bytes()),
                v.session_id,
                v.supersedes,
                native_subjects(&v.before, &v.after),
            ),
            AgentObjectV1::PortableAssociation(v) => (
                AgentCatalogKind::Associations,
                AgentCatalogKey::derive("association", v.facts.association_id.as_bytes()),
                v.facts.session_id,
                v.facts.supersedes,
                portable_subjects(&v.before, &v.after),
            ),
            AgentObjectV1::CheckpointVersion(v) => (
                AgentCatalogKind::Checkpoints,
                AgentCatalogKey::derive("checkpoint", v.checkpoint_id.as_bytes()),
                v.session_id,
                v.supersedes,
                native_subjects(&v.before, &v.after),
            ),
            AgentObjectV1::PortableCheckpointVersion(v) => (
                AgentCatalogKind::Checkpoints,
                AgentCatalogKey::derive("checkpoint", v.facts.checkpoint_id.as_bytes()),
                v.facts.session_id,
                v.facts.supersedes,
                portable_subjects(&v.before, &v.after),
            ),
            _ => return Err(Error::Corrupt("agent provenance object kind")),
        };
        let key = key.map_err(agent_error)?;
        if base.repository() != self.objects.repository()
            || base.lookup(catalog, key).await? != expected
            || supersedes != expected
        {
            return Err(Error::Stale);
        }
        let session_key =
            AgentCatalogKey::derive("session", session_id.as_bytes()).map_err(agent_error)?;
        let session_ref = base
            .lookup(AgentCatalogKind::Sessions, session_key)
            .await?
            .ok_or(Error::Stale)?;
        let AgentObjectV1::SessionSnapshot(session) = base.read_object(session_ref).await? else {
            return Err(Error::Corrupt("agent provenance session"));
        };
        if session.session_id != session_id {
            return Err(Error::Corrupt("agent provenance session identity"));
        }
        for reference in object.dependencies().map_err(agent_error)? {
            base.read_object(reference).await?;
        }
        subjects.push(AgentRelationSubjectV1::Session { session_id });
        match object {
            AgentObjectV1::Association(v) => {
                association_subjects(&v.facts(), &session, &mut subjects)?
            }
            AgentObjectV1::PortableAssociation(v) => {
                association_subjects(&v.facts, &session, &mut subjects)?
            }
            AgentObjectV1::CheckpointVersion(v) => {
                checkpoint_subjects(base, &v.facts(), &mut subjects).await?
            }
            AgentObjectV1::PortableCheckpointVersion(v) => {
                checkpoint_subjects(base, &v.facts, &mut subjects).await?
            }
            _ => unreachable!(),
        }
        let replacement = self.stage_agent_object(object).await?;
        let mut edits = vec![AgentCatalogEditV1 {
            catalog,
            key,
            expected,
            replacement: Some(replacement),
        }];
        let mut keys = BTreeSet::new();
        for subject in subjects {
            let key = subject.catalog_key().map_err(agent_error)?;
            if !keys.insert(key) {
                continue;
            }
            let expected = base.lookup(AgentCatalogKind::Relations, key).await?;
            if let Some(previous) = expected {
                let AgentObjectV1::RelationPage(page) = base.read_object(previous).await? else {
                    return Err(Error::Corrupt("agent relation kind"));
                };
                if page.subject != subject {
                    return Err(Error::Corrupt("agent relation subject"));
                }
            }
            let page = self
                .stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
                    schema_version: 1,
                    subject,
                    selections: vec![replacement],
                    next: expected,
                }))
                .await?;
            edits.push(AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Relations,
                key,
                expected,
                replacement: Some(page),
            });
        }
        let delta = AgentDeltaV1 {
            schema_version: 1,
            edits,
        };
        delta.validate().map_err(agent_error)?;
        Ok(delta)
    }
}
fn native_subjects(
    before: &jj_agent::CodeAnchorV1,
    after: &jj_agent::CodeAnchorV1,
) -> Vec<AgentRelationSubjectV1> {
    before
        .operations
        .iter()
        .chain(&after.operations)
        .map(|id| AgentRelationSubjectV1::Operation { operation_id: *id })
        .chain(
            before
                .commits
                .iter()
                .chain(&after.commits)
                .map(|id| AgentRelationSubjectV1::Revision { commit_id: *id }),
        )
        .collect()
}
fn portable_subjects(
    before: &jj_agent::CodeAnchorV2,
    after: &jj_agent::CodeAnchorV2,
) -> Vec<AgentRelationSubjectV1> {
    before
        .operations
        .iter()
        .chain(&after.operations)
        .map(|evidence| AgentRelationSubjectV1::PortableOperation {
            evidence: evidence.clone(),
        })
        .chain(before.commits.iter().chain(&after.commits).map(|v| {
            AgentRelationSubjectV1::PortableRevision {
                evidence: v.evidence.clone(),
                commit: v.commit,
            }
        }))
        .collect()
}
fn association_subjects(
    facts: &jj_agent::CodeAssociationFactsV1,
    session: &jj_agent::AgentSessionSnapshotV1,
    subjects: &mut Vec<AgentRelationSubjectV1>,
) -> Result<()> {
    for id in &facts.tool_call_ids {
        if !session.tools.contains_key(id) {
            return Err(Error::Stale);
        }
        subjects.push(AgentRelationSubjectV1::Tool {
            session_id: facts.session_id,
            tool_call_id: *id,
        });
    }
    if let Some(checkpoint_id) = facts.checkpoint_id {
        subjects.push(AgentRelationSubjectV1::Checkpoint { checkpoint_id });
    }
    Ok(())
}
async fn checkpoint_subjects(
    base: &dyn AgentSnapshotRead,
    facts: &jj_agent::AgentCheckpointFactsV1,
    subjects: &mut Vec<AgentRelationSubjectV1>,
) -> Result<()> {
    let AgentObjectV1::SessionSnapshot(session) = base.read_object(facts.session_snapshot).await?
    else {
        return Err(Error::Corrupt("checkpoint session snapshot"));
    };
    if session.session_id != facts.session_id
        || !session.transcripts.contains(&facts.transcript_manifest)
        || facts
            .event_batches
            .iter()
            .any(|v| !session.event_batches.contains(v))
        || facts
            .tools
            .iter()
            .any(|(id, v)| session.tools.get(id) != Some(v))
    {
        return Err(Error::Corrupt("checkpoint pinned session dependencies"));
    }
    for reference in &facts.associations {
        let valid = match base.read_object(*reference).await? {
            AgentObjectV1::Association(v) => {
                v.session_id == facts.session_id
                    && v.checkpoint_id.is_none_or(|id| id == facts.checkpoint_id)
            }
            AgentObjectV1::PortableAssociation(v) => {
                v.facts.session_id == facts.session_id
                    && v.facts
                        .checkpoint_id
                        .is_none_or(|id| id == facts.checkpoint_id)
            }
            _ => false,
        };
        if !valid {
            return Err(Error::Corrupt("checkpoint association ownership"));
        }
    }
    subjects.push(AgentRelationSubjectV1::Checkpoint {
        checkpoint_id: facts.checkpoint_id,
    });
    for id in facts.tools.keys() {
        subjects.push(AgentRelationSubjectV1::Tool {
            session_id: facts.session_id,
            tool_call_id: *id,
        });
    }
    Ok(())
}
