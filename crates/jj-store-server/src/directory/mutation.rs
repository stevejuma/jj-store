use super::*;
impl Manager {
    pub(super) async fn apply(
        &self,
        root: &mut Root,
        actor: PrincipalId,
        command: DirectoryCommand,
    ) -> Result<DirectoryResult> {
        match command {
            DirectoryCommand::SaveProfile {
                expected,
                mut details,
            } => {
                if root.profiles.get(&actor) != expected.as_ref() {
                    return Err(Error::Stale);
                }
                validation::profile(&mut details)?;
                if let Some(digest) = details
                    .avatar
                    .as_deref()
                    .and_then(|s| s.strip_prefix("blob:"))
                {
                    self.publish_media(root, actor, digest)?;
                }
                if !root.profiles.contains_key(&actor) && root.profiles.len() >= MAX_ENTRIES {
                    return Err(Error::Limit);
                }
                let revision = self
                    .stage(
                        "profile",
                        &SavedProfile {
                            principal: actor,
                            details: details.clone(),
                        },
                    )
                    .await?;
                root.profiles.insert(actor, revision.clone());
                Ok(DirectoryResult::Profile {
                    profile: UserProfile {
                        principal: actor,
                        revision: Some(revision),
                        details,
                    },
                })
            }
            DirectoryCommand::Upload { bytes } => {
                let (asset, preview) =
                    tokio::task::spawn_blocking(move || jj_collab::ImageAssetV1::ingest(&bytes))
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .map_err(|_| Error::Invalid)?;
                let digest = self.stage_media(&asset.bytes).await?;
                let preview_digest = self.stage_media(&preview.bytes).await?;
                for (id, media_type) in [
                    (&digest, asset.media_type),
                    (&preview_digest, preview.media_type),
                ] {
                    if !root.media.contains_key(id) && root.media.len() >= MAX_ENTRIES {
                        return Err(Error::Limit);
                    }
                    let record = root.media.entry(id.clone()).or_insert_with(|| MediaRecord {
                        media_type,
                        owners: BTreeSet::new(),
                        preview: preview_digest.clone(),
                        published: false,
                    });
                    if record.owners.len() >= MAX_ENTRIES {
                        return Err(Error::Limit);
                    }
                    record.owners.insert(actor);
                }
                Ok(DirectoryResult::Media {
                    asset: digest,
                    preview: preview_digest,
                })
            }
            DirectoryCommand::SaveEmoji {
                id,
                expected,
                shortcode,
                asset,
            } => {
                if root.emojis.get(&id) != expected.as_ref() {
                    return Err(Error::Stale);
                }
                if let Some(reference) = &expected {
                    let old: SavedEmoji = self.record(reference, "emoji").await?;
                    if old.owner != actor || old.retired {
                        return Err(Error::Denied);
                    }
                } else if root.emojis.len() >= MAX_ENTRIES {
                    return Err(Error::Limit);
                }
                let shortcode = validation::shortcode(&shortcode)?;
                for (other, reference) in &root.emojis {
                    let emoji: SavedEmoji = self.record(reference, "emoji").await?;
                    if *other != id && !emoji.retired && emoji.shortcode == shortcode {
                        return Err(Error::Stale);
                    }
                }
                self.publish_media(root, actor, &asset)?;
                let preview = root
                    .media
                    .get(&asset)
                    .ok_or(Error::Invalid)?
                    .preview
                    .clone();
                self.publish_media(root, actor, &preview)?;
                let emoji = SavedEmoji {
                    id,
                    owner: actor,
                    shortcode,
                    asset,
                    preview,
                    retired: false,
                };
                let revision = self.stage("emoji", &emoji).await?;
                root.emojis.insert(id, revision.clone());
                Ok(DirectoryResult::Emoji {
                    emoji: emoji.view(revision),
                })
            }
            DirectoryCommand::RetireEmoji { id, expected } => {
                if root.emojis.get(&id) != Some(&expected) {
                    return Err(Error::Stale);
                }
                let mut emoji: SavedEmoji = self.record(&expected, "emoji").await?;
                if emoji.owner != actor {
                    return Err(Error::Denied);
                }
                emoji.retired = true;
                let revision = self.stage("emoji", &emoji).await?;
                root.emojis.insert(id, revision.clone());
                Ok(DirectoryResult::Emoji {
                    emoji: emoji.view(revision),
                })
            }
        }
    }
    fn publish_media(&self, root: &mut Root, actor: PrincipalId, digest: &str) -> Result<()> {
        let media = root.media.get_mut(digest).ok_or(Error::Invalid)?;
        if !media.published && !media.owners.contains(&actor) {
            return Err(Error::Denied);
        }
        media.published = true;
        Ok(())
    }
    pub(super) async fn change(
        &self,
        caller: &Caller,
        request: DirectoryRequest,
    ) -> Result<DirectoryReceipt> {
        let _gate = self.gate.lock().await;
        let mut selection = self.selection().await?;
        let actor = self.personal(&selection.root, caller)?;
        let intent = Self::digest(&serde_json::to_vec(&request).map_err(|_| Error::Invalid)?);
        if let Some(receipt) = self
            .saved_receipt(&selection.root, actor, request.request_id, Some(&intent))
            .await?
        {
            return Ok(receipt);
        }
        let result = self
            .apply(&mut selection.root, actor, request.command)
            .await?;
        self.commit(selection, caller, request.request_id, intent, result)
            .await
    }
    pub(super) async fn binding(
        &self,
        caller: &Caller,
        request: RequestId,
        token: TokenId,
        binding: Option<Binding>,
    ) -> Result<DirectoryReceipt> {
        if !matches!(caller, Caller::Operator(_)) {
            return Err(Error::Denied);
        }
        let _gate = self.gate.lock().await;
        let mut selection = self.selection().await?;
        let actor = self.actor(&selection.root, caller)?;
        let intent =
            Self::digest(&serde_json::to_vec(&(token, &binding)).map_err(|_| Error::Invalid)?);
        if let Some(receipt) = self
            .saved_receipt(&selection.root, actor, request, Some(&intent))
            .await?
        {
            return Ok(receipt);
        }
        let revoked = binding.is_none();
        let principal = if let Some(binding) = binding {
            if selection.root.bindings.get(&token).is_some_and(|old| {
                old.principal != binding.principal || old.verifier != binding.verifier
            }) {
                return Err(Error::Denied);
            }
            if !selection.root.bindings.contains_key(&token)
                && selection.root.bindings.len() >= MAX_ENTRIES
            {
                return Err(Error::Limit);
            }
            let principal = binding.principal;
            selection.root.bindings.insert(token, binding);
            principal
        } else {
            selection
                .root
                .bindings
                .get_mut(&token)
                .map(|binding| {
                    binding.revoked = true;
                    binding.principal
                })
                .ok_or(Error::Invalid)?
        };
        self.commit(
            selection,
            caller,
            request,
            intent,
            DirectoryResult::Identity {
                token,
                principal,
                revoked,
            },
        )
        .await
    }
}
