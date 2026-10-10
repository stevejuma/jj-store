//! Durable terminal receipts and cancellation linearized with credential acceptance.
use super::*;
impl Supervisor {
    pub(in crate::agent_execution) fn chatgpt_cancel(&self, id: &str) -> Result<()> {
        let mut entries = self
            .0
            .chatgpt
            .attempts
            .lock()
            .map_err(|_| Error::Unavailable)?;
        if let Some(entry) = entries.get_mut(id) {
            if entry.view.state != "authorizing" {
                return Ok(());
            }
            self.chatgpt_receipt(id, "cancelled", None)?;
            if let Some(task) = entry.task.take() {
                task.abort();
            }
            entry.view.state = "cancelled".into();
            entry.view.error = None;
        }
        Ok(())
    }
    fn chatgpt_receipt(&self, id: &str, state: &str, error: Option<&str>) -> Result<()> {
        self.update(|s| {
            let (_, view) = s
                .chatgpt_requests
                .values_mut()
                .find(|(_, v)| v.id == id)
                .ok_or(Error::Invalid)?;
            view.state = state.into();
            view.error = error.map(str::to_owned);
            Ok(())
        })
    }
    pub(super) fn chatgpt_attempt(&self, id: &str, state: &str, error: Option<&str>) -> Result<()> {
        let mut entries = self
            .0
            .chatgpt
            .attempts
            .lock()
            .map_err(|_| Error::Unavailable)?;
        if let Some(entry) = entries.get_mut(id) {
            if entry.view.state != "authorizing" {
                return Ok(());
            }
            self.chatgpt_receipt(id, state, error)?;
            entry.view.state = state.into();
            entry.view.error = error.map(str::to_owned);
            entry.task = None;
        }
        Ok(())
    }
}
