use super::*;
use jj_lib::{
    backend::TreeValue, merge::Merge, merged_tree::MergedTree,
    merged_tree_builder::MergedTreeBuilder, repo_path::RepoPathBuf,
};
use std::collections::{BTreeMap, BTreeSet};
pub(super) type Entries = BTreeMap<String, TreeValue>;
pub(super) fn entries(tree: &MergedTree, exclusions: &[String]) -> Result<Entries> {
    let mut values = Entries::new();
    let mut bytes = 0usize;
    for (path, value) in tree.entries() {
        let path = path.as_internal_file_string();
        if scan::excluded(path, exclusions) {
            continue;
        }
        restore_io::checked(path)?;
        let value = value.map_err(jj)?;
        let value = value.as_resolved().ok_or_else(|| {
            Error::Jj("Resolve checkpoint conflicts before restoring the vault".into())
        })?;
        if let Some(value) = value {
            if !matches!(value, TreeValue::File { .. }) {
                return Err(Error::Jj(
                    "Whole-vault restore currently requires regular files".into(),
                ));
            }
            bytes += path.len();
            if values.len() >= 100_000 || bytes > 8 * 1024 * 1024 {
                return Err(Error::Binding);
            }
            values.insert(path.to_owned(), value.clone());
        }
    }
    Ok(values)
}
pub(super) fn changed(before: &Entries, after: &Entries) -> Vec<String> {
    before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|path| before.get(path) != after.get(path))
        .collect()
}
pub(super) fn identity(tree: &MergedTree) -> Result<String> {
    tree.tree_ids()
        .as_resolved()
        .map(|id| id.hex())
        .ok_or(Error::Binding)
}
pub(super) async fn tree(session: &Session, entries: &Entries) -> Result<MergedTree> {
    let mut builder = MergedTreeBuilder::new(session.repo.store().empty_merged_tree());
    for (path, value) in entries {
        builder.set_or_remove(
            RepoPathBuf::from_internal_string(path.clone()).map_err(jj)?,
            Merge::resolved(Some(value.clone())),
        );
    }
    builder.write_tree().await.map_err(jj)
}
