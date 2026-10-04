use crate::{
    AnchorId, ArtifactId, Error, Result, ReviewCatalogKey, ReviewObjectKind, ReviewObjectRef,
    ReviewThreadId, ThreadStatusV1,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const MAX_RELATION_ENTRIES: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "subject", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReviewRelationSubjectV1 {
    OriginalAnchor { anchor_id: AnchorId },
    Artifact { artifact_id: ArtifactId },
}
impl ReviewRelationSubjectV1 {
    pub fn catalog_key(&self) -> Result<ReviewCatalogKey> {
        match self {
            Self::OriginalAnchor { anchor_id } => {
                ReviewCatalogKey::derive("anchor-relation", anchor_id.as_bytes())
            }
            Self::Artifact { artifact_id } => {
                ReviewCatalogKey::derive("artifact-relation", artifact_id.as_bytes())
            }
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ReviewThreadRelationV1 {
    pub thread_id: ReviewThreadId,
    pub snapshot: ReviewObjectRef,
}
impl ReviewThreadRelationV1 {
    fn validate(self) -> Result<()> {
        if self.snapshot.kind == ReviewObjectKind::ThreadSnapshot {
            Ok(())
        } else {
            Err(Error::Invalid("review relation snapshot"))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewRelationPageV1 {
    pub schema_version: u32,
    pub subject: ReviewRelationSubjectV1,
    pub entries: Vec<ReviewThreadRelationV1>,
    pub next: Option<ReviewObjectRef>,
}
impl ReviewRelationPageV1 {
    pub fn validate(&self) -> Result<()> {
        validate_entries(self.schema_version, &self.entries)?;
        if self
            .next
            .is_some_and(|value| value.kind != ReviewObjectKind::RelationPage)
        {
            return Err(Error::Invalid("review relation continuation"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> BTreeSet<ReviewObjectRef> {
        self.entries
            .iter()
            .map(|entry| entry.snapshot)
            .chain(self.next)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewStatusPageV1 {
    pub schema_version: u32,
    pub status: ThreadStatusV1,
    pub entries: Vec<ReviewThreadRelationV1>,
    pub next: Option<ReviewObjectRef>,
}
impl ReviewStatusPageV1 {
    pub fn validate(&self) -> Result<()> {
        validate_entries(self.schema_version, &self.entries)?;
        if self
            .next
            .is_some_and(|value| value.kind != ReviewObjectKind::StatusPage)
        {
            return Err(Error::Invalid("review status continuation"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> BTreeSet<ReviewObjectRef> {
        self.entries
            .iter()
            .map(|entry| entry.snapshot)
            .chain(self.next)
            .collect()
    }
}

pub fn review_status_key(status: ThreadStatusV1) -> Result<ReviewCatalogKey> {
    let value = match status {
        ThreadStatusV1::Open => b"open".as_slice(),
        ThreadStatusV1::Resolved => b"resolved".as_slice(),
    };
    ReviewCatalogKey::derive("thread-status", value)
}

fn validate_entries(schema_version: u32, entries: &[ReviewThreadRelationV1]) -> Result<()> {
    if schema_version != 1 || entries.is_empty() || entries.len() > MAX_RELATION_ENTRIES {
        return Err(Error::Invalid("review relation entries"));
    }
    let mut threads = BTreeSet::new();
    for entry in entries {
        entry.validate()?;
        if !threads.insert(entry.thread_id) {
            return Err(Error::Invalid("duplicate review relation thread"));
        }
    }
    Ok(())
}
