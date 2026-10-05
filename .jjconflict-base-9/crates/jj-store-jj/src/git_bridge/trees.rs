use super::*;
use tokio_util::compat::{FuturesAsyncReadCompatExt as _, TokioAsyncReadCompatExt as _};

impl GitConverter {
    pub(super) fn import_tree(
        &mut self,
        id: b::TreeId,
        depth: usize,
    ) -> BoxFuture<'_, Result<b::TreeId>> {
        async move {
            if depth >= self.limits.depth {
                return Err("Git tree depth limit".into());
            }
            if let Some(tree) = self.trees.get(&id.hex()) {
                return Ok(tree.clone());
            }
            let tree = self.backend.read_tree(RepoPath::root(), &id).await?;
            let mut entries = Vec::new();
            for entry in tree.entries() {
                let value = match entry.value() {
                    b::TreeValue::File {
                        id,
                        executable,
                        copy_id,
                    } => {
                        let reader = self.backend.read_file(RepoPath::root(), id).await?;
                        let (file, status) = self.native.put_file(reader.compat()).await?;
                        acknowledged(status)?;
                        b::TreeValue::File {
                            id: b::FileId::from_bytes(file.as_bytes()),
                            executable: *executable,
                            copy_id: copy_id.clone(),
                        }
                    }
                    b::TreeValue::Symlink(id) => {
                        let target = self.backend.read_symlink(RepoPath::root(), id).await?;
                        let (key, status) = self.native.put(&NativeSymlink(target)).await?;
                        acknowledged(status)?;
                        b::TreeValue::Symlink(b::SymlinkId::from_bytes(key.as_bytes()))
                    }
                    b::TreeValue::Tree(id) => {
                        b::TreeValue::Tree(self.import_tree(id.clone(), depth + 1).await?)
                    }
                    b::TreeValue::GitSubmodule(id) => b::TreeValue::GitSubmodule(id.clone()),
                };
                entries.push((entry.name().to_owned(), value));
            }
            let tree = b::Tree::from_sorted_entries(entries);
            let (key, status) = self.native.put(&tree).await?;
            acknowledged(status)?;
            let result = b::TreeId::from_bytes(key.as_bytes());
            self.trees.insert(id.hex(), result.clone());
            Ok(result)
        }
        .boxed()
    }
    pub(super) fn export_tree(
        &mut self,
        id: b::TreeId,
        depth: usize,
    ) -> BoxFuture<'_, Result<b::TreeId>> {
        async move {
            if depth >= self.limits.depth {
                return Err("native tree depth limit".into());
            }
            let key = format!("native:{}", id.hex());
            if let Some(tree) = self.trees.get(&key) {
                return Ok(tree.clone());
            }
            let tree = if id.as_bytes() == [0; 64] {
                b::Tree::default()
            } else {
                self.native
                    .get::<b::Tree>(m::ObjectKey::Tree(m::TreeId::from_slice(id.as_bytes())?))
                    .await?
            };
            let mut entries = Vec::new();
            for entry in tree.entries() {
                let value = match entry.value() {
                    b::TreeValue::File {
                        id,
                        executable,
                        copy_id,
                    } => {
                        let spool = self
                            .native
                            .get_file(m::FileId::from_slice(id.as_bytes())?)
                            .await?;
                        let id = self
                            .backend
                            .write_file(RepoPath::root(), &mut spool.reader()?.compat())
                            .await?;
                        b::TreeValue::File {
                            id,
                            executable: *executable,
                            copy_id: copy_id.clone(),
                        }
                    }
                    b::TreeValue::Symlink(id) => {
                        let target = self
                            .native
                            .get::<NativeSymlink>(m::ObjectKey::Symlink(m::SymlinkId::from_slice(
                                id.as_bytes(),
                            )?))
                            .await?;
                        b::TreeValue::Symlink(
                            self.backend
                                .write_symlink(RepoPath::root(), &target.0)
                                .await?,
                        )
                    }
                    b::TreeValue::Tree(id) => {
                        b::TreeValue::Tree(self.export_tree(id.clone(), depth + 1).await?)
                    }
                    b::TreeValue::GitSubmodule(id) if id.as_bytes().len() == 20 => {
                        b::TreeValue::GitSubmodule(id.clone())
                    }
                    b::TreeValue::GitSubmodule(_) => {
                        return Err("unsupported submodule object format".into());
                    }
                };
                entries.push((entry.name().to_owned(), value));
            }
            let result = self
                .backend
                .write_tree(RepoPath::root(), &b::Tree::from_sorted_entries(entries))
                .await?;
            self.trees.insert(key, result.clone());
            Ok(result)
        }
        .boxed()
    }
}
