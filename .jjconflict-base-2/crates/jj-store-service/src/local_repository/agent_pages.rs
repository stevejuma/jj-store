use super::{LocalAgentSnapshot, LocalError, LocalRepository, LocalResult};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jj_store_jj::local_metadata::GitOid;
use jj_store_wal::publication::{
    AgentCatalogKey, AgentObjectRef, AgentObjectV1, AgentSessionSnapshotV1,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalAgentSessionRow {
    pub object: AgentObjectRef,
    pub session: AgentSessionSnapshotV1,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalAgentPage {
    pub version: u32,
    pub snapshot: LocalAgentSnapshot,
    pub sessions: Vec<LocalAgentSessionRow>,
    pub next_cursor: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    version: u32,
    commit: GitOid,
    after: AgentCatalogKey,
    limit: usize,
}
impl LocalRepository {
    pub async fn agent_sessions(
        &self,
        limit: usize,
        cursor: Option<&str>,
    ) -> LocalResult<LocalAgentPage> {
        if !(1..=256).contains(&limit) {
            return Err(LocalError::Invalid);
        }
        let current = self.metadata.read()?;
        let (selected, after) = if let Some(cursor) = cursor {
            if cursor.len() > 2048 {
                return Err(LocalError::Invalid);
            }
            let bytes = URL_SAFE_NO_PAD
                .decode(cursor)
                .map_err(|_| LocalError::Invalid)?;
            let cursor: Cursor = serde_json::from_slice(&bytes).map_err(|_| LocalError::Invalid)?;
            if cursor.version != 1
                || cursor.limit != limit
                || serde_json::to_vec(&cursor).map_err(|_| LocalError::Invalid)? != bytes
            {
                return Err(LocalError::Invalid);
            }
            let current = current.ok_or(LocalError::Stale)?;
            let selected = self
                .metadata
                .history(&current)?
                .into_iter()
                .find(|s| s.commit() == cursor.commit)
                .ok_or(LocalError::Stale)?;
            (Some(selected), Some(cursor.after))
        } else {
            (current, None)
        };
        let store = self
            .agent_store_at(
                selected.as_ref(),
                jj_store_model::RepositoryId::new([0; 16]),
            )
            .await?;
        let page = store.reads().list_sessions(after, limit).await?;
        let mut sessions = Vec::with_capacity(page.selections.len());
        let mut bytes = 1024;
        for (_, object) in page.selections {
            let AgentObjectV1::SessionSnapshot(session) = store.object(object).await? else {
                return Err(LocalError::Corrupt);
            };
            let row = LocalAgentSessionRow { object, session };
            bytes += serde_json::to_vec(&row)
                .map_err(|_| LocalError::Corrupt)?
                .len();
            if bytes > 4 * 1024 * 1024 {
                return Err(LocalError::Limit);
            }
            sessions.push(row);
        }
        let next_cursor = page
            .next_after
            .map(|after| {
                let cursor = Cursor {
                    version: 1,
                    commit: selected.as_ref().ok_or(LocalError::Corrupt)?.commit(),
                    after,
                    limit,
                };
                Ok::<_, LocalError>(
                    URL_SAFE_NO_PAD
                        .encode(serde_json::to_vec(&cursor).map_err(|_| LocalError::Corrupt)?),
                )
            })
            .transpose()?;
        Ok(LocalAgentPage {
            version: 2,
            snapshot: store.snapshot,
            sessions,
            next_cursor,
        })
    }
}
