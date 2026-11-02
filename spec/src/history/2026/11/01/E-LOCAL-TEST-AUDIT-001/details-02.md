---
schema: 1
id: E-LOCAL-TEST-AUDIT-001
---

# E-LOCAL-TEST-AUDIT-001 details 2

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
node --test scripts/*.test.mjs scripts/spec/*.test.mjs
```

**Notes**

Native Node 26.9.0: 286 tests passed; zero failures, cancellations, skips or todos. Cold remote testing subsequently selected explicitly by the user.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storage-harness
```

**Notes**

3 harness self-tests passed. This does not establish real RustFS qualification.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:contracts
```

**Notes**

8 contract tests and the contract TypeScript check passed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm web:test
```

**Notes**

180 tests passed; zero failures or skips.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:test --maxWorkers=1 --no-file-parallelism web/tests/vite/production-stack-adapter.test.ts web/tests/vite/local-browser.test.ts -t 'production branch drops resolve native ownership|local_preparation_preserves_the_failed_query_pin'
```

**Notes**

Both exact timeout candidates passed unchanged in a fresh process: 2 passed, 40 deliberately filtered cases. The interrupted unbounded native run reported 2 failures at 5474ms and 5088ms amid substantial unrelated local workloads; it did not complete. Native Rust workspace compilation and web:check were also interrupted; neither is a passing lane.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm client:check && JJ_STORE_VALIDATION_RUNTIME=native pnpm history:check
```

**Notes**

Both client type checks passed, including the generated history-contract comparison.
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
pnpm exec spec-validator doctor --json; pnpm spec:search JST-GOV-007
```

**Notes**

Doctor passes with the existing Docker-wrapper script-alias warning. QMD finds the owning governance requirement. Registry latest and installed validator are both 0.2.0 (Deno-free). The initially documented search separator was rejected; retry without the separator succeeds.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker container jj-store-validation-458152da-f244-4046-9fad-6c3ae83a13f7: node --test --test-concurrency=1 <all scripts/*.test.mjs and scripts/spec/*.test.mjs except catalog-browser.test.mjs>
```

**Notes**

284/285 passed. The built browser asset closure test could not open web/dist/.vite/manifest.json because this extra run preceded its web:build prerequisite. No assertion or product fix was needed; cold static catalog acceptance remains in the aggregate lane.
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
Remote Docker container jj-store-validation-458152da-f244-4046-9fad-6c3ae83a13f7: pnpm web:build && node --test scripts/view-loading-assets.test.mjs
```

**Notes**

The production web bundle, generated manifest and strict staged documentation build complete. The exact asset-closure regression passes 1/1 after preparing its required built output. Ordinary Vite chunk-size/dynamic-import advisories remain; no baseline is changed.
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
Remote Docker container jj-store-validation-458152da-f244-4046-9fad-6c3ae83a13f7: node --test --test-concurrency=1 <all scripts/*.test.mjs and scripts/spec/*.test.mjs except catalog-browser.test.mjs>
```

**Notes**

All 285 extra script tests pass with zero failures, cancellations, skips or todos after the actual web build; 47.17 seconds. Includes governance/release/spec, launcher/cache tooling, package client checks/tests and built-asset coverage. The separate cold catalog browser is still run by the root aggregate.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker aggregate pnpm test: completed test:storybook shard 1/4
```

**Notes**

273 pass, 2 fail across 54 files. StoreWorkspaces KeyboardSearchAndRepositoryOverview cannot find No matching workspaces. AppearanceOverlay SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes observes everforest-light overlay rgb(45, 53, 59) versus app rgb(253, 246, 227). Fresh exact reproductions remain required before classification or quarantine. Subsequent shards continue as required by the existing bounded runner.
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
Remote Docker: pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte -t 'SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes|KeyboardSearchAndRepositoryOverview'
```

**Notes**

Independent fresh Docker attempt 1: jj-store-validation-118119b8-96e0-48c3-9edd-5d6f13c81d51. Both original failures pass unchanged; 2 pass, 6 deliberately filtered. No quarantine tag or skip policy existed in this source snapshot. This contrasts with the two full-suite failures and demonstrates suite-context dependence without claiming a root-cause fix.
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
Remote Docker: pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte -t 'SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes|KeyboardSearchAndRepositoryOverview'
```

**Notes**

Independent fresh Docker attempt 2: jj-store-validation-5b0a911e-3f5b-4b81-8168-a46e771c0df8. Both original failures pass unchanged; 2 pass, 6 deliberately filtered. No quarantine tag or skip policy existed in this source snapshot. This contrasts with the two full-suite failures and demonstrates suite-context dependence without claiming a root-cause fix.
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
Remote Docker: pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte -t 'SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes|KeyboardSearchAndRepositoryOverview'
```

**Notes**

Independent fresh Docker attempt 3: jj-store-validation-0ac041dd-efb2-48d7-ae46-9de8b7dc88e2. Both original failures pass unchanged; 2 pass, 6 deliberately filtered. No quarantine tag or skip policy existed in this source snapshot. This contrasts with the two full-suite failures and demonstrates suite-context dependence without claiming a root-cause fix.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node .local/test-audit-20261101/remote-run.mjs test
```

**Notes**

Full Docker receipt jj-store-validation-458152da-f244-4046-9fad-6c3ae83a13f7, source 6dc050256ce94df163fad61f40625e10b3ea1391, no archive substitutions. Governance 120, storage harness 3, each of the three real-storage/LocalTrusted targets 1, contracts 8, web unit 180, catalog unit 1208 pass (one pre-existing actual-observation-capture prerequisite skip). Storybook shards: 273 pass/2 fail; 284 pass; 238 pass; 251 pass. Total 1046 pass/2 fail from 1048 cases. The existing shell aggregate stops after the failed Storybook lane, so cold static catalog and the Rust workspace have not run in this attempt.
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
Remote Docker: pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte -t 'SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes|KeyboardSearchAndRepositoryOverview'
```

**Notes**

Post-quarantine exact selection: jj-store-validation-d9e59cf7-6789-428f-9b62-873e453f93f0. All 8 discovered cases are skipped: the 2 original selected cases by explicit quarantine, 6 neighbors by the name filter. Neither quarantined assertion is claimed passing.
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
Remote Docker: pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

Post-quarantine full families: jj-store-validation-50c52811-57b4-418c-979a-9c6269cb38d3. All 6 enabled neighboring cases pass and exactly 2 cases are reported skipped. The cases remain in the reported inventory, with original names, play functions and assertions intact.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker, 4 CPUs / 12 GiB: pnpm test --no-fail-fast
```

**Notes**

Receipt jj-store-validation-71fde3ba-7fa9-4ca0-a186-3b01ab9429c5. The first two Storybook shards pass (273 pass/2 quarantine skips; 284 pass). Shard 3 has 237 pass and one unhandled browser/RPC disconnect before AppearanceDiscussions completes; shard 4 passes 251. The original first baseline had passed that story in 2028ms. No assertion failure identifies this as a case-specific defect. cgroup memory.peak was 12884971520 bytes against memory.max 12884901888, with max=36665, oom=0 and oom_kill=0. These observations do not prove a memory cause. The case remains enabled; no unhandled-error guard is weakened. Cold catalog and Cargo workspace remain unrun because the aggregate stopped at the runner error.
<!-- /run -->
<!-- /evidence-field -->

