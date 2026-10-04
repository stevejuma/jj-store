use crate::*;
use jj_store_model::{PrincipalId, RepositoryId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use unicode_segmentation::UnicodeSegmentation as _;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReactionValueV1 {
    Like,
    Dislike,
    Emoji {
        sequence: String,
    },
    Custom {
        emoji_id: CustomEmojiId,
        version: CollabObjectRef,
    },
}
impl ReactionValueV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Emoji { sequence }
                if sequence.len() > 128
                    || sequence.graphemes(true).count() != 1
                    || emojis::get(sequence).is_none() =>
            {
                Err(Error::Invalid("Unicode emoji sequence"))
            }
            Self::Custom { version, .. } => expect(*version, CollabObjectKind::CustomEmojiVersion),
            _ => Ok(()),
        }
    }
    pub fn imported(value: &str) -> Result<Self> {
        let result = match value {
            "+" => Self::Like,
            "-" => Self::Dislike,
            other => Self::Emoji {
                sequence: other.into(),
            },
        };
        result.validate()?;
        Ok(result)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReactionOriginV1 {
    Human,
    Agent {
        identity: String,
        authorization: String,
    },
    Imported {
        source: String,
        external_id: String,
        original_value: String,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReactionV1 {
    pub schema_version: u32,
    pub id: ReactionId,
    pub repository: RepositoryId,
    pub actor: PrincipalId,
    pub target: ReactionTargetV1,
    pub value: ReactionValueV1,
    pub origin: ReactionOriginV1,
    pub accepted_at_millis: i64,
}
impl ReactionV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        self.target.validate()?;
        self.value.validate()?;
        if self.accepted_at_millis < 0 {
            return Err(Error::Invalid("reaction time"));
        }
        match &self.origin {
            ReactionOriginV1::Agent {
                identity,
                authorization,
            } if !label(identity, 1024) || !label(authorization, 4096) => {
                Err(Error::Invalid("agent reaction authorization"))
            }
            ReactionOriginV1::Imported {
                source,
                external_id,
                original_value,
            } if !label(source, 64) || !label(external_id, 4096) || !label(original_value, 128) => {
                Err(Error::Invalid("imported reaction provenance"))
            }
            _ => Ok(()),
        }
    }
    pub fn slot_key(&self) -> Result<CollabCatalogKey> {
        slot_key(self.repository, self.actor, &self.target)
    }
}
pub fn slot_key(
    repository: RepositoryId,
    actor: PrincipalId,
    target: &ReactionTargetV1,
) -> Result<CollabCatalogKey> {
    let bytes = [
        repository.as_bytes().as_slice(),
        actor.as_bytes().as_slice(),
        target.exact_key()?.as_bytes().as_slice(),
    ]
    .concat();
    CollabCatalogKey::derive("reaction-slot", &bytes)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RetractionReasonV1 {
    Clear,
    Replace,
    ResolveConflict,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReactionRetractionV1 {
    pub schema_version: u32,
    pub id: RetractionId,
    pub repository: RepositoryId,
    pub actor: PrincipalId,
    pub target: ReactionTargetV1,
    pub observed: BTreeSet<CollabObjectRef>,
    pub reason: RetractionReasonV1,
    pub accepted_at_millis: i64,
}
impl ReactionRetractionV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        self.target.validate()?;
        if self.observed.is_empty()
            || self.observed.len() > MAX_SLOT_RECORDS
            || self.accepted_at_millis < 0
        {
            return Err(Error::Invalid("reaction retraction"));
        }
        for reference in &self.observed {
            expect(*reference, CollabObjectKind::Reaction)?;
        }
        Ok(())
    }
}
pub const MAX_SLOT_RECORDS: usize = 4096;
/// An immutable snapshot of an observed-remove slot. History is retained as strong edges.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReactionSlotV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub actor: PrincipalId,
    pub target: ReactionTargetV1,
    pub additions: BTreeSet<CollabObjectRef>,
    pub retractions: BTreeSet<CollabObjectRef>,
}
impl ReactionSlotV1 {
    pub fn empty(repository: RepositoryId, actor: PrincipalId, target: ReactionTargetV1) -> Self {
        Self {
            schema_version: 1,
            repository,
            actor,
            target,
            additions: BTreeSet::new(),
            retractions: BTreeSet::new(),
        }
    }
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        self.target.validate()?;
        if self.additions.len() > MAX_SLOT_RECORDS || self.retractions.len() > MAX_SLOT_RECORDS {
            return Err(Error::Limit);
        }
        for reference in &self.additions {
            expect(*reference, CollabObjectKind::Reaction)?;
        }
        for reference in &self.retractions {
            expect(*reference, CollabObjectKind::ReactionRetraction)?;
        }
        Ok(())
    }
    pub fn key(&self) -> Result<CollabCatalogKey> {
        slot_key(self.repository, self.actor, &self.target)
    }
    /// This is data-only convergence. No timestamp or input ordering selects a value.
    pub fn merge(&self, other: &Self) -> Result<Self> {
        if self.key()? != other.key()? {
            return Err(Error::Invalid("reaction slot merge"));
        }
        let mut result = self.clone();
        result.additions.extend(&other.additions);
        result.retractions.extend(&other.retractions);
        result.validate()?;
        Ok(result)
    }
    pub fn resolve(
        &self,
        objects: &BTreeMap<CollabObjectRef, CollabObjectV1>,
    ) -> Result<ReactionSelectionV1> {
        self.validate()?;
        let mut active = self.additions.clone();
        let mut reaction_ids = BTreeMap::new();
        let mut retraction_ids = BTreeMap::new();
        for reference in &self.retractions {
            let Some(CollabObjectV1::ReactionRetraction(retraction)) = objects.get(reference)
            else {
                return Err(Error::Invalid("missing reaction retraction"));
            };
            retraction.validate()?;
            if retraction_ids.insert(retraction.id, *reference).is_some() {
                return Err(Error::Invalid("retraction identity equivocation"));
            }
            if slot_key(retraction.repository, retraction.actor, &retraction.target)?
                != self.key()?
                || !retraction.observed.is_subset(&self.additions)
            {
                return Err(Error::Invalid("retraction slot membership"));
            }
            for removed in &retraction.observed {
                active.remove(removed);
            }
        }
        let mut values = BTreeSet::new();
        for reference in &self.additions {
            let Some(CollabObjectV1::Reaction(reaction)) = objects.get(reference) else {
                return Err(Error::Invalid("missing reaction"));
            };
            reaction.validate()?;
            if reaction_ids.insert(reaction.id, *reference).is_some() {
                return Err(Error::Invalid("reaction identity equivocation"));
            }
            if reaction.slot_key()? != self.key()? {
                return Err(Error::Invalid("reaction slot membership"));
            }
            if active.contains(reference) {
                values.insert(reaction.value.clone());
            }
        }
        Ok(ReactionSelectionV1 {
            active,
            value: if values.len() == 1 {
                values.iter().next().cloned()
            } else {
                None
            },
            alternatives: values,
        })
    }
    /// Stage an atomic observed replacement; the provider publishes the new slot and records together.
    pub fn apply(
        &self,
        observed: &BTreeSet<CollabObjectRef>,
        value: Option<ReactionValueV1>,
        resolve_conflict: bool,
        ids: (ReactionId, RetractionId),
        at: i64,
        objects: &BTreeMap<CollabObjectRef, CollabObjectV1>,
    ) -> Result<ReactionTransitionV1> {
        let (reaction_id, retraction_id) = ids;
        let selected = self.resolve(objects)?;
        if selected.active != *observed {
            return Err(Error::Stale);
        }
        if selected.is_conflict() != resolve_conflict {
            return Err(Error::Invalid(
                "reaction conflict requires explicit resolution",
            ));
        }
        if let Some(value) = &value {
            value.validate()?;
        }
        let mut next = self.clone();
        let mut staged = BTreeMap::new();
        if selected.value == value && !selected.is_conflict() {
            return Ok(ReactionTransitionV1 { slot: next, staged });
        }
        if !observed.is_empty() {
            let object = CollabObjectV1::ReactionRetraction(ReactionRetractionV1 {
                schema_version: 1,
                id: retraction_id,
                repository: self.repository,
                actor: self.actor,
                target: self.target.clone(),
                observed: observed.clone(),
                reason: if resolve_conflict {
                    RetractionReasonV1::ResolveConflict
                } else if value.is_some() {
                    RetractionReasonV1::Replace
                } else {
                    RetractionReasonV1::Clear
                },
                accepted_at_millis: at,
            });
            let reference = object.reference()?;
            next.retractions.insert(reference);
            staged.insert(reference, object);
        }
        if let Some(value) = value {
            let object = CollabObjectV1::Reaction(ReactionV1 {
                schema_version: 1,
                id: reaction_id,
                repository: self.repository,
                actor: self.actor,
                target: self.target.clone(),
                value,
                origin: ReactionOriginV1::Human,
                accepted_at_millis: at,
            });
            let reference = object.reference()?;
            next.additions.insert(reference);
            staged.insert(reference, object);
        }
        next.validate()?;
        Ok(ReactionTransitionV1 { slot: next, staged })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReactionSelectionV1 {
    pub active: BTreeSet<CollabObjectRef>,
    pub value: Option<ReactionValueV1>,
    pub alternatives: BTreeSet<ReactionValueV1>,
}
impl ReactionSelectionV1 {
    pub fn is_conflict(&self) -> bool {
        self.alternatives.len() > 1
    }
}
pub struct ReactionTransitionV1 {
    pub slot: ReactionSlotV1,
    pub staged: BTreeMap<CollabObjectRef, CollabObjectV1>,
}
