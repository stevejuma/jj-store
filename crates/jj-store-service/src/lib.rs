//! Shared request authorization and application transaction services.
#![forbid(unsafe_code)]
pub mod access;
pub mod administration;
pub mod auth;
pub mod directory;
pub mod events;
pub mod freshness;
pub mod guard;
pub mod local_browser;
pub mod local_repository;
pub mod native;
mod native_agent;
pub mod native_inventory;
pub mod registry;
pub mod replication_control;

pub mod transport;

pub mod code_indexes;
pub mod index_tasks;
pub mod kind_tasks;
pub mod local_index_tasks;
pub mod pagination;
pub mod reads;
pub mod review_mutations;

pub mod collab_mutations;
/// Owned domain contracts reexported for typed application transports.
pub use jj_collab as collab_model;
pub use jj_review as review_model;
pub mod collab_pages;
pub mod collab_reads;
pub mod collaboration;
pub mod commands;
pub mod mutations;

pub mod uploads;

pub mod adapter;

pub mod work;

pub mod git_bridge;

pub mod agent_execution;
mod agent_feedback;
pub mod capture_management;
pub mod local_management;

pub mod storage_backend;
pub mod storage_management;
pub mod sync_management;

pub mod search_management;
pub mod server_settings;

pub mod content_search;

mod thread_directory;

pub mod working_copy;
pub mod working_tree;

pub mod conversion;

pub mod stack_workspace;

pub mod agent_administration;
pub mod connections;
pub mod qualified_review;
pub mod remote_agents;

mod conversation_discovery;

pub mod runtime_policy;
pub mod runtime_reload;

pub mod workflow;

pub mod workflow_host;

pub mod read_indexes;

/// Forge contracts and provider-independent external effects.
pub use jj_forge as forge;
