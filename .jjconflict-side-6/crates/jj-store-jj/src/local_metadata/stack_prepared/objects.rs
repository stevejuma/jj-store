use super::super::objects as git;
use super::*;
use gix::objs::Write as _;
impl MetadataStore {
    pub(super) fn stack_candidate(
        &self,
        repo: &gix::Repository,
        record: &Record,
    ) -> Result<GitOid> {
        let bytes = serde_json::to_vec(record).map_err(Error::other)?;
        if bytes.len() > self.limits.root_bytes {
            return Err(Error::Limit);
        }
        let record_blob = git::blob(repo, &bytes)?;
        let retained_tree = git::tree(repo, git::retained_links(&record.retained))?;
        let tree = git::tree(
            repo,
            BTreeMap::from([
                (
                    "record".into(),
                    (record_blob, gix::objs::tree::EntryKind::Blob),
                ),
                (
                    "capsule".into(),
                    (record.blob, gix::objs::tree::EntryKind::Blob),
                ),
                (
                    "retained".into(),
                    (retained_tree, gix::objs::tree::EntryKind::Tree),
                ),
            ]),
        )?;
        let mut bytes = format!("tree {tree}\n");
        for parent in git::parents(None, &record.retained) {
            bytes.push_str(&format!("parent {parent}\n"));
        }
        bytes.push_str("author jj-store <retention@local.invalid> 0 +0000\ncommitter jj-store <retention@local.invalid> 0 +0000\n\nLocal stack prepared retention v1\n");
        if bytes.len() > self.limits.object_bytes {
            return Err(Error::Limit);
        }
        let commit = GitOid::from_git(
            repo.objects
                .write_buf(gix::objs::Kind::Commit, bytes.as_bytes())
                .map_err(Error::other)?,
        )?;
        git::sync_object(repo, commit)?;
        Ok(commit)
    }
    pub(super) fn stack_archive(
        &self,
        repo: &gix::Repository,
        commit: GitOid,
        key: &StackPreparationKey,
        sync: bool,
    ) -> Result<StackRetention> {
        let bytes = git::object(
            repo,
            commit,
            gix::objs::Kind::Commit,
            self.limits.object_bytes,
        )?;
        let decoded = gix::objs::CommitRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
            .map_err(Error::other)?;
        let links = git::tree_links(
            repo,
            GitOid::from_git(decoded.tree())?,
            self.limits.root_bytes,
        )?;
        if links.len() != 3 {
            return Err(Error::Invalid("stack retention root"));
        }
        let (record_blob, gix::objs::tree::EntryKind::Blob) = links
            .get("record")
            .copied()
            .ok_or(Error::Invalid("stack retention record"))?
        else {
            return Err(Error::Invalid("stack retention record kind"));
        };
        let bytes = git::object(
            repo,
            record_blob,
            gix::objs::Kind::Blob,
            self.limits.root_bytes,
        )?;
        let record: Record = serde_json::from_slice(&bytes).map_err(Error::other)?;
        if serde_json::to_vec(&record).map_err(Error::other)? != bytes || record.version != 1 {
            return Err(Error::Invalid("stack retention format"));
        }
        record.key.validate()?;
        if record.key != *key {
            return Err(Error::IntentConflict);
        }
        if record.retained.len() > self.limits.objects {
            return Err(Error::Limit);
        }
        let capsule = GitEvidenceCapsuleV1::decode(
            record.capsule,
            &git::object(
                repo,
                record.blob,
                gix::objs::Kind::Blob,
                self.limits.object_bytes,
            )?,
        )?;
        if capsule.repository != key.repository
            || capsule.operation_id != record.operation
            || capsule.view_id != record.view
            || links.get("capsule") != Some(&(record.blob, gix::objs::tree::EntryKind::Blob))
        {
            return Err(Error::Invalid("stack retention capsule binding"));
        }
        let mut expected = capsule.retention(record.blob)?;
        expected.extend(git::audit(repo, &expected, self.limits, sync)?);
        if record.retained != expected {
            return Err(Error::Invalid("stack retention incomplete closure"));
        }
        let (retained_tree, gix::objs::tree::EntryKind::Tree) = links
            .get("retained")
            .copied()
            .ok_or(Error::Invalid("stack retention links"))?
        else {
            return Err(Error::Invalid("stack retention links kind"));
        };
        if git::tree_links(repo, retained_tree, self.limits.root_bytes)?
            != git::retained_links(&expected)
        {
            return Err(Error::Invalid("stack retention tree links"));
        }
        let parents = decoded
            .parents()
            .map(GitOid::from_git)
            .collect::<Result<Vec<_>>>()?;
        if parents != git::parents(None, &expected) {
            return Err(Error::Invalid("stack retention parents"));
        }
        if sync {
            git::sync_object(repo, commit)?;
            git::sync_object(repo, record_blob)?;
            git::sync_object(repo, retained_tree)?;
            git::sync_object(repo, GitOid::from_git(decoded.tree())?)?;
        }
        Ok(StackRetention {
            commit,
            key: key.clone(),
            capsule,
        })
    }
}
