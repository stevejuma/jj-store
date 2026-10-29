//! JJ adaptation of shared per-participant privacy and safe stream accumulation.
use super::{
    Result,
    runtime_privacy::{AdmittedSanitizer, error},
};
use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use jj_store_conversations::control_v3;
use serde_json::Value;
#[cfg(test)]
use serde_json::json;

#[derive(Default)]
pub(super) struct Stream(control_v3::Stream);
impl Stream {
    #[cfg(test)]
    pub(super) fn is_drained(&self) -> bool {
        self.0.is_drained()
    }
    pub(super) fn frames(
        &mut self,
        id: &str,
        sanitizer: &TranscriptSanitizer,
        raw: Value,
    ) -> Result<Vec<Value>> {
        self.0
            .frames(id, &AdmittedSanitizer(sanitizer), raw)
            .map_err(error)
    }
    pub(super) fn observe(&mut self, id: &str, request: &str, value: &mut Value) -> Result<()> {
        self.0.observe(id, request, value).map_err(error)
    }
}
#[cfg(test)]
mod tests;
