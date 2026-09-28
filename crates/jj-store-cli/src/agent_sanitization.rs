//! Fail-closed in-process hook sanitization backed by MongoDB Kingfisher.
use crate::agent_capture_files::restrict_directory;
use jj_agent::{SanitizedMediaType, SanitizedPayloadV1};
use jj_store_jj::operation_stores::AuthorityResult;
use kingfisher_core::Blob;
use kingfisher_rules::{RuleCacheConfig, RulesDatabase, get_builtin_rules};
use kingfisher_scanner::{Scanner, ScannerConfig};
use serde_json::Value;
use std::{fs, path::Path, sync::Arc};

pub const MAX_HOOK_BYTES: usize = 4 * 1024 * 1024;
pub const KINGFISHER_POLICY: &str = "mongodb-kingfisher@2.7.0/builtin-medium/v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SanitizedHookPayload {
    Sanitized {
        payload: SanitizedPayloadV1,
        policy: &'static str,
    },
    Omitted,
}

pub fn sanitize_hook_payload(raw: &[u8], workspace: &Path, cache: &Path) -> SanitizedHookPayload {
    sanitize(raw, workspace, cache).unwrap_or(SanitizedHookPayload::Omitted)
}

pub fn warm_kingfisher_cache(cache: &Path) -> AuthorityResult<()> {
    scanner(cache).map(|_| ())
}

pub fn kingfisher_cache_ready(cache: &Path) -> bool {
    fs::symlink_metadata(cache).is_ok_and(|value| value.is_dir() && !value.file_type().is_symlink())
        && fs::read_dir(cache).is_ok_and(|entries| {
            entries.filter_map(Result::ok).any(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|value| value == "vscdb")
                    && entry.file_type().is_ok_and(|value| value.is_file())
            })
        })
}

fn sanitize(raw: &[u8], workspace: &Path, cache: &Path) -> Option<SanitizedHookPayload> {
    if raw.len() > MAX_HOOK_BYTES {
        return None;
    }
    let mut value: Value = match serde_json::from_slice(raw) {
        Ok(Value::Object(value)) => Value::Object(value),
        _ => return None,
    };
    let workspace = workspace.to_str()?;
    normalize_workspace_paths(&mut value, workspace);
    let canonical = serde_json::to_vec(&value).ok()?;
    let scanner = scanner(cache).ok()?;
    let blob = Blob::from_bytes(canonical.clone());
    let mut spans = scanner
        .scan_blob_at_path(&blob, "")
        .ok()?
        .into_iter()
        .map(|finding| (finding.location.start_offset, finding.location.end_offset))
        .collect::<Vec<_>>();
    spans.sort_unstable();
    let spans = merge_spans(spans, canonical.len())?;
    let mut sanitized = canonical;
    for (start, end) in spans.into_iter().rev() {
        sanitized.splice(start..end, b"[REDACTED]".iter().copied());
    }
    if serde_json::from_slice::<Value>(&sanitized).is_err() {
        return None;
    }
    Some(SanitizedHookPayload::Sanitized {
        payload: SanitizedPayloadV1::new(SanitizedMediaType::Json, &sanitized).ok()?,
        policy: KINGFISHER_POLICY,
    })
}

fn scanner(cache: &Path) -> AuthorityResult<Scanner> {
    fs::create_dir_all(cache).map_err(|_| "Kingfisher cache is unavailable")?;
    restrict_directory(cache).map_err(|_| "Kingfisher cache is unavailable")?;
    let rules = get_builtin_rules(None).map_err(|_| "Kingfisher rules unavailable")?;
    let database =
        RulesDatabase::from_rule_collection_with_cache(rules, &RuleCacheConfig::new(cache))
            .map_err(|_| "Kingfisher rules could not be compiled")?;
    let config = ScannerConfig {
        enable_base64_decoding: false,
        enable_dedup: false,
        redact_secrets: true,
        ..ScannerConfig::default()
    };
    Ok(Scanner::with_config(Arc::new(database), config))
}

fn normalize_workspace_paths(value: &mut Value, workspace: &str) {
    match value {
        Value::String(text) => {
            if text.contains(workspace) {
                *text = text.replace(workspace, "$WORKSPACE");
            }
        }
        Value::Array(values) => {
            for value in values {
                normalize_workspace_paths(value, workspace);
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                normalize_workspace_paths(value, workspace);
            }
        }
        _ => {}
    }
}

fn merge_spans(spans: Vec<(usize, usize)>, length: usize) -> Option<Vec<(usize, usize)>> {
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in spans {
        if start >= end || end > length {
            return None;
        }
        if let Some(previous) = merged.last_mut()
            && start <= previous.1
        {
            previous.1 = previous.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    Some(merged)
}
