---
schema: 1
id: E-M11-003
---

# E-M11-003 details 3

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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-2HUdkt JJ_STORE_DEV_PORT=7477 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Final bundle rebuilt with scrolling and bounded hierarchy. Owned persistent fixture cold-restarted: 3350ms server/search readiness, 172352KiB RSS.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7477/ JJ_STORE_REAL_REPOSITORY=760bab18beeed9f256b58d749e0194d2 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-2HUdkt JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-native-project-tasks pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

Exact browser gate fails at first repository link. Svelte interpolated regex braces and native form validation prevented publication; changed pattern to a literal expression and added real browser link submission story.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check AGENTS.md scripts/dev-demo.mjs scripts/real-browser-acceptance.mjs spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/projects.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/controller/contracts.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-work-data-source.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work-integration.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/store-error.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/store-route-history.svelte.ts web/src/lib/controller/work-types.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/repository-sections.ts web/src/lib/ui/store-work catalog/components/store-work/StoreWork.stories.svelte catalog/fixtures/StoreWorkFixture.svelte web/tests/real/work-store.spec.ts web/tests/store-route-codec.test.ts web/tests/vite/production-work.test.ts
```

**Notes**

Four unformatted files; subsequently formatted. Exact file list is preserved in the tool output.
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
pnpm exec prettier --write web/src/lib/controller/contracts.ts web/src/lib/controller/native-data-source.ts web/src/lib/ui/store-application/StoreRepositoryApplication.svelte catalog/components/store-work/StoreWork.stories.svelte web/src/lib/ui/store-work/StoreWork.svelte web/tests/real/work-store.spec.ts
```

**Notes**

Affected source formatting applied.
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
pnpm spec:first
```

**Notes**

31 protected files map to six changed canonical chapters before final ledger/verification updates.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Four final Work stories pass, including actual valid Repository ID form submission; accessibility addon errors are enforced.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-2HUdkt JJ_STORE_DEV_PORT=7477 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Final pattern fix cold rebuild/restart preserves the owned fixture: 4456ms readiness and 152480KiB RSS.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7477/ JJ_STORE_REAL_REPOSITORY=760bab18beeed9f256b58d749e0194d2 JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-2HUdkt JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-native-project-tasks pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

Exact formerly failing browser gate passes in 16s after rebuilt cold stack. Real links, project selection, cross-target parent/child, state update, reply/edit/tombstone, explicit refresh, lost real publication response, durable-status recovery without duplicate comment, reload/route identity, zero axe WCAG violations and 420px overflow assertion pass. Screenshot reviewed; visual baseline remains unapproved.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-2HUdkt pnpm dev:stop
```

**Notes**

Owned server/storage stopped; then verified no owned processes remain and removed only this disposable root. Fixture install/remove also cleaned the three transient checkout hook files.
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
pnpm spec:validate
```

**Notes**

19 chapters and 115 requirements pass.
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
pnpm check:boundaries
```

**Notes**

15 package dependency boundaries pass.
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
pnpm check:progress
```

**Notes**

77 tasks and pending browser evidence pass bookkeeping.
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
pnpm web:check
```

**Notes**

Post-pattern-fix production check has zero errors/warnings.
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
pnpm check:architecture
```

**Notes**

Final formatted production source retains 296 passing modules and unchanged limits.
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
pnpm exec prettier --write AGENTS.md scripts/dev-demo.mjs scripts/real-browser-acceptance.mjs spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/milestones.md spec/src/projects.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/controller/contracts.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-work-data-source.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work-integration.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/store-error.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/store-route-history.svelte.ts web/src/lib/controller/work-types.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/repository-sections.ts web/src/lib/ui/store-work catalog/components/store-work/StoreWork.stories.svelte catalog/fixtures/StoreWorkFixture.svelte web/tests/real/work-store.spec.ts web/tests/store-route-codec.test.ts web/tests/vite/production-work.test.ts
```

**Notes**

All changed UI, catalog, harness and canonical files formatted; final write changed only ledger formatting.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7478 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm web:test:real
```

**Notes**

Fresh full gate rejected before Playwright: firstStartupMillis=1447, restartStartupMillis=32918, fixturePreparationMillis=316900, totalRssKiB=152400. The initial configured search instance was still constructing the fixture projection. Corrected reset to finish full fixture/search preparation, stop that instance and measure a new physical server/storage restart opening the persisted fixture. The 30000ms/524288KiB gates remain unchanged. Cold cache-loss search rebuild performance is separate and is not qualified by this slice. Runner stopped and removed this disposable root.
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
pnpm exec prettier --write scripts/dev-demo.mjs scripts/real-browser-acceptance.mjs spec/src/cli.md spec/src/spec-governance.md spec/src/compatibility.md
```

**Notes**

Formatted corrected cold persisted-fixture startup protocol and owning documentation.
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
pnpm exec prettier --write web/tests/real/work-store.spec.ts
```

**Notes**

Strengthened browser evidence to assert a full non-null native operation pin survives WorkStore mutations and reload, while initial Projects still opens without a code pin.
<!-- /run -->
<!-- /evidence-field -->

