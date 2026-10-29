//! Compose recorded WC bytes with the complete sparse/ignored physical projection.
use super::*;
use jj_store_jj::local_browser::{self as code, working_tree as baseline};
use std::sync::Arc;
impl Observer {
    pub async fn status(
        &self,
        access: Arc<dyn code::Access>,
        root: std::path::PathBuf,
        operation: jj_store_model::OperationId,
    ) -> Result<baseline::Status> {
        let observed = self
            .observe(access.clone(), root.clone(), operation)
            .await?;
        let mut entries: BTreeMap<_, _> = observed
            .current
            .baseline
            .entries
            .iter()
            .map(|e| (e.path.clone(), e.clone()))
            .collect();
        for change in &observed.projection.changes {
            match &change.after {
                None => {
                    entries.remove(&change.path);
                }
                Some(entry) => {
                    entries.insert(
                        change.path.clone(),
                        baseline::Entry {
                            path: entry.path.clone(),
                            kind: match entry.kind {
                                Kind::File { executable } => baseline::Kind::File { executable },
                                Kind::Symlink => baseline::Kind::Symlink,
                            },
                            bytes: entry.bytes,
                            content: entry.content,
                            data: entry.data.clone(),
                        },
                    );
                }
            }
        }
        let result = self
            .reader
            .working_tree_status(
                access.clone(),
                observed.current.baseline.clone(),
                entries.into_values().collect(),
            )
            .await
            .map_err(observer::query_error)?;
        // Counts may be expensive: re-observe everything before displaying them.
        let after = self.observe(access.clone(), root, operation).await?;
        if observed != after {
            return Err(Error::Stale);
        }
        access.authorize().await.map_err(observer::query_error)?;
        Ok(result)
    }
}
