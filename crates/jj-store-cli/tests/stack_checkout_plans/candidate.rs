use super::*;
use jj_lib::{
    backend as b, config::StackedConfig, default_backend_factories::default_backend_factories,
    object_id::ObjectId as _, ref_name::WorkspaceNameBuf, repo::Repo as _, repo_path::RepoPathBuf,
    settings::UserSettings,
};
use jj_store_jj::local_evidence::{OperationArchiveV1, capture_operation_archive};
pub type FileChanges<'a> = Vec<(&'a str, Option<(&'a [u8], bool)>)>;
pub async fn candidate(f: &Fixture, changes: FileChanges<'_>) -> OperationArchiveV1 {
    candidate_with_symlink(f, changes, None).await
}
async fn candidate_with_symlink(
    f: &Fixture,
    changes: FileChanges<'_>,
    link: Option<(&str, &str)>,
) -> OperationArchiveV1 {
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let loader = jj_lib::repo::RepoLoader::init_from_file_system(
        &settings,
        &f.root().join(".jj/repo"),
        &default_backend_factories(),
    )
    .unwrap();
    let state = f.witness().await;
    let op = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(
            state.baseline.operation.as_bytes(),
        ))
        .await
        .unwrap();
    let repo = loader.load_at(&op).await.unwrap();
    let source = repo
        .store()
        .get_commit(&b::CommitId::from_bytes(state.baseline.commit.as_bytes()))
        .unwrap();
    let mut tree = jj_lib::tree_builder::TreeBuilder::new(
        repo.store().clone(),
        source.tree_ids().as_resolved().unwrap().clone(),
    );
    for (path, value) in changes {
        let path = RepoPathBuf::from_internal_string(path).unwrap();
        if let Some((bytes, executable)) = value {
            let id = repo
                .store()
                .write_file(&path, &mut futures::io::Cursor::new(bytes))
                .await
                .unwrap();
            tree.set(
                path,
                b::TreeValue::File {
                    id,
                    executable,
                    copy_id: b::CopyId::placeholder(),
                },
            );
        } else {
            tree.remove(path);
        }
    }
    if let Some((path, target)) = link {
        let path = RepoPathBuf::from_internal_string(path).unwrap();
        let id = repo.store().write_symlink(&path, target).await.unwrap();
        tree.set(path, b::TreeValue::Symlink(id));
    }
    let tree = tree.write_tree().await.unwrap();
    let mut tx = repo.start_transaction();
    let commit = tx
        .repo_mut()
        .rewrite_commit(&source)
        .set_tree(jj_lib::merged_tree::MergedTree::resolved(
            repo.store().clone(),
            tree,
        ))
        .write()
        .await
        .unwrap();
    tx.repo_mut().rebase_descendants().await.unwrap();
    tx.repo_mut()
        .set_wc_commit(WorkspaceNameBuf::from("default"), commit.id().clone())
        .unwrap();
    let repo = tx
        .write("Unpublished tree checkout fixture")
        .await
        .unwrap()
        .leave_unpublished();
    capture_operation_archive(
        &settings,
        &f.root().join(".jj/repo"),
        state.baseline.repository,
        m::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap(),
        Default::default(),
    )
    .await
    .unwrap()
}
