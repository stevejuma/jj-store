use crate::{Error, Result, ReviewCatalogKey, ReviewObjectKind, ReviewObjectRef};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCatalogKind {
    Threads,
    Comments,
    ThreadEvents,
    AnchorRelations,
    ArtifactRelations,
    Statuses,
}
impl ReviewCatalogKind {
    fn accepts(self, value: ReviewObjectRef) -> bool {
        matches!(
            (self, value.kind),
            (Self::Threads, ReviewObjectKind::ThreadSnapshot)
                | (Self::Comments, ReviewObjectKind::CommentRevision)
                | (Self::ThreadEvents, ReviewObjectKind::EventPage)
                | (
                    Self::AnchorRelations | Self::ArtifactRelations,
                    ReviewObjectKind::RelationPage
                )
                | (Self::Statuses, ReviewObjectKind::StatusPage)
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewCatalogEditV1 {
    pub catalog: ReviewCatalogKind,
    pub key: ReviewCatalogKey,
    pub expected: Option<ReviewObjectRef>,
    pub replacement: Option<ReviewObjectRef>,
}
impl ReviewCatalogEditV1 {
    fn validate(&self) -> Result<()> {
        if self.expected == self.replacement
            || self
                .expected
                .into_iter()
                .chain(self.replacement)
                .any(|value| !self.catalog.accepts(value))
        {
            return Err(Error::Invalid("review catalog edit"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewDeltaV1 {
    pub schema_version: u32,
    pub edits: Vec<ReviewCatalogEditV1>,
}
impl ReviewDeltaV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.edits.is_empty() || self.edits.len() > 4096 {
            return Err(Error::Invalid("review delta"));
        }
        let mut selected = BTreeSet::new();
        for edit in &self.edits {
            edit.validate()?;
            if !selected.insert((edit.catalog, edit.key)) {
                return Err(Error::Invalid("duplicate review catalog edit"));
            }
        }
        Ok(())
    }
}
