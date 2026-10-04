//! Bounded turn selection only; every selected batch still needs its own proof.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct CaptureDrainScheduler {
    after: Option<CaptureDrainPartition>,
}
impl CaptureDrainScheduler {
    /// Selection advances the turn even if the caller later withholds delivery.
    /// Pending contents remain borrowed; no request, receipt or body is retained.
    pub fn select<'a>(
        &mut self,
        pending: &'a [PendingCaptureV1],
    ) -> AuthorityResult<Option<&'a PendingCaptureV1>> {
        if pending.len() > crate::agent_capture::MAX_ENTRIES {
            return Err("capture scheduling entry limit exceeded".into());
        }
        let mut identities = BTreeSet::new();
        let mut blocked = BTreeSet::new();
        let mut heads: BTreeMap<_, &PendingCaptureV1> = BTreeMap::new();
        for entry in pending {
            entry.validate()?;
            if !identities.insert(entry.batch.batch_id) {
                return Err("duplicate capture scheduling batch identity".into());
            }
            let partition = CaptureDrainPartition::of(entry);
            if matches!(entry.state, PendingCaptureStateV1::LocalPrepared { .. }) {
                blocked.insert(partition);
            }
            heads
                .entry(partition)
                .and_modify(|head| {
                    if capture_order(entry, head).is_lt() {
                        *head = entry;
                    }
                })
                .or_insert(entry);
        }
        heads.retain(|partition, _| !blocked.contains(partition));
        let selected = heads
            .iter()
            .find(|(partition, _)| self.after.is_none_or(|after| **partition > after))
            .or_else(|| heads.first_key_value());
        let Some((partition, entry)) = selected else {
            return Ok(None);
        };
        self.after = Some(*partition);
        Ok(Some(*entry))
    }
}
