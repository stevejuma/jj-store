//! Virtual roots for the pinned native profile. These identities are reserved;
//! writing equal content as an ordinary object still uses its content hash.
use jj_lib::content_hash::blake2b_hash;
use jj_lib::{backend as b, op_store as o};
use jj_store_model as m;

pub const ROOT_COMMIT: m::CommitId = m::CommitId::new([0; 64]);
pub const ROOT_CHANGE: m::ChangeId = m::ChangeId::new([0; 16]);
pub const ROOT_VIEW: m::ViewId = m::ViewId::new([0; 64]);
pub const ROOT_OPERATION: m::OperationId = m::OperationId::new([0; 64]);
pub fn empty_tree_id() -> b::TreeId {
    b::TreeId::new(blake2b_hash(&b::Tree::default()).to_vec())
}
pub fn commit() -> b::Commit {
    b::make_root_commit(
        b::ChangeId::from_bytes(ROOT_CHANGE.as_bytes()),
        empty_tree_id(),
    )
}
pub fn view() -> o::View {
    o::View::make_root(b::CommitId::from_bytes(ROOT_COMMIT.as_bytes()))
}
pub fn operation() -> o::Operation {
    o::Operation::make_root(o::ViewId::from_bytes(ROOT_VIEW.as_bytes()))
}
