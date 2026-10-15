use super::*;
use PrCommand as P;
use jj_store_service::forge as f;
async fn call(ctx: &Context, action: f::Action) -> AuthorityResult<f::Response> {
    let identity = match &action {
        f::Action::Pr { request_id, .. } | f::Action::Recover { request_id } => {
            Some(request_id.clone())
        }
        f::Action::Submit { plan } => Some(plan.request_id.clone()),
        f::Action::Restack { plan } => Some(plan.request_id.clone()),
        _ => None,
    };
    local_stacks::forge::handle(
        ctx.access.clone(),
        ctx.root.clone(),
        f::Request {
            version: 1,
            token: None,
            action,
        },
    )
    .await
    .map_err(|e| {
        format!(
            "{e}{}",
            identity
                .map(|id| format!("; original request: {id}"))
                .unwrap_or_default()
        )
        .into()
    })
}
pub(super) async fn run(
    ui: &Ui,
    format: Format,
    no_pager: bool,
    args: &StackArgs,
    command: &StackCommand,
) -> AuthorityResult<()> {
    let ctx = Context::open(&args.directory).await?;
    ctx.check_pin(args.expect_operation, args.expect_revision)?;
    let mut action = match command {
        StackCommand::Restack { bookmark } => f::Action::RestackPreview {
            bookmark: bookmark.clone(),
        },
        StackCommand::Submit { bookmark, ready } => f::Action::Preview {
            bookmark: bookmark.clone(),
            draft: !*ready,
        },
        StackCommand::Pr { command: P::List } => f::Action::Status,
        StackCommand::Pr {
            command:
                P::Configure {
                    provider,
                    api_url,
                    repository,
                    source_repository,
                    remote,
                    target_remote,
                    target,
                    native_stacks,
                },
        } => {
            if args.dry_run {
                return Err(invalid("Forge configuration is explicit; omit --dry-run"));
            }
            f::Action::Configure {
                connection: f::Connection {
                    provider: match provider.as_str() {
                        "github" => f::Provider::Github,
                        "gitlab" => f::Provider::Gitlab,
                        _ => f::Provider::Forgejo,
                    },
                    api_url: api_url.clone(),
                    repository: repository.clone(),
                    source_repository: source_repository.clone().unwrap_or(repository.clone()),
                    remote: remote.clone(),
                    target_remote: target_remote.clone(),
                    target: target.clone(),
                    native_stacks: *native_stacks,
                },
            }
        }
        StackCommand::Pr {
            command: P::Show { number },
        } => f::Action::PullRequest { number: *number },
        StackCommand::Pr {
            command: P::OperationStatus { request_id },
        } => f::Action::OperationStatus {
            request_id: request_id.clone(),
        },
        StackCommand::Pr {
            command: P::Recover { request_id },
        } => f::Action::Recover {
            request_id: request_id.clone(),
        },
        StackCommand::Pr { command } => {
            let (number, action) = match command {
                P::Edit {
                    number,
                    title,
                    body_file,
                } => {
                    let response = call(&ctx, f::Action::PullRequest { number: *number }).await?;
                    let pr = response
                        .pull_requests
                        .first()
                        .ok_or("Pull request unavailable")?;
                    let body = selection::text(&args.directory.join(body_file), 128 * 1024, false)?;
                    (
                        *number,
                        f::PrAction::Edit {
                            title: title.clone(),
                            body,
                            expected_body: pr.body.clone(),
                        },
                    )
                }
                P::Draft { number, ready } => (*number, f::PrAction::Draft { draft: !*ready }),
                P::Checks { number } => (*number, f::PrAction::Checks),
                P::Jobs { number, run } => (*number, f::PrAction::Jobs { run: run.clone() }),
                P::Logs { number, run, job } => (
                    *number,
                    f::PrAction::Logs {
                        run: run.clone(),
                        job: job.clone(),
                    },
                ),
                P::Rerun {
                    number,
                    run,
                    failed_only,
                } => (
                    *number,
                    f::PrAction::Rerun {
                        run: run.clone(),
                        failed_only: *failed_only,
                    },
                ),
                P::Cancel { number, run } => (*number, f::PrAction::Cancel { run: run.clone() }),
                P::Merge { number, method } => {
                    if !args.yes && !args.dry_run {
                        return Err(invalid("Merge requires --yes; use --dry-run to inspect"));
                    }
                    let response = call(&ctx, f::Action::PullRequest { number: *number }).await?;
                    let pr = response
                        .pull_requests
                        .first()
                        .ok_or("Pull request unavailable")?;
                    (
                        *number,
                        f::PrAction::Merge {
                            method: method.clone(),
                            expected_sha: pr.sha.clone(),
                        },
                    )
                }
                P::MergeStatus { number, receipt } => (
                    *number,
                    f::PrAction::MergeStatus {
                        receipt: receipt.clone(),
                    },
                ),
                _ => return Err(invalid("Invalid forge command")),
            };
            if args.dry_run {
                let value = serde_json::to_value(&action)?;
                return crate::server_commands::emit(
                    ui,
                    format,
                    no_pager,
                    value.clone(),
                    &serde_json::to_string_pretty(&value)?,
                );
            }
            f::Action::Pr {
                request_id: format!(
                    "{:x}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_nanos()
                ),
                number,
                action,
            }
        }
        _ => return Err(invalid("Invalid forge command")),
    };
    let mut response = call(&ctx, action.clone()).await?;
    if matches!(command, StackCommand::Submit { .. }) && !args.dry_run {
        let plan = response
            .plan
            .take()
            .ok_or("Configure a forge with stack pr configure first")?;
        action = f::Action::Submit { plan };
        response = call(&ctx, action).await?;
    }
    if matches!(command, StackCommand::Restack { .. }) && !args.dry_run {
        if !args.yes {
            return Err(invalid(
                "Restack rewrites and pushes the selected chain; use --dry-run, then --yes",
            ));
        }
        let plan = response
            .restack
            .take()
            .ok_or("Restack is unavailable")?
            .plan;
        response = call(&ctx, f::Action::Restack { plan }).await?;
    }
    let text = if let Some(log) = &response.log {
        log.clone()
    } else {
        serde_json::to_string_pretty(&response)?
    };
    crate::server_commands::emit(
        ui,
        format,
        no_pager,
        serde_json::to_value(&response)?,
        &text,
    )?;
    if response
        .receipt
        .as_ref()
        .is_some_and(|r| r.outcome != f::Outcome::Complete)
    {
        return Err(
            "Submission is incomplete; inspect the reported request before retrying".into(),
        );
    }
    Ok(())
}
