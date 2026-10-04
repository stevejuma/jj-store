//! Owned integration boundary for pinned jj-lib 0.45.1.
#![forbid(unsafe_code)]

pub mod backend;
pub mod codec;
mod copy_query;
pub mod dependencies;
pub mod factories;
pub mod git_bridge;
pub mod local_browser;
pub mod local_evidence;
pub mod local_metadata;
pub mod local_repository;
pub mod native_dto;
pub mod objects;
pub mod operation_stores;
pub mod policy;
pub mod query;
pub mod replication;
pub mod roots;

mod runtime;

pub mod commands;
pub mod conversion;
