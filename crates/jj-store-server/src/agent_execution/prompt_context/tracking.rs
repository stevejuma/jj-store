//! Backfill only delivery bookkeeping; legacy prompt frames stay immutable.
use super::*;
impl Supervisor {
    pub(in crate::agent_execution) async fn initialize_section_tracking(
        &self,
        id: &str,
    ) -> Result<()> {
        let saved = self.saved(id)?;
        if !saved.delivered_sections.is_empty()
            || saved.events.iter().any(|e| {
                e.payload["type"] == "prompt"
                    && e.payload["sentContext"]["sectionRevisions"].is_object()
            })
            || self.read(|s| delivery::needs_initial_context(s, id))?
        {
            return Ok(());
        }
        let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
        let mut revisions = BTreeMap::new();
        for section in origin::sections(&saved) {
            let section: Section = serde_json::from_value(runtime::safe(
                &sanitizer,
                serde_json::to_value(section).map_err(|_| Error::Invalid)?,
            )?)
            .map_err(|_| Error::Invalid)?;
            revisions.insert(section.kind.clone(), journal::digest(&section)?);
        }
        self.update(|state| {
            let mut reserved = false;
            for intent in state
                .intents
                .values_mut()
                .filter(|i| i.outcome.session_id.as_deref() == Some(id))
            {
                if let Some(prompt) = &mut intent.prompt
                    && prompt.generation == saved.context_generation
                    && prompt.carries_initial_context
                    && prompt.state == delivery::DeliveryState::Reserved
                {
                    prompt.sections = revisions.clone();
                    reserved = true;
                }
            }
            if !reserved {
                // Keep the existing legacy initial-delivery interpretation; never
                // infer delivery of reply revisions from unstructured context.
                state
                    .sessions
                    .get_mut(id)
                    .ok_or(Error::Invalid)?
                    .delivered_sections = revisions;
            }
            Ok(())
        })
    }
}
