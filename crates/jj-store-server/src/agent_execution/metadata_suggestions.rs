//! ACP suggestions are observed titles, never regeneration requests or inference.
use super::*;
use jj_store_service::agent_execution::metadata as api;
impl Supervisor {
    pub(super) async fn metadata_suggestions(&self, id: &str) -> Result<()> {
        let saved = self.saved(id)?;
        let Some(event) = saved.events.iter().rev().find(|event| {
            event.committed
                && event.payload["update"]["sessionUpdate"] == "session_info_update"
                && event.payload["update"].get("title").is_some()
        }) else {
            return Ok(());
        };
        let query = api::Query {
            session: id.into(),
            repository: None,
        };
        let capture = self.metadata_capture(query.clone()).await?;
        if !capture.suggestions_changed {
            return Ok(());
        }
        if self.read(|s| {
            Ok(s.metadata_jobs.values().any(|j| {
                j.capture.repository == capture.repository
                    && j.capture.target == capture.target
                    && matches!(j.command.action, api::Action::CaptureTitles)
                    && matches!(
                        j.view.state.as_str(),
                        "prepared" | "publishing" | "outcome_unknown"
                    )
            }))
        })? {
            return Err(Error::Busy);
        }
        // A definitive stale refusal may retry against a newer committed selection.
        // An unchanged selection retains its request identity and cannot loop.
        let selection = journal::digest(&(&capture.revision, &capture.sources))?;
        let request = RequestId::new(publication::identity(&format!(
            "metadata-acp-title/{id}/{}/{selection}",
            event.sequence
        )));
        if let Some(job) =
            self.read(|s| Ok(s.metadata_jobs.get(&request).map(|j| j.view.clone())))?
        {
            return if matches!(
                job.state.as_str(),
                "prepared" | "generating" | "publishing" | "outcome_unknown"
            ) {
                Err(Error::Busy)
            } else {
                Ok(())
            };
        }
        self.metadata_submit(
            self.0.resources.actor,
            api::Command {
                version: 1,
                request_id: request,
                query,
                expected: capture.revision,
                action: api::Action::CaptureTitles,
            },
        )
        .await?;
        Err(Error::Busy)
    }
}
