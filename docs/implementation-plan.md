# jj-store implementation plan

Accepted 26 September 2026. The canonical requirements in `spec/src` govern
behavior. [milestones.md](../spec/src/milestones.md) is the single
implementation progress ledger. The [original attachment](reference/README.md)
is archived unchanged as non-normative reference material.

## Summary

Create `/Users/stevejuma/code/lapismd/jj-store` as an independent Rust workspace
initialized with `jj git init --colocate`. Follow jj-nostr's specification, CLI
presentation, validation, and verified-slice commit conventions.

The first release supports **new native repositories** through a custom JJ CLI,
RustFS-backed server, structured APIs, browser UI, typed mutations, and Tantivy
search. Shared access uses **repository-scoped tokens**. Passive replication and
existing-repository migration remain explicit follow-on milestones.

The development repository is Git-colocated; repositories served by the product
use native JJ storage without requiring Git, SQL, or Nostr.

## Execution scope update — 27 September 2026

The user reopened Storybook-only work: directly adapt Community's shell,
navigation, graphs, branch rows, files and diffs. Branches (JJ bookmarks), JJ
Log and Op Log are separate pages; fixture state demonstrates drill-down to
diffs. M4.4 owns tooling/provenance, M4.5 shell mechanics, M4.6 repository
views. Production UI integration, mutations/search wiring and deployed
acceptance remain deferred under M4.3. The existing browser and backend work
remain preserved.

The user subsequently reopened production browsing integration. M4.9–M4.14
promote the reviewed Storybook application into the served browser, add missing
bounded read contracts, typed hash restoration, explicit SSE refresh,
virtualized collections and persistent layout. M4.15 adds the authenticated
repository command search against the selected Tantivy snapshot. M4.16 adds
preview/apply/status controls for core repository commands using the durable
native mutation contract. Conflict-editing controls remain deferred within M4.3
under M4.17.

M4.10 adds separate signed browsing windows for bookmark/change lookup,
recursive paths with bounded last-change attribution, operation changes grouped
by native change ID and exact comparison paths. The prior complete read routes
remain compatible. Stable record/path anchors can begin forward or backward
windows; opaque server cursors remain process-local continuations and never
become durable URLs.

M4.13 proceeds in verified slices. The first slice persists shell, per-page
Column Canvas and file-tree geometry through versioned layout keys, injects
isolated memory storage in Storybook, and bounds the existing fixture-backed
bookmark, graph, changed-file, directory and file-tree collections. Remaining
work continues commit/operation inspection cursors, removes eager recursive tree
reads, adds lazy directory expansion, validates the upstream FileListing change
in Community and consumes a released Design Core version before M4.14.

## Repository foundation and progress tracking

Create a tracked agent guide, unchanged attachment archive, this execution plan,
a canonical mdBook specification, and the single progress ledger. The book
covers architecture, identity/codecs, object storage, publication/recovery, JJ
integration, CLI, APIs/mutations, security, adapters/search, compatibility, and
verification.

Use `JST-<AREA>-NNN` requirement IDs, exactly one verification row per
requirement, and mapped spec-first checks through the released
`@lapismd/spec-validator`. Preserve `pnpm spec:first`, `spec:validate`,
`spec:build`, `spec:check`, `check`, `test`, and `build` entry points.

The ledger contains milestone gates and smaller tasks such as M2.1, recording:

- Planned, In progress, Blocked, Partial, Done, or Deferred status.
- Dependencies, acceptance criteria, requirement IDs, remaining gaps, next
  action.
- Named tests, exact commands, actual results, failure counts, evidence links.
- Upstream revisions, storage/API format changes, changed files.

Require evidence in the same Jujutsu change as every implementation slice, even
when milestone status does not change. Update status when gates advance or
regress. Reference the enclosing change rather than a self-referential hash.
Documentation-only edits update the ledger when they change scope, gates,
status, or cited evidence.

