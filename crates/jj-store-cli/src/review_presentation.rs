//! Text and lossless JSON presentation for ReviewStore CLI results.
use crate::{
    commands::{Format, error},
    presentation,
};
use jj_cli::{cli_util::CommandHelper, command_error::CommandError, ui::Ui};
use jj_store_service::collaboration::{ReviewCommentContent, ReviewThreadDetail, ReviewThreadPage};

pub(crate) fn page(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    page: &ReviewThreadPage,
) -> Result<(), CommandError> {
    let rows = page
        .threads
        .iter()
        .map(|thread| {
            format!(
                "{}  {:?}  comments={}  workflow={}\n",
                thread.thread_id, thread.status, thread.comment_count, thread.workflow_version,
            )
        })
        .collect::<String>();
    let cursor = page
        .next_cursor
        .as_deref()
        .map(|value| format!("Next cursor: {value}\n"))
        .unwrap_or_default();
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(page).map_err(error)?,
        &format!("{rows}{cursor}"),
    )
}

pub(crate) fn detail(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    detail: &ReviewThreadDetail,
) -> Result<(), CommandError> {
    let comments = detail
        .comments
        .iter()
        .map(|comment| {
            let body = match &comment.content {
                ReviewCommentContent::Visible { text, .. } => text.as_str(),
                ReviewCommentContent::Tombstoned { .. } => "(tombstoned)",
                ReviewCommentContent::Hidden { .. } => "(hidden)",
            };
            format!(
                "{}  {}\n{}\n",
                comment.comment_id, comment.revision_id, body
            )
        })
        .collect::<String>();
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(detail).map_err(error)?,
        &format!(
            "Thread: {}\nStatus: {:?}\nWorkflow version: {}\n{comments}",
            detail.thread.thread_id, detail.thread.status, detail.thread.workflow_version,
        ),
    )
}

pub(crate) fn result(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    value: &impl serde::Serialize,
) -> Result<(), CommandError> {
    let value = serde_json::to_value(value).map_err(error)?;
    let status = value
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let request = value
        .pointer("/receipt/request_id")
        .or_else(|| value.get("request_id"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    presentation::result(
        ui,
        helper,
        format,
        &value,
        &format!("Review request {request}: {status}\n"),
    )
}
