---
schema: 1
id: E-M11-004
---

# E-M11-004

<!-- evidence-field:id markdown -->
## Id

E-M11-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

main-workspace-integration
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change msxvwtvr.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean main default workspace at fd58dc7c, clean native-project-tasks feature workspace at 1bdb15c4. JJ merge preserves both histories; only milestones.md conflicts textually. Combined baseline route codec passes seven tests; focused Work/evolution controller lane passes eleven. Web/catalog type checks have zero errors/warnings; architecture passes 312 modules. No backend, transport, schema, harness or dependency differences from the verified feature tip.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
- spec/src/ui.md
- spec/src/api.md
- web/tests/vite/store-route-history.test.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- scripts/check-progress.mjs
- scripts/governance.test.mjs
- spec/src/spec-governance.md
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- web/tests/real/native-repository.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Dependency resolution and existing temporary source-validation setup retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing WorkStore manifest format 3 and local WAL authority unchanged. Evo Log full commit anchor and WorkStore project/task identities coexist in shared routes; page changes clear incompatible selections while browser history retains each destination and full code operation pin. No format migration is executed by this integration. Progress validates every merge parent and rejects imported parent evidence as new integration evidence. Published spec-first merge qualification uses explicit common-ancestor and current commit IDs; no validator package patch.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Merged UI controller/history and shell combine independent main Evo Log/conflict/editor presentation with WorkStore Projects. This multi-consumer integration justifies all web/catalog unit tests, directly affected story families and the complete six-scenario real-browser suite on a fresh physical stack. Backend code/transport trees are byte-identical to nrtskmus; reuse E-M11-001/002/003 native, RPC/HTTP/CLI and RustFS evidence. Unrelated root aggregates/full Rust workspace are out of scope; visual baselines remain visual-pending. Progress CLI governance change is shared source infrastructure; run the full direct-consumer development/governance/architecture source suite, including a real disposable JJ merge and parent-evidence variant rejection. This does not alter storage/runtime startup.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-004 details 2](details-02.md)
- [E-M11-004 details 3](details-03.md)
- [E-M11-004 details 4](details-04.md)
