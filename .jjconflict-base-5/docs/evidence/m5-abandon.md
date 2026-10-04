# Native typed abandon

Abandon accepts distinct full commit IDs and explicit retain_bookmarks and
restore_descendants flags. It uses the pinned native descendant-transform and
reference-update facilities with project-owned authorization, limits and strict
publication. The adapted upstream algorithm is attributed in the third-party
notice and source header.

Targets must belong to committed history. Already-hidden targets are skipped and
reported; if no visible targets remain, no_changes returns without publication.
Root, protected transitions and invalid/uncommitted batches fail atomically.
Native rewrites rebase descendant content by default or preserve it through
reparenting when restore_descendants is true. Bookmarks use native delete/retain
semantics, including conflicted terms and merge parents. Abandoned working-copy
changes receive new empty replacements; multiple workspaces sharing one
abandoned commit retain a shared replacement. Physical working files are
unchanged.

Prepared and retained results add abandoned/skipped_hidden lists. Both default
to empty and are omitted when empty, preserving earlier V1 canonical report and
receipt bytes. Preview never publishes; apply and retries use the existing
strict manifest/receipt boundary. Historical objects and evolution remain
readable.

Five focused native fixtures cover all four option combinations; binary files,
executable modes, copy IDs, symlinks and submodules; adjacent targets; hidden
no-op and mixed hidden/visible input; merge parents and conflicted bookmarks;
multiple workspaces; protected/root/invalid/unknown batches;
evolution/historical reads; and V1 receipt/report compatibility. A separate HTTP
fixture covers preview, apply, status, retry and hidden no-op. Browser controls
remain deferred; generated API types are the only web change.

Exact commands and failure counts are retained in E-M5-005 of the
[single progress ledger](../../spec/src/milestones.md). Native memory-backed
acceptance does not substitute for real storage or deployment qualification.

Aggregate validation passed: normal checks, all 286 Rust tests (six external
tests explicitly ignored), build, formatting and all 13 specification lanes.
Contract validation includes 37 actual responses. Existing browser acceptance
passed eight cold interaction/accessibility tests. M5.2 is complete; conflict
resolution and separate deployment/deferred-browser gates retain their ledger
status.
