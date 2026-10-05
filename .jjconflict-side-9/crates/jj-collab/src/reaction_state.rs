//! Pure reaction/library projection; providers validate targets and publish receipts.
use crate::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct ReactionStateV1 {
    pub root: ReactionRootV1,
    pub slots: BTreeMap<CollabCatalogKey, CollabObjectRef>,
    pub emojis: BTreeMap<CustomEmojiId, CollabObjectRef>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReactionCommandResultV1 {
    pub schema_version: u32,
    pub slot: CollabObjectRef,
    pub active: BTreeSet<CollabObjectRef>,
    pub value: Option<ReactionValueV1>,
    pub conflict: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CustomEmojiCommandV1 {
    Define {
        emoji_id: CustomEmojiId,
        expected: Option<CollabObjectRef>,
        shortcode: String,
        label: String,
        bytes: Vec<u8>,
    },
    Retire {
        emoji_id: CustomEmojiId,
        expected: CollabObjectRef,
    },
}
impl CustomEmojiCommandV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Define {
                expected,
                shortcode,
                label: name,
                bytes,
                ..
            } => {
                if bytes.is_empty() || bytes.len() > MAX_IMAGE_BYTES {
                    return Err(Error::Limit);
                }
                if !label(name, 256)
                    || shortcode.is_empty()
                    || shortcode.len() > 64
                    || !shortcode
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-".contains(&b))
                {
                    return Err(Error::Invalid("custom emoji command"));
                }
                if let Some(reference) = expected {
                    expect(*reference, CollabObjectKind::CustomEmojiVersion)?;
                }
            }
            Self::Retire { expected, .. } => {
                expect(*expected, CollabObjectKind::CustomEmojiVersion)?
            }
        }
        Ok(())
    }
}
impl ReactionCommandResultV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        expect(self.slot, CollabObjectKind::ReactionSlot)?;
        if self.active.len() > MAX_SLOT_RECORDS
            || (self.conflict && (self.value.is_some() || self.active.len() < 2))
            || (!self.conflict && (self.active.is_empty() != self.value.is_none()))
        {
            return Err(Error::Invalid("reaction result"));
        }
        for reference in &self.active {
            expect(*reference, CollabObjectKind::Reaction)?;
        }
        if let Some(value) = &self.value {
            value.validate()?;
        }
        Ok(())
    }
}
impl ReactionStateV1 {
    pub fn empty() -> Self {
        Self {
            root: ReactionRootV1::empty(),
            slots: BTreeMap::new(),
            emojis: BTreeMap::new(),
        }
    }
    pub fn read(
        repository: RepositoryId,
        root: ReactionRootV1,
        graph: &CollabGraphV1,
    ) -> Result<Self> {
        root.validate()?;
        graph.closure(root.dependencies())?;
        let slots = graph.catalog(root.slots)?;
        for (key, reference) in &slots {
            let CollabObjectV1::ReactionSlot(slot) = graph.get(*reference)? else {
                return Err(Error::Invalid("reaction slot catalog kind"));
            };
            if slot.repository != repository || slot.key()? != *key {
                return Err(Error::Invalid("reaction slot repository or key"));
            }
        }
        let mut emojis = BTreeMap::new();
        let mut names = BTreeSet::new();
        for (key, reference) in graph.catalog(root.emojis)? {
            let CollabObjectV1::CustomEmojiVersion(emoji) = graph.get(reference)? else {
                return Err(Error::Invalid("emoji catalog kind"));
            };
            if entity_key("emoji", emoji.emoji_id.as_bytes())? != key
                || (!emoji.retired && !names.insert(emoji.shortcode.clone()))
            {
                return Err(Error::Invalid("emoji catalog key or duplicate shortcode"));
            }
            emojis.insert(emoji.emoji_id, reference);
        }
        Ok(Self {
            root,
            slots,
            emojis,
        })
    }
    pub fn apply(
        &self,
        command: &ReactionCommandV1,
        repository: RepositoryId,
        actor: PrincipalId,
        request: RequestId,
        at: i64,
        graph: &mut CollabGraphV1,
    ) -> Result<(ReactionRootV1, ReactionCommandResultV1)> {
        command.validate()?;
        let value = match command {
            ReactionCommandV1::Set { value, .. } => Some(value),
            ReactionCommandV1::Clear { .. } => None,
            ReactionCommandV1::ResolveConflict { value, .. } => value.as_ref(),
        };
        if let Some(ReactionValueV1::Custom { emoji_id, version }) = value {
            if self.emojis.get(emoji_id) != Some(version) {
                return Err(Error::Stale);
            }
            let CollabObjectV1::CustomEmojiVersion(emoji) = graph.get(*version)? else {
                return Err(Error::Invalid("custom reaction definition"));
            };
            if emoji.retired {
                return Err(Error::Invalid("retired custom reaction"));
            }
        }
        let empty = ReactionSlotV1::empty(repository, actor, command.target().clone());
        let key = empty.key()?;
        let slot = match self.slots.get(&key) {
            Some(reference) => {
                let CollabObjectV1::ReactionSlot(slot) = graph.get(*reference)? else {
                    return Err(Error::Invalid("reaction slot kind"));
                };
                slot.clone()
            }
            None => empty,
        };
        if slot.repository != repository || slot.actor != actor {
            return Err(Error::Invalid("reaction slot actor"));
        }
        let ids = identities(actor, request, key);
        let applied = command.apply(
            &slot,
            ReactionId::new(ids),
            RetractionId::new(ids),
            at,
            graph,
        )?;
        for (expected, object) in applied.staged {
            if graph.stage(object)? != expected {
                return Err(Error::Invalid("staged reaction identity"));
            }
        }
        let state = applied.slot.resolve(&graph.objects)?;
        let reference = graph.stage(CollabObjectV1::ReactionSlot(applied.slot))?;
        let mut root = self.root.clone();
        root.slots = graph.catalog_update(root.slots, BTreeMap::from([(key, reference)]))?;
        let result = ReactionCommandResultV1 {
            schema_version: 1,
            slot: reference,
            conflict: state.is_conflict(),
            active: state.active,
            value: state.value,
        };
        result.validate()?;
        Ok((root, result))
    }
    pub fn apply_emoji(
        &self,
        command: &CustomEmojiCommandV1,
        actor: PrincipalId,
        at: i64,
        graph: &mut CollabGraphV1,
    ) -> Result<(ReactionRootV1, CollabObjectRef)> {
        command.validate()?;
        let (id, expected) = match command {
            CustomEmojiCommandV1::Define {
                emoji_id, expected, ..
            } => (*emoji_id, *expected),
            CustomEmojiCommandV1::Retire { emoji_id, expected } => (*emoji_id, Some(*expected)),
        };
        if self.emojis.get(&id).copied() != expected {
            return Err(Error::Stale);
        }
        let value = match command {
            CustomEmojiCommandV1::Define {
                shortcode,
                label,
                bytes,
                ..
            } => {
                for (other, reference) in &self.emojis {
                    let CollabObjectV1::CustomEmojiVersion(emoji) = graph.get(*reference)? else {
                        return Err(Error::Invalid("emoji definition"));
                    };
                    if *other != id && !emoji.retired && emoji.shortcode == *shortcode {
                        return Err(Error::Invalid("duplicate emoji shortcode"));
                    }
                }
                let (asset, preview) = ImageAssetV1::ingest(bytes)?;
                let asset = graph.stage(CollabObjectV1::ImageAsset(asset))?;
                let preview = graph.stage(CollabObjectV1::ImageAsset(preview))?;
                CustomEmojiVersionV1 {
                    schema_version: 1,
                    emoji_id: id,
                    shortcode: shortcode.clone(),
                    label: label.clone(),
                    actor,
                    accepted_at_millis: at,
                    previous: expected,
                    asset,
                    preview,
                    retired: false,
                }
            }
            CustomEmojiCommandV1::Retire { expected, .. } => {
                let CollabObjectV1::CustomEmojiVersion(emoji) = graph.get(*expected)? else {
                    return Err(Error::Invalid("emoji definition"));
                };
                let mut value = emoji.clone();
                value.previous = Some(*expected);
                value.actor = actor;
                value.accepted_at_millis = at;
                value.retired = true;
                value
            }
        };
        let reference = graph.stage(CollabObjectV1::CustomEmojiVersion(value))?;
        let mut root = self.root.clone();
        root.emojis = graph.catalog_update(
            root.emojis,
            BTreeMap::from([(entity_key("emoji", id.as_bytes())?, reference)]),
        )?;
        Ok((root, reference))
    }
}
fn identities(actor: PrincipalId, request: RequestId, key: CollabCatalogKey) -> [u8; 16] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/reaction-command/v1\0");
    hash.update(actor.as_bytes());
    hash.update(request.as_bytes());
    hash.update(key.as_bytes());
    let digest: [u8; 32] = hash.finalize().into();
    let mut id = [0; 16];
    id.copy_from_slice(&digest[..16]);
    id
}
