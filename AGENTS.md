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
Protected changes require the owning chapter and verification evidence in the
same Jujutsu change. The change map is in `spec/src/spec-governance.md`.

Keep storage, model, and WAL independent of UI, Tantivy, Git transport, SQL,
Nostr, and controller runtimes. Keep upstream JJ imports in `jj-store-jj` and
`jj-store-cli`; service code consumes owned interfaces. Client working-copy
files are not server state. Credentials never belong in a repository checkout.

## Workflow and progress

1. Inspect `jj --no-pager st`; preserve unrelated work. Stop on overlapping
   work.
2. Read the owning specification and `spec/src/milestones.md`.
3. Name the narrow acceptance/regression target and record its before state.
4. Update canonical requirements before or with implementation.
5. Add regression evidence and run the exact target, then all required wider
   lanes from the workspace test-scope ladder.
6. Update the single progress ledger in the same change: actual commands,
   results, failures, files, versions/formats, gaps, and next action. Add an
   evidence entry even when milestone status has not changed. Documentation-only
   work updates the ledger when it changes scope, gates, status, or evidence.
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
Git-backed companion profile in `spec/src/local-jj.md` implements immutable
metadata selected by an owned Git ref, shared WorkStore semantics and
server-free project/task/comment CLI, local agent capture and portable
provenance, and repository-wide review persistence. Portable code/agent review
anchors, review CLI qualification, automatic hook code associations and the
native portable evidence carrier remain pending. Filesystem-only and mixed
LocalTrusted browser serving now supports explicit device folder registration,
operation-pinned code reads, companion-pinned WorkStore editing and qualified
recorded AgentStore reads. Indexed search, recoverable code commands, full
reviews, local conversion and cold full-flow qualification remain pending.
Storage/identity qualification blocks dependent stages; the milestone ledger
owns the exact supported stages and acceptance evidence.

Production components live under `web/src/lib`; catalog fixtures and harnesses
may import them, while production MUST remain independent of catalog state.
Inspect Community first and Design Core second before introducing presentation
components. Fix a shared-library deficiency in its owning repository and consume
only a released registry version in jj-store. Credentials stay memory-only; SSE
announces freshness and displayed snapshots change only on explicit refresh.
Stories remain visual-pending; baseline approval is separate from interaction
and accessibility acceptance. Manual-token authenticated repository discovery is
implemented. Expanded replication pressure/fault qualification, its browser UI,
cold two-instance real-RustFS qualification, archive/export tooling, additional
deployment topologies, shared release qualification, controller dispatch,
backup/restore, stock-JJ filesystem mirroring, continuous Git mirroring, Nostr
export, other harnesses and CRDT semantics remain deferred.

## Local execution

Use explicit development binary paths. Do not replace the installed `jj`,
install services, restart unrelated daemons, publish packages, or migrate
existing repos as part of a development slice. Disposable test services must be
isolated from existing deployments and cleaned up after use. No user credentials
in fixtures.

Generated `target/`, `node_modules/`, and `spec/book/` are ignored. Keep
secrets, local loader/editor configuration, and parent workspace guidance
uncommitted.
