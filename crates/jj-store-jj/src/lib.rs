//! Owned integration boundary for pinned jj-lib 0.45.1.
#![forbid(unsafe_code)]

pub mod backend;
pub mod codec;
mod copy_query;
pub mod dependencies;
pub mod factories;
pub mod objects;
pub mod operation_stores;
pub mod policy;
pub mod roots;

mod runtime;
