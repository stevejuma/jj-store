use super::*;
use gix::objs::Write as _;
use jj_store_wal::publication::{GitObjectKind, GitObjectV1};
use tokio::io::AsyncReadExt as _;

fn kind(value: gix::objs::Kind) -> GitObjectKind {
    match value {
        gix::objs::Kind::Blob => GitObjectKind::Blob,
        gix::objs::Kind::Tree => GitObjectKind::Tree,
        gix::objs::Kind::Commit => GitObjectKind::Commit,
        gix::objs::Kind::Tag => GitObjectKind::Tag,
    }
}
fn git_kind(value: GitObjectKind) -> gix::objs::Kind {
    match value {
        GitObjectKind::Blob => gix::objs::Kind::Blob,
        GitObjectKind::Tree => gix::objs::Kind::Tree,
        GitObjectKind::Commit => gix::objs::Kind::Commit,
        GitObjectKind::Tag => gix::objs::Kind::Tag,
    }
}
impl GitConverter {
    pub async fn retain_object(&mut self, oid: &str) -> Result<()> {
        self.capture(oid.to_owned(), 0).await
    }
    pub(super) fn capture(&mut self, oid: String, depth: usize) -> BoxFuture<'_, Result<()>> {
        async move {
            if depth >= self.limits.depth {
                return Err("Git object depth limit".into());
            }
            if self
                .records
                .values()
                .any(|r| matches!(r, GitRecordV1::Object(v) if v.oid == oid))
            {
                return Ok(());
            }
            let (data, object_kind, dependencies) = self.read_raw(&oid)?;
            for dependency in &dependencies {
                self.capture(dependency.clone(), depth + 1).await?;
            }
            let (body, status) = self.native.put_file(data.as_slice()).await?;
            acknowledged(status)?;
            self.record(GitRecordV1::Object(GitObjectV1 {
                oid,
                kind: object_kind,
                body,
                bytes: data.len() as u64,
                dependencies,
            }))
        }
        .boxed()
    }
    fn read_raw(&mut self, oid: &str) -> Result<(Vec<u8>, GitObjectKind, BTreeSet<String>)> {
        if !jj_store_wal::publication::valid_git_oid(oid) {
            return Err("unsupported Git OID".into());
        }
        let repo = self.backend.git_repo();
        let id = gix::ObjectId::from_hex(oid.as_bytes())?;
        let size = repo.find_header(id)?.size();
        if size > self.limits.object_bytes
            || self
                .bytes
                .checked_add(size)
                .is_none_or(|total| total > self.limits.bytes)
        {
            return Err("Git object byte limit".into());
        }
        let object = repo.find_object(id)?;
        if gix::objs::compute_hash(gix::hash::Kind::Sha1, object.kind, &object.data)? != id {
            return Err("Git object hash mismatch".into());
        }
        self.bytes += size;
        let mut dependencies = BTreeSet::new();
        match object.kind {
            gix::objs::Kind::Blob => {}
            gix::objs::Kind::Tree => {
                for entry in
                    gix::objs::TreeRef::from_bytes(&object.data, gix::hash::Kind::Sha1)?.entries
                {
                    if !entry.mode.is_commit() {
                        dependencies.insert(entry.oid.to_string());
                    }
                }
            }
            gix::objs::Kind::Commit => {
                let commit = object.try_to_commit_ref()?;
                // Pinned JJ uses lossy string decoding and defaults invalid times.
                // Refuse those inputs before conversion; retain exact valid bytes.
                std::str::from_utf8(commit.message)?;
                if commit
                    .encoding
                    .is_some_and(|encoding| !encoding.eq_ignore_ascii_case(b"utf-8"))
                {
                    return Err("unsupported Git commit encoding".into());
                }
                for signature in [commit.author()?, commit.committer()?] {
                    std::str::from_utf8(signature.name)?;
                    std::str::from_utf8(signature.email)?;
                    signature.time()?;
                }
                for (name, value) in &commit.extra_headers {
                    if *name == b"encoding" && !value.eq_ignore_ascii_case(b"utf-8") {
                        return Err("unsupported Git commit encoding".into());
                    }
                    if *name == b"jj:trees" || *name == b"jj:conflict-labels" {
                        return Err("Git conflict header unsupported".into());
                    }
                }
                dependencies.insert(commit.tree().to_string());
                dependencies.extend(commit.parents().map(|id| id.to_string()));
            }
            gix::objs::Kind::Tag => {
                dependencies.insert(object.try_to_tag_ref()?.target().to_string());
            }
        }
        Ok((object.data.clone(), kind(object.kind), dependencies))
    }
    pub(super) fn restore(&mut self, oid: String, depth: usize) -> BoxFuture<'_, Result<()>> {
        async move {
            if depth >= self.limits.depth {
                return Err("Git restore depth limit".into());
            }
            if self.restored.contains(&oid) {
                return Ok(());
            }
            let descriptor = self
                .records
                .values()
                .find_map(|r| match r {
                    GitRecordV1::Object(v) if v.oid == oid => Some(v.clone()),
                    _ => None,
                })
                .ok_or("missing exact Git representation")?;
            if descriptor.bytes > self.limits.object_bytes {
                return Err("Git object byte limit".into());
            }
            for dependency in &descriptor.dependencies {
                self.restore(dependency.clone(), depth + 1).await?;
            }
            self.bytes = self
                .bytes
                .checked_add(descriptor.bytes)
                .ok_or("Git restore byte limit")?;
            if self.bytes > self.limits.bytes {
                return Err("Git restore byte limit".into());
            }
            let spool = self.native.get_file(descriptor.body).await?;
            if spool.len() != descriptor.bytes {
                return Err("Git body length mismatch".into());
            }
            let mut data = Vec::new();
            spool.reader()?.read_to_end(&mut data).await?;
            let object_kind = git_kind(descriptor.kind);
            let id = gix::objs::compute_hash(gix::hash::Kind::Sha1, object_kind, &data)?;
            if id != gix::ObjectId::from_hex(oid.as_bytes())? {
                return Err("Git representation hash mismatch".into());
            }
            let repo = self.backend.git_repo();
            if repo.objects.write_buf(object_kind, &data)? != id {
                return Err("Git cache reconstruction identity".into());
            }
            self.restored.insert(oid);
            Ok(())
        }
        .boxed()
    }
}
