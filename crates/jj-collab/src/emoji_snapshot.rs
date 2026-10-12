//! Portable library selections are data, never grants or identity assertions.
use crate::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EmojiSnapshotV1 {
    pub source: String,
    pub shortcode: String,
    pub bytes: Vec<u8>,
}
impl EmojiSnapshotV1 {
    pub fn validate(&self) -> Result<()> {
        let fields: Vec<_> = self.source.split(':').collect();
        if fields.len() != 4
            || fields[0] != "installation"
            || fields[1].len() != 32
            || fields[2].len() != 32
            || fields[3].len() != 64
            || !fields[1..].iter().all(|s| {
                s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            })
            || self.shortcode.is_empty()
            || self.shortcode.len() > 64
            || !self
                .shortcode
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-".contains(&b))
            || self.bytes.is_empty()
            || self.bytes.len() > MAX_IMAGE_BYTES
        {
            return Err(Error::Invalid("emoji snapshot"));
        }
        Ok(())
    }
    pub fn stage(&self, graph: &mut CollabGraphV1) -> Result<ReactionValueV1> {
        self.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/library-emoji/v1\0");
        hash.update(serde_json::to_vec(self)?);
        let mut id = [0; 16];
        id.copy_from_slice(&hash.finalize()[..16]);
        let emoji_id = CustomEmojiId::new(id);
        let (asset, preview) = ImageAssetV1::ingest(&self.bytes)?;
        let asset = graph.stage(CollabObjectV1::ImageAsset(asset))?;
        let preview = graph.stage(CollabObjectV1::ImageAsset(preview))?;
        let version = graph.stage(CollabObjectV1::CustomEmojiVersion(CustomEmojiVersionV1 {
            schema_version: 1,
            emoji_id,
            shortcode: format!("library_{emoji_id}"),
            label: self.shortcode.clone(),
            source: Some(self.source.clone()),
            actor: jj_store_model::PrincipalId::new([0; 16]),
            accepted_at_millis: 0,
            previous: None,
            asset,
            preview,
            retired: false,
        }))?;
        Ok(ReactionValueV1::Custom { emoji_id, version })
    }
}
pub(crate) fn validate_snapshots(values: &[EmojiSnapshotV1]) -> Result<()> {
    if values.len() > 32 || values.iter().map(|s| s.bytes.len()).sum::<usize>() > MAX_IMAGE_BYTES {
        return Err(Error::Limit);
    }
    let mut names = std::collections::BTreeSet::new();
    for value in values {
        value.validate()?;
        if !names.insert(&value.shortcode) {
            return Err(Error::Invalid("duplicate message emoji"));
        }
    }
    Ok(())
}

pub fn message_emoji_definitions(
    message: Option<&HumanMessageRevisionV1>,
    graph: &CollabGraphV1,
) -> Result<std::collections::BTreeMap<String, CustomEmojiVersionV1>> {
    let mut result = std::collections::BTreeMap::new();
    if let Some(message) = message {
        for (name, reference) in &message.emojis {
            let CollabObjectV1::CustomEmojiVersion(value) = graph.get(*reference)? else {
                return Err(Error::Invalid("message emoji definition"));
            };
            result.insert(name.clone(), value.clone());
        }
    }
    Ok(result)
}
