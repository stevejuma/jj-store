---
schema: 1
id: E-M11-005
---

# E-M11-005 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

40/40 direct controller tests in7 files. Earlier31/31,36/36 and39/39 runs also passed. Covers independent sidebar filtering/pins, review authorization/cache removal, synchronous busy refresh, fail-closed project Reviews, Back during pending code/reference reads and full native thread navigation.
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
pnpm web:test
```

**Notes**

60/60 Node tests, including full-ID route codec and legacy operation pins.
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
pnpm web:check
```

**Notes**

Final UI check0 errors0 warnings after asynchronous code/restorer guards.
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
pnpm catalog:check
```

**Notes**

Corrected stories/type check0 errors0 warnings; catalog84 modules.
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
pnpm test:governance
```

**Notes**

34/34 including owning asset-manifest regressions.
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
pnpm check:architecture
```

**Notes**

344 production/catalog modules within unchanged counted-line budgets.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass; production remains independent of catalog fixtures.
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
pnpm spec:validate
```

**Notes**

20 canonical chapters125 requirements,4 validators.
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
pnpm spec:first
```

**Notes**

75 protected files mapped to9 changed canonical chapters before final ledger completion.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

Earlier selectors run reached comment edit then stale work; busy was observed before discovery began. Refresh now starts busy synchronously and mutation controls stay disabled through discovery/sidebar/current-view reads.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

Two final runs49.3s and similar reached project edit, links, hierarchy/state, external SSE, reply/edit/tombstone and committed lost acknowledgement before Back during linked-code loading was overwritten. Action guard alone was insufficient; native route restorer now checks destination identity after reference reads, and entering Projects invalidates code reads.
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Three final steering builds/restarts passed. Final first cold2327ms; restart2788ms; fixture preparation0ms; server50208KiB + RustFS122048KiB =172256KiB. Built asset/CSP limits unchanged; persisted volume and search ready. Main7460 untouched.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

Final third run passed linked-code Back and reached native review open. Back exposed an intermediate Files history entry. Code connection now preserves the installation landing entry; switched review navigation replaces the intermediate entry; completed code navigation avoids a second page push. Target review pins use its independently connected code operation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

Final42/42 in7 direct controller files after single-step history and separate code-operation guards. Intermediate41/41 also passed.
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
pnpm web:check
```

**Notes**

Final final controller integration0 errors0 warnings.
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
pnpm spec:first
```

**Notes**

77 protected files across9 owning canonical chapters.
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
pnpm check:architecture
```

**Notes**

344 modules within unchanged budgets after final route guards.
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
pnpm web:test
```

**Notes**

Final60/60 Node tests after history integration.
<!-- /run -->
<!-- /evidence-field -->

