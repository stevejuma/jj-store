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
7. Run `pnpm spec:check`, `pnpm check`, `pnpm test`, and `pnpm build` for
   executable changes. Documentation-only changes run formatting and consuming
   checkers.
8. Commit each verified slice with Jujutsu, using an explicit message. Use the
   enclosing change as the ledger's revision, not its own future commit hash.

Never call a milestone Done from compilation, substituted tests, or screenshots.
Keep unresolved failures and incomplete gates explicit. A failed M0 storage or
identity proof blocks dependent implementation; do not weaken the contract. Real
RustFS evidence is distinct from fake-store evidence. Cold runs are required for
browser/lifecycle changes. Visual baselines require explicit approval.

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

The user deferred further UI changes on 27 September 2026. Preserve the existing
browser implementation and continue non-UI backend, API, CLI, storage, search
and operational work. UI features and browser deployment acceptance resume only
when requested. Track that deferred scope in M4.3 of the single progress ledger;
existing functional checks remain enabled.

## Local execution

Use explicit development binary paths. Do not replace the installed `jj`,
install services, restart unrelated daemons, publish packages, or migrate
existing repos as part of a development slice. Disposable test services must be
isolated from existing deployments and cleaned up after use. No user credentials
in fixtures.

Generated `target/`, `node_modules/`, and `spec/book/` are ignored. Keep
secrets, local loader/editor configuration, and parent workspace guidance
uncommitted.
