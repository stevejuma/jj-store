use super::{ai_turns::*, *};

impl Session {
    pub(super) fn ai_description(
        &self,
        title: &str,
        extra: Option<&AiTurnContext>,
    ) -> Result<String> {
        let mut active = self.active_turns()?;
        for (key, context) in active.clone() {
            if self.ai_turn(&context.conversation, &context.turn)?.phase != AiTurnPhase::Running {
                active.remove(&key);
            }
        }
        if let Some(context) = extra {
            active.insert(context.key(), context.clone());
        }
        let mut description = title.to_owned();
        if !active.is_empty() {
            description.push_str("\n\nLapis-AI-Observation-Version: 1\n");
            for context in active.values() {
                description.push_str("Lapis-AI-Observation: ");
                description.push_str(&serde_json::to_string(context).map_err(jj)?);
                description.push('\n');
            }
        }
        if description.len() > 4096 {
            return Err(Error::Jj("Too many active AI attribution contexts".into()));
        }
        Ok(description)
    }
    pub async fn capture_observed(&mut self, exclusions: &[String]) -> Result<Capture> {
        let description = self.ai_description("Automatic vault backup", None)?;
        self.capture(exclusions, &description).await
    }
    pub async fn begin_ai_turn(
        &mut self,
        context: &AiTurnContext,
        exclusions: &[String],
    ) -> Result<AiTurnRecord> {
        context.validate()?;
        let path = self.turn_path(&context.conversation, &context.turn)?;
        if path.exists() {
            let record = self.ai_turn(&context.conversation, &context.turn)?;
            if record.context != *context {
                return Err(Error::Binding);
            }
            if record.phase == AiTurnPhase::Running {
                let mut active = self.active_turns()?;
                active.insert(context.key(), context.clone());
                self.save_active(&active)?;
            }
            return Ok(record);
        }
        // Validate the bounded portable description before admitting a new turn.
        self.ai_description("Automatic vault backup", Some(context))?;
        self.capture_observed(exclusions).await?;
        let before = self.checkpoint()?.ok_or(Error::Binding)?;
        let record = AiTurnRecord {
            version: 1,
            context: context.clone(),
            started_at: now(),
            ended_at: None,
            phase: AiTurnPhase::Running,
            outcome: None,
            before,
            after: None,
        };
        save(&path, &record)?;
        let mut active = self.active_turns()?;
        active.insert(context.key(), context.clone());
        self.save_active(&active)?;
        Ok(record)
    }
    pub fn bind_ai_turn(&self, context: &AiTurnContext) -> Result<AiTurnRecord> {
        context.validate()?;
        self.ai_description("Automatic vault backup", Some(context))?;
        let mut record = self.ai_turn(&context.conversation, &context.turn)?;
        if record.phase != AiTurnPhase::Running
            || record.context.message != context.message
            || record.context.agent != context.agent
            || record.context.harness != context.harness
            || record.context.requested_model != context.requested_model
        {
            return Err(Error::Binding);
        }
        record.context = context.clone();
        save(
            &self.turn_path(&context.conversation, &context.turn)?,
            &record,
        )?;
        let mut active = self.active_turns()?;
        active.insert(context.key(), context.clone());
        self.save_active(&active)?;
        Ok(record)
    }
    pub async fn end_ai_turn(
        &mut self,
        conversation: &str,
        turn: &str,
        outcome: AiTurnOutcome,
        exclusions: &[String],
    ) -> Result<AiTurnRecord> {
        let mut record = self.ai_turn(conversation, turn)?;
        if record.phase != AiTurnPhase::Running {
            return Ok(record);
        }
        // Retire attribution durably before capture. A crash or failed capture
        // cannot attribute later human edits to a finished agent turn.
        record.phase = AiTurnPhase::Incomplete;
        record.outcome = Some(outcome.clone());
        record.ended_at = Some(now());
        save(&self.turn_path(conversation, turn)?, &record)?;
        let description = self.ai_description("AI turn observed changes", Some(&record.context))?;
        let result = self.capture(exclusions, &description).await;
        let mut active = self.active_turns()?;
        active.remove(&record.context.key());
        self.save_active(&active)?;
        result?;
        record.after = self.checkpoint()?;
        if outcome != AiTurnOutcome::Unknown {
            record.phase = AiTurnPhase::Complete;
        }
        save(&self.turn_path(conversation, turn)?, &record)?;
        Ok(record)
    }
}
