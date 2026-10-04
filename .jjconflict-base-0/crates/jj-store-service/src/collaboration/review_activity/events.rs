//! Activity needs every accepted time, but no event presentation or causal sort.
use super::*;
use jj_store_wal::publication::ReviewThreadId;
use std::collections::BTreeSet;

const MAX_EVENTS: u64 = 4096;
fn expected_count(count: u64) -> Result<()> {
    if count == 0 {
        return Err(Error::Corrupt);
    }
    if count > MAX_EVENTS {
        return Err(Error::Limit);
    }
    Ok(())
}
pub(super) async fn pages(
    pin: &ReviewSelection<'_>,
    thread: ReviewThreadId,
    root: ReviewObjectRef,
    expected: u64,
) -> Result<i64> {
    expected_count(expected)?;
    let mut next = Some(root);
    let mut visited = BTreeSet::new();
    let mut count = 0;
    let mut latest = None;
    while let Some(reference) = next {
        if !visited.insert(reference) {
            return Err(Error::Corrupt);
        }
        if visited.len() as u64 > MAX_EVENTS {
            return Err(Error::Limit);
        }
        let ReviewObjectV1::EventPage(page) =
            pin.store.read_object(reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if page.thread_id != thread || page.entries.is_empty() {
            return Err(Error::Corrupt);
        }
        for selection in page.entries {
            count += 1;
            if count > MAX_EVENTS {
                return Err(Error::Limit);
            }
            if selection.sequence != count {
                return Err(Error::Corrupt);
            }
            let ReviewObjectV1::ThreadEvent(event) = pin
                .store
                .read_object(selection.event)
                .await
                .map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if event.thread_id != thread || event.sequence != count {
                return Err(Error::Corrupt);
            }
            latest = Some(latest.map_or(event.accepted_at_millis, |time: i64| {
                time.max(event.accepted_at_millis)
            }));
        }
        next = page.next;
    }
    if count != expected {
        return Err(Error::Corrupt);
    }
    latest.ok_or(Error::Corrupt)
}
pub(super) async fn graph(
    pin: &ReviewSelection<'_>,
    thread: ReviewThreadId,
    root: ReviewObjectRef,
    expected: u64,
) -> Result<i64> {
    expected_count(expected)?;
    let ReviewObjectV1::EventGraph(graph) = pin.store.read_object(root).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    if graph.thread_id != thread || graph.nodes.len() as u64 != expected {
        return Err(Error::Corrupt);
    }
    // The provider admits the complete canonical graph and its dependencies.
    // The maximum is independent of causal order and does not need a sort.
    let mut latest = None;
    for node in graph.nodes {
        let ReviewObjectV1::ThreadEvent(event) =
            pin.store.read_object(node.event).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if event.thread_id != thread {
            return Err(Error::Corrupt);
        }
        latest = Some(latest.map_or(event.accepted_at_millis, |time: i64| {
            time.max(event.accepted_at_millis)
        }));
    }
    latest.ok_or(Error::Corrupt)
}
