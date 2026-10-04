use crate::{IntegrationId, Result, SourceInstanceId, SourcePositionV1, activity::valid_label};
use serde::{Deserialize, Serialize};

/// Device-local reader progress acknowledged only after sanitized evidence is durable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureCursorV1 {
    pub schema_version: u32,
    pub integration: IntegrationId,
    pub source_instance_id: SourceInstanceId,
    pub generation: u64,
    pub parser_version: String,
    pub native_session_key: Option<String>,
    pub position: SourcePositionV1,
}

impl CaptureCursorV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self.generation == 0
            || !valid_label(&self.parser_version, 128)
            || self
                .native_session_key
                .as_ref()
                .is_some_and(|value| !valid_label(value, 1024))
        {
            return Err(crate::Error::Invalid("capture cursor"));
        }
        Ok(())
    }
}
