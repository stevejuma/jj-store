use super::*;
use jj_lib::{op_store::OpStore as _, simple_op_store::SimpleOpStore};
impl OperationArchiveV1 {
    /// Proves upstream operation-store reconstruction in a private disposable
    /// cache, without creating source metadata, an index or operation heads.
    pub async fn verify_materialization(&self) -> Result<()> {
        self.validate()?;
        let temp = tempfile::tempdir()?;
        let store = SimpleOpStore::init(
            temp.path(),
            o::RootOperationData {
                root_commit_id: git_native_commit_id(m::GitSha1Id::new([0; 20])),
            },
        )
        .map_err(Error::other)?;
        for (id, view) in &self.views {
            let native = o::View::try_from(view.clone())?;
            if *id != m::ViewId::new([0; 64])
                && view_id(&store.write_view(&native).await.map_err(Error::other)?)? != *id
            {
                return Err(Error::Invalid("materialized view identity"));
            }
            if store
                .read_view(&native_view_id(*id))
                .await
                .map_err(Error::other)?
                != native
            {
                return Err(Error::Invalid("materialized view fields"));
            }
        }
        for (id, op) in &self.operations {
            let native = o::Operation::try_from(op.clone())?;
            if *id != m::OperationId::new([0; 64])
                && operation_id(&store.write_operation(&native).await.map_err(Error::other)?)?
                    != *id
            {
                return Err(Error::Invalid("materialized operation identity"));
            }
            if store
                .read_operation(&native_operation_id(*id))
                .await
                .map_err(Error::other)?
                != native
            {
                return Err(Error::Invalid("materialized operation fields"));
            }
        }
        Ok(())
    }
}
