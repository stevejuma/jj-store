//! Browser-specific explicit metadata pins and provider-independent summaries.
use super::{LocalRepository, LocalWorkSnapshot, MetadataCommit};
use crate::work::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct LocalWorkBrowsePage {
    pub version: u32,
    pub snapshot: LocalWorkSnapshot,
    pub projects: Vec<WorkProjectSummary>,
    pub tasks: Vec<WorkTaskSummary>,
    pub total_matches: usize,
    pub next_cursor: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    version: u32,
    commit: MetadataCommit,
    query: WorkBrowseQuery,
    offset: usize,
}
impl LocalRepository {
    pub fn work_browse_at(
        &self,
        commit: MetadataCommit,
        query: &WorkBrowseQuery,
    ) -> Result<LocalWorkBrowsePage> {
        let selected = self.selected_metadata(commit)?;
        let mut identity = query.clone();
        identity.cursor = None;
        let offset = if let Some(text) = &query.cursor {
            if text.len() > 4096 {
                return Err(Error::InvalidCursor);
            }
            let bytes = URL_SAFE_NO_PAD
                .decode(text)
                .map_err(|_| Error::InvalidCursor)?;
            let c: Cursor = serde_json::from_slice(&bytes).map_err(|_| Error::InvalidCursor)?;
            if c.version != 1
                || c.commit != commit
                || c.query != identity
                || c.offset > 4096
                || serde_json::to_vec(&c).map_err(|_| Error::InvalidCursor)? != bytes
            {
                return Err(Error::InvalidCursor);
            }
            c.offset
        } else {
            0
        };
        let rows = browse_rows(&self.work_state(Some(&selected))?, query, offset)?;
        Ok(LocalWorkBrowsePage {
            version: 2,
            snapshot: Self::work_snapshot(Some(&selected)),
            projects: rows.projects,
            tasks: rows.tasks,
            total_matches: rows.total_matches,
            next_cursor: if rows.more {
                Some(
                    URL_SAFE_NO_PAD.encode(
                        serde_json::to_vec(&Cursor {
                            version: 1,
                            commit,
                            query: identity,
                            offset: offset.checked_add(rows.count).ok_or(Error::Limit)?,
                        })
                        .map_err(|_| Error::Corrupt)?,
                    ),
                )
            } else {
                None
            },
        })
    }
}
