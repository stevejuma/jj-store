# Atomic native conflict term resolution

The resolve_conflicts command accepts one visible target and a bounded set of
exact paths. Each path carries the read API's full ordered-term fingerprint and
an explicit select_term or delete action. Selection includes base/remove and
absent terms. Native file IDs, executable modes, copy histories, symlink IDs,
submodule values and subtree IDs retain their semantics.

Every selected path is validated against the original tree before replacement
trees or commits are staged. Invalid, overlapping, missing and stale paths
reject the entire batch. Native rewriting preserves the target change ID,
rebases descendants, updates bookmarks and logical workspaces, and retains
predecessor history. Partial resolution leaves other conflicts and labels in
native form. The existing strict publication guard and durable receipt service
handle preview, apply, retries and status. No editor, shell or UI is added.

Focused acceptance in `crates/jj-store-cli/tests/native_lifecycle/resolve.rs`
covers native variants, five-term conflicts, absent labels, directory selection,
atomic failure before staging, descendant/reference/evolution effects,
protection, permissions, revocation, request identity and HTTP responses. The
shared read-side fingerprint golden fixture remains unchanged. Exact commands,
failures and results are recorded in E-M5-006 of the
[single progress ledger](../../spec/src/milestones.md).

This slice adds only select_term and delete. Authorized uploaded replacement
files remain an M5.3 gate; a raw uploaded_file_id is rejected. Test storage is
not real RustFS or TLS deployment evidence. Browser controls and visual work
remain deferred.

Aggregate checks passed: 291 Rust tests (six external tests explicitly ignored),
40 captured HTTP/SSE response schemas, build, formatting and all 13 canonical
specification lanes. The existing browser lane passed eight cold interaction and
accessibility tests. Service consumers construct resolution types through
service-owned exports. M5.3 remains in progress for authorized uploads.
