//! Custom JJ command and authenticated remote native workspace integration.
#![forbid(unsafe_code)]
pub mod agent_capture;
mod agent_capture_files;
mod agent_capture_intent;
pub mod agent_commands;
pub mod agent_hook;
pub mod agent_integrations;
mod agent_probe;
mod agent_provenance_journal;
pub mod agent_publication;
mod agent_removal;
pub mod agent_sanitization;
pub mod agent_setup;
mod agent_setup_files;
mod agent_setup_templates;
pub mod attachment;
pub mod binding;
pub mod commands;
pub mod credentials;
pub mod git_bridge;
pub mod initialization;
pub mod local_agent_publication;
pub mod migration;
pub mod presentation;
pub mod remote;
mod replication_endpoint;
pub mod review_commands;
mod review_presentation;
pub mod server_commands;
mod store_commands;
pub mod sync_commands;

pub mod token_commands;
mod token_journal;

mod local_diagnostics;
pub mod work_commands;

pub mod workspace_management;
