use super::{LocalAgentSnapshot, LocalError, LocalRepository, LocalResult};
use crate::domain::{AgentCatalogKey, AgentCatalogKind, AgentObjectRef, AgentObjectV1};
use crate::metadata::GitOid;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalAgentObjectRow {
    pub object: AgentObjectRef,
    pub value: AgentObjectV1,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalAgentObjectPage {
    pub version: u32,
    pub snapshot: LocalAgentSnapshot,
    pub catalog: AgentCatalogKind,
    pub objects: Vec<LocalAgentObjectRow>,
    pub next_cursor: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    version: u32,
    commit: GitOid,
    catalog: AgentCatalogKind,
    after: AgentCatalogKey,
    limit: usize,
}
impl LocalRepository {
    pub async fn agent_provenance_page(
        &self,
        catalog: AgentCatalogKind,
        limit: usize,
        cursor: Option<&str>,
    ) -> LocalResult<LocalAgentObjectPage> {
        if !(1..=256).contains(&limit)
            || !matches!(
                catalog,
                AgentCatalogKind::Associations | AgentCatalogKind::Checkpoints
            )
        {
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
            let c: Cursor = serde_json::from_slice(&bytes).map_err(|_| LocalError::Invalid)?;
            if c.version != 2
                || c.catalog != catalog
                || c.limit != limit
                || serde_json::to_vec(&c).map_err(|_| LocalError::Invalid)? != bytes
            {
                return Err(LocalError::Invalid);
            }
            let current = current.ok_or(LocalError::Stale)?;
            let selected = self
                .metadata
                .history(&current)?
                .into_iter()
                .find(|s| s.commit() == c.commit)
                .ok_or(LocalError::Stale)?;
            (Some(selected), Some(c.after))
        } else {
            (current, None)
        };
        let store = self
            .agent_store_at(
                selected.as_ref(),
                jj_store_model::RepositoryId::new([0; 16]),
            )
            .await?;
        let page = store.reads().list_catalog(catalog, after, limit).await?;
        let mut objects = Vec::with_capacity(page.selections.len());
        let mut bytes = 1024;
        for (_, object) in page.selections {
            let expected = if catalog == AgentCatalogKind::Associations {
                crate::domain::AgentObjectKind::Association
            } else {
                crate::domain::AgentObjectKind::CheckpointVersion
            };
            if object.kind != expected {
                return Err(LocalError::Corrupt);
            }
            let value = store.object(object).await?;
            let row = LocalAgentObjectRow { object, value };
            bytes += serde_json::to_vec(&row)
                .map_err(|_| LocalError::Corrupt)?
                .len();
            if bytes > 4 * 1024 * 1024 {
                return Err(LocalError::Limit);
            }
            objects.push(row);
        }
        let next_cursor = page
            .next_after
            .map(|after| {
                Ok::<_, LocalError>(
                    URL_SAFE_NO_PAD.encode(
                        serde_json::to_vec(&Cursor {
                            version: 2,
                            commit: selected.as_ref().ok_or(LocalError::Corrupt)?.commit(),
                            catalog,
                            after,
                            limit,
                        })
                        .map_err(|_| LocalError::Corrupt)?,
                    ),
                )
            })
            .transpose()?;
        Ok(LocalAgentObjectPage {
            version: 2,
            snapshot: store.snapshot,
            catalog,
            objects,
            next_cursor,
        })
    }
}
