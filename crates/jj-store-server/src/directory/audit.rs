use super::*;
impl Manager {
    /// Startup verifies the selected typed closure and media existence. Media
    /// digests are rechecked over the bounded bytes on every authorized read.
    pub async fn audit(&self) -> Result<()> {
        let root = self.selection().await?.root;
        for (id, reference) in &root.profiles {
            let profile: SavedProfile = self.record(reference, "profile").await?;
            let mut normalized = profile.details.clone();
            validation::profile(&mut normalized)?;
            if profile.principal != *id || profile.details != normalized {
                return Err(Error::Corrupt);
            }
            if let Some(digest) = profile
                .details
                .avatar
                .as_deref()
                .and_then(|s| s.strip_prefix("blob:"))
            {
                if !root.media.get(digest).is_some_and(|m| m.published) {
                    return Err(Error::Corrupt);
                }
            }
        }
        let mut names = BTreeSet::new();
        for (id, reference) in &root.emojis {
            let emoji: SavedEmoji = self.record(reference, "emoji").await?;
            if emoji.id != *id
                || validation::shortcode(&emoji.shortcode)? != emoji.shortcode
                || (!emoji.retired && !names.insert(emoji.shortcode))
            {
                return Err(Error::Corrupt);
            }
            for digest in [&emoji.asset, &emoji.preview] {
                if !root.media.get(digest).is_some_and(|m| m.published) {
                    return Err(Error::Corrupt);
                }
            }
        }
        for (digest, media) in &root.media {
            if digest.len() != 64
                || !validation::hexadecimal(digest)
                || media.owners.is_empty()
                || media.owners.len() > MAX_ENTRIES
                || !root.media.contains_key(&media.preview)
            {
                return Err(Error::Corrupt);
            }
            let head = self
                .store
                .head(&self.key(&format!("media/{digest}"))?)
                .await
                .map_err(persistence::stored_object_error)?;
            if head.len > jj_collab::MAX_IMAGE_BYTES as u64 {
                return Err(Error::Corrupt);
            }
        }
        for binding in root.bindings.values() {
            if binding.verifier.len() != 64 || !validation::hexadecimal(&binding.verifier) {
                return Err(Error::Corrupt);
            }
        }
        for (identity, reference) in &root.receipts {
            let (_, request) = identity.split_once(':').ok_or(Error::Corrupt)?;
            let saved: SavedReceipt = self.record(reference, "receipt").await?;
            if saved.receipt.request_id.to_string() != request
                || saved.intent.len() != 64
                || !validation::hexadecimal(&saved.intent)
            {
                return Err(Error::Corrupt);
            }
        }
        Ok(())
    }
}
