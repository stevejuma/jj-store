use super::*;
use ranges::{Evidence, Required};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;
pub(super) const MAX_CHANNELS: usize = 4096;
pub(super) const MAX_RANGES: usize = 4096;
#[cfg(test)]
mod tests;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    pub channels: Vec<Channel>,
}
impl Default for Catalog {
    fn default() -> Self {
        Self {
            version: 1,
            channels: vec![],
        }
    }
}
impl Catalog {
    pub fn validate(&self) -> CaptureResult<()> {
        if self.version != 1
            || self.channels.len() > MAX_CHANNELS
            || self.range_count() > MAX_RANGES
        {
            return Err("invalid publication catalog bounds".into());
        }
        let mut keys = BTreeSet::new();
        let mut batches = BTreeSet::new();
        for channel in &self.channels {
            if channel.key.generation == 0
                || channel.key.digest == [0; 32]
                || !keys.insert((
                    channel.key.integration,
                    channel.key.source,
                    channel.key.generation,
                ))
            {
                return Err("invalid publication channel".into());
            }
            channel.latest.validate()?;
            let mut previous = None;
            if let Some(frontier) = &channel.frontier {
                frontier.range.validate()?;
                validate_evidence(frontier)?;
                if frontier.evidence.is_none() || channel.receipts == 0 || channel.chain == [0; 32]
                {
                    return Err("invalid publication frontier".into());
                }
                if !batches.insert((
                    channel.key.integration,
                    channel.key.source,
                    frontier.range.batch,
                )) {
                    return Err("duplicate publication frontier".into());
                }
                previous = Some(&frontier.range);
            } else if channel.receipts != 0 || channel.chain != [0; 32] {
                return Err("publication frontier missing".into());
            }
            for required in &channel.pending {
                required.range.validate()?;
                validate_evidence(required)?;
                if previous.is_some_and(|old| !required.range.follows(old))
                    || !batches.insert((
                        channel.key.integration,
                        channel.key.source,
                        required.range.batch,
                    ))
                {
                    return Err("publication range order differs".into());
                }
                previous = Some(&required.range);
            }
            if previous != Some(&channel.latest) {
                return Err("publication latest range differs".into());
            }
        }
        Ok(())
    }
    fn range_count(&self) -> usize {
        self.channels.iter().map(|c| c.pending.len()).sum()
    }
    pub fn add(&mut self, channel: Channel) -> CaptureResult<()> {
        if self.channels.len() == MAX_CHANNELS {
            return Err("publication channel capacity exhausted".into());
        }
        self.channels.push(channel);
        Ok(())
    }
    pub fn register(&mut self, key: Key, range: Range) -> CaptureResult<()> {
        let count = self.range_count();
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.key == key)
            .ok_or("publication channel unavailable")?;
        if let Some(old) = channel
            .pending
            .iter()
            .find(|r| r.range.batch == range.batch)
        {
            return if old.range == range {
                Ok(())
            } else {
                Err("publication range binding differs".into())
            };
        }
        if channel
            .frontier
            .as_ref()
            .is_some_and(|r| range.covered_by(&r.range))
        {
            return Ok(());
        }
        if count == MAX_RANGES {
            return Err("publication range capacity exhausted".into());
        }
        if (!channel.pending.is_empty() || channel.frontier.is_some())
            && !range.follows(&channel.latest)
        {
            return Err("publication range must advance".into());
        }
        channel.latest = range.clone();
        channel.pending.push(Required {
            range,
            evidence: None,
        });
        self.validate()
    }
    pub fn confirm(&mut self, key: Key, range: &Range, evidence: Evidence) -> CaptureResult<()> {
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.key == key)
            .ok_or("publication channel unavailable")?;
        if let Some(required) = channel
            .pending
            .iter_mut()
            .find(|r| r.range.batch == range.batch)
        {
            if &required.range != range
                || required.evidence.as_ref().is_some_and(|e| *e != evidence)
            {
                return Err("publication receipt binding differs".into());
            }
            required.evidence = Some(evidence);
        } else if let Some(frontier) = &channel.frontier
            && frontier.range == *range
            && frontier.evidence.as_ref() != Some(&evidence)
        {
            return Err("publication frontier receipt differs".into());
        } else if !channel
            .frontier
            .as_ref()
            .is_some_and(|r| range.covered_by(&r.range))
        {
            return Err("publication requirement missing".into());
        }
        let confirmed = channel
            .pending
            .iter()
            .take_while(|r| r.evidence.is_some())
            .count();
        for next in channel.pending.drain(..confirmed) {
            let mut hash = Sha256::new_with_prefix(b"jj-store/reader-publication-chain/v1\0");
            hash.update(channel.chain);
            hash.update(serde_json::to_vec(&(channel.key, &next))?);
            channel.chain = hash.finalize().into();
            channel.receipts = channel
                .receipts
                .checked_add(1)
                .ok_or("publication receipt count exhausted")?;
            channel.frontier = Some(next);
        }
        self.validate()
    }
}
fn validate_evidence(required: &Required) -> CaptureResult<()> {
    if let Some(e) = &required.evidence
        && (e.request
            != crate::agent_publication::request_id(
                required.range.repository,
                required.range.batch,
            )
            || e.publication
                != crate::agent_publication::publication_id(
                    required.range.repository,
                    required.range.batch,
                )
            || e.receipt_digest == [0; 32])
    {
        return Err("invalid publication receipt identity".into());
    }
    Ok(())
}
