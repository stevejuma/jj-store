use super::*;
use std::io::Read as _;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectionFile {
    version: u32,
    operation: jj_store_model::OperationId,
    revision: u64,
    selections: Vec<local_stacks::Selection>,
}
pub(super) fn text(path: &Path, limit: usize, stdin: bool) -> AuthorityResult<String> {
    let mut bytes = Vec::new();
    if stdin && path == Path::new("-") {
        std::io::stdin()
            .take(limit as u64 + 1)
            .read_to_end(&mut bytes)?;
    } else {
        fs::File::open(path)?
            .take(limit as u64 + 1)
            .read_to_end(&mut bytes)?;
    }
    if bytes.len() > limit {
        return Err("Input exceeds the supported byte limit".into());
    }
    Ok(String::from_utf8(bytes)?)
}
fn input_path(args: &StackArgs, path: &Path) -> PathBuf {
    args.directory.join(path)
}
pub(super) fn message(args: &StackArgs, value: &MessageArgs) -> AuthorityResult<String> {
    match (&value.message, &value.message_file) {
        (Some(value), None) => Ok(value.clone()),
        (None, Some(file)) => text(&input_path(args, file), 65536, false),
        _ => Err(invalid("Supply -m or --message-file")),
    }
}
pub(super) fn validate(
    args: &StackArgs,
    value: &SelectionArgs,
    optional: bool,
    empty: bool,
) -> AuthorityResult<()> {
    if value.empty && !empty {
        return Err(invalid("--empty is supported only by commit"));
    }
    if !optional
        && !value.all
        && !value.empty
        && value.paths.is_empty()
        && value.selection_file.is_none()
    {
        return Err(invalid(
            "Select paths after --, --all, --empty, or --selection-file",
        ));
    }
    if !value.hunk.is_empty() && (value.paths.len() != 1 || args.expect_operation.is_none()) {
        return Err(invalid(
            "--hunk requires one path and --expect-operation/--expect-revision",
        ));
    }
    if let Some(file) = &value.selection_file {
        let manifest: SelectionFile =
            serde_json::from_str(&text(&input_path(args, file), 1024 * 1024, false)?)?;
        if manifest.version != 1
            || manifest.selections.is_empty()
            || manifest.selections.len() > 256
        {
            return Err(invalid("Invalid version-1 selection file"));
        }
    }
    Ok(())
}
pub(super) async fn selections(
    ctx: &Context,
    args: &StackArgs,
    value: &SelectionArgs,
    default: &str,
) -> AuthorityResult<Vec<local_stacks::Selection>> {
    if let Some(file) = &value.selection_file {
        let manifest: SelectionFile =
            serde_json::from_str(&text(&input_path(args, file), 1024 * 1024, false)?)?;
        ctx.check_pin(Some(manifest.operation), Some(manifest.revision))?;
        return Ok(manifest.selections);
    }
    if value.empty {
        return Ok(vec![]);
    }
    let source = ctx
        .read
        .resolve_revision(value.from.as_deref().unwrap_or(default))
        .await?;
    let paths = if value.all {
        ctx.read.changed_paths(&source).await?
    } else {
        value
            .paths
            .iter()
            .map(|p| ctx.path(p))
            .collect::<AuthorityResult<Vec<_>>>()?
    };
    if value.all && paths.is_empty() {
        return Err("No changed files; use commit --empty for an empty commit".into());
    }
    Ok(paths
        .into_iter()
        .map(|path| local_stacks::Selection {
            source: source.clone(),
            path,
            hunks: (!value.hunk.is_empty()).then(|| value.hunk.clone()),
        })
        .collect())
}
