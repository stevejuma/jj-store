//! Foreground server composition, with no command parser or upstream JJ imports.
#![forbid(unsafe_code)]
#[cfg(all(test, unix))]
extern crate self as jj_store_server;
pub mod agent_administration;
pub mod agent_execution;
pub mod capture_admission;
pub mod capture_authority;
pub mod capture_config;
pub mod capture_control_state;
pub mod capture_import_state;
mod capture_journal_files;
pub mod capture_management;
pub mod capture_runtime;
pub mod capture_source;
pub mod capture_supervisor;
#[cfg(all(test, unix))]
#[path = "../tests/capture_source/fixture.rs"]
#[allow(dead_code)]
mod capture_test_fixture;
pub mod capture_worker;
pub mod config;
pub mod connections;
pub mod directory;
pub mod installation;
mod listener;
pub mod local_repositories;
pub mod opencode_import;
pub mod opencode_read_api;
pub mod opencode_source;
pub mod policy;
pub mod process;
pub mod registry;
mod registry_catalog;
mod registry_persistence;
pub mod remote_agents;
mod replication;
pub mod search;
mod search_config;
pub mod server_settings;
pub mod stack_checkout_plans;
pub mod stack_combinations;
pub mod stack_command_journal;
pub mod stack_preparation;
pub mod storage;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid server configuration: {0}")]
    Config(&'static str),
    #[error("server filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("server service failed: {0}")]
    Service(&'static str),
    #[error("{0}")]
    CaptureCapacity(#[from] jj_store_capture::agent_capture::CaptureCapacityRefusal),
}
impl Error {
    pub(crate) fn capture_spool(
        error: Box<dyn std::error::Error + Send + Sync>,
        fallback: &'static str,
    ) -> Self {
        error
            .downcast_ref::<jj_store_capture::agent_capture::CaptureCapacityRefusal>()
            .copied()
            .map_or(Self::Service(fallback), Self::CaptureCapacity)
    }
    pub(crate) fn capacity_refusal(
        &self,
    ) -> Option<jj_store_capture::agent_capture::CaptureCapacityRefusal> {
        match self {
            Self::CaptureCapacity(refusal) => Some(*refusal),
            _ => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;

pub mod workspaces;

pub mod storage_management;
pub mod sync_management;

pub mod search_management;
mod search_worker;

mod presentation_avatar;

pub mod runtime_reload;
