//! Exact Git tree reads behind a caller's selected capsule inventory.
use super::*;
use crate::evidence::EvidenceCapsule;
use jj_store_model as m;

impl MetadataStore {
    /// Immutable reducer capability. Public facades must first prove the capsule
    /// is in their selected domain inventory. This never consults a checkout,
    /// current operation, mutable ref, or JJ extra-metadata database.
    pub(crate) fn read_capsule_file(
        &self,
        capsule: &EvidenceCapsule,
        commit: m::GitSha1Id,
        path: &str,
    ) -> Result<(m::GitSha1Id, Vec<u8>)> {
        if !safe_path(path) || path.split('/').count() > 64 {
            return Err(Error::Invalid("retained file path"));
        }
        let saved = capsule.commits.get(&commit).ok_or(Error::Stale)?;
        let [root] = saved.root_tree.as_slice() else {
            return Err(Error::Invalid("conflicted retained tree"));
        };
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        let bytes = objects::object(
            &repo,
            commit.to_string().parse()?,
            gix::objs::Kind::Commit,
            self.limits.object_bytes,
        )?;
        let physical = gix::objs::CommitRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
            .map_err(Error::other)?;
        if physical
            .extra_headers()
            .find(JJ_TREES_COMMIT_HEADER)
            .is_some()
            || physical.tree() != root.to_string()
        {
            return Err(Error::Invalid(
                "capsule tree does not match retained commit",
            ));
        }
        let mut tree: GitOid = root.to_string().parse()?;
        let mut components = path.split('/').peekable();
        while let Some(component) = components.next() {
            let links = objects::tree_links(&repo, tree, self.limits.object_bytes)?;
            let (id, kind) = links.get(component).copied().ok_or(Error::Stale)?;
            if components.peek().is_some() {
                if kind != gix::objs::tree::EntryKind::Tree {
                    return Err(Error::Invalid("retained path is not a directory"));
                }
                tree = id;
            } else {
                if !matches!(
                    kind,
                    gix::objs::tree::EntryKind::Blob | gix::objs::tree::EntryKind::BlobExecutable
                ) {
                    return Err(Error::Invalid("retained path is not a regular file"));
                }
                let bytes = objects::object(&repo, id, gix::objs::Kind::Blob, 256 * 1024)?;
                return Ok((id.to_string().parse().map_err(Error::other)?, bytes));
            }
        }
        Err(Error::Invalid("empty retained path"))
    }
}
