//! Server-free CLI integration for the existing ordinary local Stacks engine.
mod args;
mod context;
mod mutations;
mod queries;
mod selection;
use crate::commands::Format;
pub use args::StackArgs;
use args::*;
use context::Context;
use jj_cli::ui::Ui;
use jj_store_jj::{
    local_browser,
    local_stacks::{self, Command, State},
    operation_stores::{AuthorityError, AuthorityResult},
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub(crate) struct Exit {
    pub code: u8,
    message: String,
}
impl std::fmt::Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for Exit {}
fn invalid(message: &str) -> AuthorityError {
    Box::new(Exit {
        code: 2,
        message: message.into(),
    })
}
pub(crate) fn error_code(error: &AuthorityError) -> u8 {
    error.downcast_ref::<Exit>().map_or(1, |e| e.code)
}
fn inspection(value: &StackCommand) -> bool {
    matches!(
        value,
        StackCommand::Status
            | StackCommand::List { .. }
            | StackCommand::Show { .. }
            | StackCommand::Diff { .. }
            | StackCommand::Hunks { .. }
            | StackCommand::Conflicts { .. }
            | StackCommand::Workspace {
                command: WorkspaceCommand::List
            }
    )
}
fn validate(args: &StackArgs, value: &StackCommand) -> AuthorityResult<()> {
    let read = inspection(value);
    if args.dry_run && (read || matches!(value, StackCommand::Refresh)) {
        return Err(invalid(
            "--dry-run is for mutations; use --no-snapshot for inspection",
        ));
    }
    if args.no_snapshot && !read && !args.dry_run {
        return Err(invalid(
            "--no-snapshot is for inspection; mutations use native snapshots",
        ));
    }
    let destructive = matches!(
        value,
        StackCommand::Workspace {
            command: WorkspaceCommand::Delete { .. }
        } | StackCommand::Branch {
            command: BranchCommand::Delete { .. }
        } | StackCommand::Abandon { .. }
            | StackCommand::Discard { .. }
            | StackCommand::Reconcile
    );
    if destructive && !args.yes && !args.dry_run {
        return Err(invalid(
            "This action requires --yes; use --dry-run to inspect its scope",
        ));
    }
    match value {
        StackCommand::Commit { selection: s, .. } => selection::validate(args, s, false, true)?,
        StackCommand::Edit { selection: s, .. } => selection::validate(args, s, true, false)?,
        StackCommand::Split { selection: s, .. }
        | StackCommand::Transfer { selection: s, .. }
        | StackCommand::Discard { selection: s } => selection::validate(args, s, false, false)?,
        StackCommand::Workspace {
            command:
                WorkspaceCommand::Update {
                    name: None,
                    target: None,
                },
        } => return Err(invalid("Supply --name or --target")),
        _ => {}
    }
    Ok(())
}
pub(crate) async fn run(
    ui: &Ui,
    format: Format,
    no_pager: bool,
    args: StackArgs,
) -> AuthorityResult<()> {
    let Some(command) = &args.command else {
        return crate::server_commands::help(ui, no_pager, &["stack".into()]);
    };
    validate(&args, command)?;
    let initial = Context::open(&args.directory).await?;
    initial.check_pin(args.expect_operation, args.expect_revision)?;
    let mut warning = None;
    let ctx = if !args.no_snapshot && !args.dry_run {
        let refreshed = local_stacks::execute(
            initial.access.clone(),
            initial.root.clone(),
            initial.request(Command::Refresh),
        )
        .await?;
        if refreshed
            .warning
            .as_ref()
            .is_some_and(|s| s.starts_with("JJ operation "))
        {
            crate::server_commands::emit(
                ui,
                format,
                no_pager,
                serde_json::to_value(&refreshed)?,
                &queries::state_text(&refreshed),
            )?;
            return Err(Box::new(Exit { code: 3, message: "Snapshot applied; working-copy bookkeeping needs attention. Do not repeat the command.".into() }));
        }
        warning = refreshed.warning;
        Context::open(&args.directory).await?
    } else {
        initial
    };
    ctx.check_pin(args.expect_operation, args.expect_revision)?;
    let (value, text, attention) = if inspection(command) {
        let (mut value, mut text) = queries::query(&ctx, command).await?;
        if let Some(warning) = warning {
            value["warning"] = warning.clone().into();
            text.push_str(&format!("Warning: {warning}\n"));
        }
        (value, text, false)
    } else if matches!(command, StackCommand::Refresh) {
        let mut state = ctx.state;
        state.warning = warning;
        (
            serde_json::to_value(&state)?,
            queries::state_text(&state),
            false,
        )
    } else {
        let native = mutations::command(&ctx, &args, command).await?;
        let request = ctx.request(native);
        let preview =
            local_stacks::preview(ctx.access.clone(), ctx.root.clone(), request.clone()).await?;
        if args.dry_run {
            (
                serde_json::json!({"version":1,"operation":ctx.state.operation,"revision":ctx.state.revision,"dry_run":true,"snapshot":"last recorded snapshot","preview":preview}),
                format!(
                    "Dry run · last recorded snapshot\n{}\n{}\n",
                    preview.title, preview.summary
                ),
                false,
            )
        } else {
            let mut state =
                local_stacks::execute(ctx.access.clone(), ctx.root.clone(), request).await?;
            let attention = state
                .warning
                .as_ref()
                .is_some_and(|s| s.starts_with("JJ operation "));
            if state.warning.is_none() {
                state.warning = warning;
            }
            (
                serde_json::to_value(&state)?,
                queries::state_text(&state),
                attention,
            )
        }
    };
    if inspection(command) && serde_json::to_vec(&value)?.len() > 4 * 1024 * 1024 {
        return Err("Result exceeds 4 MiB; use a smaller --limit or a single path".into());
    }
    crate::server_commands::emit(ui, format, no_pager, value, &text)?;
    if attention {
        return Err(Box::new(Exit{code:3,message:"Operation applied; inspect the reported native operation. Do not repeat the command.".into()}));
    }
    Ok(())
}
