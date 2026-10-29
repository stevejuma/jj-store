use super::{ai_turns::*, batch_plan as plan, restore_io as files, *};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AiUndoRequest {
    pub request_id: String,
    pub conversation: String,
    pub turn: String,
    pub paths: Vec<String>,
    pub expected: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiUndoPreview {
    pub paths: Vec<String>,
    pub expected: String,
}
pub(super) fn visible(path: &str) -> bool {
    !path.split('/').any(|part| part == ".lapis")
}
pub(super) fn regular(value: Option<FileContents>) -> Result<(Option<Vec<u8>>, bool)> {
    match value {
        None => Ok((None, false)),
        Some(FileContents::Regular { data, executable }) => Ok((Some(data), executable)),
        _ => Err(Error::Jj("AI undo requires regular files".into())),
    }
}
impl Session {
    pub(super) fn idle_ai(&self) -> Result<()> {
        for context in self.active_turns()?.values() {
            if self.ai_turn(&context.conversation, &context.turn)?.phase == AiTurnPhase::Running {
                return Err(Error::Jj(
                    "Wait for active agents before undoing changes".into(),
                ));
            }
        }
        Ok(())
    }
    pub(super) async fn ai_paths(
        &self,
        record: &AiTurnRecord,
        exclusions: &[String],
    ) -> Result<Vec<String>> {
        if record.phase != AiTurnPhase::Complete {
            return Err(Error::Jj(
                "The turn has no confirmed ending checkpoint".into(),
            ));
        }
        let before = self.retained_commit(&record.before.commit).await?;
        let after = self
            .retained_commit(&record.after.as_ref().ok_or(Error::Binding)?.commit)
            .await?;
        let before = plan::entries(&before.tree(), exclusions)?;
        let after = plan::entries(&after.tree(), exclusions)?;
        Ok(plan::changed(&before, &after)
            .into_iter()
            .filter(|path| visible(path))
            .collect())
    }
    pub(super) async fn matches_ai_file(&self, checkpoint: &str, path: &str) -> Result<bool> {
        let (expected, executable) = regular(self.file_at(checkpoint, path).await?)?;
        let root = files::root(&self.binding)?;
        let current = files::read(&root, path)?;
        Ok(current == expected
            && (current.is_none() || files::executable(&root, path)? == executable))
    }
    pub async fn preview_ai_undo(
        &self,
        conversation: &str,
        turn: &str,
        paths: &[String],
        exclusions: &[String],
    ) -> Result<AiUndoPreview> {
        self.idle_ai()?;
        let record = self.ai_turn(conversation, turn)?;
        let allowed = self.ai_paths(&record, exclusions).await?;
        let mut selected = if paths.is_empty() {
            allowed.clone()
        } else {
            paths.to_vec()
        };
        selected.sort();
        selected.dedup();
        if selected.is_empty()
            || selected.len() > 1000
            || selected.iter().any(|path| !allowed.contains(path))
        {
            return Err(Error::Binding);
        }
        let after = &record.after.as_ref().ok_or(Error::Binding)?.commit;
        for path in &selected {
            if !self.matches_ai_file(after, path).await? {
                return Err(Error::Jj(format!(
                    "{path} changed after this turn; review it before undoing"
                )));
            }
            // Refuse unsupported target content before any safety/write step.
            regular(self.file_at(&record.before.commit, path).await?)?;
        }
        let identity =
            serde_json::to_vec(&(conversation, turn, &record.before.commit, after, &selected))
                .map_err(jj)?;
        Ok(AiUndoPreview {
            paths: selected,
            expected: crate::lower_hex(Sha256::digest(identity)),
        })
    }
}
