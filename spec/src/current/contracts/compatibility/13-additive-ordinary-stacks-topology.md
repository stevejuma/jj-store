# Additive ordinary Stacks topology

HTTP V1 local workspace state adds optional operation-owned `topology`, with
bounded completeness, ordered lanes, same-tip alias names and integrated saved
branches. The additive `unapply_branches` command removes an explicit set in one
operation/config-guarded transaction; existing `unapply` remains compatible.
TOML version1 and stable bookmark identities are unchanged. Existing workspaces
are corrected only through explicit reconciliation previews, including eligible
legacy head IDs. No read migrates or repairs source graphs. Focused frontend
history accepts `stack_history=live|pinned`; legacy focused links default to
live, and explicit historical snapshots remain pinned across reload/navigation.
Native JJ identity, object, WAL, manifest and durable layout formats stay
unchanged.

Local topology V1 additionally carries an optional full `inventory`, per-head
`base_commit` and `shared`, and each lane's exclusive `unapply` membership.
Existing optional-field consumers remain compatible. The guarded topology POST
is a read query with exact operation/target identity, available before TOML
setup; it creates no durable metadata and changes no format. Incomplete results
must preserve ordinary bookmark visibility without claiming unproven grouping.
