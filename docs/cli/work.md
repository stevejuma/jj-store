Native projects and tasks use the bound repository as their project home.

Use `jj project create`, `jj project link`, `jj task create`, and
`jj task comment`. Every mutation takes `--request-id` and exact applicable
versions, shown by `project show` and `task show`. When `--id` is omitted, the
CLI derives a stable entity ID from the home repository, request ID and entity
kind. Keep the exact request and body across retries; `project status` and
`task status` recover the durable receipt after an unknown outcome.

Task edits replace the complete title, body, state, parent and repository
fields. Provide the complete intended values so identical retries remain
deterministic; omitting a parent or repository clears that field. Comments
support replies, edits and tombstones. `--format json` preserves full IDs,
state/version fields, pinned roots and page continuations.

The local native service is required. An upstream is optional. First WorkStore
publication upgrades the home to format 3 and freezes its origin ledger; M10
replication is unavailable for that home until WorkStore replication is
supported. Use a separate coordination home when linked code repositories need
M10 sync. A repository link grants no code access. Git refs, server-free
storage, relay synchronization and agent dispatch are outside this feature.

Git-backed JJ repositories use the same project/task commands through an
in-process local service. No server connection is needed. The first successful
metadata write selects an independent repository identity under
`refs/lapis/state/v1`; list and status do not initialize metadata. Local JSON
responses use version 2 and explicit Git metadata identifiers. Request receipts
and pinned page cursors survive process restart. JJ workspaces sharing one
repository also share metadata and the device actor. Native version-1 responses
and native authentication remain unchanged.
