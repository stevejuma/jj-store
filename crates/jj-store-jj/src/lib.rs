//! Owned integration boundary for pinned jj-lib 0.45.1.
#![forbid(unsafe_code)]

#[cfg(feature = "native")]
pub mod agent_workspaces;
#[cfg(feature = "native")]
pub mod backend;
#[cfg(feature = "native")]
pub mod codec;
#[cfg(feature = "native")]
mod copy_query;
#[cfg(feature = "native")]
pub mod dependencies;
#[cfg(feature = "native")]
pub mod factories;
#[cfg(feature = "native")]
pub mod git_bridge;
#[cfg(any(feature = "native", feature = "local-history"))]
mod line_diff;
#[cfg(any(feature = "native", feature = "local-history"))]
pub mod local_browser;
pub mod local_evidence;
#[cfg(feature = "native")]
pub mod local_execution;
pub mod local_metadata;
pub mod local_repository;
#[cfg(feature = "native")]
pub mod local_stacks;
pub mod native_dto;
#[cfg(feature = "native")]
pub mod objects;
#[cfg(feature = "native")]
pub mod operation_stores;
#[cfg(feature = "native")]
pub mod policy;
#[cfg(any(feature = "native", feature = "local-history"))]
pub mod query;
#[cfg(any(feature = "native", feature = "local-history"))]
mod read_support;
#[cfg(feature = "native")]
pub mod replication;
#[cfg(feature = "native")]
pub mod roots;
#[cfg(feature = "vault-backup")]
pub mod vault_workspace;

#[cfg(feature = "native")]
mod runtime;

#[cfg(feature = "native")]
pub mod commands;
#[cfg(feature = "native")]
pub mod conversion;
