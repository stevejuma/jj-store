//! Shared request authorization and application transaction services.
#![forbid(unsafe_code)]
pub mod access;
pub mod administration;
pub mod auth;
pub mod events;
pub mod guard;
pub mod local_repository;
pub mod native;
mod native_agent;
pub mod registry;
pub mod replication_control;

pub mod transport;

pub mod index_tasks;
pub mod pagination;
pub mod reads;
pub mod review_mutations;

pub mod collaboration;
pub mod commands;
pub mod mutations;

pub mod uploads;

pub mod adapter;

pub mod work;

pub mod git_bridge;

pub mod local_management;

pub mod sync_management;
