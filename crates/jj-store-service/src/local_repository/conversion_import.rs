//! Typed conversion publication only in a newly prepared local destination.
use super::*;
use crate::conversion::{
    import::{add_map, request_id},
    *,
};
use jj_store_jj::local_metadata::{BatchUpdate, Domain, DomainUpdate, Preparation, PublishOutcome};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeMap, sync::Arc};
impl LocalRepository {
    pub async fn import_conversion_history(
        &self,
        archive: &RepositoryArchiveV1,
        repository: m::RepositoryId,
        actor: m::PrincipalId,
        request: m::RequestId,
    ) -> crate::conversion::Result<CollaborationMappingsV1> {
        if self.metadata.read()?.is_some() {
            return Err("conversion history requires a new destination".into());
        }
        let archive_bytes = archive.encode()?;
        let archive_blob = self.metadata.stage_blob(&archive_bytes)?;
        let mut mappings = CollaborationMappingsV1::default();
        for (index, source) in archive.collaboration.iter().enumerate() {
            let base = self.metadata.read()?;
            let agents = Arc::new(records::GitRecords::new(
                repository,
                self.metadata.clone(),
                BTreeMap::new(),
            )?);
            let agent = agent_io::AgentIo::new(
                agents.clone(),
                agent_evidence::EvidenceIo::new(self.metadata.clone(), None, Vec::new())?,
            );
            let reviews = Arc::new(records::GitRecords::new(
                repository,
                self.metadata.clone(),
                BTreeMap::new(),
            )?);
            let review = review_io::ReviewIo::new(reviews.clone());
            let (aroot, amap) = transfer_agent(&source.agent, &agent).await?;
            let (rroot, rmap) = transfer_review(&source.review, &review).await?;
            let (collab, graph, cmap) = transfer_collab(&source.collab, repository, &amap, &rmap)?;
            add_map(&mut mappings.objects, "agent", &amap)?;
            add_map(&mut mappings.objects, "review", &rmap)?;
            add_map(&mut mappings.objects, "collaboration", &cmap)?;
            let ar = agent.stage_root(&aroot).await?;
            let rr = review.stage_root(&rroot).await?;
            let mut bytes = BTreeMap::from([
                (
                    Domain::Agent,
                    serde_json::to_vec(&agent_descriptor::LocalAgentRootV2 {
                        version: 2,
                        repository,
                        root: ar,
                        records: agents.inventory(),
                        evidence: Vec::new(),
                    })?,
                ),
                (
                    Domain::Review,
                    serde_json::to_vec(&review_io::LocalReviewRootV1 {
                        version: 1,
                        repository,
                        root: rr,
                        records: reviews.inventory(),
                        evidence: Vec::new(),
                    })?,
                ),
            ]);
            let mut retained = agents.retained();
            retained.extend(reviews.retained());
            retained.insert(archive_blob, metadata::RetainedKind::Blob);
            if let Some(work) = &source.work {
                bytes.insert(
                    Domain::Work,
                    serde_json::to_vec(&work::LocalWorkRootV1::from_state(
                        repository,
                        work.state()?,
                    ))?,
                );
            }
            let external = collab_external::ExternalV1 {
                agent: agents.inventory(),
                review: reviews.inventory(),
                evidence: Vec::new(),
            };
            if let Some(root) = collab.thread {
                let (value, refs) =
                    collab_graph::stage(self, base.as_ref(), repository, root, &graph, &external)
                        .await?;
                bytes.insert(Domain::Thread, value);
                retained.extend(refs);
            }
            if let Some(root) = collab.reaction {
                let (value, refs) =
                    collab_graph::stage(self, base.as_ref(), repository, root, &graph, &external)
                        .await?;
                bytes.insert(Domain::Reaction, value);
                retained.extend(refs);
            }
            let previous = index
                .checked_sub(1)
                .and_then(|i| archive.collaboration.get(i));
            let domains = bytes
                .into_iter()
                .filter(|(domain, _)| {
                    let name = match domain {
                        Domain::Agent => "agent",
                        Domain::Review => "review",
                        Domain::Work => "work",
                        Domain::Thread => "thread",
                        Domain::Reaction => "reaction",
                    };
                    index == 0
                        || previous
                            .is_none_or(|p| p.versions.get(name) != source.versions.get(name))
                })
                .map(|(domain, bytes)| {
                    (
                        domain,
                        DomainUpdate {
                            expected: base.as_ref().and_then(|s| s.selection(domain)).cloned(),
                            bytes,
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>();
            if !domains.is_empty() {
                let result = serde_json::to_vec(&(source.source_pin.as_str(), archive_blob))?;
                let mut digest = Sha256::new_with_prefix(b"jj-store/local-conversion/v1\0");
                digest.update(&archive_bytes);
                digest.update((index as u64).to_be_bytes());
                digest.update(repository.as_bytes());
                let update = BatchUpdate {
                    repository,
                    actor,
                    request: request_id(request, index),
                    intent: m::IntentDigest::new(digest.finalize().into()),
                    domains,
                    result,
                    retained,
                };
                match self.metadata.prepare_many(base.as_ref(), &update)? {
                    Preparation::Committed(_) => {}
                    Preparation::Ready(candidate) => {
                        if !matches!(
                            self.metadata.publish(*candidate)?,
                            PublishOutcome::Committed(_)
                        ) {
                            return Err("local conversion staging publication unknown".into());
                        }
                    }
                }
            }
            let selected = self
                .metadata
                .read()?
                .ok_or("conversion destination identity")?;
            let versions = [
                Domain::Agent,
                Domain::Review,
                Domain::Work,
                Domain::Thread,
                Domain::Reaction,
            ]
            .into_iter()
            .filter_map(|domain| {
                let name = match domain {
                    Domain::Agent => "agent",
                    Domain::Review => "review",
                    Domain::Work => "work",
                    Domain::Thread => "thread",
                    Domain::Reaction => "reaction",
                };
                selected.selection(domain).map(|s| (name.into(), s.version))
            })
            .collect();
            mappings.selections.push(SelectionMappingV1 {
                source: source.source_pin.clone(),
                destination: selected.commit().to_string(),
                versions,
            });
            // Read through the same selected provider audits used after reopening.
            self.work_state(Some(&selected))?;
            self.agent_store_at(Some(&selected), repository).await?;
            self.review_store_at(Some(&selected), repository).await?;
            self.thread_store_at(Some(&selected)).await?;
            self.reaction_store_at(Some(&selected), repository).await?;
        }
        Ok(mappings)
    }
}
