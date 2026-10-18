use super::{restore_io as files, *};
use std::io::{Read as _, Write as _};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RestoreRequest {
    pub request_id: String,
    pub checkpoint: String,
    pub path: String,
    pub expected: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestorePreview {
    pub checkpoint: String,
    pub path: String,
    pub expected: String,
    pub bytes: usize,
    pub deleted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    request: RestoreRequest,
    safety: Checkpoint,
    result: Option<Capture>,
}
fn regular(contents: Option<FileContents>) -> Result<(Option<Vec<u8>>, bool)> {
    match contents {
        None => Ok((None, false)),
        Some(FileContents::Regular { data, executable }) => Ok((Some(data), executable)),
        Some(FileContents::Symlink { .. }) => {
            Err(Error::Jj("Restore of symbolic links is unavailable".into()))
        }
    }
}
fn save(path: &Path, receipt: &Receipt) -> Result<()> {
    let parent = path.parent().ok_or(Error::Binding)?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(&serde_json::to_vec(receipt).map_err(jj)?)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|e| Error::Io(e.error))?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
impl Session {
    /// A preview is read-only and anchored to a retained checkpoint and exact
    /// current disk bytes. It grants no authority to choose a filesystem root.
    pub async fn preview_file_restore(
        &self,
        checkpoint: &str,
        path: &str,
    ) -> Result<RestorePreview> {
        files::checked(path)?;
        let (target, _) = regular(self.file_at(checkpoint, path).await?)?;
        let current = files::read(&files::root(&self.binding)?, path)?;
        Ok(RestorePreview {
            checkpoint: checkpoint.into(),
            path: path.into(),
            expected: files::digest(current.as_deref()),
            bytes: target.as_ref().map_or(0, Vec::len),
            deleted: target.is_none(),
        })
    }
    /// Safety checkpoint precedes every write. The durable request receipt
    /// distinguishes a retry after publication from a new restore command.
    pub async fn restore_file(
        &mut self,
        request: &RestoreRequest,
        exclusions: &[String],
    ) -> Result<Capture> {
        files::checked(&request.path)?;
        if exclusions.iter().any(|prefix| {
            request.path == *prefix || request.path.starts_with(&format!("{prefix}/"))
        }) {
            return Err(Error::Jj("This file is excluded from vault backup".into()));
        }
        if request.request_id.is_empty()
            || request.request_id.len() > 128
            || !request
                .request_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            || !(request.expected == "missing"
                || request.expected.len() == 64
                    && request.expected.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            return Err(Error::Binding);
        }
        let (target, executable) =
            regular(self.file_at(&request.checkpoint, &request.path).await?)?;
        let root = files::root(&self.binding)?;
        let directory = self.binding.state.join("restore-receipts");
        fs::create_dir_all(&directory)?;
        let path = directory.join(format!("{}.json", request.request_id));
        let mut receipt = if path.exists() {
            let mut data = Vec::new();
            File::open(&path)?.take(32769).read_to_end(&mut data)?;
            if data.len() > 32768 {
                return Err(Error::Binding);
            }
            let saved: Receipt = serde_json::from_slice(&data).map_err(jj)?;
            if saved.request != *request {
                return Err(Error::Binding);
            }
            if let Some(result) = saved.result {
                return Ok(result);
            }
            saved
        } else {
            if files::digest(files::read(&root, &request.path)?.as_deref()) != request.expected {
                return Err(Error::Jj(
                    "The file changed since the restore preview; review it again".into(),
                ));
            }
            self.capture(exclusions, "Safety checkpoint before file restore")
                .await?;
            let saved = Receipt {
                request: request.clone(),
                safety: self.checkpoint()?.ok_or(Error::Binding)?,
                result: None,
            };
            save(&path, &saved)?;
            saved
        };
        // A crash after atomic replacement leaves target bytes. Do not overwrite
        // later edits or reinterpret a reused request identity.
        if files::digest(files::read(&root, &request.path)?.as_deref())
            != files::digest(target.as_deref())
            || target.is_some() && files::executable(&root, &request.path)? != executable
        {
            self.binding.verify_saved()?;
            files::apply(
                &root,
                &request.path,
                &request.expected,
                target.as_deref(),
                executable,
            )?;
        }
        let result = self
            .capture(exclusions, &format!("Restore {}", request.path))
            .await?;
        receipt.result = Some(result.clone());
        save(&path, &receipt)?;
        Ok(result)
    }
}
