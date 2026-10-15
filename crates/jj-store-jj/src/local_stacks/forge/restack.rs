//! Explicit post-merge rewrite. A crash never causes a second local rebase.
use super::*;
use jj_lib::backend::CommitId;
pub(super) async fn preview(
    local: &LocalForge,
    client: &f::Client,
    device: &Device,
    bookmark: &str,
) -> f::Result<f::RestackReceipt> {
    let (operation, segments) = local.collect(bookmark, true).await?;
    let prs = f::engine::observe(client, device).await?;
    for segment in &segments {
        if let Some(remote) = &segment.remote_head {
            let owned = device
                .receipts()
                .into_iter()
                .rev()
                .flat_map(|r| r.plan.items)
                .find(|i| i.segment.branch == segment.branch)
                .map(|i| i.segment.head);
            if remote != &segment.head && owned.as_ref() != Some(remote) {
                return Err(f::Error::Stale);
            }
        }
    }
    let mut merged = vec![];
    let mut first = None;
    for segment in &segments {
        let latest = prs
            .iter()
            .filter(|p| p.head == segment.branch)
            .max_by_key(|p| p.number);
        if latest.is_some_and(|p| p.state == f::PrState::Merged) && first.is_none() {
            merged.push(segment.branch.clone());
        } else if latest.is_some_and(|p| p.state == f::PrState::Closed) {
            return Err(f::Error::Stale);
        } else if first.is_none() {
            first = Some(segment);
        }
    }
    let first = first.ok_or(f::Error::Invalid)?;
    if first.shared {
        return Err(f::Error::Unsupported);
    }
    let (_, repo, config, _) = read::load(&local.access, &local.root)
        .await
        .map_err(|_| f::Error::Stale)?;
    let active = config.active().ok_or(f::Error::Invalid)?;
    if config.workspaces.iter().any(|w| {
        w.id != active.id
            && w.applied
                .iter()
                .any(|b| segments.iter().any(|s| s.branch == *b))
    }) {
        return Err(f::Error::Unsupported);
    }
    if repo.op_id().hex() != operation {
        return Err(f::Error::Stale);
    }
    let target = head(
        local,
        &local.connection.target_remote,
        &local.connection.target,
    )
    .await?;
    let topology = topology::discover(&repo, &active.target, &[])
        .await
        .map_err(|_| f::Error::Stale)?;
    let expected_base = topology
        .inventory
        .iter()
        .flat_map(|l| &l.heads)
        .find(|h| h.names.contains(&first.branch))
        .and_then(|h| h.base_commit.clone())
        .ok_or(f::Error::Invalid)?;
    Ok(f::RestackReceipt {
        plan: f::RestackPlan {
            account_id: local.account_id.clone(),
            request_id: format!(
                "restack-{:x}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| f::Error::State)?
                    .as_nanos()
            ),
            operation,
            revision: config.revision,
            connection: local.connection.clone(),
            bookmark: bookmark.into(),
            branch: first.branch.clone(),
            expected_base,
            target_head: target,
            merged,
        },
        outcome: f::Outcome::Pending,
        operation: None,
        submission: None,
    })
}
pub(super) async fn target_head(local: &LocalForge) -> f::Result<String> {
    head(
        local,
        &local.connection.target_remote,
        &local.connection.target,
    )
    .await
}
async fn head(local: &LocalForge, remote: &str, target: &str) -> f::Result<String> {
    let target_local = target_local(local, remote);
    target_local.verify_remote().await?;
    target_local
        .remote_head(target)
        .await?
        .ok_or(f::Error::Stale)
}
fn target_local(local: &LocalForge, remote: &str) -> LocalForge {
    let mut approved = local.connection.clone();
    approved.remote = remote.into();
    approved.source_repository = approved.repository.clone();
    LocalForge {
        account_id: local.account_id.clone(),
        access: local.access.clone(),
        root: local.root.clone(),
        connection: approved,
        credential: local
            .credential
            .as_ref()
            .map(|s| zeroize::Zeroizing::new(s.to_string())),
    }
}
pub(super) async fn apply(
    local: &LocalForge,
    client: &f::Client,
    device: &mut Device,
    plan: f::RestackPlan,
) -> f::Result<(f::RestackReceipt, Option<f::Receipt>)> {
    if plan.connection != local.connection || plan.account_id != local.account_id {
        return Err(f::Error::Stale);
    }
    let mut record = if let Some(old) = device.restack(&plan.request_id) {
        if serde_json::to_vec(&old.plan).ok() != serde_json::to_vec(&plan).ok() {
            return Err(f::Error::Stale);
        }
        if old.operation.is_none() {
            return Err(f::Error::Unknown);
        }
        if old.outcome == f::Outcome::Rejected {
            return Ok((old, None));
        }
        if old.submission.is_none() {
            let read = ReadSession::open(local.access.clone(), local.root.clone())
                .await
                .map_err(|_| f::Error::Stale)?;
            if Some(read.operation().to_string()) != old.operation {
                return Err(f::Error::Stale);
            }
        }
        old
    } else {
        let current = preview(local, client, device, &plan.bookmark).await?;
        if current.plan.operation != plan.operation
            || current.plan.branch != plan.branch
            || current.plan.expected_base != plan.expected_base
            || current.plan.target_head != plan.target_head
            || current.plan.merged != plan.merged
        {
            return Err(f::Error::Stale);
        }
        let mut record = f::RestackReceipt {
            plan: plan.clone(),
            outcome: f::Outcome::Running,
            operation: None,
            submission: None,
        };
        device.put_restack(record.clone())?;
        let destination = target_local(local, &plan.connection.target_remote)
            .verify_remote()
            .await?;
        local
            .git(vec![
                "fetch".into(),
                "--upload-pack=git-upload-pack".into(),
                "--no-tags".into(),
                "--no-write-fetch-head".into(),
                destination,
                format!("refs/heads/{}", plan.connection.target),
            ])
            .await?;
        if head(
            local,
            &plan.connection.target_remote,
            &plan.connection.target,
        )
        .await?
            != plan.target_head
        {
            return Err(f::Error::Stale);
        }
        let (_, repo, _, _) = read::load(&local.access, &local.root)
            .await
            .map_err(|_| f::Error::Stale)?;
        if repo.op_id().hex() != plan.operation {
            return Err(f::Error::Stale);
        }
        let id = CommitId::try_from_hex(&plan.target_head).ok_or(f::Error::Invalid)?;
        git::get_git_backend(repo.store())
            .map_err(|_| f::Error::Unsupported)?
            .import_head_commits([&id])
            .map_err(|_| f::Error::Stale)?;
        let state = super::super::execute(
            local.access.clone(),
            local.root.clone(),
            Request {
                version: 1,
                binding_generation: local.access.generation(),
                operation: Some(plan.operation.parse().map_err(|_| f::Error::Invalid)?),
                revision: Some(plan.revision),
                command: Command::ForgeRestack {
                    branch: plan.branch.clone(),
                    target: plan.connection.target.clone(),
                    target_remote: plan.connection.target_remote.clone(),
                    target_sha: plan.target_head.clone(),
                    expected_base: plan.expected_base.clone(),
                    merged: plan.merged.clone(),
                },
            },
        )
        .await
        .map_err(|_| f::Error::Unknown)?;
        record.operation = Some(state.operation.to_string());
        record.outcome = if state.warning.is_some() || !state.conflicts.is_empty() {
            f::Outcome::Rejected
        } else {
            f::Outcome::Complete
        };
        device.put_restack(record.clone())?;
        if record.outcome != f::Outcome::Complete {
            return Ok((record, None));
        }
        record
    };
    let submission = if let Some(saved) = &record.submission {
        saved.clone()
    } else {
        let (op, segments) = local.segments(&plan.bookmark).await?;
        let mut p = f::planner::plan(
            format!("{}-submit", plan.request_id),
            op,
            plan.connection.clone(),
            segments,
            &client.list().await?,
            true,
        )?;
        p.account_id = local.account_id.clone();
        record.submission = Some(p.clone());
        device.put_restack(record.clone())?;
        p
    };
    let receipt = f::engine::submit(local, client, device, submission, true).await?;
    Ok((record, Some(receipt)))
}
