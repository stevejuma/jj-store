use super::*;

pub(super) fn page(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    page: &WorkPage,
) -> Result<(), CommandError> {
    let mut text = format!(
        "WorkStore {} version {} (publication {})\n",
        page.snapshot.repo_id, page.snapshot.work_state_version, page.snapshot.published_seq.0
    );
    for project in &page.projects {
        text.push_str(&format!(
            "Project {} v{}  {}\n{}\n",
            project.id, project.version, project.title, project.body
        ));
        for (repository, active) in &project.repositories {
            text.push_str(&format!(
                "  Repository {repository}  {}\n",
                if *active { "active" } else { "retired" }
            ));
        }
    }
    for task in &page.tasks {
        text.push_str(&format!(
            "Task {} v{}  {:?}  {}\n{}\n",
            task.id, task.version, task.state, task.title, task.body
        ));
        if let Some(repository) = task.repository {
            text.push_str(&format!("  Repository {repository}\n"));
        }
        if let Some(parent) = task.parent {
            text.push_str(&format!("  Parent {parent}\n"));
        }
    }
    for comment in &page.comments {
        text.push_str(&format!(
            "Comment {} v{} reply={:?}\n{}\n",
            comment.id,
            comment.version,
            comment.reply_to,
            comment.body.as_deref().unwrap_or("[deleted]")
        ));
    }
    if let Some(cursor) = &page.next_cursor {
        text.push_str(&format!("Next cursor: {cursor}\n"));
    }
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(page).map_err(error)?,
        &text,
    )
}
pub(super) fn result(
    ui: &Ui,
    helper: &CommandHelper,
    format: Format,
    result: &impl serde::Serialize,
) -> Result<(), CommandError> {
    let value = serde_json::to_value(result).map_err(error)?;
    let text = serde_json::to_string_pretty(&value).map_err(error)? + "\n";
    presentation::result(ui, helper, format, &value, &text)
}
