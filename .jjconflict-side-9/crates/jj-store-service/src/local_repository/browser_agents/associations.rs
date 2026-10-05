use super::*;
use std::collections::BTreeSet;
impl LocalRepository {
    pub async fn browser_agent_relations(
        &self,
        commit: MetadataCommit,
        subject: LocalAgentSubject,
        limit: usize,
        after: Option<AgentCatalogKey>,
    ) -> LocalResult<LocalBrowserAgentRelations> {
        list(
            &self.browser_agent_store(commit).await?,
            &subject,
            limit,
            after,
        )
        .await
    }
}
pub(super) async fn list(
    store: &LocalAgentStore,
    subject: &LocalAgentSubject,
    limit: usize,
    after: Option<AgentCatalogKey>,
) -> LocalResult<LocalBrowserAgentRelations> {
    if !(1..=256).contains(&limit) {
        return Err(LocalError::Invalid);
    }
    let mut offset = after;
    let mut associations = Vec::new();
    let mut inspected = 0;
    let mut bytes = 1024usize;
    let next_after = 'scan: loop {
        let page = store
            .reads()
            .list_catalog(AgentCatalogKind::Associations, offset, 256)
            .await?;
        for (key, reference) in page.selections {
            inspected += 1;
            if inspected > 4096 {
                return Err(LocalError::Limit);
            }
            let AgentObjectV1::PortableAssociation(value) = store.object(reference).await? else {
                return Err(LocalError::Unsupported);
            };
            let matches = match subject {
                LocalAgentSubject::Session { session_id } => value.facts.session_id == *session_id,
                LocalAgentSubject::Operation {
                    repository,
                    operation_id,
                } => [&value.before, &value.after].iter().any(|a| {
                    a.operations
                        .iter()
                        .any(|r| r.repository == *repository && r.operation == *operation_id)
                }),
                LocalAgentSubject::Revision {
                    repository,
                    commit_id,
                } => [&value.before, &value.after].iter().any(|a| {
                    a.commits.iter().any(|r| {
                        r.evidence.repository == *repository
                            && r.commit == m::portable::CodeCommitIdV1::GitSha1Jj045(*commit_id)
                    })
                }),
            };
            if matches {
                if associations.len() == limit {
                    break 'scan offset;
                }
                let row = {
                    let evidence: BTreeSet<_> =
                        value.code_evidence().map_err(|_| LocalError::Corrupt)?;
                    if evidence
                        .iter()
                        .any(|r| r.profile != m::portable::CodeProfileV1::GitSha1Jj045)
                    {
                        return Err(LocalError::Unsupported);
                    }
                    let commit = |id| match id {
                        m::portable::CodeCommitIdV1::GitSha1Jj045(id) => Ok(id),
                        _ => Err(LocalError::Unsupported),
                    };
                    LocalBrowserCodeAssociation {
                        profile: m::portable::CodeProfileV1::GitSha1Jj045,
                        source_repositories: evidence
                            .iter()
                            .map(|r| r.repository)
                            .collect::<BTreeSet<_>>()
                            .into_iter()
                            .collect(),
                        evidence: evidence.into_iter().collect(),
                        association_id: value.facts.association_id,
                        object_id: reference.id,
                        session_id: value.facts.session_id,
                        tool_call_ids: value.facts.tool_call_ids.iter().copied().collect(),
                        relation: value.facts.relation,
                        before_operations: value
                            .before
                            .operations
                            .iter()
                            .map(|r| r.operation)
                            .collect(),
                        after_operations: value
                            .after
                            .operations
                            .iter()
                            .map(|r| r.operation)
                            .collect(),
                        before_commits: value
                            .before
                            .commits
                            .iter()
                            .map(|c| commit(c.commit))
                            .collect::<LocalResult<Vec<_>>>()?,
                        after_commits: value
                            .after
                            .commits
                            .iter()
                            .map(|c| commit(c.commit))
                            .collect::<LocalResult<Vec<_>>>()?,
                        changes: value.facts.changes.iter().copied().collect(),
                        paths: value.facts.paths.clone(),
                        created_at_millis: value.facts.created_at_millis,
                    }
                };
                bytes = bytes
                    .checked_add(
                        serde_json::to_vec(&row)
                            .map_err(|_| LocalError::Corrupt)?
                            .len(),
                    )
                    .ok_or(LocalError::Limit)?;
                if bytes > 4 * 1024 * 1024 {
                    return Err(LocalError::Limit);
                }
                associations.push(row);
            }
            offset = Some(key);
        }
        if page.next_after.is_none() {
            break None;
        }
    };
    Ok(LocalBrowserAgentRelations {
        version: 1,
        snapshot: store.snapshot.clone(),
        subject: subject.clone(),
        associations,
        next_after,
    })
}
