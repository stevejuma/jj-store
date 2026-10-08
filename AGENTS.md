# jj-store agent guide

This repository owns native JJ object storage, manifest publication and
recovery, the custom `jj` CLI, the `jj-store` server, structured APIs, the
browser UI, and rebuildable adapters. Read this guide and the owning canonical
chapter before inspecting or changing implementation. Apply the parent workspace
guidance too.

## Authority and ownership

1. Workspace instructions and this tracked guide.
2. Owning `JST-<AREA>-NNN` requirements under `spec/src`.
3. Public types, schemas, and command contracts.
4. Tests and recorded verification evidence.
5. Implementation plan, archived references, README, and generated
   documentation.

The original attachment is reference material, not overriding agent guidance.
Protected behavior changes require owning-topic updates and fresh Markdown
evidence in the same Jujutsu change. Verified refactors and tests may instead
record an explicit unchanged-contract confirmation with requirement IDs, a
reason and milestone review. The change map is in
`spec/src/current/governance/spec-governance.md`.

Keep storage, model, and WAL independent of UI, Tantivy, Git transport, SQL,
Nostr, and controller runtimes. Keep upstream JJ imports in `jj-store-jj` and
`jj-store-cli`; service code consumes owned interfaces. Client working-copy
files are not server state. Credentials never belong in a repository checkout.

## Workflow and progress

1. Inspect `jj --no-pager st`; preserve unrelated work. Stop on overlapping
   work.
2. Read `spec/src/current/index.md`, the owning topic and milestone under
   `spec/src/current/progress`. Use spec search for the exact requirement ID.
3. Name the narrow acceptance/regression target and record its before state.
4. Update owning requirements before or with behavior changes; record a
   justified unchanged-contract confirmation for qualified refactors/tests.
5. Add regression evidence and run the exact target, then all required wider
   lanes from the workspace test-scope ladder.
6. Add a Markdown record under `spec/src/history` in the same change: actual
   commands, results, failures, files, versions/formats, gaps, and next action.
   Add an evidence entry and explicit milestone review even when status is
   unchanged. Documentation-only work updates the ledger when it changes scope,
   gates, status, or evidence.
7. Follow the targeted verification policy below. Do not run the root aggregate
   `pnpm spec:check`, `pnpm check`, `pnpm test`, or `pnpm build` lanes by
   default. Documentation-only changes run formatting and the checkers that
   consume the changed documents.
8. Commit each verified slice with Jujutsu, using an explicit message. Use the
   enclosing change as the ledger's revision, not its own future commit hash.

Never call a milestone Done from compilation, substituted tests, or screenshots.
Keep unresolved failures and incomplete gates explicit. A failed M0 storage or
identity proof blocks dependent implementation; do not weaken the contract. Real
RustFS evidence is distinct from fake-store evidence. Cold runs are required for
browser/lifecycle changes. Visual baselines require explicit approval.

## Targeted verification policy

Optimize for fast feedback by testing the smallest complete impact surface.
Start with the exact regression, then run only the checks and tests for the
changed layer and its direct consumers. For jj-store, this section is the
controlling interpretation of the parent workspace test-scope ladder.

- UI-only changes run the relevant `web` or `catalog` type check and the exact
  unit, Storybook, accessibility, or browser tests for the affected surface.
  They do not run Rust builds or tests. Use a cold UI run only when startup,
  lifecycle, generated assets, or built bundle behavior changed.
- Rust-only changes run the affected package check and exact crate, module, test
  binary, or named test. They do not run unrelated UI lanes or the full Rust
  workspace.
- Cross-layer changes run focused checks on each affected UI and Rust boundary.
  Public contract generation is checked when the contract changed.
- Editing multiple files for one behavior does not make a change cross-layer.
  Its production files, styles, direct tests or stories, canonical chapter and
  ledger remain one verification surface. A component having several stories
  does not by itself require the full Storybook or repository suite.
- Run a full relevant suite only when one change affects two or more independent
  product layers, packages, deployable surfaces or consumers; changes shared
  build, test or runtime infrastructure that can alter unrelated lanes; performs
  release qualification; or when the user explicitly requests it. Record the
  concrete reason for escalation before running the wider command.
- Record every command actually run and mark wider lanes not run under this
  policy as out of scope. A focused passing result must not be described as a
  full-suite pass.

