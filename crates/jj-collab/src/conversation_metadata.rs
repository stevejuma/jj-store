//! Immutable application metadata; captured transcripts remain in AgentStore.
use crate::*;
use jj_agent::{AgentObjectKind, AgentObjectRef, AgentSessionId};
use jj_store_model::{PrincipalId, RequestId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConversationMetadataTargetV1 {
    Conversation { id: String },
    Recorded { session: AgentSessionId },
}
impl ConversationMetadataTargetV1 {
    pub fn key(&self) -> Result<CollabCatalogKey> {
        if let Self::Conversation { id } = self {
            if !label(id, 128) {
                return Err(Error::Invalid("conversation identity"));
            }
        }
        CollabCatalogKey::derive("conversation_metadata", &serde_json::to_vec(self)?)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversationMetadataSourceV1 {
    pub session: AgentSessionId,
    pub snapshot: AgentObjectRef,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GeneratedConversationMetadataV1 {
    pub title: String,
    pub summary: String,
    pub keywords: Vec<String>,
    pub goals: Vec<String>,
    pub sources: Vec<ConversationMetadataSourceV1>,
    pub provider: String,
    pub model: String,
    pub prompt_version: u32,
    pub generated_at_millis: i64,
    pub truncated: bool,
}
impl GeneratedConversationMetadataV1 {
    pub fn validate(&self) -> Result<()> {
        if !label(&self.title, 256)
            || self.title.split_whitespace().count() > 7
            || self.summary.trim().is_empty()
            || self.summary.len() > 4096
            || self.keywords.len() > 20
            || self.goals.len() > 6
            || self.sources.is_empty()
            || self.sources.len() > 16
            || !label(&self.provider, 64)
            || !label(&self.model, 256)
            || self.prompt_version != 1
            || self.generated_at_millis < 0
            || self
                .keywords
                .iter()
                .chain(&self.goals)
                .any(|s| !label(s, 256))
        {
            return Err(Error::Invalid("generated conversation metadata"));
        }
        let mut terms = std::collections::BTreeSet::new();
        if self
            .keywords
            .iter()
            .any(|k| !terms.insert(k.trim().to_lowercase()))
        {
            return Err(Error::Invalid("duplicate metadata keyword"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for s in &self.sources {
            if s.snapshot.kind != AgentObjectKind::SessionSnapshot || !seen.insert(s.session) {
                return Err(Error::Invalid("metadata source"));
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ConversationMetadataValueV1 {
    pub custom_title: Option<String>,
    pub generated: Option<GeneratedConversationMetadataV1>,
    pub agent_titles: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub agent_title_sources: BTreeMap<String, ConversationMetadataSourceV1>,
}
impl ConversationMetadataValueV1 {
    pub fn validate(&self) -> Result<()> {
        if self.custom_title.as_ref().is_some_and(|s| !label(s, 1024))
            || self.agent_titles.len() > 16
            || self
                .agent_titles
                .iter()
                .any(|(k, v)| !label(k, 128) || !label(v, 1024))
        {
            return Err(Error::Invalid("conversation title"));
        }
        if self.agent_title_sources.len() != self.agent_titles.len()
            || self.agent_title_sources.iter().any(|(id, s)| {
                !self.agent_titles.contains_key(id)
                    || s.snapshot.kind != AgentObjectKind::SessionSnapshot
            })
        {
            return Err(Error::Invalid("ACP title provenance"));
        }
        if let Some(g) = &self.generated {
            g.validate()?;
        }
        Ok(())
    }
    pub fn source_refs(&self) -> Vec<&ConversationMetadataSourceV1> {
        self.generated
            .iter()
            .flat_map(|g| g.sources.iter())
            .chain(self.agent_title_sources.values())
            .collect()
    }
    pub fn search_text(&self) -> String {
        let mut fields = vec![self.title().unwrap_or("").to_owned()];
        if let Some(g) = &self.generated {
            fields.push(g.summary.clone());
            fields.extend(g.keywords.clone());
            fields.extend(g.goals.clone());
        }
        fields.join("\n")
    }
    pub fn title(&self) -> Option<&str> {
        self.custom_title
            .as_deref()
            .or_else(|| self.generated.as_ref().map(|g| g.title.as_str()))
            .or_else(|| {
                let first = self.agent_titles.values().next()?;
                self.agent_titles
                    .values()
                    .all(|v| v == first)
                    .then_some(first.as_str())
            })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversationMetadataV1 {
    pub schema_version: u32,
    pub target: ConversationMetadataTargetV1,
    pub thread_id: ThreadId,
    pub value: ConversationMetadataValueV1,
    pub actor: PrincipalId,
    pub request_id: RequestId,
    pub accepted_at_millis: i64,
    pub previous: Option<CollabObjectRef>,
}
impl ConversationMetadataV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        self.target.key()?;
        self.value.validate()?;
        if self.accepted_at_millis < 0 {
            return Err(Error::Invalid("metadata time"));
        }
        if let Some(p) = self.previous {
            expect(p, CollabObjectKind::ConversationMetadata)?;
        }
        Ok(())
    }
}
impl ThreadStateV1 {
    pub(crate) fn set_metadata(
        &self,
        command: &ThreadCommandV1,
        actor: PrincipalId,
        request: RequestId,
        at: i64,
        graph: &mut CollabGraphV1,
    ) -> Result<(ThreadRootV1, ThreadCommandResultV1)> {
        let ThreadCommandV1::SetMetadata {
            thread_id,
            expected_thread,
            expected_metadata,
            target,
            value,
        } = command
        else {
            unreachable!()
        };
        let key = target.key()?;
        if self.threads.get(thread_id) != Some(expected_thread)
            || self.metadata.get(&key).copied() != *expected_metadata
        {
            return Err(Error::Stale);
        }
        let CollabObjectV1::ThreadVersion(mut thread) = graph.get(*expected_thread)?.clone() else {
            return Err(Error::Invalid("metadata owner"));
        };
        if thread.status == ThreadStatusV1::Tombstoned {
            return Err(Error::TargetUnavailable);
        }
        if let Some(old) = expected_metadata {
            let CollabObjectV1::ConversationMetadata(old) = graph.get(*old)? else {
                return Err(Error::Invalid("metadata selection"));
            };
            if old.thread_id != *thread_id {
                return Err(Error::Invalid("metadata owner changed"));
            }
        }
        let metadata = graph.stage(CollabObjectV1::ConversationMetadata(
            ConversationMetadataV1 {
                schema_version: 1,
                target: target.clone(),
                thread_id: *thread_id,
                value: *value.clone(),
                actor,
                request_id: request,
                accepted_at_millis: at,
                previous: *expected_metadata,
            },
        ))?;
        let event = graph.stage(CollabObjectV1::ThreadEvent(ThreadEventV1 {
            schema_version: 1,
            id: ThreadEventId::new(derive_event(request, b"metadata")),
            thread_id: *thread_id,
            actor,
            accepted_at_millis: at,
            previous: Some(thread.event),
            event: ThreadEventKindV1::Metadata { version: metadata },
        }))?;
        thread.previous = Some(*expected_thread);
        thread.event = event;
        thread.actor = actor;
        thread.accepted_at_millis = at;
        let version = graph.stage(CollabObjectV1::ThreadVersion(thread))?;
        let mut root = self.root.clone();
        root.threads = graph.catalog_update(
            root.threads,
            BTreeMap::from([(entity_key("thread", thread_id.as_bytes())?, version)]),
        )?;
        root.metadata = graph.catalog_update(root.metadata, BTreeMap::from([(key, metadata)]))?;
        Self::read(root.clone(), graph)?;
        Ok((
            root,
            ThreadCommandResultV1 {
                thread_id: *thread_id,
                version,
                event,
                message: None,
                link: None,
                reassigned_from: None,
            },
        ))
    }
}
fn derive_event(request: RequestId, domain: &[u8]) -> [u8; 16] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(request.as_bytes());
    h.update(domain);
    let digest = h.finalize();
    digest[..16].try_into().unwrap()
}
