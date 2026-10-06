---
schema: 1
id: E-M21-011
---

# E-M21-011 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/working-tree-http.test.ts
```

**Notes**

working-client-before.log: missing owned HttpWorkingTree module fails the exact regression suite before implementation; zero tests could import.
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
pnpm catalog:test web/tests/vite/working-tree-http.test.ts
```

**Notes**

working-client-first.log: five transport regressions pass after the owned client/validation implementation; later cancellation and bounds cases are included in the final direct lane.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/working-tree-http.test.ts web/tests/vite/stack-working-tree.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts
```

**Notes**

working-client-owner-first.log: 33 pass, one missing-transport test attempted to inspect a removed spy. Retained the original spy for the no-call assertion; no production assertion weakened.
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.local/stacks-backend-evidence/working-transport-cold-captures.json pnpm catalog:test web/tests/vite/working-tree-http.test.ts web/tests/vite/stack-working-tree.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts
```

**Notes**

working-client-owner-repaired.log: 34 pass, including runtime validation of prior actual cold registered-server responses.
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
pnpm catalog:test web/tests/vite/working-tree-http.test.ts web/tests/vite/stack-working-tree.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/stack-workspace-reads.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

working-client-direct-regressions.log: initial direct boundary coverage passes. Final expanded transport, source and owner cases are recorded next.
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
pnpm catalog:test web/tests/vite/working-tree-http.test.ts web/tests/vite/stack-working-tree.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/stack-workspace-reads.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

working-client-direct-final.log: 67 pass in eight files after expanded UTF-8 ordering, tree/label, aggregate-byte and held-body cancellation cases. This ordinary invocation did not supply cold capture evidence; that case subsequently became an explicit skip and is separately qualified below.
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
pnpm catalog:test web/tests/vite/*stack*.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/working-tree-http.test.ts
```

**Notes**

working-client-controller-regressions.log: complete directly affected controller/history/drag/read/source surface passes 159 tests in 20 files; one explicit real-capture validation skips in the ordinary lane and is separately passed below.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

working-client-production-stories.log: both production application-shell read/preparation interaction and accessibility stories pass in a fresh browser process.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/*.stories.svelte
```

**Notes**

working-client-all-stack-stories.log: all 90 directly affected Stacks interaction/accessibility stories pass in 21 files with a fresh browser process; no screenshot comparisons or baseline updates.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

working-client-web-first.log and working-client-web-final.log: zero errors and warnings.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

working-client-catalog-types.log: zero errors and warnings; boundaries pass for 227 modules.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build
```

**Notes**

working-client-browser-build.log: fresh production bundle, manifest and 139-file documentation closure built. Existing large-chunk and plugin-timing notices retained; no Rust/executable build.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.local/stacks-backend-evidence/working-client-browser-captures.json node scripts/stacks-browser-acceptance.mjs
```

**Notes**

working-client-cold-browser-first.log: one fresh built-browser/isolated filesystem-server test passes against real dirty JJ checkout. Actual path-free current requests, modified README, added unsnapshotted file and ignored obstacle; histories/adjacent diffs remain valid. Operations, bookmarks, companion, physical bytes and checkout/tree_state metadata byte-identical before/after. Fresh fixture and disposable server cleaned up; existing NO_COLOR warning retained.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.local/stacks-backend-evidence/working-client-browser-captures.json pnpm catalog:test web/tests/vite/stack-working-tree.test.ts -t "validator accepts actual cold server observation evidence"
```

**Notes**

working-client-cold-validation.log: focused runtime validator accepts actual responses from this cold browser; one pass and other cases skipped by the exact filter.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```

**Notes**

working-client-ownership.log: production ownership and reference-source boundaries pass; no Community/Changeyard checkout dependency.
<!-- /run -->

<!-- run -->
### Run 16

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

working-client-architecture-first.log and working-client-architecture-final.log: all 1090 source modules within existing ownership/size bounds.
<!-- /run -->

<!-- run -->
### Run 17

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

working-client-boundaries.log: 17 package boundaries pass; no upstream JJ imports added to UI/service orchestration.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:governance
```

**Notes**

working-client-governance.log: all 50 governance/dev-demo/manifest/architecture/ownership regressions pass. Changed isolated fixture script uses existing explicitly built binaries and cleans up only its own state.
<!-- /run -->
<!-- /evidence-field -->

