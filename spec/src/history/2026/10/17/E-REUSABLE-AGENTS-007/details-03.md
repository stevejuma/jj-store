---
schema: 1
id: E-REUSABLE-AGENTS-007
---

# E-REUSABLE-AGENTS-007 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write .changeset/shared-read-admission.md crates/jj-store-jj/README.md docs/maintenance/coverage.json docs/site/technical/local.md spec/src/current/contracts/architecture/10-pinned-history-query-ownership.md spec/src/current/contracts/architecture/26-reusable-agent-workflows.md spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md spec/src/current/contracts/documentation/03-delivery-and-validation.md
```

**Notes**

Affected authored Markdown and source-review manifest formatting; no prior evidence rewritten.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node .local/reusable-conversations/client-format.mjs
```

**Notes**

All changed authored files pass Prettier, separately from Rust formatting.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

171tasks/864evidence records pass; M33.4isInprogress with only the read-admission prerequisite qualified.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Crate README/package scope also requires its mapped public-distribution chapter. Added the narrow contained-module and native/companion qualification note; no gate removed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

7validators/4182chapters/207requirements pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Final mapped gate:16 protected files / 5 canonical chapters pass, including contained public-distribution review and qualified unchanged-contract evidence.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.99.0; jj-lib0.45.1; parent168f67b0. Original source SHA256: {"query/admission.rs": "621dc4d6b04d3f954a5574267a49d5f20e46879c5715f9a6a0440f4bc5fc6ab4", "query/content.rs": "88b9133478c32d7483b59e70c2e6f6b4d9347ad7b8581f78a7defb2f808ab9d1", "query/language/mod.rs": "a0bff8653cf0f6f3c5d1c4423d73f2bb3c3dee25d67f5c26c9f87c190e4602dd", "query/mod.rs": "9028bacdd7784dae54bc40142cbbc137064c47fbb363662a9d09c61b763e855f", "query/predicates.rs": "c56f924fc61625830e93604e7258039d6e8110d1e0e752d807e99be77071f86f", "query/workers.rs": "4252dd39f2188d9f7f086ea47a8e7edcfacab38a83014b32cc3856bcb2a132ea"}. Independent Notes persistence seam separately committed23da9d80 with604catalog/389AIpolicy checks; not a JJ backend qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native/companion publication, operation and source membership, derived cache formats, GraphError/GraphStats schemas, all budget values and worker semaphore sharing unchanged. The new read_support modules are private and remain native-gated; no independent local-history feature is exposed yet. No new dependency or wire/persistence encoding.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

M33.4 source prerequisite: separate owned errors/diagnostics, limits, accounting, cancellation and non-queuing worker admission from native query/store execution, retaining original public re-exports. Cancellation uses the existing shared-token accessor; clone admission test now proves a real shared permit through public status rather than private field identity. Full owning crate and direct server/schema consumers cover native and local readers. No UI/RustFS/provider/Tauri/Notes functional claim for this slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Expose focused local-history without native object/WAL dependencies by separating remaining query/cache helpers, then admitted Tauri reader and native capture ownership.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-007 extraction preserves JST-JJ-003/005 pinned read and bounded
  rebuild semantics; JST-API-001/004/006 errors, statistics and all generated
  OpenAPI remain exact. All native/local public query paths re-export the same
  owned types. No source authorization, store authority, budget, queue or
  persisted format changes. JST-DOC-001/002 source review explicitly leaves
  independent history/Tauri work unqualified.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M33 remains In progress. M33.4 enters In progress for shared
  read-admission prerequisites; its independent feature, bridge and native
  event-owner gates remain required. M33.3 retains store/workspace/client/UI
  work and M33.5-.8 remain Planned. The separate Notes seam qualifies no JJ
  backend.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change utywrnpw
<!-- /evidence-field -->

