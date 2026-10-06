# Typed mutation HTTP transport

`POST /v1/repos/{repo}/mutations/preview` and `POST .../mutations` accept a
versioned typed request with request_id, exact expected snapshot and command.
`GET .../mutations/{request_id}` resolves that principal's durable receipt. All
three require current repository write permission, including before parsing
command bodies. Unknown fields, unsupported command variants, malformed IDs,
unknown query parameters and bodies above 512 KiB fail explicitly. Native
commands share the serving instance's worker pool with graph/content reads.
Capabilities advertise only implemented commands and the request limit.

Preview returns prepared provisional native results or the already committed
receipt. Apply returns committed or outcome_unknown; status returns committed,
not_known or outcome_unknown. These tagged results use HTTP 200; unknown does
not prove acceptance or rejection. Stale snapshots and changed intent return 409
with distinct codes. Semantic protection returns 409 protected_transition, while
missing, revoked or insufficient credentials return 401/403 access_denied. No
rejection is persisted as a receipt, so later not_known status is never proof of
rejection. Retain the original request ID on every connection, delivery or
timeout failure. Once apply/status has parsed and authorized an identity, an
HTTP deadline or oversized result returns outcome_unknown with that identity.
Preview cannot publish and does not authorize later apply.

Generated OpenAPI and TypeScript contracts include these routes and owned DTOs.
The generated client declaration remains an API artifact; M4.16 separately
qualifies the shared browser controls and real native mutation workflow. The
review-command contract enumerates all eight owned command tags, including
`resolve_comment_conflict` with its required observed revision IDs; its contract
test checks the exact set rather than a variant count.

The typed capability registry also advertises reconcile_operations. This command
requires `base` as a full current operation ID and an exact expected head set
with at least two heads. It merges all heads explicitly; GET and HEAD routes
remain incapable of reconciliation. The base selects report comparison and merge
ordering, while native JJ computes actual common ancestors. API types are
regenerated without browser feature work.

Abandon is an advertised typed command with explicit full commit IDs and boolean
retain_bookmarks/restore_descendants options. Reports include nonempty abandoned
and skipped_hidden lists. Empty lists are omitted to preserve V1 receipt bytes.
An all-hidden request returns HTTP 409 no_changes with zero publications; this
rejection is not a retained receipt. Snapshot and request identity rules remain
identical to other commands. Generated schemas/types expose this API and M4.16
connects the corresponding browser mutation controls.

ResolveConflicts is advertised as resolve_conflicts and accepts a commit plus
1–256 conflict paths with fingerprints. Each resolution is select_term with an
ordered zero-based term index, or delete. A stale fingerprint or a path that is
no longer a conflict returns HTTP 409 stale_conflict; malformed/overlapping
paths and invalid term indices return 400. Preview/apply/status retain the
existing authorization, limits, atomic snapshot and retry behavior. Uploaded
replacement-file actions use the authorized endpoint specified below. M4.17
connects these declarations to the shared conflict editor without changing the
native resolution contract.

M4.24 hosts this workflow in the AppShell structural sheet. Built-in text-side
accept-all controls retain an exact native term selection when the center bytes
match that side; custom edits clear a previous selection until staged. Native
term IDs and fingerprints remain in the typed request even though metadata cards
are omitted from the editor. Uploads capture the selected descriptor so path
switching cannot rebind staged bytes to another conflict.

The conflict sheet reuses the repository file tree in an independently scrolling
left body sidebar. Its leading header toggle retains per-path drafts while
showing or hiding the browser; narrow layouts return to the editor after file
selection. Delete and staged-file actions target the exact active descriptor
from the header, alongside atomic Preview and prepared/recovery actions. The
sheet has no footer or Cancel action. Close retains the existing pending-upload
and mutation guards, and path navigation stays locked while an upload or
prepared mutation binds its captured descriptors.

EditFile is advertised as `edit_file` and carries the exact local bookmark name
and expected single target, canonical source and destination paths, bounded
UTF-8 replacement contents and the new child change description. The same
complete command, snapshot expectation and request ID are sent to preview and
apply; a lost result continues through the existing status route. The generated
OpenAPI and TypeScript unions expose this command on both mutation endpoints.
The browser offers it only for a current ordinary text file reached through an
unambiguous single-target bookmark, then follows a committed receipt to the new
bookmark target and destination path. Historical, conflicted, binary and
oversized selections remain read-only.
