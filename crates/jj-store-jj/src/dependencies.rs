//! Exhaustive native graph edges, including evolution-only and removed terms.
use crate::codec::{CodecError, NativeCodec, NativeFile, NativeSymlink};
use jj_lib::object_id::ObjectId as _;
use jj_lib::{backend as b, op_store as o};
use jj_store_model::{self as m, ObjectKey};
use std::collections::BTreeSet;

pub trait NativeDependencies: NativeCodec {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError>;
}
#[derive(Default)]
struct Edges(BTreeSet<ObjectKey>);
macro_rules! edge_method {
    ($name:ident,$kind:ident,$id:ident,$native:ty) => {
        fn $name(&mut self, id: &$native) -> Result<(), CodecError> {
            self.0
                .insert(ObjectKey::$kind(m::$id::from_slice(id.as_bytes())?));
            Ok(())
        }
    };
}
impl Edges {
    edge_method!(commit, Commit, CommitId, b::CommitId);
    edge_method!(tree, Tree, TreeId, b::TreeId);
    edge_method!(file, File, FileId, b::FileId);
    edge_method!(symlink, Symlink, SymlinkId, b::SymlinkId);
    edge_method!(view, View, ViewId, o::ViewId);
    edge_method!(operation, Operation, OperationId, o::OperationId);
    fn copy(&mut self, id: &b::CopyId) -> Result<(), CodecError> {
        if !id.as_bytes().is_empty() {
            self.0
                .insert(ObjectKey::Copy(m::CopyId::from_slice(id.as_bytes())?));
        }
        Ok(())
    }
    fn target(&mut self, target: &o::RefTarget) -> Result<(), CodecError> {
        for id in target.iter().flatten() {
            self.commit(id)?;
        }
        Ok(())
    }
}
impl NativeDependencies for b::Commit {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        let mut edges = Edges::default();
        for id in self.parents.iter().chain(&self.predecessors) {
            edges.commit(id)?;
        }
        for id in &self.root_tree {
            edges.tree(id)?;
        }
        Ok(edges.0)
    }
}
impl NativeDependencies for b::Tree {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        let mut edges = Edges::default();
        for entry in self.entries() {
            match entry.value() {
                b::TreeValue::File { id, copy_id, .. } => {
                    edges.file(id)?;
                    edges.copy(copy_id)?;
                }
                b::TreeValue::Symlink(id) => edges.symlink(id)?,
                b::TreeValue::Tree(id) => edges.tree(id)?,
                b::TreeValue::GitSubmodule(_) => {}
            }
        }
        Ok(edges.0)
    }
}
impl NativeDependencies for b::CopyHistory {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        let mut edges = Edges::default();
        for id in &self.parents {
            edges.copy(id)?;
        }
        Ok(edges.0)
    }
}
impl NativeDependencies for o::View {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        let mut edges = Edges::default();
        for id in self.head_ids.iter().chain(self.wc_commit_ids.values()) {
            edges.commit(id)?;
        }
        for target in self
            .local_bookmarks
            .values()
            .chain(self.local_tags.values())
            .chain(self.git_refs.values())
            .chain(self.git_heads.values())
        {
            edges.target(target)?;
        }
        for remote in self.remote_views.values() {
            for remote_ref in remote.bookmarks.values().chain(remote.tags.values()) {
                edges.target(&remote_ref.target)?;
            }
        }
        Ok(edges.0)
    }
}
impl NativeDependencies for o::Operation {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        let mut edges = Edges::default();
        edges.view(&self.view_id)?;
        for id in &self.parents {
            edges.operation(id)?;
        }
        if let Some(predecessors) = &self.commit_predecessors {
            for (id, parents) in predecessors {
                edges.commit(id)?;
                for parent in parents {
                    edges.commit(parent)?;
                }
            }
        }
        Ok(edges.0)
    }
}
impl NativeDependencies for NativeFile {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        Ok(BTreeSet::new())
    }
}
impl NativeDependencies for NativeSymlink {
    fn dependencies(&self) -> Result<BTreeSet<ObjectKey>, CodecError> {
        Ok(BTreeSet::new())
    }
}
