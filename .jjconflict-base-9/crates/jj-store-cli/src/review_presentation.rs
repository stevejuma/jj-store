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
    page_rows(
        ui,
        helper,
        format,
        page,
        &page.threads,
        page.next_cursor.as_deref(),
    )
}

pub(crate) fn page_rows(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    value: &impl serde::Serialize,
    threads: &[jj_store_service::collaboration::ReviewThreadSummary],
    next: Option<&str>,
) -> Result<(), CommandError> {
    let rows = threads
        .iter()
        .map(|thread| {
            format!(
                "{}  {:?}  comments={}  workflow={}\n",
                thread.thread_id, thread.status, thread.comment_count, thread.workflow_version,
            )
        })
        .collect::<String>();
    let cursor = next
        .map(|value| format!("Next cursor: {value}\n"))
        .unwrap_or_default();
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(value).map_err(error)?,
        &format!("{rows}{cursor}"),
    )
}

pub(crate) fn detail(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    detail: &ReviewThreadDetail,
) -> Result<(), CommandError> {
    detail_rows(ui, helper, format, detail, &detail.thread, &detail.comments)
}

pub(crate) fn detail_rows(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    value: &impl serde::Serialize,
    thread: &jj_store_service::collaboration::ReviewThreadSummary,
    comments: &[jj_store_service::collaboration::ReviewComment],
) -> Result<(), CommandError> {
    let comments = comments
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
        &serde_json::to_value(value).map_err(error)?,
        &format!(
            "Thread: {}\nStatus: {:?}\nWorkflow version: {}\n{comments}",
            thread.thread_id, thread.status, thread.workflow_version,
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
