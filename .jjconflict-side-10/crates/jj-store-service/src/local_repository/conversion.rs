//! Read-only conversion captures every audited companion selection and exact Git retention.
use super::*;
use crate::conversion::*;
use jj_store_jj::local_metadata::Domain;
use std::{collections::BTreeMap, sync::Arc};
impl LocalRepository {
    pub async fn conversion_archive(
        &self,
        operation: m::OperationId,
    ) -> crate::conversion::Result<RepositoryArchiveV1> {
        let selected = self
            .metadata
            .read()?
            .ok_or("local repository identity is absent")?;
        self.check_identity(Some(&selected))?;
        let repository = selected.repository();
        let code = jj_store_jj::conversion::archive_local(
            &self.repo_path,
            repository,
            operation,
            Some(selected.commit()),
        )
        .await?;
        let mut size = serde_json::to_vec(&code)?.len();
        if size > 16 * 1024 * 1024 {
            return Err("conversion archive byte limit".into());
        }
        let mut history = self.metadata.history(&selected)?;
        history.reverse();
        let mut collaboration = Vec::new();
        for pin in history {
            let work = pin
                .selection(Domain::Work)
                .map(|_| -> crate::conversion::Result<_> {
                    WorkSnapshotArchiveV1::from_state(self.work_state(Some(&pin))?)
                })
                .transpose()?;
            let mut agent =
                archive_agent(&self.agent_store_at(Some(&pin), repository).await?).await?;
            let mut review =
                archive_review(&self.review_store_at(Some(&pin), repository).await?).await?;
            let thread = self.thread_store_at(Some(&pin)).await?;
            let reaction = self.reaction_store_at(Some(&pin), repository).await?;
            let mut graph = thread.graph.objects.clone();
            graph.extend(reaction.graph.objects.clone());
            let mut agents = thread.external.agent.clone();
            agents.extend(reaction.external.agent.clone());
            let mut reviews = thread.external.review.clone();
            reviews.extend(reaction.external.review.clone());
            let mut evidence = BTreeMap::new();
            for entry in thread
                .external
                .evidence
                .iter()
                .chain(&reaction.external.evidence)
            {
                evidence.insert(entry.reference.capsule, entry.clone());
            }
            let agent_io = super::agent_io::AgentIo::new(
                Arc::new(super::records::GitRecords::new(
                    repository,
                    self.metadata.clone(),
                    agents,
                )?),
                super::agent_evidence::EvidenceIo::new(
                    self.metadata.clone(),
                    Some(&pin),
                    evidence.into_values().collect(),
                )?,
            );
            let review_io = super::review_io::ReviewIo::new(Arc::new(
                super::records::GitRecords::new(repository, self.metadata.clone(), reviews)?,
            ));
            crate::conversion::history::include_external(
                &mut agent,
                &mut review,
                graph.values(),
                &agent_io,
                &review_io,
            )
            .await?;
            let mut versions = BTreeMap::new();
            for (name, domain) in [
                ("work", Domain::Work),
                ("agent", Domain::Agent),
                ("review", Domain::Review),
                ("thread", Domain::Thread),
                ("reaction", Domain::Reaction),
            ] {
                if let Some(selection) = pin.selection(domain) {
                    versions.insert(name.into(), selection.version);
                }
            }
            let state = CollaborationSelectionV1 {
                source_pin: format!("local:{}", pin.commit()),
                versions,
                work,
                agent,
                review,
                collab: CollabGraphArchiveV1 {
                    thread: pin.selection(Domain::Thread).map(|_| thread.state.root),
                    reaction: pin.selection(Domain::Reaction).map(|_| reaction.state.root),
                    objects: graph.into_iter().collect(),
                },
            };
            size = size
                .checked_add(serde_json::to_vec(&state)?.len())
                .ok_or("conversion archive byte limit")?;
            if size > 16 * 1024 * 1024 {
                return Err("conversion archive byte limit".into());
            }
            collaboration.push(state);
        }
        let value = RepositoryArchiveV1 {
            version: 1,
            repository,
            source_pin: format!("local:{}:{operation}", selected.commit()),
            code,
            collaboration,
            native_records: Vec::new(),
            native_receipts: Vec::new(),
        };
        value.encode()?;
        Ok(value)
    }
}
