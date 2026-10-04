//! Explicit versioned product surfaces; registration is not runtime qualification.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureProfile {
    CodexCliJsonlV1,
    CodexDesktopJsonlV1,
    CursorDesktopTranscriptV1,
    CursorCliTranscriptV1,
    OpencodeReadOnlyApiV1,
    OpencodeExportJsonV1,
}

impl CaptureProfile {
    pub fn is_cursor(self) -> bool {
        matches!(
            self,
            Self::CursorDesktopTranscriptV1 | Self::CursorCliTranscriptV1
        )
    }
    pub fn integration(self) -> jj_agent::IntegrationId {
        if self.is_cursor() {
            jj_agent::IntegrationId::Cursor
        } else if matches!(
            self,
            Self::OpencodeReadOnlyApiV1 | Self::OpencodeExportJsonV1
        ) {
            jj_agent::IntegrationId::OpenCode
        } else {
            jj_agent::IntegrationId::Codex
        }
    }
    pub fn parser(self) -> String {
        format!(
            "{}@{}/v1",
            if self.is_cursor() {
                "cursor-jsonl"
            } else {
                "codex-jsonl"
            },
            self.product_version()
        )
    }
    pub fn normalizer(self) -> jj_agent::NormalizerProfileV1 {
        jj_agent::NormalizerProfileV1 {
            integration: self.integration(),
            product_version: self.product_version().into(),
            schema_version: 1,
        }
    }
    pub fn product_version(self) -> &'static str {
        match self {
            Self::CodexCliJsonlV1 => "0.156.1",
            Self::CodexDesktopJsonlV1 => "0.159.2",
            Self::CursorDesktopTranscriptV1 => "3.22.7",
            Self::CursorCliTranscriptV1 => "2026.09.26-dd393fe",
            Self::OpencodeReadOnlyApiV1 | Self::OpencodeExportJsonV1 => "1.18.30",
        }
    }
}
