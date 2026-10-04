//! Compact directory facts; closure admission belongs to the selected provider.
use super::review_threads::{ReviewSelection, verify_thread_key};
use super::*;
use jj_store_wal::publication::{ReviewCatalogKey, ReviewObjectRef, ReviewObjectV1};
mod events;

impl ReviewThreadPager {
    pub async fn activity(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<ReviewActivitySummary> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(self.cursors.timeout(), async {
            let pin = self.pin(&session).await?;
            let page = pin.store.list_threads(None, limit).await.map_err(map_wal)?;
            let activity = resolve(
                &ReviewSelection {
                    repository: pin.snapshot.repo_id,
                    store: &pin.store,
                },
                page.selections,
                page.next_after.is_some(),
            )
            .await?;
            authorize_epoch(&session, pin.snapshot.epoch).await?;
            Ok(ReviewActivitySummary {
                version: 1,
                snapshot: pin.snapshot,
                activity,
            })
        })
        .await
    }
}

pub(crate) async fn resolve(
    pin: &ReviewSelection<'_>,
    selections: Vec<(ReviewCatalogKey, ReviewObjectRef)>,
    threads_partial: bool,
) -> Result<ReviewActivity> {
    if pin.store.repository() != pin.repository || selections.len() > MAX_PAGE {
        return Err(Error::Corrupt);
    }
    let mut activity = ReviewActivity {
        threads: selections.len(),
        threads_partial,
        updated_at_millis: None,
    };
    for (key, reference) in selections {
        let latest = match pin.store.read_object(reference).await.map_err(map_wal)? {
            ReviewObjectV1::ThreadSnapshot(value) => {
                verify_thread_key(key, value.thread_id)?;
                events::pages(
                    pin,
                    value.thread_id,
                    value.event_page_root,
                    value.last_event_sequence,
                )
                .await?
            }
            ReviewObjectV1::ThreadMerge(value) => {
                verify_thread_key(key, value.thread_id)?;
                events::graph(
                    pin,
                    value.thread_id,
                    value.event_graph_root,
                    value.thread_version,
                )
                .await?
            }
            _ => return Err(Error::Corrupt),
        };
        activity.updated_at_millis = Some(
            activity
                .updated_at_millis
                .map_or(latest, |time| time.max(latest)),
        );
    }
    Ok(activity)
}

#[cfg(test)]
mod tests;
