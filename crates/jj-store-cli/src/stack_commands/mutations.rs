use super::*;
pub(super) async fn position(
    ctx: &Context,
    value: &PositionArgs,
) -> AuthorityResult<(String, local_stacks::Placement)> {
    let (reference, placement) = match (&value.above, &value.below) {
        (Some(rev), None) => (rev, local_stacks::Placement::Above),
        (None, Some(rev)) => (rev, local_stacks::Placement::Below),
        _ => return Err(invalid("Supply exactly one of --above or --below")),
    };
    Ok((ctx.read.resolve_revision(reference).await?, placement))
}
pub(super) async fn command(
    ctx: &Context,
    args: &StackArgs,
    value: &StackCommand,
) -> AuthorityResult<Command> {
    use StackCommand as C;
    Ok(match value {
        C::Init { name, target } => Command::Setup {
            name: name.clone(),
            target: target.clone(),
        },
        C::Workspace { command } => match command {
            WorkspaceCommand::Create { name } => Command::Create { name: name.clone() },
            WorkspaceCommand::Switch { workspace } => Command::Switch {
                workspace: ctx.workspace(workspace)?,
            },
            WorkspaceCommand::Delete { workspace } => Command::Delete {
                workspace: ctx.workspace(workspace)?,
            },
            WorkspaceCommand::Update { name, target } => {
                let active = ctx.active()?;
                Command::Update {
                    workspace: active.id.clone(),
                    name: name.as_ref().unwrap_or(&active.name).clone(),
                    target: target.as_ref().unwrap_or(&active.target).clone(),
                }
            }
            WorkspaceCommand::List => return Err("Not a mutation".into()),
        },
        C::Apply { bookmark } => Command::Apply {
            branch: bookmark.clone(),
        },
        C::Unapply {
            bookmark,
            branch_only: true,
        } => Command::Unapply {
            branch: bookmark.clone(),
        },
        C::Unapply {
            bookmark,
            branch_only: false,
        } => {
            let active = ctx.active()?;
            let topology = ctx.read.topology(&active.target).await?;
            let lanes = topology
                .lanes
                .iter()
                .filter(|lane| {
                    lane.heads
                        .first()
                        .is_some_and(|h| h.names.contains(bookmark))
                })
                .collect::<Vec<_>>();
            if lanes.len() != 1 {
                return Err("Choose an applied top-level head; use --branch-only to remove an individual saved member".into());
            }
            Command::UnapplyBranches {
                branches: lanes[0].unapply.clone(),
            }
        }
        C::Create { name, parent } => Command::CreateStack {
            name: name.clone(),
            parent: parent.clone(),
        },
        C::Branch { command } => match command {
            BranchCommand::Create {
                name,
                position: value,
            } => {
                let (commit, placement) = position(ctx, value).await?;
                Command::CreateBranch {
                    name: name.clone(),
                    commit,
                    placement,
                }
            }
            BranchCommand::Rename { old, new } => Command::RenameBranch {
                branch: old.clone(),
                name: new.clone(),
            },
            BranchCommand::Delete { name } => Command::DeleteBranch {
                branch: name.clone(),
            },
        },
        C::Rebase { bookmark } => Command::Rebase {
            branch: bookmark.clone(),
        },
        C::SquashBranch { bookmark } => Command::SquashBranch {
            branch: bookmark.clone(),
        },
        C::Commit {
            branch,
            message,
            selection: value,
        } => Command::Commit {
            branch: branch.clone(),
            message: selection::message(args, message)?,
            selections: selection::selections(ctx, args, value, "@").await?,
        },
        C::Edit {
            revision,
            message,
            selection: value,
        } => Command::Edit {
            commit: ctx.read.resolve_revision(revision).await?,
            message: selection::message(args, message)?,
            selections: selection::selections(ctx, args, value, "@").await?,
        },
        C::Split {
            revision,
            message,
            selection: value,
        } => {
            let commit = ctx.read.resolve_revision(revision).await?;
            Command::Split {
                selections: selection::selections(ctx, args, value, &commit).await?,
                commit,
                message: selection::message(args, message)?,
            }
        }
        C::Transfer {
            to,
            selection: value,
        } => Command::Transfer {
            target: ctx.read.resolve_revision(to).await?,
            selections: selection::selections(ctx, args, value, "@").await?,
        },
        C::Reword { revision, message } => Command::Reword {
            commit: ctx.read.resolve_revision(revision).await?,
            message: selection::message(args, message)?,
        },
        C::Empty {
            position: value,
            message,
        } => {
            let (commit, placement) = position(ctx, value).await?;
            Command::Empty {
                commit,
                placement,
                message: selection::message(args, message)?,
            }
        }
        C::Move {
            revision,
            position: value,
        } => {
            let (target, placement) = position(ctx, value).await?;
            Command::Move {
                commit: ctx.read.resolve_revision(revision).await?,
                target,
                placement,
            }
        }
        C::Squash { revision, into } => Command::Squash {
            commit: ctx.read.resolve_revision(revision).await?,
            target: ctx.read.resolve_revision(into).await?,
        },
        C::Uncommit { revision } => Command::Uncommit {
            commit: ctx.read.resolve_revision(revision).await?,
        },
        C::Abandon { revision } => Command::Abandon {
            commit: ctx.read.resolve_revision(revision).await?,
        },
        C::Discard { selection: value } => Command::Discard {
            selections: selection::selections(ctx, args, value, "@").await?,
        },
        C::Resolve { path, file } => Command::Resolve {
            path: ctx.path(path)?,
            content: selection::text(
                &if file == Path::new("-") {
                    file.clone()
                } else {
                    args.directory.join(file)
                },
                local_browser::Limits::default().max_text_bytes,
                true,
            )?,
        },
        C::Refresh => Command::Refresh,
        C::Reconcile => Command::Reconcile,
        C::Undo => Command::Undo,
        C::Redo => Command::Redo,
        _ => return Err("Not a mutation".into()),
    })
}
