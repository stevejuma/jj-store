---
schema: 1
id: E-M18-001
---

# E-M18-001 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/progress-mapping.test.mjs (before)
```

**Notes**


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
node --test scripts/progress-mapping.test.mjs scripts/governance.test.mjs
```

**Notes**

32 tests
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
pnpm contracts:generate (before merge repair)
```

**Notes**

missing cached append witnesses
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
cargo test -p jj-store-wal --test agent_catalog selected_cached_agent_reader_retains_audited_append_witnesses --locked (before)
```

**Notes**

same missing witness field compile boundary
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
pnpm contracts:generate
```

**Notes**

103 paths, 30 local-prefix paths
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
cargo check -p jj-store-cli -p jj-store-server --all-targets --locked
```

**Notes**


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
cargo test -p jj-store-wal --test agent_catalog --locked
```

**Notes**

26 tests
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
cargo test -p jj-store-server --lib process::tests::capture_factory --locked
```

**Notes**

10 tests including cold startup/lifetime/recovery
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts (before)
```

**Notes**


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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

18 tests; original single-flight test revised to await both coalesced reads after delivery
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
pnpm contracts:check (first run)
```

**Notes**

cold import acknowledgement is unknown; exact standalone target reproduces 503 and is now qualified through original receipt/replay
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact (before cold recovery assertion)
```

**Notes**


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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (shared dependencies)
```

**Notes**

browser import of another workspace dependencies failed
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --offline --frozen-lockfile
```

**Notes**

tracked pnpm hook checksum absent from registry lock
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
pnpm install --offline --frozen-lockfile --ignore-pnpmfile
```

**Notes**


<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (registry Design Core)
```

**Notes**

seven inherited main surfaces require local upstream APIs absent from released 0.2.0
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
pnpm install --offline --no-frozen-lockfile
```

**Notes**

reuses tracked main local Design Core hook; tracked registry lock restored verbatim; temporary local preview qualification
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
pnpm web:check
```

**Notes**

0 errors/warnings with main-equivalent upstream validation setup
<!-- /run -->
<!-- /evidence-field -->

