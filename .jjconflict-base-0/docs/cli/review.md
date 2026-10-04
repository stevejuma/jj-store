# Repository review threads

`jj review` reads and updates the ReviewStore selected by the current bound
native workspace or the selected Git-backed JJ companion. Native access works
over the owner-only LocalTrusted socket or the configured authenticated native
RPC connection. Local access requires no server. Read-only commands do not
initialize companion metadata or a device actor.

For a local repository, `jj review repository --request-id ID --body TEXT`
creates a repository discussion and initializes metadata on the first write.
`list --after KEY --at-metadata COMMIT` resumes selected committed history;
`show THREAD --at-metadata COMMIT` reads historical detail. Native `--cursor`
and local selectors cannot be mixed. Code and agent anchors remain gated until
their portable evidence is qualified.

List and show commands return current-root thread state. `--format json`
preserves complete stable IDs, exact selected revisions, snapshot IDs, cursors,
comments and events.

Every mutation requires an explicit request ID. Replies require the selected
workflow version, comment changes require the selected revision ID, and workflow
changes require the selected thread snapshot ID. Reusing a request ID with
changed intent fails. Retrying the same request returns its durable receipt.

When a replicated comment has concurrent revisions,
`jj review resolve-comment-conflict` requires every displayed alternative
through repeated `--expected-revision-id` arguments. The CLI sorts the bounded
set before sending the typed command. A missing or stale alternative fails
without selecting a winner; a successful replacement records every superseded
revision.

Thread creation requires `--anchor PATH`, a complete `ArtifactAnchorV1` JSON
document for the current repository. This keeps code, diff, agent, revision,
operation and repository anchors exact rather than inferring an identity from a
display label. Comment bodies use safe Markdown unless `--plain-text` is given.

Use `jj review status REQUEST_ID` after an interrupted command. `not_known`
means no committed receipt is visible and is not proof that the command was
rejected.