The progress checker runs in normal and specification checks. It validates IDs,
dependencies, evidence links/test names, completion requirements, and
same-change updates. A passing checker validates bookkeeping; tests establish
completion.

## Architecture and public contracts

Create model, object storage, WAL, JJ integration/codecs, service, RPC, HTTP,
adapters, Tantivy, CLI, server, and testkit crates as milestones begin. Native
codecs initially live with JJ integration. Storage/publication never depend on
UI, search, or future projections.

Pin jj-cli and jj-lib together at 0.45.1, upstream revision
`7c41cdeb16b6b321c64e789a966b6adf723816a5`. Start with Rust 1.97.1, edition
2024, and Apache-2.0. Commit dependency locks, reference revisions, and
attribution. Use pinned SimpleBackend identity and SimpleOpStore operation/view
hashing as references. Complete project codecs include copy histories and
submodule entries missing from SimpleBackend, native ordering, signatures,
roots, and absent versus empty predecessors. Envelope checksums are distinct
from native IDs.

Immutable objects, dependency inventories, receipts, and attempt-unique WAL
candidates precede one conditional manifest update. Only selected records are
committed. Native head updates preserve siblings; strict mutations reject stale
snapshots. Durable receipts resolve retries/lost acknowledgements after restart
and checkpointing.

Register remote Backend, OpStore, and OpHeadsStore through CliRunner. Server
stores run in process and share the publication coordinator. Physical checkouts
remain client-local; native views contain logical workspace mappings. Use
protobuf/tonic RPC and Axum versioned JSON/SSE with generated OpenAPI/client
types. Both transports enforce the same principal, limits, and semantic policy.

### CLI

Build custom `jj`, extending `jj store`: init, attach, status, capabilities,
doctor, serve-local, credential login/logout. Expose server `jj-store` with
explicit run, configuration, diagnostics, and token administration.

Follow jj-nostr custom-command Markdown help, short/long help, keyword help,
Hint errors, JSON output, TTY paging, color controls, completions, and build
information. Preserve normal upstream JJ behavior and publish tested
capabilities. Use explicit development build paths and never replace installed
jj.

### APIs and browser

Implement the attachment's /v1 read routes, snapshot-bound pagination, committed
SSE, mutation preview/apply/status, and bounded conflict-content uploads.
Initial commands: New, Describe, bookmark create/move/delete, Abandon,
ResolveConflicts, ReconcileOperations. Server rebase/squash/undo follow later;
ordinary CLI support has separate parity coverage.

Use Svelte 5, TypeScript, and Vite, served by the server. Include repository
selection, snapshot state, commit/operation/evolution tabs, file/diff
inspection, conflict editing, mutation previews/results, search, and index
freshness.

### Authentication

Use revocable repository read/write/admin tokens and a separate operator
capability for repository creation. Store token verifiers server-side and
credentials outside checkouts. Browser tokens stay in memory. CLI uses the OS
credential facility with environment injection for automation. Authorize before
objects, search, previews, and publication. Shared deployments require TLS;
unauthenticated loopback requires an explicit development setting.

## Implementation sequence and gates

