use crate::{Error, Result, ReviewCatalogKey, ReviewObjectKind, ReviewObjectRef, ReviewRootId};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewRootV1 {
    pub schema_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_events: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_relations: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_relations: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_requests: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<ReviewObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<ReviewObjectRef>,
}
impl ReviewRootV1 {
    pub const fn empty() -> Self {
        Self {
            schema_version: 1,
            threads: None,
            comments: None,
            thread_events: None,
            anchor_relations: None,
            artifact_relations: None,
            statuses: None,
            work_requests: None,
            aliases: None,
            sources: None,
        }
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self
                .dependencies_unchecked()
                .iter()
                .any(|value| value.kind != ReviewObjectKind::IndexPage)
        {
            return Err(Error::Invalid("review root"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<Vec<ReviewObjectRef>> {
        self.validate()?;
        Ok(self.dependencies_unchecked())
    }
    pub fn root_id(&self) -> Result<ReviewRootId> {
        Ok(ReviewRootId::new(
            Sha256::digest(crate::encode_root(self)?).into(),
        ))
    }
    fn dependencies_unchecked(&self) -> Vec<ReviewObjectRef> {
        [
            self.threads,
            self.comments,
            self.thread_events,
            self.anchor_relations,
            self.artifact_relations,
            self.statuses,
            self.work_requests,
            self.aliases,
            self.sources,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "page", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReviewIndexPageV1 {
    Leaf {
        prefix: String,
        entries: BTreeMap<ReviewCatalogKey, ReviewObjectRef>,
    },
    Branch {
        prefix: String,
        children: BTreeMap<String, ReviewObjectRef>,
    },
}
impl ReviewIndexPageV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Leaf { prefix, entries } => {
                validate_prefix(prefix)?;
                if entries.len() > 32
                    || (entries.is_empty() && !prefix.is_empty())
                    || entries
                        .keys()
                        .any(|key| !key.to_string().starts_with(prefix))
                {
                    return Err(Error::Invalid("review catalog leaf"));
                }
            }
            Self::Branch { prefix, children } => {
                validate_prefix(prefix)?;
                if prefix.len() >= 64
                    || children.is_empty()
                    || children.len() > 16
                    || children.iter().any(|(nibble, child)| {
                        nibble.len() != 1
                            || !nibble.bytes().all(is_hex)
                            || child.kind != ReviewObjectKind::IndexPage
                    })
                {
                    return Err(Error::Invalid("review catalog branch"));
                }
            }
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Vec<ReviewObjectRef> {
        let values: BTreeSet<_> = match self {
            Self::Leaf { entries, .. } => entries.values().copied().collect(),
            Self::Branch { children, .. } => children.values().copied().collect(),
        };
        values.into_iter().collect()
    }
}

fn validate_prefix(prefix: &str) -> Result<()> {
    if prefix.len() > 64 || !prefix.bytes().all(is_hex) {
        return Err(Error::Invalid("review catalog prefix"));
    }
    Ok(())
}
fn is_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}
