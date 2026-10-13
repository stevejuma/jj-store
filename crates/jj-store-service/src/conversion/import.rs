//! Destination preparation selects every supported historical collaboration state.
use super::*;
use sha2::{Digest as _, Sha256};
use std::sync::Arc;
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CollaborationMappingsV1 {
    pub selections: Vec<SelectionMappingV1>,
    pub objects: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelectionMappingV1 {
    pub source: String,
    pub destination: String,
    pub versions: BTreeMap<String, u64>,
}
pub(crate) fn request_id(request: m::RequestId, index: usize) -> m::RequestId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/conversion-request/v1\0");
    hash.update(request.as_bytes());
    hash.update((index as u64).to_be_bytes());
    m::RequestId::from_slice(&hash.finalize()[..16]).expect("fixed identity width")
}
pub(crate) fn add_map<K: Serialize, V: Serialize>(
    target: &mut BTreeMap<String, String>,
    domain: &str,
    map: &BTreeMap<K, V>,
) -> Result<()> {
    for (old, new) in map {
        let key = format!("{domain}:{}", serde_json::to_string(old)?);
        let value = serde_json::to_string(new)?;
        if target
            .insert(key, value.clone())
            .is_some_and(|old| old != value)
        {
            return Err("inconsistent conversion mapping".into());
        }
    }
    Ok(())
}
impl crate::native::NativeSession {
    /// Private staging destination only; never run against an existing product repository.
    pub async fn import_conversion_history(
        &self,
        archive: &RepositoryArchiveV1,
        request: m::RequestId,
    ) -> Result<CollaborationMappingsV1> {
        let principal = self.principal(crate::auth::Permission::Admin).await?.id();
        let initial = self.snapshot().await?;
        if initial.manifest().published.0 != 0 {
            return Err("conversion history requires a new destination".into());
        }
        archive.encode()?;
        let mut mappings = CollaborationMappingsV1::default();
        for (index, selection) in archive.collaboration.iter().enumerate() {
            let (agent, agents) = transfer_agent(&selection.agent, &self.coordinator).await?;
            let (review, reviews) = transfer_review(&selection.review, &self.coordinator).await?;
            let (collab, graph, objects) =
                transfer_collab(&selection.collab, self.repository, &agents, &reviews)?;
            add_map(&mut mappings.objects, "agent", &agents)?;
            add_map(&mut mappings.objects, "review", &reviews)?;
            add_map(&mut mappings.objects, "collaboration", &objects)?;
            let mut roots = BTreeMap::from([
                (
                    Domain::Agent,
                    self.coordinator.stage_agent_root(&agent).await?,
                ),
                (
                    Domain::Review,
                    self.coordinator.stage_review_root(&review).await?,
                ),
                // WorkStore destinations freeze origin execution. Links remain original IDs.
                (
                    Domain::Work,
                    self.coordinator
                        .stage_conversion_work(
                            &selection
                                .work
                                .as_ref()
                                .map(|s| s.state())
                                .transpose()?
                                .unwrap_or_default(),
                        )
                        .await?,
                ),
            ]);
            if let Some(root) = collab.thread {
                roots.insert(
                    Domain::Thread,
                    self.coordinator
                        .stage_native_thread_root(&root, &graph)
                        .await?,
                );
            }
            if let Some(root) = collab.reaction {
                roots.insert(
                    Domain::Reaction,
                    self.coordinator
                        .stage_native_reaction_root(&root, &graph)
                        .await?,
                );
            }
            let base = self.snapshot().await?;
            let mut changes = Vec::new();
            for (domain, root) in roots {
                let current = base.manifest().domain_state(domain);
                let expected = match current {
                    Some(state) => state,
                    None => DomainState {
                        root: match domain {
                            Domain::Work => {
                                self.coordinator
                                    .stage_conversion_work(&WorkStateV1::default())
                                    .await?
                            }
                            Domain::Thread => {
                                self.coordinator
                                    .stage_native_thread_root(
                                        &jj_collab::ThreadRootV1::empty(),
                                        &jj_collab::CollabGraphV1::default(),
                                    )
                                    .await?
                            }
                            Domain::Reaction => {
                                self.coordinator
                                    .stage_native_reaction_root(
                                        &jj_collab::ReactionRootV1::empty(),
                                        &jj_collab::CollabGraphV1::default(),
                                    )
                                    .await?
                            }
                            _ => return Err("missing destination domain".into()),
                        },
                        version: 0,
                    },
                };
                // Select each source historical version, including identical-body versions.
                let name = domain_name(domain);
                let previous = index
                    .checked_sub(1)
                    .and_then(|i| archive.collaboration.get(i));
                if current.is_none()
                    || index == 0
                    || previous.is_none_or(|p| p.versions.get(name) != selection.versions.get(name))
                {
                    changes.push(DomainSelectionV1 {
                        domain,
                        expected,
                        root,
                    });
                }
            }
            if !changes.is_empty() {
                let first = changes.remove(0);
                let id = request_id(request, index);
                let mut publication = DomainRequest::new(
                    principal,
                    id,
                    m::PublicationId::new(*id.as_bytes()),
                    first.domain,
                    first.expected,
                    first.root,
                    serde_json::to_vec(&selection.source_pin)?,
                )?;
                for change in changes {
                    publication = publication.with_additional(change)?;
                }
                if !matches!(
                    self.coordinator.publish_domain(&publication).await?,
                    PublishOutcome::Committed(_)
                ) {
                    return Err("conversion staging outcome unknown".into());
                }
            }
            let pin = self.snapshot().await?;
            let versions = [
                Domain::Agent,
                Domain::Review,
                Domain::Work,
                Domain::Thread,
                Domain::Reaction,
            ]
            .into_iter()
            .filter_map(|d| {
                pin.manifest()
                    .domain_state(d)
                    .map(|s| (domain_name(d).into(), s.version))
            })
            .collect();
            mappings.selections.push(SelectionMappingV1 {
                source: selection.source_pin.clone(),
                destination: pin.digest().to_string(),
                versions,
            });
        }
        self.principal(crate::auth::Permission::Admin).await?;
        Ok(mappings)
    }
    pub async fn copy_conversion_to(
        &self,
        destination: Arc<dyn jj_store_object::ObjectStore>,
    ) -> Result<String> {
        self.principal(crate::auth::Permission::Admin).await?;
        let objects = Arc::new(jj_store_jj::objects::NativeObjects::new(
            destination.clone(),
            self.repository,
            16 * 1024 * 1024,
        )?);
        let copied = self
            .coordinator
            .copy_native_to(destination, objects, self.limits.inventory_nodes)
            .await?;
        Ok(copied.snapshot.digest().to_string())
    }
    fn conversion_code_request(
        &self,
        principal: m::PrincipalId,
        request: m::RequestId,
        operation: m::OperationId,
        archive_digest: &str,
    ) -> Request {
        let id = request_id(request, 4096);
        Request {
            principal,
            request_id: id,
            publication_id: m::PublicationId::new(*id.as_bytes()),
            transition: Some(Transition::Native {
                remove: BTreeSet::from([jj_store_jj::roots::ROOT_OPERATION]),
                add: operation,
            }),
            settings: None,
            result: archive_digest.as_bytes().to_vec(),
            kind: SemanticKind::OperationPublished,
        }
    }
    pub async fn publish_conversion_code(
        &self,
        request: m::RequestId,
        operation: m::OperationId,
        archive_digest: &str,
    ) -> Result<bool> {
        let principal = self.principal(crate::auth::Permission::Admin).await?.id();
        let request = self.conversion_code_request(principal, request, operation, archive_digest);
        Ok(matches!(
            self.publish_request(&request).await?,
            PublishOutcome::Committed(_)
        ))
    }
    pub async fn conversion_code_receipt(
        &self,
        request: m::RequestId,
        operation: m::OperationId,
        archive_digest: &str,
    ) -> Result<bool> {
        let principal = self.principal(crate::auth::Permission::Admin).await?.id();
        let request = self.conversion_code_request(principal, request, operation, archive_digest);
        let receipt = self.coordinator.status(&request).await?;
        self.principal(crate::auth::Permission::Admin).await?;
        Ok(receipt.is_some())
    }
}
fn domain_name(domain: Domain) -> &'static str {
    match domain {
        Domain::Agent => "agent",
        Domain::Review => "review",
        Domain::Work => "work",
        Domain::Thread => "thread",
        Domain::Reaction => "reaction",
        Domain::Git => "git",
        Domain::Workflow => "workflow",
    }
}
