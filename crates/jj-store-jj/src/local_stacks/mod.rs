//! Ordinary local JJ megamerges. App settings are local; JJ owns code and checkout.
mod basic;
mod cleanup;
mod config;
mod conflicts;
mod content;
mod destructive;
mod graph;
mod history;
pub use conflicts::{Conflict, conflict};
mod hunks;
mod inspection;
mod mutations;
mod operation_description;
mod policy;
mod preview;
mod read;
pub use read::ReadSession;
mod settings;
mod state;
use self::config::{Config, SavedWorkspace};
use crate::local_browser::Access;
pub use hunks::{Hunk, HunkRequest, HunkResponse, hunks};
pub use inspection::{CommitEligibility, Inspection, InspectionRequest, inspect};
use jj_lib::{
    commit::Commit,
    default_backend_factories::{default_backend_factories, default_working_copy_factories},
    matchers::NothingMatcher,
    object_id::ObjectId as _,
    op_store::RefTarget,
    repo::{ReadonlyRepo, Repo as _},
    rewrite,
    working_copy::{SnapshotOptions, WorkingCopyFreshness},
    workspace::Workspace,
};
pub use preview::{Preview, preview};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use state::state;
mod topology;
mod topology_query;
use std::{path::PathBuf, sync::Arc};
pub use topology::{Head as TopologyHead, Lane as TopologyLane, Topology};
pub use topology_query::{TopologyRequest, TopologyResponse, read_topology};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The workspace changed. Refresh before trying again.")]
    Stale,
    #[error("Invalid local workspace settings or branch selection.")]
    Invalid,
    #[error("A workspace command is already running.")]
    Busy,
    #[error("Local workspace access is unavailable.")]
    Denied,
    #[error("Local JJ workspace: {0}")]
    Jj(String),
}
type Result<T> = std::result::Result<T, Error>;
fn jj(error: impl std::fmt::Display) -> Error {
    Error::Jj(error.to_string())
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "LocalStacksCommand")]
pub enum Command {
    Refresh,
    Reconcile,
    Undo,
    Redo,
    DeleteBranch {
        branch: String,
    },
    Abandon {
        commit: String,
    },
    Discard {
        selections: Vec<Selection>,
    },
    Resolve {
        path: String,
        content: String,
    },
    Move {
        commit: String,
        target: String,
        placement: Placement,
    },
    Squash {
        commit: String,
        target: String,
    },
    Uncommit {
        commit: String,
    },
    Rebase {
        branch: String,
    },
    SquashBranch {
        branch: String,
    },
    Setup {
        name: String,
        target: String,
    },
    Create {
        name: String,
    },
    Switch {
        workspace: String,
    },
    Apply {
        branch: String,
    },
    Unapply {
        branch: String,
    },
    UnapplyBranches {
        branches: Vec<String>,
    },
    SetTarget {
        branch: String,
    },
    Update {
        workspace: String,
        name: String,
        target: String,
    },
    Delete {
        workspace: String,
    },
    Transfer {
        target: String,
        selections: Vec<Selection>,
    },
    Split {
        commit: String,
        message: String,
        selections: Vec<Selection>,
    },
    Edit {
        commit: String,
        message: String,
        selections: Vec<Selection>,
    },
    CreateStack {
        name: String,
        parent: Option<String>,
    },
    CreateBranch {
        commit: String,
        name: String,
        placement: Placement,
    },
    RenameBranch {
        branch: String,
        name: String,
    },
    Reword {
        commit: String,
        message: String,
    },
    Empty {
        commit: String,
        placement: Placement,
        message: String,
    },
    Commit {
        branch: String,
        message: String,
        selections: Vec<Selection>,
    },
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "LocalStacksPlacement")]
pub enum Placement {
    Above,
    Below,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalStacksSelection")]
pub struct Selection {
    pub source: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hunks: Option<Vec<String>>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalStacksRequest")]
pub struct Request {
    pub version: u32,
    pub binding_generation: u64,
    pub operation: Option<jj_store_model::OperationId>,
    pub revision: Option<u64>,
    pub command: Command,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksWorkspace")]
pub struct NamedWorkspace {
    pub id: String,
    pub name: String,
    pub target: String,
    pub target_commit: String,
    pub applied: Vec<String>,
    pub merge_commit: String,
    pub wip_commit: String,
    pub conflicted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    #[serde(default)]
    pub membership_changed: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "LocalStacksState")]
pub struct State {
    pub version: u32,
    pub operation: jj_store_model::OperationId,
    pub revision: u64,
    pub active: Option<String>,
    pub workspaces: Vec<NamedWorkspace>,
    /// Publication succeeded but settings/checkout need attention. Never retry the command.
    pub warning: Option<String>,
    #[serde(default)]
    pub can_undo: bool,
    #[serde(default)]
    pub can_redo: bool,
    #[serde(default)]
    pub conflicts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<Topology>,
}

/// Runs under HTTP admission, JJ's WC lock and a local settings lock; never shells out.
pub async fn execute(access: Arc<dyn Access>, root: PathBuf, request: Request) -> Result<State> {
    if request.version != 1 || access.generation() != request.binding_generation {
        return Err(Error::Stale);
    }
    access.authorize().await.map_err(|_| Error::Denied)?;
    let settings = settings::load(&root)?;
    let mut workspace = Workspace::load(
        &settings,
        &root,
        &default_backend_factories(),
        &default_working_copy_factories(),
    )
    .map_err(jj)?;
    if workspace.workspace_root() != root
        || workspace.repo_path().canonicalize().map_err(jj)?
            != access.metadata_path().canonicalize().map_err(jj)?
    {
        return Err(Error::Denied);
    }
    let storage = config::Storage::open(&root)?;
    let mut config = storage.read()?;
    let mut repo = workspace.repo_loader().load_at_head().await.map_err(jj)?;
    let refresh = matches!(request.command, Command::Refresh);
    if !refresh
        && (request.operation.as_ref().map(|v| v.to_string()) != Some(repo.op_id().hex())
            || request.revision != Some(config.revision))
    {
        return Err(Error::Stale);
    }
    let name = workspace.workspace_name().to_owned();
    let current = repo
        .view()
        .get_wc_commit_id(&name)
        .ok_or(Error::Invalid)?
        .clone();
    let mut wc = repo.store().get_commit(&current).map_err(jj)?;
    let ignores = settings::ignores(&repo, &root)?;
    let matcher = settings::tracking(&settings)?;
    let max_size = settings::max_file_size(&settings)?;
    let mut locked = workspace.start_working_copy_mutation().await.map_err(jj)?;
    if WorkingCopyFreshness::check_stale(locked.locked_wc(), &wc, &repo)
        .await
        .map_err(jj)?
        != WorkingCopyFreshness::Fresh
    {
        return Err(Error::Stale);
    }
    let mut detached = false;
    // An external edit/undo must not silently attach a named workspace to another change.
    if !matches!(request.command, Command::Undo | Command::Redo)
        && let Some(active) = config.active()
        && !resolve_any(repo.as_ref(), &active.wip_ref)
            .is_ok_and(|w| w.change_id() == wc.change_id())
    {
        if !refresh {
            return Err(Error::Stale);
        }
        config.active = None;
        config.revision += 1;
        storage.save(&config)?;
        detached = true;
    }
    let (tree, snapshot_stats) = locked
        .locked_wc()
        .snapshot(&SnapshotOptions {
            base_ignores: ignores,
            progress: None,
            start_tracking_matcher: matcher.as_ref(),
            force_tracking_matcher: &NothingMatcher,
            max_new_file_size: max_size,
        })
        .await
        .map_err(jj)?;
    let snapshot_warning = (!snapshot_stats.untracked_paths.is_empty()).then(|| {
        "Some new files were not snapshotted; check JJ auto-track rules and file-size limits."
            .to_string()
    });
    if tree.tree_ids_and_labels() != wc.tree().tree_ids_and_labels() {
        let mut tx = repo.start_transaction();
        tx.set_is_snapshot(true);
        wc = tx
            .repo_mut()
            .rewrite_commit(&wc)
            .set_tree(tree)
            .write()
            .await
            .map_err(jj)?;
        tx.repo_mut()
            .set_wc_commit(name.clone(), wc.id().clone())
            .map_err(jj)?;
        tx.repo_mut().rebase_descendants().await.map_err(jj)?;
        repo = tx
            .commit("snapshot working copy for Stacks")
            .await
            .map_err(jj)?;
    }
    // Commands refer to the shown operation. A new physical edit requires a new preview.
    if !refresh && request.operation.as_ref().map(|v| v.to_string()) != Some(repo.op_id().hex()) {
        locked.finish(repo.op_id().clone()).await.map_err(jj)?;
        return Err(Error::Stale);
    }
    access.authorize().await.map_err(|_| Error::Denied)?;
    let mut warning = snapshot_warning;
    if detached {
        warning = Some("The CLI selected another working change. Your named workspaces are retained; choose one to return to it.".into());
    }
    if !refresh {
        let mut tx = repo.start_transaction();
        let mut description = operation_description::describe(&request.command, &config);
        match request.command {
            Command::Setup {
                ref name,
                ref target,
            } => {
                if !config.workspaces.is_empty() {
                    return Err(Error::Invalid);
                }
                config.namespace = settings.get_rng().new_change_id(16).hex();
                let saved = config.new_workspace(name, target, &settings)?;
                build(tx.repo_mut(), &repo, &saved, Some(&wc), true).await?;
                config.active = Some(saved.id.clone());
                config.workspaces.push(saved);
            }
            Command::Create { ref name } => {
                let target = config
                    .active()
                    .or_else(|| config.workspaces.first())
                    .ok_or(Error::Invalid)?
                    .target
                    .clone();
                let saved = config.new_workspace(name, &target, &settings)?;
                build(tx.repo_mut(), &repo, &saved, None, false).await?;
                config.active = Some(saved.id.clone());
                config.workspaces.push(saved);
            }
            Command::Switch { ref workspace } => {
                let saved = config
                    .workspaces
                    .iter()
                    .find(|w| &w.id == workspace)
                    .ok_or(Error::Invalid)?;
                let target = resolve_any(repo.as_ref(), &saved.wip_ref)?;
                tx.repo_mut()
                    .set_wc_commit(name.clone(), target.id().clone())
                    .map_err(jj)?;
                config.active = Some(saved.id.clone());
            }
            Command::Apply { ref branch } | Command::Unapply { ref branch } => {
                if matches!(request.command, Command::Apply { .. }) {
                    resolve(repo.as_ref(), branch)?;
                }
                let saved = config.active_mut().ok_or(Error::Invalid)?;
                if matches!(request.command, Command::Apply { .. }) {
                    if !saved.applied.iter().any(|b| b == branch) {
                        saved.applied.push(branch.clone());
                    }
                } else {
                    saved.applied.retain(|b| b != branch);
                }
                let rebuilt = build(tx.repo_mut(), &repo, saved, Some(&wc), false).await?;
                tx.repo_mut()
                    .set_wc_commit(name.clone(), rebuilt.id().clone())
                    .map_err(jj)?;
            }
            Command::UnapplyBranches { ref branches } => {
                topology::validate_unapply(&config, branches)?;
                let saved = config.active_mut().ok_or(Error::Invalid)?;
                saved.applied.retain(|b| !branches.contains(b));
                let rebuilt = build(tx.repo_mut(), &repo, saved, Some(&wc), false).await?;
                tx.repo_mut()
                    .set_wc_commit(name.clone(), rebuilt.id().clone())
                    .map_err(jj)?;
            }
            Command::SetTarget { ref branch } => {
                resolve(repo.as_ref(), branch)?;
                let saved = config.active_mut().ok_or(Error::Invalid)?;
                saved.target = branch.clone();
                let rebuilt = build(tx.repo_mut(), &repo, saved, Some(&wc), false).await?;
                tx.repo_mut()
                    .set_wc_commit(name.clone(), rebuilt.id().clone())
                    .map_err(jj)?;
            }
            Command::Update {
                ref workspace,
                name: ref new_name,
                ref target,
            } => {
                if config.active.as_ref() != Some(workspace) {
                    return Err(Error::Invalid);
                }
                config.validate_name(new_name, Some(workspace))?;
                resolve(repo.as_ref(), target)?;
                let saved = config.active_mut().ok_or(Error::Invalid)?;
                saved.name = new_name.trim().into();
                if saved.target != *target {
                    saved.target = target.clone();
                    let rebuilt = build(tx.repo_mut(), &repo, saved, Some(&wc), false).await?;
                    tx.repo_mut()
                        .set_wc_commit(name.clone(), rebuilt.id().clone())
                        .map_err(jj)?;
                }
            }
            Command::Delete { ref workspace } => {
                if !config.workspaces.iter().any(|saved| &saved.id == workspace) {
                    return Err(Error::Invalid);
                }
                config.workspaces.retain(|saved| &saved.id != workspace);
                if config.active.as_ref() == Some(workspace) {
                    config.active = None;
                }
                // Keep WC, user and retention bookmarks unchanged. JJ owns unfinished work.
            }
            Command::Commit {
                ref branch,
                ref message,
                ref selections,
            } => {
                mutations::commit(
                    tx.repo_mut(),
                    &repo,
                    &name,
                    &settings,
                    &config,
                    &wc,
                    branch,
                    message,
                    selections,
                )
                .await?;
            }
            Command::CreateStack { .. }
            | Command::CreateBranch { .. }
            | Command::RenameBranch { .. }
            | Command::Reword { .. }
            | Command::Empty { .. } => {
                basic::execute(
                    tx.repo_mut(),
                    &repo,
                    &name,
                    &settings,
                    &mut config,
                    &wc,
                    &request.command,
                )
                .await?;
            }
            Command::Transfer { .. } | Command::Split { .. } | Command::Edit { .. } => {
                content::execute(
                    tx.repo_mut(),
                    &repo,
                    &name,
                    &settings,
                    &config,
                    &request.command,
                )
                .await?;
            }
            Command::Move { .. }
            | Command::Squash { .. }
            | Command::Uncommit { .. }
            | Command::Rebase { .. }
            | Command::SquashBranch { .. } => {
                graph::execute(
                    tx.repo_mut(),
                    &repo,
                    &name,
                    &settings,
                    &config,
                    &request.command,
                )
                .await?;
            }
            Command::DeleteBranch { .. } | Command::Abandon { .. } | Command::Discard { .. } => {
                destructive::execute(
                    tx.repo_mut(),
                    &repo,
                    &name,
                    &settings,
                    &mut config,
                    &wc,
                    &request.command,
                )
                .await?
            }
            Command::Resolve {
                ref path,
                ref content,
            } => {
                conflicts::execute(
                    tx.repo_mut(),
                    &repo,
                    &name,
                    &settings,
                    &config,
                    &wc,
                    path,
                    content,
                )
                .await?
            }
            Command::Reconcile => {
                let saved = config.active().ok_or(Error::Invalid)?;
                for obsolete in cleanup::candidates(&repo, saved).await? {
                    tx.repo_mut().record_abandoned_commit(&obsolete);
                }
                let wip = build(tx.repo_mut(), &repo, saved, Some(&wc), false).await?;
                tx.repo_mut()
                    .set_wc_commit(name.clone(), wip.id().clone())
                    .map_err(jj)?;
            }
            Command::Undo | Command::Redo => {
                description = history::restore(
                    tx.repo_mut(),
                    &repo,
                    matches!(request.command, Command::Redo),
                )
                .await?;
            }
            Command::Refresh => unreachable!(),
        }
        // Setup and Create select their freshly retained WIP.
        if matches!(
            request.command,
            Command::Setup { .. } | Command::Create { .. }
        ) {
            let wip = tx
                .repo()
                .view()
                .get_local_bookmark(config.active().ok_or(Error::Invalid)?.wip_ref.as_ref())
                .as_normal()
                .ok_or(Error::Invalid)?
                .clone();
            tx.repo_mut().set_wc_commit(name.clone(), wip).map_err(jj)?;
        }
        tx.repo_mut().rebase_descendants().await.map_err(jj)?;
        repo = tx.commit(description).await.map_err(jj)?;
        let mut checkout_failed = false;
        let new_wc = repo
            .store()
            .get_commit(repo.view().get_wc_commit_id(&name).ok_or(Error::Invalid)?)
            .map_err(jj)?;
        if let Err(error) = locked.locked_wc().check_out(&new_wc).await {
            checkout_failed = true;
            warning = Some(format!(
                "JJ operation {} applied; checkout needs attention: {error}. Inspect JJ state before another command.",
                repo.op_id().hex()
            ));
        }
        if !matches!(request.command, Command::Undo | Command::Redo) {
            config.revision += 1;
            if let Err(error) = storage.save(&config) {
                warning = Some(format!(
                    "JJ operation {} applied; local settings could not be saved: {error}. Do not repeat the command.",
                    repo.op_id().hex()
                ));
            }
        }
        if checkout_failed {
            return state(&repo, &config, &name, warning).await;
        }
    }
    if let Err(error) = locked.finish(repo.op_id().clone()).await {
        warning = Some(format!(
            "JJ operation {} applied; working-copy bookkeeping needs attention: {error}. Inspect JJ before another command.",
            repo.op_id().hex()
        ));
    }
    state(&repo, &config, &name, warning).await
}

fn resolve(repo: &dyn jj_lib::repo::Repo, reference: &str) -> Result<Commit> {
    if reference.starts_with(config::INTERNAL_PREFIX) {
        return Err(Error::Invalid);
    }
    resolve_any(repo, reference)
}
fn resolve_any(repo: &dyn jj_lib::repo::Repo, reference: &str) -> Result<Commit> {
    let target = if let Some((branch, remote)) = reference.rsplit_once('@') {
        &repo
            .view()
            .get_remote_bookmark(jj_lib::ref_name::RemoteRefSymbol {
                name: branch.as_ref(),
                remote: remote.as_ref(),
            })
            .target
    } else {
        repo.view().get_local_bookmark(reference.as_ref())
    };
    repo.store()
        .get_commit(target.as_normal().ok_or(Error::Invalid)?)
        .map_err(jj)
}
fn parents(repo: &dyn jj_lib::repo::Repo, saved: &SavedWorkspace) -> Result<Vec<Commit>> {
    let references = std::iter::once(&saved.target).chain(&saved.applied);
    let mut parents = Vec::new();
    for reference in references {
        let commit = resolve(repo, reference)?;
        if !parents.iter().any(|c: &Commit| c.id() == commit.id()) {
            parents.push(commit);
        }
    }
    Ok(parents)
}
fn needs_rebuild(repo: &dyn jj_lib::repo::Repo, saved: &SavedWorkspace) -> Result<bool> {
    let merge = resolve_any(repo, &saved.merge_ref)?;
    Ok(merge.parent_ids()
        != parents(repo, saved)?
            .iter()
            .map(|c| c.id().clone())
            .collect::<Vec<_>>())
}
async fn build(
    mut_repo: &mut jj_lib::repo::MutableRepo,
    _repo: &ReadonlyRepo,
    saved: &SavedWorkspace,
    old_wip: Option<&Commit>,
    first: bool,
) -> Result<Commit> {
    let parents = parents(mut_repo, saved)?;
    let tree = rewrite::merge_commit_trees(mut_repo, &parents)
        .await
        .map_err(jj)?;
    let parent_ids = parents.iter().map(|c| c.id().clone()).collect();
    let previous = if old_wip.is_some() && !first {
        Some(resolve_any(mut_repo, &saved.merge_ref)?)
    } else {
        None
    };
    let builder = if let Some(previous) = &previous {
        mut_repo
            .rewrite_commit(previous)
            .set_parents(parent_ids)
            .set_tree(tree.clone())
    } else {
        mut_repo.new_commit(parent_ids, tree.clone())
    };
    let merge = builder
        .set_description(format!("Workspace {} merge", saved.name))
        .write()
        .await
        .map_err(jj)?;
    let wip = if let Some(old) = old_wip {
        if first {
            mut_repo
                .new_commit(vec![merge.id().clone()], old.tree())
                .write()
                .await
                .map_err(jj)?
        } else {
            rewrite::rebase_commit(mut_repo, old.clone(), vec![merge.id().clone()])
                .await
                .map_err(jj)?
        }
    } else {
        mut_repo
            .new_commit(vec![merge.id().clone()], tree)
            .write()
            .await
            .map_err(jj)?
    };
    mut_repo.set_local_bookmark_target(
        saved.merge_ref.as_ref(),
        RefTarget::normal(merge.id().clone()),
    );
    mut_repo.set_local_bookmark_target(saved.wip_ref.as_ref(), RefTarget::normal(wip.id().clone()));
    Ok(wip)
}

/// Inspection-only target selection; does not create config or change a checkout.
pub fn configured_target(root: &std::path::Path) -> Result<Option<String>> {
    let config = config::Storage::read_only(root)?;
    Ok(config.active().map(|w| w.target.clone()))
}
