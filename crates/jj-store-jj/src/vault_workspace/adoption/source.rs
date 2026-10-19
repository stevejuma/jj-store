use super::*;
use crate::local_repository::{WorkspaceInfo, inspect_workspace};
use jj_lib::{object_id::ObjectId as _, op_store::OperationId, working_copy::WorkingCopy as _};

pub(super) struct Original {
    pub history: ExistingHistory,
    pub git: PathBuf,
    info: WorkspaceInfo,
    witnesses: Vec<(PathBuf, String)>,
}
impl Original {
    pub fn verify(&self, binding: &Binding) -> Result<()> {
        if inspect_workspace(&binding.root).map_err(jj)? != self.info {
            return Err(Error::Binding);
        }
        for (path, identity) in &self.witnesses {
            if root_identity(path)? != *identity {
                return Err(Error::Binding);
            }
        }
        Ok(())
    }
}
pub(super) async fn load(
    binding: &Binding,
    expected: Option<&ExistingHistory>,
) -> Result<Original> {
    if expected
        .is_some_and(|value| !valid_id(&value.operation, 128) || !valid_id(&value.commit, 40))
    {
        return Err(Error::Binding);
    }
    binding.validate()?;
    let info = inspect_workspace(&binding.root).map_err(jj)?;
    if info.path != binding.root || info.metadata == binding.metadata() {
        return Err(Error::Binding);
    }
    let paths = [
        binding.root.join(".jj"),
        binding.root.join(".jj/working_copy"),
        info.metadata.clone(),
        info.git.clone(),
    ];
    let witnesses = paths
        .into_iter()
        .map(|path| root_identity(&path).map(|identity| (path, identity)))
        .collect::<Result<Vec<_>>>()?;
    let settings = settings()?;
    let loader =
        RepoLoader::init_from_file_system(&settings, &info.metadata, &default_backend_factories())
            .map_err(jj)?;
    let operation = if let Some(expected) = expected {
        OperationId::try_from_hex(&expected.operation).ok_or(Error::Binding)?
    } else {
        let heads = loader.op_heads_store().get_op_heads().await.map_err(jj)?;
        let [head] = heads.as_slice() else {
            return Err(Error::Jj(
                "Choose one existing repository operation before importing history".into(),
            ));
        };
        head.clone()
    };
    let operation = loader.load_operation(&operation).await.map_err(jj)?;
    let repo = loader.load_at(&operation).await.map_err(jj)?;
    let wc = LocalWorkingCopy::load(
        repo.store().clone(),
        binding.root.clone(),
        binding.root.join(".jj/working_copy"),
        &settings,
    )
    .map_err(jj)?;
    let commit = repo
        .view()
        .get_wc_commit_id(wc.workspace_name())
        .ok_or(Error::Binding)?;
    let history = ExistingHistory {
        operation: repo.op_id().hex(),
        commit: commit.hex(),
    };
    if expected.is_some_and(|value| *value != history) {
        return Err(Error::Binding);
    }
    let result = Original {
        history,
        git: info.git.clone(),
        info,
        witnesses,
    };
    result.verify(binding)?;
    Ok(result)
}
fn valid_id(value: &str, width: usize) -> bool {
    value.len() == width
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
