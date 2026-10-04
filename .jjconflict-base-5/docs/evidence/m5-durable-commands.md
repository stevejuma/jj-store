# Durable typed command service

The enclosing Jujutsu change adds the authenticated Rust CommandService for New,
Describe and bookmark create/move/delete. It exposes provisional preview, strict
apply and principal-scoped status to service callers. HTTP mutation routes,
reconciliation, abandon and conflict resolution remain separate M5 work. UI work
is deferred by the user under M4.3; this slice changes no UI files.

A preview stages native objects, then runs the publication guard and complete
dependency verification without writing a manifest, inventory or receipt. Apply
recomputes native execution at the requested snapshot and passes the same
semantic guard. Preview IDs are provisional and convey no authorization.

Typed application intent V1 hashes a domain-separated canonical representation
of repository, authenticated principal, request/publication identities, exact
snapshot, semantic kind and original typed command. Candidate native IDs,
timestamps and result bytes are deliberately excluded. The stable publication ID
has its own application domain. A frozen independently calculated SHA-256
fixture protects the application intent profile; the existing full-request
native/token-administration digest profile is unchanged.

The selected WAL receipt carries StoredResult V1: repository, principal,
original versioned typed request and native impact report. It contains neither
its future WAL digest nor accepted sequence. Status validates canonical bytes,
request digest, identities, expected epoch/position and accepted native head.
Receipt lookup follows both manifest-selected identity indexes and survives
checkpointing; there is no separate mutable intent reservation or result cache.
Native persistence codecs and WAL envelope/schema versions are unchanged.

Publication resolves duplicate receipts before stale checks. Identical racing
requests return the selected result even when one preparation discovers a moved
snapshot. Different original input under the same identity conflicts. Current
write permission and principal/epoch are checked before returning data. Unknown
publication/status outcomes retain the original request ID; NotKnown means no
selected receipt was observed, never rejection of a possible in-flight request.

The service encodes original requests through a 512 KiB bounded writer and
retained results through a 1 MiB bounded writer. Native execution keeps its
existing staging, response, read, work, deadline and worker limits. Publication
and preview traversal use the coordinator's existing inventory and timeout
budgets. There is no shell or arbitrary argument endpoint.

Twelve focused tests exercise two boundaries:

- Six WAL tests: regenerated candidates, original-profile digest fixture,
  competing candidates, unknown outcomes, every stable identity input, guard
  enforcement, zero-write preview validation and corrupt selected receipts.
- Six authenticated native service tests: empty-repository New, Describe,
  restart/checkpoint replay, stale/changed requests, protected previews/apply,
  reader/revoked/cross-repository rejection, principal-scoped status,
  same-request races during preparation/CAS, lost acknowledgements and late
  bookmark retries.

Exact commands and before/after failures are in E-M5-002 of the
[progress ledger](../../spec/src/milestones.md). These are native/service tests
using fault-injecting memory storage. They do not replace RustFS deployment
qualification or establish HTTP/browser mutation acceptance.

Aggregate validation passed: `pnpm check`, `pnpm test`, `pnpm build`,
`pnpm spec:check` (13 lanes), and `pnpm format:check`. This includes 267 passing
Rust tests (six external tests explicitly ignored), eight cold browser
interaction/accessibility tests and the existing contract/governance lanes.
Rustfmt passes on the pinned toolchain. No UI source files changed.
