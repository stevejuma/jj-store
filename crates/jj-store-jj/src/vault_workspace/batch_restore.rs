use super::{batch_plan as plan, restore_io as files, *};
use std::io::{Read as _, Write as _};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VaultRestoreRequest {
    pub request_id: String,
    pub checkpoint: String,
    pub expected: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultRestorePreview {
    pub checkpoint: String,
    pub expected: String,
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    request: VaultRestoreRequest,
    safety: Checkpoint,
    files: Vec<String>,
    next: usize,
    result: Option<Capture>,
}
fn save(path: &Path, value: &Receipt) -> Result<()> {
    let data = serde_json::to_vec(value).map_err(jj)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err(Error::Binding);
    }
    let parent = path.parent().ok_or(Error::Binding)?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(&data)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|e| Error::Io(e.error))?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
fn regular(value: Option<FileContents>) -> Result<(Option<Vec<u8>>, bool)> {
    match value {
        Some(FileContents::Regular { data, executable }) => Ok((Some(data), executable)),
        None => Ok((None, false)),
        _ => Err(Error::Binding),
    }
}
impl Session {
    pub async fn preview_vault_restore(
        &self,
        checkpoint: &str,
        exclusions: &[String],
    ) -> Result<VaultRestorePreview> {
        let target = self.retained_commit(checkpoint).await?;
        let after = plan::entries(&target.tree(), exclusions)?;
        let live = scan::capture(self.repo.store(), &self.binding.root, exclusions).await?;
        let before = plan::entries(&live, exclusions)?;
        let mut preview = VaultRestorePreview {
            checkpoint: checkpoint.into(),
            expected: plan::identity(&live)?,
            added: 0,
            modified: 0,
            deleted: 0,
        };
        for path in plan::changed(&before, &after) {
            // Reject unsupported file/directory transitions before mutation.
            files::read(&files::root(&self.binding)?, &path)?;
            match (before.contains_key(&path), after.contains_key(&path)) {
                (false, true) => preview.added += 1,
                (true, false) => preview.deleted += 1,
                _ => preview.modified += 1,
            }
        }
        Ok(preview)
    }
    pub async fn restore_vault(
        &mut self,
        request: &VaultRestoreRequest,
        exclusions: &[String],
    ) -> Result<Capture> {
        if request.request_id.is_empty()
            || request.request_id.len() > 128
            || !request
                .request_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(Error::Binding);
        }
        let target = self.retained_commit(&request.checkpoint).await?;
        let desired = plan::entries(&target.tree(), exclusions)?;
        let desired_tree = plan::tree(self, &desired).await?;
        let directory = self.binding.state.join("vault-restore-receipts");
        fs::create_dir_all(&directory)?;
        let path = directory.join(format!("{}.json", request.request_id));
        let mut receipt: Receipt = if path.exists() {
            let mut data = Vec::new();
            File::open(&path)?
                .take(16 * 1024 * 1024 + 1)
                .read_to_end(&mut data)?;
            if data.len() > 16 * 1024 * 1024 {
                return Err(Error::Binding);
            }
            let value: Receipt = serde_json::from_slice(&data).map_err(jj)?;
            if value.request != *request || value.next > value.files.len() {
                return Err(Error::Binding);
            }
            if let Some(result) = value.result {
                return Ok(result);
            }
            value
        } else {
            let preview = self
                .preview_vault_restore(&request.checkpoint, exclusions)
                .await?;
            if preview.expected != request.expected {
                return Err(Error::Jj(
                    "Vault files changed since the restore preview; review it again".into(),
                ));
            }
            self.capture(exclusions, "Safety checkpoint before vault restore")
                .await?;
            let safety = self.checkpoint()?.ok_or(Error::Binding)?;
            let before_commit = self.retained_commit(&safety.commit).await?;
            if plan::identity(&before_commit.tree())? != request.expected {
                return Err(Error::Jj(
                    "Vault files changed before restoration could start".into(),
                ));
            }
            let before = plan::entries(&before_commit.tree(), exclusions)?;
            let value = Receipt {
                request: request.clone(),
                safety,
                files: plan::changed(&before, &desired),
                next: 0,
                result: None,
            };
            save(&path, &value)?;
            value
        };
        let root = files::root(&self.binding)?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while let Some(file) = receipt.files.get(receipt.next) {
            if std::time::Instant::now() > deadline {
                return Err(Error::Jj(
                    "Vault restore reached its work limit; retry to continue the same restore"
                        .into(),
                ));
            }
            if scan::excluded(file, exclusions) {
                return Err(Error::Binding);
            }
            let (expected, _) = regular(self.file_at(&receipt.safety.commit, file).await?)?;
            let (target, executable) = regular(self.file_at(&request.checkpoint, file).await?)?;
            let actual = files::read(&root, file)?;
            if files::digest(actual.as_deref()) != files::digest(target.as_deref())
                || target.is_some() && files::executable(&root, file)? != executable
            {
                self.binding.verify_saved()?;
                files::apply(
                    &root,
                    file,
                    &files::digest(expected.as_deref()),
                    target.as_deref(),
                    executable,
                )?;
            }
            receipt.next += 1;
            save(&path, &receipt)?;
        }
        let actual = scan::capture(self.repo.store(), &self.binding.root, exclusions).await?;
        if actual.tree_ids_and_labels() != desired_tree.tree_ids_and_labels() {
            return Err(Error::Jj(
                "Files changed while restoring the vault; later edits were retained".into(),
            ));
        }
        let result = self.capture(exclusions, "Restore vault checkpoint").await?;
        receipt.result = Some(result.clone());
        save(&path, &receipt)?;
        Ok(result)
    }
}
