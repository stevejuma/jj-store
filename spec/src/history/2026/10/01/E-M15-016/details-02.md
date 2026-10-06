---
schema: 1
id: E-M15-016
---

# E-M15-016 details 2

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
CARGO_INCREMENTAL=0 cargo test -p jj-collab --test replies --locked
```

**Notes**

Initial reply regression could not compile before optional reply fields were added.
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
CARGO_INCREMENTAL=0 cargo test -p jj-collab --test replies --locked
```

**Notes**

Initial 1 reply regression passed.
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
CARGO_INCREMENTAL=0 pnpm contracts:generate
```

**Notes**

Ran twice while adding hydrated human timeline, exact custom versions and Agent message witnesses; generation succeeded.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial composition errors: generated model, rune shadowing and Community props; corrected owners.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Thread controller valid callback declared after use; changed to a function declaration.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Client and production controller exceeded budgets; extracted binary admission, Thread integration and snapshot refresh owners.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Reply details description/current member optionality corrected.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings.
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
pnpm check:architecture
```

**Notes**

472 modules passed before final fixture additions; latest check recorded separately.
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
CARGO_INCREMENTAL=0 cargo test -p jj-collab --locked
```

**Notes**

25 tests pass including fixed legacy byte witness, exact parent identity and serialized cold closure.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts
```

**Notes**

Initial 7 targeted controller tests pass.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test native_collab --test local_collab --test agent_sessions --locked
```

**Notes**

Agent witness assertions referenced an unavailable crate alias; switched to owned WAL reexports.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test native_collab --test local_collab --test agent_sessions --locked
```

**Notes**

22 selected service tests pass; later exact-target and cold-reply assertions require final rerun.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Fixture initially rendered only the shell; switched to production StoreRepositoryApplication.
<!-- /run -->

<!-- run -->
### Run 15

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

Message action hover readiness needed its real visibility boundary.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Interaction passed; axe found 2 ARIA labels on unroled reaction containers. Added group roles.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Fixture model and story query types corrected.
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
pnpm catalog:check
```

**Notes**

Catalog rejects imports of API client; fixture now consumes the public Thread model contract.
<!-- /run -->
<!-- /evidence-field -->

