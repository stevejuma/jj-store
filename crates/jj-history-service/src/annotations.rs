//! Annotate through the same retained read lifetime and native worker owner.
use crate::{Context, Handle, History, Result, frames::BlameQuery, service::Output};
use crate::{Error, blame};

impl History {
    pub async fn blame(
        &self,
        caller: &Context,
        vault: &str,
        handle: Handle,
        query: BlameQuery,
    ) -> Result<blame::Page> {
        let selection = self.selected(caller, vault, handle).await?;
        let prepared = {
            let entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            match &entries.get(&handle).ok_or(Error::Denied)?.output {
                Output::Ready(prepared) => prepared.clone(),
                Output::Preparing => return Err(Error::Busy),
                Output::Failed(error) => return Err(error.clone()),
            }
        };
        let key = blame::Selection {
            scope: handle.0.to_string(),
            repository: selection.access.repository(),
            operation: prepared.snapshot().operation,
            commit: query.commit.to_string(),
            path: query.path,
        };
        self.executor
            .blame_code_index(
                selection.access.clone(),
                prepared,
                self.blame.clone(),
                key,
                query.continuation,
            )
            .await
    }
}
