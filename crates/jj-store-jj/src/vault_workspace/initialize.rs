use super::*;
use jj_lib::{
    git_backend::GitBackend,
    ref_name::WorkspaceName,
    signing::Signer,
    workspace_store::{SimpleWorkspaceStore, WorkspaceStore as _},
};
use std::io::Write as _;

/// Stage initialization outside the vault. A complete binding is installed last;
/// interrupted, private staging can be discarded without touching user files.
pub(super) async fn ensure(binding: &Binding, settings: &UserSettings) -> Result<()> {
    if binding.state.join("vault.json").exists() {
        return Ok(());
    }
    if binding.state.exists() {
        return Err(Error::Binding);
    }
    let staged = tempfile::Builder::new()
        .prefix(".lapis-vault-init-")
        .tempdir_in(if binding.state == binding.root.join(".jj") {
            binding::private(&binding.root, "vault-history-staging")?
        } else {
            binding.state.parent().ok_or(Error::Binding)?.to_owned()
        })?;
    let metadata = staged.path().join("repo");
    fs::create_dir(&metadata)?;
    let repo = ReadonlyRepo::init(
        settings,
        &metadata,
        &|settings, path| {
            Ok(Box::new(GitBackend::init_internal(
                settings,
                path,
                gix::hash::Kind::Sha1,
            )?))
        },
        Signer::from_settings(settings).map_err(jj)?,
        ReadonlyRepo::default_op_store_initializer(),
        ReadonlyRepo::default_op_heads_store_initializer(),
        ReadonlyRepo::default_index_store_initializer(),
        ReadonlyRepo::default_submodule_store_initializer(),
    )
    .await
    .map_err(jj)?;
    let name = WorkspaceName::DEFAULT.to_owned();
    let mut tx = repo.start_transaction();
    tx.repo_mut()
        .check_out(name.clone(), &repo.store().root_commit())
        .await
        .map_err(jj)?;
    let repo = tx
        .commit("initialize vault backup workspace")
        .await
        .map_err(jj)?;
    let wc = staged.path().join("working_copy");
    fs::create_dir(&wc)?;
    LocalWorkingCopy::init(
        repo.store().clone(),
        binding.root.clone(),
        wc.clone(),
        repo.op_id().clone(),
        name.clone(),
        settings,
    )
    .map_err(jj)?;
    fs::write(wc.join("type"), "local")?;
    SimpleWorkspaceStore::load(&metadata)
        .map_err(jj)?
        .add(&name, &binding.root)
        .map_err(jj)?;
    let mut file = File::create(staged.path().join("vault.json"))?;
    file.write_all(
        &serde_json::to_vec(&StoredBinding {
            version: 1,
            binding: binding.clone(),
            root_identity: root_identity(&binding.root)?,
            previous_states: vec![],
            relocations: vec![],
        })
        .map_err(jj)?,
    )?;
    file.sync_all()?;
    // The command lease serializes this publication for the native binding.
    fs::rename(staged.path(), &binding.state)?;
    File::open(binding.state.parent().ok_or(Error::Binding)?)?.sync_all()?;
    Ok(())
}
