# Repository review threads

`jj review` reads and updates the ReviewStore selected by the current bound
native workspace. It works over the owner-only LocalTrusted socket or the
configured authenticated native RPC connection.

List and show commands return current-root thread state. `--format json`
preserves complete stable IDs, exact selected revisions, snapshot IDs, cursors,
comments and events.

Every mutation requires an explicit request ID. Replies require the selected
workflow version, comment changes require the selected revision ID, and workflow
changes require the selected thread snapshot ID. Reusing a request ID with
changed intent fails. Retrying the same request returns its durable receipt.

Thread creation requires `--anchor PATH`, a complete `ArtifactAnchorV1` JSON
document for the current repository. This keeps code, diff, agent, revision,
operation and repository anchors exact rather than inferring an identity from a
display label. Comment bodies use safe Markdown unless `--plain-text` is given.

Use `jj review status REQUEST_ID` after an interrupted command. `not_known`
means no committed receipt is visible and is not proof that the command was
rejected.
