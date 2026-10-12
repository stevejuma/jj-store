use super::*;
impl Manager {
    pub(super) async fn selection(&self) -> Result<Selection> {
        let key = self.key("manifest.json")?;
        let download = match self.store.get(&key, None).await {
            Ok(v) => v,
            Err(jj_store_object::Error::NotFound) => {
                return Ok(Selection {
                    root: Root::default(),
                    generation: 0,
                    version: None,
                });
            }
            Err(_) => return Err(Error::Unavailable),
        };
        let version = download.metadata.version.clone();
        let bytes = download
            .read_bounded(4096)
            .await
            .map_err(|_| Error::Unavailable)?;
        let manifest: Manifest = serde_json::from_slice(&bytes).map_err(|_| Error::Corrupt)?;
        if manifest.format != 1
            || manifest.installation != self.installation
            || serde_json::to_vec(&manifest).map_err(|_| Error::Corrupt)? != bytes
        {
            return Err(Error::Corrupt);
        }
        let root: Root = self.record(&manifest.root, "root").await?;
        if root.profiles.len() > MAX_ENTRIES
            || root.bindings.len() > MAX_ENTRIES
            || root.emojis.len() > MAX_ENTRIES
            || root.media.len() > MAX_ENTRIES
            || root.receipts.len() > MAX_RECEIPTS
        {
            return Err(Error::Limit);
        }
        Ok(Selection {
            root,
            generation: manifest.generation,
            version: Some(version),
        })
    }
    pub(super) async fn record<T: Serialize + DeserializeOwned>(
        &self,
        digest: &str,
        kind: &str,
    ) -> Result<T> {
        if digest.len() != 64 || !validation::hexadecimal(digest) {
            return Err(Error::Corrupt);
        }
        let bytes = self
            .store
            .get(&self.key(&format!("objects/{digest}.json"))?, None)
            .await
            .map_err(stored_object_error)?
            .read_bounded(MAX_RECORD)
            .await
            .map_err(|_| Error::Unavailable)?;
        if Self::digest(&bytes) != digest {
            return Err(Error::Corrupt);
        }
        let record: Envelope<T> = serde_json::from_slice(&bytes).map_err(|_| Error::Corrupt)?;
        if record.format != 1
            || record.installation != self.installation
            || record.kind != kind
            || serde_json::to_vec(&record).map_err(|_| Error::Corrupt)? != bytes
        {
            return Err(Error::Corrupt);
        }
        Ok(record.data)
    }
    async fn put_immutable(&self, key: Key, bytes: &[u8]) -> Result<()> {
        let upload = StagedUpload::from_bytes(bytes, MAX_RECORD)
            .await
            .map_err(|_| Error::Limit)?;
        match self.store.put(&key, Condition::Absent, &upload).await {
            Ok(WriteOutcome::Applied(_)) => Ok(()),
            _ => {
                let existing = self
                    .store
                    .get(&key, None)
                    .await
                    .map_err(|_| Error::Unknown)?
                    .read_bounded(MAX_RECORD)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                if existing != bytes {
                    Err(Error::Corrupt)
                } else {
                    Ok(())
                }
            }
        }
    }
    pub(super) async fn stage<T: Serialize>(&self, kind: &str, data: &T) -> Result<String> {
        let bytes = serde_json::to_vec(&Envelope {
            format: 1,
            installation: self.installation.clone(),
            kind: kind.into(),
            data,
        })
        .map_err(|_| Error::Invalid)?;
        if bytes.len() as u64 > MAX_RECORD {
            return Err(Error::Limit);
        }
        let digest = Self::digest(&bytes);
        self.put_immutable(self.key(&format!("objects/{digest}.json"))?, &bytes)
            .await?;
        Ok(digest)
    }
    pub(super) async fn stage_media(&self, bytes: &[u8]) -> Result<String> {
        let digest = Self::digest(bytes);
        self.put_immutable(self.key(&format!("media/{digest}"))?, bytes)
            .await?;
        Ok(digest)
    }
    pub(super) async fn saved_receipt(
        &self,
        root: &Root,
        actor: PrincipalId,
        request: RequestId,
        intent: Option<&str>,
    ) -> Result<Option<DirectoryReceipt>> {
        let Some(reference) = root.receipts.get(&format!("{actor}:{request}")) else {
            return Ok(None);
        };
        let saved: SavedReceipt = self.record(reference, "receipt").await?;
        if saved.receipt.request_id != request {
            return Err(Error::Corrupt);
        }
        if intent.is_some_and(|i| i != saved.intent) {
            return Err(Error::Invalid);
        }
        Ok(Some(saved.receipt))
    }
    pub(super) async fn commit(
        &self,
        mut selection: Selection,
        caller: &Caller,
        request: RequestId,
        intent: String,
        result: DirectoryResult,
    ) -> Result<DirectoryReceipt> {
        let actor = self.actor(&selection.root, caller)?;
        if selection.root.receipts.len() >= MAX_RECEIPTS {
            return Err(Error::Limit);
        }
        let receipt = DirectoryReceipt {
            request_id: request,
            result,
        };
        let reference = self
            .stage(
                "receipt",
                &SavedReceipt {
                    intent: intent.clone(),
                    receipt: receipt.clone(),
                },
            )
            .await?;
        selection
            .root
            .receipts
            .insert(format!("{actor}:{request}"), reference);
        let root = self.stage("root", &selection.root).await?;
        let manifest = Manifest {
            format: 1,
            installation: self.installation.clone(),
            generation: selection.generation.checked_add(1).ok_or(Error::Limit)?,
            root,
        };
        let bytes = serde_json::to_vec(&manifest).map_err(|_| Error::Invalid)?;
        let upload = StagedUpload::from_bytes(&bytes, 4096)
            .await
            .map_err(|_| Error::Unavailable)?;
        let outcome = self
            .store
            .put(
                &self.key("manifest.json")?,
                selection
                    .version
                    .map_or(Condition::Absent, Condition::Matches),
                &upload,
            )
            .await;
        match outcome {
            Ok(WriteOutcome::Applied(_)) => {
                let domain = if matches!(receipt.result, DirectoryResult::Emoji { .. }) {
                    jj_store_service::freshness::Domain::EmojiLibrary
                } else {
                    jj_store_service::freshness::Domain::Profiles
                };
                self.signals.notify(None, domain, None);
                Ok(receipt)
            }
            Ok(WriteOutcome::PreconditionFailed) => Err(Error::Stale),
            _ => {
                let current = self.selection().await.map_err(|_| Error::Unknown)?;
                self.actor(&current.root, caller)?;
                self.saved_receipt(&current.root, actor, request, Some(&intent))
                    .await?
                    .ok_or(Error::Unknown)
            }
        }
    }
}

/// Missing selected objects are corruption; an unreachable backend remains unavailable.
pub(super) fn stored_object_error(error: jj_store_object::Error) -> Error {
    if matches!(error, jj_store_object::Error::NotFound) {
        Error::Corrupt
    } else {
        Error::Unavailable
    }
}
