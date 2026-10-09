use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreadRootV1 {
    pub schema_version: u32,
    pub threads: Option<CollabObjectRef>,
    pub messages: Option<CollabObjectRef>,
    pub links: Option<CollabObjectRef>,
    pub assignments: Option<CollabObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CollabObjectRef>,
}
impl ThreadRootV1 {
    pub const fn empty() -> Self {
        Self {
            schema_version: 1,
            threads: None,
            messages: None,
            links: None,
            assignments: None,
            metadata: None,
        }
    }
    pub fn dependencies(&self) -> Vec<CollabObjectRef> {
        [
            self.threads,
            self.messages,
            self.links,
            self.assignments,
            self.metadata,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        for reference in self.dependencies() {
            expect(reference, CollabObjectKind::IndexPage)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReactionRootV1 {
    pub schema_version: u32,
    pub slots: Option<CollabObjectRef>,
    pub emojis: Option<CollabObjectRef>,
}
impl ReactionRootV1 {
    pub const fn empty() -> Self {
        Self {
            schema_version: 1,
            slots: None,
            emojis: None,
        }
    }
    pub fn dependencies(&self) -> Vec<CollabObjectRef> {
        [self.slots, self.emojis].into_iter().flatten().collect()
    }
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        for reference in self.dependencies() {
            expect(reference, CollabObjectKind::IndexPage)?;
        }
        Ok(())
    }
}
/// Bounded radix pages; providers path-copy pages and validate keys and closure membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "page", rename_all = "snake_case", deny_unknown_fields)]
pub enum CollabIndexPageV1 {
    Leaf {
        prefix: String,
        entries: BTreeMap<CollabCatalogKey, CollabObjectRef>,
    },
    Branch {
        prefix: String,
        children: BTreeMap<String, CollabObjectRef>,
    },
}
impl CollabIndexPageV1 {
    pub fn validate(&self) -> Result<()> {
        let prefix = match self {
            Self::Leaf { prefix, .. } | Self::Branch { prefix, .. } => prefix,
        };
        if prefix.len() > 64 || !prefix.bytes().all(hex_digit) {
            return Err(Error::Invalid("collaboration radix prefix"));
        }
        match self {
            Self::Leaf { entries, .. }
                if entries.len() > 32
                    || (entries.is_empty() && !prefix.is_empty())
                    || entries.keys().any(|k| !k.to_string().starts_with(prefix)) =>
            {
                Err(Error::Invalid("collaboration radix leaf"))
            }
            Self::Branch { children, .. } => {
                if prefix.len() >= 64
                    || children.is_empty()
                    || children.len() > 16
                    || children
                        .keys()
                        .any(|n| n.len() != 1 || !n.bytes().all(hex_digit))
                {
                    return Err(Error::Invalid("collaboration radix branch"));
                }
                for reference in children.values() {
                    expect(*reference, CollabObjectKind::IndexPage)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    pub fn dependencies(&self) -> Vec<CollabObjectRef> {
        match self {
            Self::Leaf { entries, .. } => entries.values().copied().collect(),
            Self::Branch { children, .. } => children.values().copied().collect(),
        }
    }
}
fn hex_digit(b: u8) -> bool {
    b.is_ascii_digit() || (b'a'..=b'f').contains(&b)
}