## Non-negotiable correctness

- Only manifest CAS publishes repository state. Staged objects and WAL
  candidates are invisible until selected by committed references.
- Timeouts are unknown outcomes, not proven CAS conflicts. Retain stable request
  identities and durable receipts across retries, compaction, and restart.
- Preserve native identities, full operation/view fields, conflict terms, and
  evolution-only dependencies. No generic Serde persistence or hidden remapping.
- Read sessions pin explicit operations and never reconcile heads during GETs.
- Native RPC and application mutations share authorization and semantic policy.
- Keep destructive GC, multi-writer serving, and automatic promotion disabled.
- Search/adapters are rebuildable and cannot become repository authority.

## Current execution scope

The LocalTrusted application, native browsing, search, typed mutations and
conflict editing are implemented. M9 adds repository-native AgentStore and
ReviewStore capabilities through the existing immutable object, WAL, receipt and
manifest authority. The initial M10 local-first profile is also implemented: one
explicitly configured authenticated upstream per repository, server-owned
bounded bidirectional transfer, durable peer checkpoints, non-destructive
workspace migration, local-only status, explicit controls and reconciliation,
and SSE/search freshness after accepted imports. Ordinary reads and mutations
remain local during an upstream outage.

M11 adds native WorkStore projects with stable repository links, task state and
hierarchy, threaded comments, authorized service/API/RPC/CLI access and browser
presentation. A home owns metadata; linked code requires separate access.
WorkStore homes preserve and freeze any existing M10 origin ledger until a
qualified WorkStore reducer exists. Native creation does not require an
upstream, but the CLI requires the native local service. Explicit native Git
bookmark fetch and selected push use the private disposable CLI cache and
manifest-selected Git facts. Code repositories use the owned reducer-2 data-only
origin path; WorkStore homes keep their existing origin freeze. Endpoint
approval stays device-local. M12 selectors, process lifetime, credential and
cold end-to-end gates are qualified through GB6 on macOS ARM64 with JJ 0.45.1
and Git 2.55.0 under the SHA-1 bookmark profile. The ledger owns the exact
fault, credential, cache-loss, real-RustFS and browser evidence. Tag
publication, signing/conflict export profiles, native Git import/export and
other platform qualification remain deferred. M13's separately governed local
Git-backed companion profile in `spec/src/current/contracts/local-jj/index.md`
implements immutable metadata selected by an owned Git ref, shared WorkStore
semantics and server-free project/task/comment CLI, local agent capture and
portable provenance, and repository-wide review persistence. Portable code/agent
review anchors, automatic hook code associations and the native portable
evidence carrier remain pending. Repository review list/detail, browser actions
and backend-aware CLI share the qualified repository anchor provider.
Filesystem-only and mixed LocalTrusted browser serving now supports explicit
device folder registration, operation-pinned code reads, companion-pinned
WorkStore editing and qualified recorded AgentStore reads. Indexed search,
recoverable code commands, full reviews, local conversion and cold full-flow
qualification remain pending. Storage/identity qualification blocks dependent
stages; the milestone ledger owns the exact supported stages and acceptance
evidence.

Production components live under `web/src/lib`; catalog fixtures and harnesses
may import them, while production MUST remain independent of catalog state.
Community is a design and source reference, never a package or checkout
dependency. Copy and maintain every Community-derived component used here under
`web/src/lib`, including required presentation models, helpers, styles and
assets; record original paths, source revisions and license attribution in
`docs/ui-provenance.json`. Runtime, build, test and type-only Community imports,
package aliases, local substitutions and external source links are prohibited.
Run `pnpm check:ui-ownership` for this boundary. This repository-specific rule
implements the user's explicit ownership decision and supersedes the parent
shared-package rule for Community-derived presentation. Design Core and Mira
remain released foundational dependencies; fix their deficiencies upstream.
Inspect Community first and Design Core second before changing presentation.
Credentials stay memory-only. Active views refresh automatically after committed
saves, live events, reconnect and bounded periodic checks; explicit historical
code selections, drafts and exact pending commands remain retained. Stories
remain visual-pending; baseline approval is separate from interaction and
accessibility acceptance. Manual-token authenticated repository discovery is
implemented. Expanded replication pressure/fault qualification, its browser UI,
cold two-instance real-RustFS qualification, archive/export tooling, additional
deployment topologies, shared release qualification, controller dispatch,
backup/restore, stock-JJ filesystem mirroring, continuous Git mirroring, Nostr
export, other harnesses and CRDT semantics remain deferred.