| Milestone     | Implementation                                                                                                                                                    | Completion gate                                                                                                                      |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Bootstrap     | Colocated repository; governance, specification, ledger, Rust/tooling skeleton, boundaries, validation                                                            | Book/governance checks pass and foundation committed independently                                                                   |
| M0            | Pin references and RustFS image/topology; compile upstream custom-backend and RPC integration; audit traits/codecs; prove context propagation and bounded queries | Compatibility matrix, identity golden fixtures, real RustFS conditional-write evidence; failed assumptions block dependent work      |
| M1            | Typed IDs, envelopes, complete codecs, streamed S3, integrity checks, fault storage                                                                               | Every variant round-trips with correct hash; explicit corruption failures; real create/read/range/CAS passes                         |
| M2            | Dependency inventories, unique WAL candidates, CAS, receipts, unknown outcomes, committed traversal, checkpoints/catalogs, cache-free recovery                    | Attachment fault matrix including late duplicates, competing candidates, missing dependencies, checkpoint races                      |
| M3            | Native/in-process/RPC stores, create/attach, tokens, guarded publish, JJ indexes, physical workspaces                                                             | Create/edit/describe/log/restart/op-log/evolog without Git/SQL/relays; concurrency preserves history and local edits                 |
| M4            | Pinned reads, graph/revset/tree/file/diff APIs, operation diff, SSE, generated contracts, browser                                                                 | Fixed-operation CLI/API parity; zero GET writes; coherent pagination; explicit multi-head state                                      |
| M5            | Preview/apply/status; basic commands, abandon, resolve, reconciliation, shared semantic policy                                                                    | Correct descendants/bookmarks/workspaces/evolution; stale/invalid requests publish nothing; duplicate receipts; no RPC policy bypass |
| M6            | Committed adapter runner, bootstrap/checkpoints, Tantivy deterministic updates, durable payloads, scopes, freshness, rebuild                                      | Replay equivalence, safe index/cursor crashes, lost-index rebuild, fresh access control                                              |
| M8a           | Deployment examples, packaging, restore, diagnostics, limits, measured performance                                                                                | Pinned cold recovery/security/semantic/browser/storage/release checks pass; limits published                                         |
| M7, deferred  | Complete replica inventory, verified transfer, publish-last destination, lag, fenced manual promotion                                                             | Read-only complete history through applied position after primary shutdown                                                           |
| M8b, deferred | Profile audit, lossless import rehearsal, verification/rollback                                                                                                   | Exact guarantees demonstrated; incompatible profiles rejected; failed import leaves source untouched                                 |

Retain the attachment identifiers; split M8 so first-release hardening precedes
migration. Live Nostr/Git adapters, agent capture/materialization integration,
active-active storage, destructive GC, and automatic failover are out of v1.

## Validation and completion

Start at the narrowest boundary and record before failures; rerun that exact
target after edits, then the complete impact surface and required wider lanes.

Native fixtures cover merges, divergent changes, split/squash/rebase, abandon,
conflicted bookmarks, multiple workspaces, copies, symlinks, executable files,
binary content, signatures, and invisible predecessor commits.

Publication combines randomized replay tests with real RustFS races, dropped
responses, process kills, restart, missing dependencies, corruption, checkpoint
recovery. Fake evidence does not replace deployment conformance.

Mutation/access coverage includes stale previews/fingerprints, atomic multi-path
resolution, conflicting idempotency reuse, cross-repo access, revocation, and
crafted low-level publications. UI checks cover interactions/accessibility, safe
content, resumable SSE, cold builds/server lifecycle, and visual-pending
stories; visual baselines need approval.

Operational tests delete derived caches and recover acknowledged history,
rebuild search, distinguish storage readiness from degraded adapters, and record
requests, bytes, CAS retries, and indexing lag.

Commit every verified slice with canonical requirements and ledger evidence.
Record unexplained failures; compilation or a happy-path demo never establishes
completion. First release requires CLI/browser history parity, authenticated
abandon/resolve, rebuildable search, and acknowledged publication durability
under the documented failures.

### Catalog follow-up requested 27 September 2026

M4.7 adds a typed in-memory controller and complete application stories
alongside the simple shells. Reuse Community repository file tree, branch/change
picker, file-content viewer and left-sidebar profile/login footer. Match
Community fonts and icon geometry through the same shared stylesheet build
setup. All actions remain local; production integration and authentication
remain deferred.

M4.8 applies catalog review feedback: full-width bookmark cards with dividers
except the final visible card, sticky secondary headers, aligned graph avatars,
Community breadcrumbs/sidebar icons/transparent details rail, and copied
directory navigation when no file is selected. Branches, JJ Log and Op Log use
column titles instead of redundant body titles; mobile navigation remains
available. Production UI scope stays deferred.
