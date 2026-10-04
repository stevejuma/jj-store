use super::{CaptureResult, digest};
use jj_agent::SourceInstanceId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
mod opencode_index;
use opencode_index::OpenCodeIndexEntry;
pub(super) const MAX_ALIASES: usize = 4096;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    installation: [u8; 16],
    next: u64,
    sources: Vec<Source>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    opencode_sessions: Vec<OpenCodeIndexEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    id: SourceInstanceId,
    ordinal: u64,
    locations: Vec<[u8; 32]>,
    witnesses: Vec<[u8; 32]>,
    physical: Vec<[u8; 32]>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    admitted_sessions: Vec<[u8; 32]>,
}
impl Catalog {
    pub(super) fn empty(installation: [u8; 16]) -> Self {
        Self {
            version: 1,
            installation,
            next: 1,
            sources: Vec::new(),
            opencode_sessions: Vec::new(),
        }
    }
    fn source_id(&self, ordinal: u64) -> SourceInstanceId {
        let hash = digest(
            b"source-instance/v1",
            &[&self.installation, &ordinal.to_be_bytes()],
        );
        let mut bytes = [0; 16];
        bytes.copy_from_slice(&hash[..16]);
        SourceInstanceId::new(bytes)
    }
    pub(super) fn validate(&self, installation: [u8; 16], capacity: usize) -> CaptureResult<()> {
        if self.version != 1
            || self.installation != installation
            || self.next != self.sources.len() as u64 + 1
            || self.sources.len() > capacity
        {
            return Err("invalid source registry binding or capacity".into());
        }
        let mut ids = BTreeSet::new();
        let mut locations = BTreeSet::new();
        let mut witnesses = BTreeSet::new();
        let mut physical = BTreeSet::new();
        let mut admissions = 0;
        for (index, source) in self.sources.iter().enumerate() {
            if source.ordinal != index as u64 + 1
                || source.id != self.source_id(source.ordinal)
                || source.id.as_bytes() == &[0; 16]
                || !ids.insert(source.id)
                || source.locations.is_empty()
                || source.witnesses.is_empty()
                || source.physical.is_empty()
            {
                return Err("invalid source registry identity".into());
            }
            for key in &source.locations {
                if !locations.insert(*key) {
                    return Err("ambiguous source location".into());
                }
            }
            for key in &source.witnesses {
                if !witnesses.insert(*key) {
                    return Err("ambiguous source witness".into());
                }
            }
            for key in &source.physical {
                if !physical.insert(*key) {
                    return Err("ambiguous physical source".into());
                }
            }
            let mut sessions = BTreeSet::new();
            for session in &source.admitted_sessions {
                if !sessions.insert(*session) || *session == [0; 32] {
                    return Err("invalid source session admission".into());
                }
                admissions += 1;
            }
        }
        if locations.len() > MAX_ALIASES
            || witnesses.len() > MAX_ALIASES
            || physical.len() > MAX_ALIASES
            || admissions > MAX_ALIASES
        {
            return Err("source registry alias capacity exhausted".into());
        }
        self.validate_opencode_index()
    }
    fn find(
        &self,
        location: [u8; 32],
        witness: [u8; 32],
        physical_key: [u8; 32],
    ) -> CaptureResult<Option<usize>> {
        let slot = self
            .sources
            .iter()
            .position(|s| s.locations.contains(&location));
        let physical = self
            .sources
            .iter()
            .position(|s| s.witnesses.contains(&witness));
        let known_file = self
            .sources
            .iter()
            .position(|s| s.physical.contains(&physical_key));
        if slot.is_some() && physical.is_some() && slot != physical {
            return Err("conflicting source registry aliases".into());
        }
        if let Some(known) = known_file {
            if slot.or(physical).is_none() {
                return Err("unrecognized generation of registered physical source".into());
            }
            if slot.or(physical) != Some(known) {
                return Err("conflicting physical source registry aliases".into());
            }
        }
        Ok(slot.or(physical))
    }
    pub(super) fn admitted_source(
        &self,
        location: [u8; 32],
        witness: [u8; 32],
        physical_key: [u8; 32],
        session: [u8; 32],
    ) -> CaptureResult<Option<SourceInstanceId>> {
        Ok(self.find(location, witness, physical_key)?.and_then(|i| {
            self.sources[i]
                .admitted_sessions
                .contains(&session)
                .then_some(self.sources[i].id)
        }))
    }
    pub(super) fn admit(
        &mut self,
        id: SourceInstanceId,
        session: [u8; 32],
        capacity: usize,
    ) -> CaptureResult<()> {
        let source = self
            .sources
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or("source admission identity unavailable")?;
        if !source.admitted_sessions.contains(&session) {
            source.admitted_sessions.push(session);
        }
        self.validate(self.installation, capacity)
    }
    /// Mutates only the transient catalog; persistence follows successful leases.
    pub(super) fn register(
        &mut self,
        location: [u8; 32],
        witness: [u8; 32],
        physical_key: [u8; 32],
        capacity: usize,
    ) -> CaptureResult<SourceInstanceId> {
        let index = match self.find(location, witness, physical_key)? {
            Some(index) => index,
            None => {
                if self.sources.len() >= capacity {
                    return Err("source registry capacity exhausted".into());
                }
                let id = self.source_id(self.next);
                self.sources.push(Source {
                    id,
                    ordinal: self.next,
                    locations: Vec::new(),
                    witnesses: Vec::new(),
                    physical: Vec::new(),
                    admitted_sessions: Vec::new(),
                });
                self.next = self
                    .next
                    .checked_add(1)
                    .ok_or("source registry sequence exhausted")?;
                self.sources.len() - 1
            }
        };
        let source = &mut self.sources[index];
        if !source.locations.contains(&location) {
            source.locations.push(location);
        }
        if !source.witnesses.contains(&witness) {
            source.witnesses.push(witness);
        }
        if !source.physical.contains(&physical_key) {
            source.physical.push(physical_key);
        }
        let id = source.id;
        self.validate(self.installation, capacity)?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn admission_budget_and_duplicate_digests_fail_closed_without_eviction() {
        let mut catalog = Catalog::empty([8; 16]);
        let source = catalog.register([1; 32], [2; 32], [3; 32], 1).unwrap();
        for i in 1..=MAX_ALIASES {
            let mut digest = [0; 32];
            digest[..8].copy_from_slice(&(i as u64).to_be_bytes());
            catalog.sources[0].admitted_sessions.push(digest);
        }
        catalog.validate([8; 16], 1).unwrap();
        assert!(catalog.admit(source, [255; 32], 1).is_err());
        assert_eq!(catalog.sources[0].admitted_sessions.len(), MAX_ALIASES + 1);
        catalog.sources[0].admitted_sessions.pop();
        catalog.sources[0].admitted_sessions[1] = catalog.sources[0].admitted_sessions[0];
        assert!(catalog.validate([8; 16], 1).is_err());
    }
    #[test]
    fn retained_alias_budget_has_an_exact_bound_without_eviction() {
        let mut catalog = Catalog::empty([8; 16]);
        catalog.register([1; 32], [2; 32], [3; 32], 1).unwrap();
        let keys: Vec<[u8; 32]> = (0..MAX_ALIASES)
            .map(|i| {
                let mut key = [0; 32];
                key[..8].copy_from_slice(&(i as u64).to_be_bytes());
                key
            })
            .collect();
        catalog.sources[0].locations = keys.clone();
        catalog.sources[0].witnesses = keys.clone();
        catalog.sources[0].physical = keys;
        catalog.validate([8; 16], 1).unwrap();
        catalog.sources[0].locations.push([255; 32]);
        assert!(catalog.validate([8; 16], 1).is_err());
        assert_eq!(catalog.sources[0].locations.len(), MAX_ALIASES + 1);
    }
}
