# Configured server search lifecycle

Server TOML now names each indexed repository, its dedicated credential
environment variable, source scopes and bounded polling/storage settings. The
foreground server owns committed-log runners and composes their readers into
authenticated HTTP search. Invalid configuration or missing credentials fail
before listening; runtime index/storage/authorization failures degrade search
independently of authoritative repository serving.

Native lifecycle fixtures run actual background indexers and local Tantivy
files, observe indexed positions through HTTP, reopen the same generation, and
rebuild after deleting only disposable index files while remote checkpoints
remain advanced. They compare full result documents across rebuilt generations
and exact result ordering when reopening the same generation. Source matches are
verified against the exact referenced native file through HTTP.

Expanded fixtures initially assumed equal-score ordering across rebuilt indexes
and expected indexed source bodies to be echoed in stored search documents. The
owned search contract retains immutable ordering within a generation and stores
native file references, not complete source bodies. The corrected fixtures
retain complete document equality, exact same-generation ordering and verified
native source readback; no product semantics assertion was removed.

A separate regression found that a terminated runner could leave its last
healthy watch value behind. Closed health channels now report degradation.
Graceful shutdown waits for retained worker contexts, so an async callback
ending cannot release writer ownership before its blocking work exits. Forced
drop cancels/aborts the async tasks; surviving worker contexts retain their
leases.

Doctor reads live authenticated search status without acquiring index ownership.
It handles revoked credentials and unavailable listeners separately from storage
readiness, rejects redirects and bounds streamed response bytes. Configuration
and state-path fixtures reject inline secrets, duplicate entries, invalid
budgets and symlink escape from private server state.

Cold executable fixtures run both jj-store run and jj store serve-local against
an isolated local endpoint that returns 503. They observe actual background S3
requests, continued foreground serving and clean termination/restart. This is
failure/lifecycle evidence, not real RustFS deployment conformance. E-M6-008 in
the [single ledger](../../spec/src/milestones.md) records exact commands,
failures, results and the remaining remote snapshot/deployment gates.

Final validation passed: 27 native search/projection fixtures, ten cold
executable fixtures, eleven delivery fixtures and ten Tantivy durability
fixtures. Full validation passed with 357 Rust tests, zero failures and six
explicitly ignored external fixtures, all 13 specification lanes, normal checks,
workspace build and formatting. Existing browser tests stayed enabled; no UI
implementation changed. Remote snapshots and real deployment acceptance remain
open.
