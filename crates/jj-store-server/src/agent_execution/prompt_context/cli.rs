use super::*;
pub(super) fn guidance(saved: &SavedSession, executable: &Path) -> String {
    let mut text = format!(
        r#"The bundled custom `jj` client is at `{}` (paths may use privacy substitutions).
`JJ_STORE_AGENT_JJ_EXECUTABLE` contains its actual absolute path in your environment.
Use the quoted variable below; do not substitute an installed `jj`. `jj-store` is the server.

```sh
"$JJ_STORE_AGENT_JJ_EXECUTABLE" --help
"$JJ_STORE_AGENT_JJ_EXECUTABLE" store help -k commands
```

Perform writes only within the user's request. Use `--help` on the exact command
before a write. Obtain current versions using `show --format json`. Keep one
request ID and the same body for an exact operation. If the outcome is uncertain,
use the matching `status` command before deciding what to do; `not_known` alone
is not proof of rejection. Task edit replaces all task fields, so preserve fields
the user has not asked to change. `--target-repository` selects a task's linked code repository;
`-R` / `--repository` selects the physical checkout. Never initialize a project home in a code checkout.
"#,
        executable.display()
    );
    if saved.coordination.is_some() {
        text.push_str(
            r#"
## Project and task coordination

The `coordination` checkout is bound to this conversation's metadata home.
Run project/task commands there, not inside a code attachment. IDs in page context
identify the existing project/task; do not create duplicates.

```sh
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination project list --format json
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination project show <project-id> --format json
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination task show <task-id> --format json
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination task create --help
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination task edit --help
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination task comment --help
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R ./coordination task status <request-id> --format json
```
"#,
        );
    }
    let directories = if saved.view.conversation_id.is_some() {
        saved
            .view
            .repository_bindings
            .iter()
            .map(|b| format!("./repositories/{}", b.repository))
            .collect::<Vec<_>>()
    } else {
        vec![".".into()]
    };
    let directories = if directories.is_empty() {
        vec!["<code-directory>".into()]
    } else {
        directories
    };
    for directory in directories {
        text.push_str(&format!(
            r#"
## Code and reviews in `{directory}`

```sh
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} status
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} diff
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} log --limit 10
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} review list --format json
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} review show <thread-id> --format json
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} review reply --help
"$JJ_STORE_AGENT_JJ_EXECUTABLE" -R {directory} review status <request-id> --format json
```

Review replies require the exact current workflow version. Use only anchors
supported by this repository's backend and the existing review command help.
"#
        ));
    }
    text
}
