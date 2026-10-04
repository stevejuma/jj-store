# First project and task

A project belongs to a **metadata home repository**. Its tasks, comments and
repository links are published there, even when the work concerns code in
another repository. Start with a home where you have write permission.

## Create a project in the browser

Open Projects from the left sidebar. Use Add project and select the home when
the form offers more than one. Enter a meaningful title and description, then
save. The saved project appears in installation navigation. A committed save
refreshes the active view automatically.

Link code repositories to the project when needed. The link records stable
repository identity rather than a device path. You still need access to the
linked repository to inspect its code. An unavailable link is not permission to
substitute another repository with the same display name.

## Add and discuss a task

Create a task within the project with a title, body and initial state. Tasks can
have a parent and a linked repository. Add subtasks to express hierarchy rather
than copying the parent's identity. Use threaded comments to preserve the
relationship between a reply and the exact conversation it responds to.

The task editor replaces the complete intended task fields. In CLI automation,
omitting optional parent/repository values during an edit clears those fields;
it is not a partial patch. Read the current version first and preserve it with
the submitted intent.

## Recover an interrupted save

A command has a stable request identity. If the result is unknown, keep the
draft and use the displayed status/recovery controls. A missing receipt does not
establish that the save failed. Sending a new request ID can create a second
operation when the first eventually proves committed.

CLI callers use `jj project show`, `jj task show`, the relevant command's exact
version arguments, and `project status` or `task status` for receipt recovery.
See [project](../cli/jj-project.md) and [task](../cli/jj-task.md) references.

## Choose the home deliberately

Native WorkStore homes freeze their existing origin ledger because WorkStore
replication is not yet qualified. If code repositories need native upstream
sync, use a separate metadata home and link those code repositories. Local
Git-backed homes use the companion ref and can run server-free CLI commands, but
ordinary Git push does not synchronize that collaboration history automatically.
