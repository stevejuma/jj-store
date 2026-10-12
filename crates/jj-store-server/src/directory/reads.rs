use super::*;
#[async_trait::async_trait]
impl Directory for Manager {
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        Some(self.signals.subscribe())
    }
    async fn principal(&self, caller: &Caller) -> Result<PrincipalId> {
        self.personal(&self.selection().await?.root, caller)
    }
    async fn profile(&self, caller: &Caller) -> Result<UserProfile> {
        let root = self.selection().await?.root;
        let actor = self.personal(&root, caller)?;
        let revision = root.profiles.get(&actor).cloned();
        let details = if let Some(reference) = &revision {
            let record: SavedProfile = self.record(reference, "profile").await?;
            if record.principal != actor {
                return Err(Error::Corrupt);
            }
            record.details
        } else {
            ProfileDetails::default()
        };
        self.principal(caller).await?;
        Ok(UserProfile {
            principal: actor,
            revision,
            details,
        })
    }
    async fn profiles(&self, caller: &Caller, ids: Vec<PrincipalId>) -> Result<Vec<PublicProfile>> {
        let root = self.selection().await?.root;
        self.personal(&root, caller)?;
        if ids.len() > 100 {
            return Err(Error::Limit);
        }
        let mut profiles = Vec::new();
        for principal in ids {
            let Some(reference) = root.profiles.get(&principal) else {
                continue;
            };
            let p: SavedProfile = self.record(reference, "profile").await?;
            if p.principal != principal {
                return Err(Error::Corrupt);
            }
            let avatar = p.details.avatar.or_else(|| {
                p.details.email.map(|email| {
                    format!(
                        "https://www.gravatar.com/avatar/{}?d=404&s=128",
                        Self::digest(email.trim().to_lowercase().as_bytes())
                    )
                })
            });
            profiles.push(PublicProfile {
                principal,
                display_name: p.details.display_name,
                description: p.details.description,
                avatar,
            });
        }
        self.principal(caller).await?;
        Ok(profiles)
    }
    async fn emojis(&self, caller: &Caller) -> Result<Vec<LibraryEmoji>> {
        let root = self.selection().await?.root;
        let actor = self.personal(&root, caller)?;
        let mut emojis = Vec::new();
        for (id, reference) in &root.emojis {
            let emoji: SavedEmoji = self.record(reference, "emoji").await?;
            if emoji.id != *id {
                return Err(Error::Corrupt);
            }
            if !emoji.retired || emoji.owner == actor {
                emojis.push(emoji.view(reference.clone()));
            }
        }
        self.principal(caller).await?;
        emojis.sort_by(|a, b| a.shortcode.cmp(&b.shortcode));
        Ok(emojis)
    }
    async fn emoji_snapshot(
        &self,
        caller: &Caller,
        id: RequestId,
        revision: &str,
    ) -> Result<EmojiSelection> {
        let root = self.selection().await?.root;
        self.personal(&root, caller)?;
        if root.emojis.get(&id).map(String::as_str) != Some(revision) {
            return Err(Error::Stale);
        }
        let emoji: SavedEmoji = self.record(revision, "emoji").await?;
        if emoji.id != id || emoji.retired {
            return Err(Error::Stale);
        }
        let snapshot = jj_collab::EmojiSnapshotV1 {
            source: format!("installation:{}:{id}:{revision}", self.installation),
            shortcode: emoji.shortcode,
            bytes: self.media(caller, &emoji.asset).await?.bytes,
        };
        let value = snapshot
            .stage(&mut jj_collab::CollabGraphV1::default())
            .map_err(|_| Error::Unavailable)?;
        self.principal(caller).await?;
        Ok(EmojiSelection { snapshot, value })
    }
    async fn media(&self, caller: &Caller, digest: &str) -> Result<Media> {
        let root = self.selection().await?.root;
        let actor = self.personal(&root, caller)?;
        let record = root.media.get(digest).ok_or(Error::Denied)?;
        if !record.published && !record.owners.contains(&actor) {
            return Err(Error::Denied);
        }
        let bytes = self
            .store
            .get(&self.key(&format!("media/{digest}"))?, None)
            .await
            .map_err(persistence::stored_object_error)?
            .read_bounded(jj_collab::MAX_IMAGE_BYTES as u64)
            .await
            .map_err(|_| Error::Unavailable)?;
        if Self::digest(&bytes) != digest {
            return Err(Error::Corrupt);
        }
        self.principal(caller).await?;
        let content_type = match record.media_type {
            jj_collab::ImageMediaTypeV1::Png => "image/png",
            jj_collab::ImageMediaTypeV1::Jpeg => "image/jpeg",
            jj_collab::ImageMediaTypeV1::Gif => "image/gif",
            jj_collab::ImageMediaTypeV1::Webp => "image/webp",
        };
        Ok(Media {
            bytes,
            content_type,
        })
    }
    async fn execute(
        &self,
        caller: &Caller,
        request: DirectoryRequest,
    ) -> Result<DirectoryReceipt> {
        self.change(caller, request).await
    }
    async fn receipt(
        &self,
        caller: &Caller,
        request: RequestId,
    ) -> Result<Option<DirectoryReceipt>> {
        let root = self.selection().await?.root;
        let actor = self.actor(&root, caller)?;
        self.saved_receipt(&root, actor, request, None).await
    }
    async fn identities(&self, caller: &Caller) -> Result<Vec<IdentityBinding>> {
        if !matches!(caller, Caller::Operator(_)) {
            return Err(Error::Denied);
        }
        Ok(self
            .selection()
            .await?
            .root
            .bindings
            .into_iter()
            .map(|(token, b)| IdentityBinding {
                token,
                principal: b.principal,
                expires_at: b.expires_at,
                revoked: b.revoked,
            })
            .collect())
    }
    async fn enroll(
        &self,
        caller: &Caller,
        request: RequestId,
        principal: PrincipalId,
        token: &jj_store_service::auth::BearerToken,
        expires_at: Option<u64>,
    ) -> Result<DirectoryReceipt> {
        self.binding(
            caller,
            request,
            token.id(),
            Some(Binding {
                revoked: false,
                principal,
                verifier: hex::encode(token.installation_verifier(&self.installation)),
                expires_at,
            }),
        )
        .await
    }
    async fn revoke(
        &self,
        caller: &Caller,
        request: RequestId,
        token: TokenId,
    ) -> Result<DirectoryReceipt> {
        self.binding(caller, request, token, None).await
    }
}
