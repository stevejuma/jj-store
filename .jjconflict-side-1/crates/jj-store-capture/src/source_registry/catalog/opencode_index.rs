//! Atomic admission metadata for rediscovering ended API sessions, never directory authority.
use super::{CaptureResult, Catalog, MAX_ALIASES, SourceInstanceId, digest};
use crate::opencode_snapshot::{PRODUCT_VERSION, native_id_valid};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OpenCodeIndexEntry {
    endpoint: [u8; 32],
    native_session: String,
    witness: [u8; 32],
    source: SourceInstanceId,
    product_version: String,
}
impl Catalog {
    pub(super) fn validate_opencode_index(&self) -> CaptureResult<()> {
        if self.opencode_sessions.len() > MAX_ALIASES {
            return Err("OpenCode recovery index capacity exhausted".into());
        }
        let mut entries = BTreeSet::new();
        for entry in &self.opencode_sessions {
            if entry.endpoint == [0; 32]
                || entry.witness == [0; 32]
                || !native_id_valid(&entry.native_session, "ses")
                || entry.product_version != PRODUCT_VERSION
                || !entries.insert((entry.endpoint, &entry.native_session, entry.witness))
            {
                return Err("OpenCode recovery index binding unavailable".into());
            }
            let native = entry.native_session.as_bytes();
            let location = digest(b"opencode-api-location/v1", &[&entry.endpoint, native]);
            let physical = digest(b"opencode-api-lease/v1", &[&entry.endpoint, native]);
            let marker = digest(b"opencode-admitted-session/v1", &[&entry.witness]);
            let Some(source) = self.sources.iter().find(|s| s.id == entry.source) else {
                return Err("OpenCode recovery source unavailable".into());
            };
            if !source.locations.contains(&location)
                || !source.physical.contains(&physical)
                || !source.witnesses.contains(&entry.witness)
                || !source.admitted_sessions.contains(&marker)
            {
                return Err("OpenCode recovery index is not admitted".into());
            }
        }
        Ok(())
    }
    pub(in crate::source_registry) fn remember_opencode(
        &mut self,
        source: SourceInstanceId,
        endpoint: [u8; 32],
        native_session: &str,
        witness: [u8; 32],
    ) -> CaptureResult<()> {
        let entry = OpenCodeIndexEntry {
            endpoint,
            native_session: native_session.into(),
            witness,
            source,
            product_version: PRODUCT_VERSION.into(),
        };
        if self.opencode_sessions.contains(&entry) {
            return Ok(());
        }
        if self.opencode_sessions.len() == MAX_ALIASES {
            return Err("OpenCode recovery index capacity exhausted".into());
        }
        self.opencode_sessions.push(entry);
        self.validate_opencode_index()
    }
    pub(in crate::source_registry) fn opencode_sessions(&self, endpoint: [u8; 32]) -> Vec<String> {
        self.opencode_sessions
            .iter()
            .filter(|e| e.endpoint == endpoint)
            .map(|e| e.native_session.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_recovery_index_refuses_without_eviction() {
        let mut catalog = Catalog::empty([8; 16]);
        let entry = OpenCodeIndexEntry {
            endpoint: [1; 32],
            native_session: "ses_first".into(),
            witness: [2; 32],
            source: SourceInstanceId::new([3; 16]),
            product_version: PRODUCT_VERSION.into(),
        };
        catalog.opencode_sessions = vec![entry.clone(); MAX_ALIASES];
        assert!(
            catalog
                .remember_opencode(entry.source, [1; 32], "ses_next", [2; 32])
                .is_err()
        );
        assert_eq!(catalog.opencode_sessions.len(), MAX_ALIASES);
        assert!(catalog.opencode_sessions.iter().all(|e| e == &entry));
        catalog.opencode_sessions.push(entry);
        assert!(catalog.validate_opencode_index().is_err());
    }
}
