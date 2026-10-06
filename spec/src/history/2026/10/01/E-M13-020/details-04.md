---
schema: 1
id: E-M13-020
---

# E-M13-020 details 4

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
cargo fmt --all -- --check
```

**Notes**

Merged Rust formatting passes.
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
pnpm check:progress
```

**Notes**

119 tasks; both merge parents and integration evidence checked.
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
pnpm check:boundaries
```

**Notes**

16 package boundaries pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

491 modules pass unchanged budgets.
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

21 canonical chapters and 141 requirements pass.
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
pnpm spec:first --base 2994c626a214b632d4bf9b1d350c07505fc43ba6 --head e18fd5c0c4eb3bd9a7543a2f5a68747fbada3415
```

**Notes**

Explicit common-ancestor/merge-head qualification: 268 protected files, 15 canonical chapters pass. .scratch/local-browser-merge-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7482 JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu node scripts/dev-demo.mjs stop
```

**Notes**

Isolated mixed service stopped; state preserved. Filesystem fixture already stopped itself. Final listener audit retained only existing default process PID 27703 on 7460; test ports 7482 and 7483 are stopped. .scratch/local-browser-merge-cleanup.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/milestones.md spec/src/ui.md web/tests/real/local-mixed.spec.ts
```

**Notes**

Final merge ledger, canonical UI chapter and cold mixed test formatting pass.
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

JJ 0.45.1, Git SHA-1, macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native and local identities/authorities remain distinct; 84 HTTP routes (66 native and 18 local). Enclosing JJ merge change srlvquzw.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Merge touches shared native/local controller routes, native Thread transport initialization, immutable AgentStore activity projection and public contract consumers. Escalation: complete web runtime/transport tests and type/build consumers, affected Thread/browser interaction/accessibility, focused HTTP/AgentStore/filesystem tests and isolated cold filesystem plus mixed real-RustFS acceptance. Root aggregate, unrelated provider suites, release and visual baseline qualification remain out of scope. Removed only generated compiler caches (task private cache and inactive incremental cache) to free disk for nonincremental qualification; retained default binaries/runtime state and task sealed binaries/logs. Regenerated shared schema artifacts, then requalified direct TypeScript consumers. Display-only controller surface extracted into production-store-view.ts without changing refresh or publication policy. Thread display reset preserves unknown requests by repository/principal, without exposing the native pending request under a local source. Added a cold native Thread create/send/reload consumer alongside mixed navigation.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain qualified merged consumers. M13.4 indexed search/code commands and remaining M15 gates stay pending; visual baselines remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- No change to M13/M15 incomplete milestones; local search, code mutations, full
  reviews, conversion/sync and unqualified local Thread/reaction UI remain
  gated.
- Default development server on port 7460 remains its existing process; source
  merge does not restart it.
<!-- /evidence-field -->

