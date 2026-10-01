use crate::commands::error;
use jj_cli::{cli_util::CommandHelper, command_error::CommandError};
use serde_json::{Value, json};

pub(crate) fn report(helper: &CommandHelper) -> Result<Option<(Value, String)>, CommandError> {
    let path = helper.workspace_loader()?.repo_path();
    if std::fs::read_to_string(path.join("store/type"))
        .map_err(error)?
        .trim()
        != "git"
    {
        return Ok(None);
    }
    let store =
        jj_store_jj::local_repository::metadata_store(helper.settings(), path).map_err(error)?;
    let repository = jj_store_service::local_repository::LocalRepository::new(path, store);
    let selected = repository.metadata_store().read().map_err(error)?;
    let value = json!({
        "version":2, "ok":true, "profile":"local_jj_git", "server_required":false,
        "repository":selected.as_ref().map(|s| s.repository()),
        "metadata_commit":selected.as_ref().map(|s| s.commit()),
        "metadata_initialized":selected.is_some(), "actor_initialized":repository.actor().map_err(error)?.is_some(),
        "collaboration":{"projects":true,"tasks":true,"comments":true,"agents":true,"agent_code_anchors":false,"reviews":false},
    });
    let text = format!(
        "Local Git-backed JJ repository\nMetadata: {}\nProjects, tasks and comments use direct local storage.\n",
        if selected.is_some() {
            "initialized"
        } else {
            "initialized on first metadata write"
        }
    );
    Ok(Some((value, text)))
}
