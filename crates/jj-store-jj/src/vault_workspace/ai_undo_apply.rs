use super::{ai_turns::*, ai_undo::regular, restore_io as files, *};

#[derive(Serialize, Deserialize)]
struct Receipt {
    request: AiUndoRequest,
    safety: Checkpoint,
    next: usize,
    result: Option<Capture>,
}
impl Session {
    pub fn pending_ai_undo(&self) -> Result<Vec<AiUndoRequest>> {
        let directory = self.binding.state.join("ai-undo");
        if !directory.exists() {
            return Ok(vec![]);
        }
        let mut pending = Vec::new();
        for (index, entry) in fs::read_dir(directory)?.enumerate() {
            if index >= 100_000 {
                return Err(Error::Binding);
            }
            let path = entry?.path();
            if path.extension().is_none_or(|extension| extension != "json") {
                continue;
            }
            let receipt: Receipt = load(&path)?;
            if receipt.result.is_none() {
                pending.push(receipt.request);
            }
            if pending.len() > 100 {
                return Err(Error::Binding);
            }
        }
        Ok(pending)
    }
    pub async fn apply_ai_undo(
        &mut self,
        request: &AiUndoRequest,
        exclusions: &[String],
    ) -> Result<Capture> {
        identifier(&request.request_id)?;
        identifier(&request.conversation)?;
        identifier(&request.turn)?;
        self.idle_ai()?;
        let path = self
            .binding
            .state
            .join("ai-undo")
            .join(format!("{}.json", request.request_id));
        let record = self.ai_turn(&request.conversation, &request.turn)?;
        let allowed = self.ai_paths(&record, exclusions).await?;
        if request.paths.is_empty()
            || request.paths.len() > 1000
            || request.paths.iter().any(|p| !allowed.contains(p))
        {
            return Err(Error::Binding);
        }
        let mut receipt: Receipt = if path.exists() {
            let saved: Receipt = load(&path)?;
            if saved.request != *request || saved.next > request.paths.len() {
                return Err(Error::Binding);
            }
            if let Some(result) = saved.result {
                return Ok(result);
            }
            saved
        } else {
            let preview = self
                .preview_ai_undo(
                    &request.conversation,
                    &request.turn,
                    &request.paths,
                    exclusions,
                )
                .await?;
            if preview.paths != request.paths || preview.expected != request.expected {
                return Err(Error::Binding);
            }
            self.capture(exclusions, "Safety checkpoint before AI undo")
                .await?;
            let saved = Receipt {
                request: request.clone(),
                safety: self.checkpoint()?.ok_or(Error::Binding)?,
                next: 0,
                result: None,
            };
            save(&path, &saved)?;
            saved
        };
        let before = &record.before.commit;
        let after = &record.after.as_ref().ok_or(Error::Binding)?.commit;
        // Recheck already restored paths too, before continuing any partial
        // batch. A later edit in a completed path must stop the whole retry.
        for (index, selected) in request.paths.iter().enumerate() {
            let target_matches = self.matches_ai_file(before, selected).await?;
            if !target_matches
                && (index < receipt.next || !self.matches_ai_file(after, selected).await?)
            {
                return Err(Error::Jj(format!(
                    "{selected} changed during undo; review it before continuing"
                )));
            }
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while let Some(selected) = request.paths.get(receipt.next) {
            if std::time::Instant::now() > deadline {
                return Err(Error::Jj(
                    "AI undo reached its work limit; retry the same request".into(),
                ));
            }
            let (target, executable) = regular(self.file_at(before, selected).await?)?;
            // A lost progress write may leave the exact desired bytes. Otherwise
            // only the original terminal bytes authorize a replacement.
            if !self.matches_ai_file(before, selected).await? {
                if !self.matches_ai_file(after, selected).await? {
                    return Err(Error::Jj(format!(
                        "{selected} changed during undo; your later edits were retained"
                    )));
                }
                let (original, _) = regular(self.file_at(after, selected).await?)?;
                files::apply(
                    &files::root(&self.binding)?,
                    selected,
                    &files::digest(original.as_deref()),
                    target.as_deref(),
                    executable,
                )?;
            }
            receipt.next += 1;
            save(&path, &receipt)?;
        }
        for selected in &request.paths {
            if !self.matches_ai_file(before, selected).await? {
                return Err(Error::Jj(
                    "A restored file changed before verification".into(),
                ));
            }
        }
        let result = self
            .capture(
                exclusions,
                &format!(
                    "Undo observed AI turn\n\nLapis-AI-Undo: {}/{}",
                    request.conversation, request.turn
                ),
            )
            .await?;
        receipt.result = Some(result.clone());
        save(&path, &receipt)?;
        Ok(result)
    }
}
