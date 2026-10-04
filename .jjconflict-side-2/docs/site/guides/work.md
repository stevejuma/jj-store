# Projects, tasks and discussion

Projects is the installation-wide entry point for coordination. A project keeps
its metadata home even when it links several repositories. The sidebar groups
projects and repositories to make that distinction visible.

## Plan and update work

Create a project in an authorized home, then create tasks and subtasks. Assign a
linked repository only when that relationship is meaningful. Task state uses the
model's admitted values (`open`, `in_progress`, `blocked`, `done`); it does not
start an agent or trigger a controller merely because state changed.

Edit forms retain unsaved text until you explicitly submit or discard it. A
committed result updates the active views automatically. In CLI callers, task
edits provide the complete intended title, body, state, parent and repository
values along with exact current versions. Treat omitted optional values as
clearing them, not as a patch instruction.

## Comments and replies

Use Reply to attach a comment to its parent. Edits refer to the exact existing
revision; removal retains a tombstone. A concurrent revision change requires
refresh or the appropriate conflict-resolution operation. Reactions attach to
exact comment targets and follow the home's current permission checks.

A user may read a discussion without having write permission. Read-only views
retain conversation and reaction counts while omitting mutation controls. Do not
infer ownership or current authorization from an actor's display name.

## CLI automation

Use the generated [project](../cli/jj-project.md) and [task](../cli/jj-task.md)
references for the complete command tree. First inspect current state with
`show` and request JSON when automation needs full identities and versions. Each
mutation needs a request ID. Keep the complete command intent until its receipt
is known; `project status` and `task status` are recovery operations.

Stable entity IDs may be derived from the home, request identity and entity kind
when a supported creation omits `--id`. This helps identical retries identify
the same entity; it does not permit changing the title/body under the same
request identity.

Native commands require the native service. Local Git-backed commands select the
in-process companion provider. The home determines that choice, not the profile
of a linked code repository.
