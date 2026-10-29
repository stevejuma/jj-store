//! Query editor metadata and diagnostics for the pinned JJ language.
mod aliases;
pub mod identity;
pub mod operations;
pub use crate::read_support::Diagnostic;
pub use aliases::standard_aliases;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn invalid(kind: &str, message: impl AsRef<str>) -> super::Error {
    let message: String = message.as_ref().chars().take(1024).collect();
    super::Error::InvalidQuery(Diagnostic {
        kind: kind.into(),
        message,
    })
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Callable {
    pub name: String,
    pub signature: String,
    pub insert_text: String,
    pub description: String,
    pub kind: String,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct QueryInfo {
    pub version: u32,
    pub jj_version: String,
    pub max_query_bytes: usize,
    pub callables: Vec<Callable>,
    pub aliases: Vec<String>,
    pub operators: Vec<String>,
    pub operation_fields: Vec<String>,
    pub unsupported: Vec<String>,
    /// Device identity hint, available only in LocalTrusted query metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_user_email: Option<String>,
}
pub fn info(local: bool) -> QueryInfo {
    let unsupported: Vec<String> = if local {
        vec![
            "files",
            "diff_lines",
            "diff_lines_added",
            "diff_lines_removed",
            "diff_contains",
            "at_operation",
        ]
    } else {
        vec![]
    }
    .into_iter()
    .map(String::from)
    .collect();
    let mut callables: Vec<Callable> =
        serde_json::from_str(include_str!("callables.json")).expect("checked query catalog");
    callables.retain(|entry| !unsupported.contains(&entry.name));
    QueryInfo {
        version: 1,
        jj_version: "0.45.1".into(),
        max_query_bytes: 4096,
        callables,
        aliases: aliases::ALIASES
            .iter()
            .map(|(name, _)| (*name).into())
            .collect(),
        operators: ["::", "..", "|", "&", "~", ":", "+", "-"]
            .into_iter()
            .map(String::from)
            .collect(),
        operation_fields: [
            "description",
            "user",
            "host",
            "workspace",
            "id",
            "after",
            "before",
            "snapshot",
            "from",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        unsupported,
        default_user_email: None,
    }
}
pub fn require_context(
    expression: &str,
    workspace: Option<&str>,
    email: &str,
) -> super::Result<()> {
    // Inspect unquoted language tokens, distinguishing callable names from bookmarks.
    let mut quoted = None;
    let mut escape = false;
    let mut token = String::new();
    let mut needs_email = false;
    let mut needs_workspace = false;
    let mut characters = expression.chars().chain(std::iter::once(' '));
    while let Some(ch) = characters.next() {
        if escape {
            escape = false;
            continue;
        }
        if quoted.is_some() && ch == '\\' {
            escape = true;
            continue;
        }
        if quoted == Some(ch) {
            quoted = None;
            continue;
        }
        if quoted.is_some() {
            continue;
        }
        if ch.is_alphanumeric() || ch == '_' || ch == '@' {
            token.push(ch);
            continue;
        }
        // Whitespace can separate a callable name and its opening parenthesis.
        let callable = ch == '('
            || (ch.is_whitespace() && characters.clone().find(|c| !c.is_whitespace()) == Some('('));
        needs_email |= token == "mine" && callable;
        needs_workspace |= token == "@" || (token == "builtin_log" && callable);
        token.clear();
        if ch == '\'' || ch == '"' {
            quoted = Some(ch);
        }
    }
    needs_workspace |= token == "@";
    if email.is_empty() && needs_email {
        return Err(invalid(
            "missing_context",
            "Set the author email in Query context to use mine().",
        ));
    }
    if workspace.is_none() && needs_workspace {
        return Err(invalid(
            "missing_context",
            "Choose a workspace in Query context to use @ or builtin_log().",
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_and_context_match_profile() {
        assert!(!info(true).callables.iter().any(|c| c.name == "files"));
        assert!(info(false).callables.iter().any(|c| c.name == "mutable"));
        assert!(require_context("description(\"mine @\")", None, "").is_ok());
        assert!(require_context("mine()", None, "").is_err());
        assert!(require_context("mine ()", None, "").is_err());
        assert!(require_context("mine", None, "").is_ok());
        assert!(require_context("mine@origin", None, "").is_ok());
        assert!(require_context("builtin_log()", None, "").is_err());
        assert!(require_context("builtin_log()", Some("default"), "").is_ok());
        assert!(require_context("@", None, "").is_err());
        assert!(require_context("all() | @", None, "").is_err());
        assert!(require_context("mine | all()", None, "").is_ok());
    }
}