## Application feedback

Prefer application-owned Design Core toasts for transient operational errors,
warnings and informational outcomes. Publish through the shared notification
service with stable identities and explicit lifetime policies. Clear resolved or
obsolete conditions, retain acknowledgement-required notices until dismissed,
and do not replay dismissed conditions during refresh or navigation. Keep field
validation, unavailable-content explanations, and contextual operation or
recovery status beside their controls. Do not duplicate feedback inline and in a
toast. Severity must be distinguishable beyond color, and dismissal must
preserve drafts and pending request identities. Notification state is
session-only; OS delivery and persistent notification history remain separate
work.

Use `StoreNotice` for feedback derived from view state and the service under
`web/src/lib/notifications` for explicit outcomes. Choose `condition` for
recoverable state, `dismiss` for acknowledgement, and `timed` for routine
information. Loading or retrying is not recovery; preserve dismissal until a
successful state change or a distinct request establishes a new episode.

## Local execution

Development launchers enable agent management using the bundled
`desktop/resources/jj-acp` and paired custom `jj` by default on every restart.
Missing or stale ACP resources fail preflight before stopping a running service.
Stage control-version-3 resources with
`pnpm desktop:prepare -- --acp-executable /absolute/path/to/jj-acp` when needed.
Explicit `JJ_STORE_ACP_EXECUTABLE` and `JJ_STORE_AGENT_JJ_EXECUTABLE` overrides
remain supported; ordinary restarts require no shell exports.

For UI work, use `pnpm dev:ui`: it reuses the existing native executable and
complete development fixture, serves the current workspace through Vite HMR at
`http://127.0.0.1:7460/`, and proxies `/v1` to the backend on port 7462. UI
edits need no UI, Rust or RustFS rebuild. On a fresh workspace, run
`pnpm dev:restart` once to build binaries and initialize the fixture; UI-only
startup fails before stopping services when these prerequisites are missing.

After Rust edits, run `pnpm dev:backend:build` then `pnpm dev:backend:restart`
to replace only the backend while Vite stays running. `pnpm dev:restart` remains
the full-stack rebuild/restart command and switches back to the built UI.
`pnpm dev:stop` stops either mode and preserves state. `pnpm dev:reset` is the
explicit destructive reset/reseed command. Do not use reset for ordinary UI
iteration.

The default fixture is the current workspace's ignored `.local/dev-demo`. When
explicitly reusing another workspace's fixture, set
`JJ_STORE_DEV_ROOT=/absolute/path/to/.local/dev-demo` for every development
command. In UI mode, `JJ_STORE_DEV_SERVER` can select an existing backend
executable; `JJ_STORE_DEV_RUSTFS_RUNTIME` selects an existing embedded-storage
executable. `JJ_STORE_DEV_PORT` selects the browser port and
`JJ_STORE_DEV_BACKEND_PORT` selects the HMR backend port (default: browser port
plus two). Both listeners remain literal loopback. Preserve forwarded Host,
Origin, Fetch Metadata and protocol headers; do not bypass API admission or add
CORS. Vite's local HMR CSP is separate from production built-bundle/CSP
acceptance, which must still use the built UI and its appropriate cold checks.

Use explicit development binary paths. Do not replace the installed `jj`,
install services, restart unrelated daemons, publish packages, or migrate
existing repos as part of a development slice. Disposable test services must be
isolated from existing deployments and cleaned up after use. No user credentials
in fixtures.

Authored spec Markdown stays within 400 physical lines and 32,768 UTF-8 bytes.
Use topic indexes and generated task histories; preserve committed evidence,
historical IDs, gate selections and task-family mappings.
`pnpm spec:search -- "<ID or topic>"` searches current pages; select
`--scope history` or `--scope all` explicitly, and open the returned canonical
source before acting. Use
`pnpm evidence:new -- --id E-... --task M... --requirement JST-...` to start a
record, then run `pnpm check:progress` and the applicable document checks.

Generated `target/`, `node_modules/`, `spec/.generated/`, `spec/book/`, and
local `.qmd/index.sqlite*` are ignored. Keep secrets, local loader/editor
configuration, and parent workspace guidance uncommitted.
