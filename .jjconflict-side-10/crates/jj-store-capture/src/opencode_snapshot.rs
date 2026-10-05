//! Bounded memory-only native snapshots; no lease, activity admission or durability.
use crate::CaptureResult;
use serde_json::{Value, json};
use std::path::{Component, Path, PathBuf};

pub const PRODUCT_VERSION: &str = "1.18.30";
pub const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
pub const MAX_PAGE_MESSAGES: usize = 32;
pub const MAX_PAGE_PARTS: usize = 256;
pub const MAX_CURSOR_BYTES: usize = 4096;

pub fn native_id_valid(value: &str, prefix: &str) -> bool {
    value.starts_with(prefix)
        && value.len() > prefix.len()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
fn directory(value: &Value) -> CaptureResult<PathBuf> {
    let text = value.as_str().ok_or("OpenCode directory unavailable")?;
    let path = PathBuf::from(text);
    if text.len() > 4096
        || text.chars().any(char::is_control)
        || !path.is_absolute()
        || path.parent().is_none()
        || path
            .components()
            .any(|c| matches!(c, Component::CurDir | Component::ParentDir))
    {
        return Err("OpenCode directory unavailable".into());
    }
    Ok(path)
}
fn decode(bytes: &[u8]) -> CaptureResult<Value> {
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err("OpenCode response exceeds bound".into());
    }
    serde_json::from_slice(bytes).map_err(|_| "OpenCode native JSON unavailable".into())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenCodeActivity {
    Idle,
    Busy,
    Retry,
    Unsupported,
}
impl OpenCodeActivity {
    pub fn active(self) -> bool {
        matches!(self, Self::Busy | Self::Retry)
    }
    pub fn from_status_map(bytes: &[u8], session: &str) -> CaptureResult<Self> {
        if !native_id_valid(session, "ses") {
            return Err("OpenCode session identity unavailable".into());
        }
        let value = decode(bytes)?;
        let map = value
            .as_object()
            .filter(|m| m.len() <= 4096)
            .ok_or("OpenCode status map unavailable")?;
        // The pinned status service removes idle entries from its map.
        Ok(match map.get(session) {
            None => Self::Idle,
            Some(v) => match v.get("type").and_then(Value::as_str) {
                Some("idle") => Self::Idle,
                Some("busy") => Self::Busy,
                Some("retry") => Self::Retry,
                _ => Self::Unsupported,
            },
        })
    }
    pub fn native_status(self) -> Option<&'static str> {
        match self {
            Self::Idle => Some("idle"),
            Self::Busy => Some("busy"),
            Self::Retry => Some("retry"),
            Self::Unsupported => None,
        }
    }
}
/// Paths and native identities deliberately have no Debug/Serialize implementation.
#[derive(Clone, PartialEq, Eq)]
pub struct OpenCodeSessionSnapshot {
    native_session: String,
    directory: PathBuf,
}
impl OpenCodeSessionSnapshot {
    pub fn from_api_info(bytes: &[u8], expected_session: &str) -> CaptureResult<Self> {
        let session = Self::from_info(&decode(bytes)?)?;
        if session.native_session != expected_session {
            return Err("OpenCode session binding changed".into());
        }
        Ok(session)
    }
    fn from_info(info: &Value) -> CaptureResult<Self> {
        let id = info
            .get("id")
            .and_then(Value::as_str)
            .filter(|id| native_id_valid(id, "ses"))
            .ok_or("OpenCode session identity unavailable")?;
        if info.get("version").and_then(Value::as_str) != Some(PRODUCT_VERSION) {
            return Err("OpenCode session product version unavailable".into());
        }
        Ok(Self {
            native_session: id.into(),
            directory: directory(&info["directory"])?,
        })
    }
    pub fn native_session_key(&self) -> &str {
        &self.native_session
    }
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    /// One verified API status class. Exported messages never supply this evidence.
    pub fn status_record(
        &self,
        activity: OpenCodeActivity,
    ) -> CaptureResult<OpenCodeMessageSnapshot> {
        let status = activity
            .native_status()
            .ok_or("OpenCode explicit status unavailable")?;
        Ok(OpenCodeMessageSnapshot {
            directory: Some(self.directory.clone()),
            envelope: json!({"capture_format":"opencode-snapshot/v1","product_version":PRODUCT_VERSION,"session_id":self.native_session,"record":{"type":"session_status","status":{"type":status}}}),
        })
    }
    pub fn messages(&self, bytes: &[u8]) -> CaptureResult<OpenCodeMessageSnapshotPage> {
        self.page(decode(bytes)?)
    }
    pub fn from_export(bytes: &[u8]) -> CaptureResult<(Self, OpenCodeMessageSnapshotPage)> {
        let value = decode(bytes)?;
        let session = Self::from_info(&value["info"])?;
        let page = session.page(value["messages"].clone())?;
        Ok((session, page))
    }
    fn page(&self, value: Value) -> CaptureResult<OpenCodeMessageSnapshotPage> {
        let messages = value
            .as_array()
            .filter(|m| m.len() <= MAX_PAGE_MESSAGES)
            .ok_or("OpenCode message page unavailable")?;
        let mut ids = std::collections::BTreeSet::new();
        let mut total_parts = 0usize;
        let mut records = Vec::with_capacity(messages.len());
        for message in messages {
            let info = &message["info"];
            let id = info["id"]
                .as_str()
                .filter(|id| native_id_valid(id, "msg") && ids.insert(*id))
                .ok_or("OpenCode message identity unavailable")?;
            if info["sessionID"].as_str() != Some(&self.native_session) {
                return Err("OpenCode message owner changed".into());
            }
            let parts = message["parts"]
                .as_array()
                .ok_or("OpenCode message parts unavailable")?;
            total_parts = total_parts
                .checked_add(parts.len())
                .ok_or("OpenCode part count unavailable")?;
            if total_parts > MAX_PAGE_PARTS {
                return Err("OpenCode page part bound exceeded".into());
            }
            let mut part_ids = std::collections::BTreeSet::new();
            if parts.iter().any(|p| {
                p["sessionID"].as_str() != Some(&self.native_session)
                    || p["messageID"].as_str() != Some(id)
                    || !p["id"]
                        .as_str()
                        .is_some_and(|id| native_id_valid(id, "prt") && part_ids.insert(id))
            }) {
                return Err("OpenCode part owner changed".into());
            }
            let context = if info["role"].as_str() == Some("assistant") {
                directory(&info["path"]["cwd"]).ok()
            } else {
                Some(self.directory.clone())
            };
            records.push(OpenCodeMessageSnapshot {
                directory: context,
                envelope: json!({"capture_format":"opencode-snapshot/v1", "product_version":PRODUCT_VERSION,
                    "session_id":self.native_session,"record":{"type":"message","info":info,"parts":parts}}),
            });
        }
        Ok(OpenCodeMessageSnapshotPage { records })
    }
}
pub struct OpenCodeMessageSnapshotPage {
    pub records: Vec<OpenCodeMessageSnapshot>,
}
pub struct OpenCodeMessageSnapshot {
    directory: Option<PathBuf>,
    envelope: Value,
}
impl OpenCodeMessageSnapshot {
    pub fn directory(&self) -> Option<&Path> {
        self.directory.as_deref()
    }
    /// Raw native data remains memory-only until the caller verifies this context and sanitizes.
    pub fn envelope(&self) -> &Value {
        &self.envelope
    }
}
