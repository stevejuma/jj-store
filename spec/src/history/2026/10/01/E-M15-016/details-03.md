---
schema: 1
id: E-M15-016
---

# E-M15-016 details 3

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
pnpm catalog:check
```

**Notes**

0 errors/warnings; catalog boundary audit passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

2 interaction/a11y stories pass; pending action story had a duplicate text query.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

21 pass; the new pending story checked async status before completion. Its final assertion now waits for all confirmation controls to disappear; existing consumers all pass.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-wal --test collab_commands --test collab_persistence --locked
```

**Notes**

21 command/persistence tests pass including checkpoint/copy, exact receipts and native code replication guard.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

168 tests in 30 files pass. Escalated because shared route/controller refresh owners affect repository and Projects consumers.
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
cargo fmt --all
```

**Notes**

Formatted after Rust additions.
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
pnpm exec prettier --write web/src/lib/controller/thread-types.ts web/src/lib/ui/store-threads/StoreMessageReactions.svelte catalog/fixtures/thread-transport.ts catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Applied focused formatting after each composition/controller slice.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Pending status fixture used the domain argument as request ID; corrected the local fixture and added an exact-domain/request regression.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

After receipt confirmation, retained draft and recorded message share text. Scoped assertion to the actual message list.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

Fresh Chromium process: 22 stories pass interaction and accessibility. No visual baselines changed. /tmp/jj-thread-stories-qualified.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

Agent and Review exact-target custom reaction set and actor dialogs worked; clear clicks needed to wait for modal pointer ownership release.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

2 cold interaction/accessibility stories pass custom set/clear and authorized actor inspection on existing Agent/Review message components. /tmp/jj-thread-captured-stories-2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts
```

**Notes**

Final 14 Thread tests are included in the final complete 172-test catalog-unit pass; intermediate 11 and 13 targeted tests passed after receipt and privacy assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

172 tests in 30 files pass after final controller additions. Includes Projects-to-Threads navigation, exact parent edits, partial history, receipt identity, shared transactional emoji loading, lost outcomes and protected-cache disposal. /tmp/jj-thread-all-unit-final.log.
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
pnpm exec tsx --test web/tests/client.test.ts
```

**Notes**

10 client regressions pass including private credential-header raster reads, SVG rejection and byte limits. /tmp/jj-thread-client.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test native_collab --test local_collab --test agent_sessions --locked
```

**Notes**

Final 23 service tests pass; exact Agent witnesses resolve through the authorized Reaction reader, human timeline hydration and version refs match, and persisted replies/receipts survive GC and cold reopen. /tmp/jj-thread-service-final.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-rpc --test collab --locked
```

**Notes**

Existing receiver destructuring omitted the new custom definition version map; changed the consumer regression to assert exact refs.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-rpc --test collab --locked
```

**Notes**

3 Unix RPC tests pass including reconstructed transports, exact emoji refs, source membership and permissions. /tmp/jj-thread-rpc-2.log.
<!-- /run -->
<!-- /evidence-field -->

