use super::*;
use serde_json::{Value, json};
pub(super) fn state_text(state: &State) -> String {
    let mut result = format!(
        "Operation: {}\nConfiguration revision: {}\n",
        state.operation, state.revision
    );
    if state.workspaces.is_empty() {
        result.push_str("No named workspaces. Use stack init --name NAME --target BOOKMARK.\n");
    }
    for workspace in &state.workspaces {
        result.push_str(&format!(
            "{} {} ({}) · target {}\n  Merge {}\n  WIP {}\n",
            if state.active.as_ref() == Some(&workspace.id) {
                "*"
            } else {
                " "
            },
            workspace.name,
            workspace.id,
            workspace.target,
            workspace.merge_commit,
            workspace.wip_commit
        ));
        if !workspace.applied.is_empty() {
            result.push_str(&format!("  Applied: {}\n", workspace.applied.join(", ")));
        }
        if workspace.membership_changed {
            result.push_str("  Saved membership needs explicit reconciliation.\n");
        }
        if let Some(reason) = &workspace.unavailable_reason {
            result.push_str(&format!("  {reason}\n"));
        }
    }
    if !state.conflicts.is_empty() {
        result.push_str(&format!("Conflicts: {}\n", state.conflicts.join(", ")));
    }
    if let Some(warning) = &state.warning {
        result.push_str(&format!("Warning: {warning}\n"));
    }
    result
}
fn lanes_text(topology: &local_stacks::Topology) -> String {
    let mut text = String::new();
    for lane in &topology.inventory {
        for (index, head) in lane.heads.iter().enumerate() {
            text.push_str(&format!(
                "{}{}{}{}{}\n",
                if index == 0 { "" } else { "  └ " },
                head.names.join(" / "),
                if head.shared { " [Shared]" } else { "" },
                if lane.applied.iter().any(|n| head.names.contains(n)) {
                    " [Applied]"
                } else {
                    ""
                },
                if index == 0
                    && !lane.applied.is_empty()
                    && !head.names.iter().any(|n| lane.applied.contains(n))
                {
                    " [Partially applied]"
                } else {
                    ""
                }
            ));
        }
    }
    if !topology.complete {
        text.push_str("Inventory incomplete: unproven relationships remain independent.\n");
    }
    text
}
pub(super) async fn hunk(
    ctx: &Context,
    source: String,
    path: String,
) -> AuthorityResult<local_stacks::HunkResponse> {
    Ok(local_stacks::hunks(
        ctx.access.clone(),
        ctx.root.clone(),
        local_stacks::HunkRequest {
            version: 1,
            binding_generation: 1,
            operation: ctx.state.operation,
            revision: ctx.state.revision,
            source,
            path,
        },
    )
    .await?)
}
pub(super) async fn conflict(
    ctx: &Context,
    path: String,
) -> AuthorityResult<local_stacks::Conflict> {
    Ok(local_stacks::conflict(
        ctx.access.clone(),
        ctx.root.clone(),
        local_stacks::HunkRequest {
            version: 1,
            binding_generation: 1,
            operation: ctx.state.operation,
            revision: ctx.state.revision,
            source: ctx.read.resolve_revision("@").await?,
            path,
        },
    )
    .await?)
}
pub(super) async fn query(ctx: &Context, value: &StackCommand) -> AuthorityResult<(Value, String)> {
    let pin = |data: Value| json!({"version":1,"operation":ctx.state.operation,"revision":ctx.state.revision,"data":data});
    match value {
        StackCommand::Status => {
            let paths = ctx
                .read
                .changed_paths(&ctx.read.resolve_revision("@").await?)
                .await?;
            let mut text = state_text(&ctx.state);
            if !paths.is_empty() {
                text.push_str(&format!(
                    "Unfinished files:\n{}\n",
                    paths
                        .iter()
                        .map(|p| format!("  {p}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
            }
            Ok((
                json!({"version":1,"operation":ctx.state.operation,"revision":ctx.state.revision,"state":ctx.state,"paths":paths}),
                text,
            ))
        }
        StackCommand::Workspace {
            command: WorkspaceCommand::List,
        } => Ok((serde_json::to_value(&ctx.state)?, state_text(&ctx.state))),
        StackCommand::List { target, all } => {
            let target = target
                .as_deref()
                .map(Ok)
                .unwrap_or_else(|| ctx.active().map(|w| w.target.as_str()))?;
            let topology = ctx.read.topology(target).await?;
            let mut text = format!("Target: {target}\n{}", lanes_text(&topology));
            let bookmarks = if *all {
                Some(ctx.read.bookmarks()?)
            } else {
                None
            };
            if *all {
                if !topology.integrated.is_empty() {
                    text.push_str(&format!("Integrated: {}\n", topology.integrated.join(", ")));
                }
                for (name, id) in bookmarks.as_ref().unwrap() {
                    if id.is_none() {
                        text.push_str(&format!("{name} [Unavailable]\n"));
                    }
                }
            }
            Ok((
                pin(json!({"target":target,"topology":topology,"bookmarks":bookmarks})),
                text,
            ))
        }
        StackCommand::Show {
            target,
            bookmark,
            limit,
            after,
            card,
        } => {
            let target = target
                .as_deref()
                .map(Ok)
                .unwrap_or_else(|| ctx.active().map(|w| w.target.as_str()))?;
            let topology = ctx.read.topology(target).await?;
            let matches = topology
                .inventory
                .iter()
                .filter(|l| l.heads.iter().any(|h| h.names.contains(bookmark)))
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(
                    "Select a unique top-level stack; shared branches can belong to several stacks"
                        .into(),
                );
            }
            let lane = matches[0];
            let mut pages = vec![];
            let mut text = format!("Stack {}\n", lane.heads[0].names.join(" / "));
            let executor = local_browser::Executor::new(1)?;
            for head in &lane.heads {
                if card.as_ref().is_some_and(|c| !head.names.contains(c)) {
                    continue;
                }
                text.push_str(&format!(
                    "\n{}{} · {}\n",
                    head.names.join(" / "),
                    if head.shared { " [Shared]" } else { "" },
                    head.commit
                ));
                let response = executor
                    .read(
                        ctx.access.clone(),
                        ctx.state.operation,
                        local_browser::Request::StackHistory(local_browser::stacks::Request {
                            head: head.commit.parse()?,
                            base: head.base_commit.as_ref().map(|s| s.parse()).transpose()?,
                            limit: *limit as usize,
                            after: after.as_ref().map(|s| s.parse()).transpose()?,
                        }),
                    )
                    .await?;
                let page = serde_json::to_value(&response)?;
                if let Some(commits) = page
                    .get("data")
                    .and_then(|d| d.get("commits"))
                    .and_then(Value::as_array)
                {
                    for row in commits {
                        let commit = &row["commit"];
                        text.push_str(&format!(
                            "  {} {}\n",
                            commit.get("id").and_then(Value::as_str).unwrap_or(""),
                            commit
                                .get("description")
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .lines()
                                .next()
                                .unwrap_or("")
                        ));
                    }
                }
                pages.push(json!({"head":head,"page":response}));
            }
            if pages.is_empty() {
                return Err("The selected branch card is unavailable".into());
            }
            Ok((
                pin(json!({"stack":lane,"pages":pages,"complete":topology.complete})),
                text,
            ))
        }
        StackCommand::Hunks { path, from } => {
            let response = hunk(
                ctx,
                ctx.read
                    .resolve_revision(from.as_deref().unwrap_or("@"))
                    .await?,
                ctx.path(path)?,
            )
            .await?;
            let mut text = format!(
                "Operation: {}\nConfiguration revision: {}\n",
                ctx.state.operation, ctx.state.revision
            );
            for h in &response.hunks {
                text.push_str(&format!("Hunk {}\n{}\n", h.id, h.patch));
            }
            if let Some(reason) = &response.reason {
                text.push_str(&format!("{reason}\n"));
            }
            Ok((pin(serde_json::to_value(response)?), text))
        }
        StackCommand::Conflicts { path: Some(path) } => {
            let response = conflict(ctx, ctx.path(path)?).await?;
            let mut text = format!("Conflicts in {}\n", response.path);
            if let Some(reason) = &response.reason {
                text.push_str(&format!("{reason}\n"));
            }
            for (index, term) in response.terms.iter().enumerate() {
                text.push_str(&format!(
                    "\nVersion {}{}\n{}\n",
                    index + 1,
                    if index % 2 == 1 {
                        " (negative/base term)"
                    } else {
                        " (positive term)"
                    },
                    term.as_deref().unwrap_or("[Absent]")
                ));
            }
            Ok((pin(serde_json::to_value(response)?), text))
        }
        StackCommand::Conflicts { path: None } => Ok((
            pin(json!({"paths":ctx.state.conflicts})),
            format!(
                "Conflict paths (up to 50):\n{}\n",
                ctx.state.conflicts.join("\n")
            ),
        )),
        StackCommand::Diff {
            from,
            branch,
            after,
            limit,
            path,
        } => {
            let (source, base, net) = if let Some(branch) = branch {
                let topology = ctx.read.topology(&ctx.active()?.target).await?;
                let head = topology
                    .inventory
                    .iter()
                    .flat_map(|l| &l.heads)
                    .find(|h| h.names.contains(branch))
                    .ok_or("Branch comparison unavailable")?;
                (
                    head.commit.clone(),
                    head.base_commit
                        .clone()
                        .ok_or("Native lower boundary is ambiguous; inspect its graph instead")?,
                    true,
                )
            } else {
                let source = ctx
                    .read
                    .resolve_revision(from.as_deref().unwrap_or("@"))
                    .await?;
                let base = ctx.read.parent(&source).await?;
                (source, base, false)
            };
            let executor = local_browser::Executor::new(1)?;
            let response = executor
                .read(
                    ctx.access.clone(),
                    ctx.state.operation,
                    local_browser::Request::DiffPaths(local_browser::diff_summary::Request {
                        from: base.parse()?,
                        to: source.parse()?,
                        path: match path {
                            Some(p) => ctx.path(p)?,
                            None => String::new(),
                        },
                        limit: *limit as usize,
                        anchor: after.clone(),
                        direction: local_browser::Direction::Forward,
                    }),
                )
                .await?;
            let local_browser::Response::DiffPaths(page) = response else {
                return Err("Unexpected diff response".into());
            };
            let mut text = format!(
                "{}{}\n",
                if net {
                    "Read-only branch comparison "
                } else {
                    "Changes in "
                },
                source
            );
            let mut files = vec![];
            for change in &page.changes {
                text.push_str(&format!("\n{}\n", change.path));
                if net {
                    files.push(json!({"change":change}));
                    continue;
                }
                let hunks = hunk(ctx, source.clone(), change.path.clone()).await?;
                for h in &hunks.hunks {
                    text.push_str(&format!("Hunk {}\n{}\n", h.id, h.patch));
                }
                if let Some(reason) = &hunks.reason {
                    text.push_str(&format!("{reason}\n"));
                }
                files.push(json!({"change":change,"hunks":hunks}));
            }
            if let Some(next) = &page.next {
                text.push_str(&format!("Continue at this operation with --after {next}\n"));
            }
            Ok((
                pin(
                    json!({"source":source,"base":base,"net_comparison":net,"page":page,"files":files}),
                ),
                text,
            ))
        }
        _ => Err("Not an inspection command".into()),
    }
}
