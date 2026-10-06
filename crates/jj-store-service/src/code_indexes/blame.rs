use super::*;
impl Indexes {
    fn blame_start_allowed(&self, key: &Key) -> q::Result<()> {
        // A failed preparation is terminal for this observation. Do not let
        // ordinary progress polling repeatedly launch the same failed task.
        self.path_start_allowed(key).map_err(|error| match error {
            q::Error::IndexRebuildRequired | q::Error::Busy => q::Error::Unavailable,
            error => error,
        })
    }
    pub async fn native_blame(
        self: &Arc<Self>,
        read: Arc<CommittedRead>,
        commit: m::CommitId,
        path: String,
        continuation: Option<String>,
    ) -> q::Result<q::blame::Page> {
        let source = read.code_source().await?;
        let key = Key {
            binding: Binding::Native {
                repository: source.access.repository(),
                epoch: source.access.epoch(),
            },
            operation: source.access.operation(),
        };
        let prepared = match self.ready(&key) {
            Ok(ready) => ready.native().ok_or(q::Error::Corrupt)?,
            Err(q::Error::IndexRebuildRequired) => {
                self.blame_start_allowed(&key)?;
                self.start_native(read).await?;
                return Err(q::Error::IndexRebuildRequired);
            }
            Err(e) => return Err(e),
        };
        let selection = q::blame::Selection {
            scope: format!("{:?}", key.binding),
            repository: source.access.repository(),
            operation: key.operation,
            commit: commit.to_string(),
            path,
        };
        self.graph
            .blame_code_index(
                source.store,
                read,
                prepared,
                self.blame.clone(),
                selection,
                continuation,
            )
            .await
    }
    pub async fn local_blame(
        self: &Arc<Self>,
        access: Arc<dyn local::Access>,
        operation: m::OperationId,
        commit: m::GitSha1Id,
        path: String,
        continuation: Option<String>,
    ) -> q::Result<q::blame::Page> {
        Self::authorize_local(access.as_ref()).await?;
        let key = Key {
            binding: Binding::Local {
                repository: access.repository(),
                generation: access.generation(),
            },
            operation,
        };
        let prepared = match self.ready(&key) {
            Ok(ready) => ready.local().ok_or(q::Error::Corrupt)?,
            Err(q::Error::IndexRebuildRequired) => {
                self.blame_start_allowed(&key)?;
                self.start_local(access, operation).await?;
                return Err(q::Error::IndexRebuildRequired);
            }
            Err(e) => return Err(e),
        };
        let selection = q::blame::Selection {
            scope: format!("{:?}", key.binding),
            repository: access.repository(),
            operation,
            commit: commit.to_string(),
            path,
        };
        self.local
            .blame_code_index(
                access,
                prepared,
                self.blame.clone(),
                selection,
                continuation,
            )
            .await
    }
}
