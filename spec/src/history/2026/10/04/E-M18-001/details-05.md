---
schema: 1
id: E-M18-001
---

# E-M18-001 details 5

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (automatic load before async assertions)
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
node scripts/browser-fixtures.mjs
```

**Notes**


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
pnpm web:build
```

**Notes**


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
pnpm check:boundaries
```

**Notes**


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
cargo fmt --all
```

**Notes**


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
pnpm exec prettier --write affected TS/Svelte/scripts/canonical chapters excluding historical ledger
```

**Notes**


<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test codex_transcript codex_cli_exec_and_inherited_desktop_originator_keep_cli_surface_checks --locked -- --exact (before)
```

**Notes**

Pinned real CLI exec/originator metadata was refused
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
cargo test -p jj-store-capture --test codex_transcript --test codex_bound_source --test codex_activity --test codex_import --locked
```

**Notes**

39 tests; exec and inherited Desktop originator accepted only on pinned CLI surface
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
cargo check -p jj-store-capture --all-targets --locked
```

**Notes**

Affected reader boundary
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
pnpm dev:backend:build
```

**Notes**

Explicit workspace binaries; linker compact-unwind size diagnostic retained
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
cargo test -p jj-store-server --test capture_management api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free --locked -- --exact (before 120-second cold observation)
```

**Notes**

Original 60-second cold native preparation observation timed out
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
cargo test -p jj-store-server --test capture_management api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free --locked -- --exact
```

**Notes**

92.53 seconds; bounded 120-second cold observation, assertions unchanged
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
pnpm contracts:check (replay before bounded exact-body busy retry)
```

**Notes**

Original receipt completed but immediate replay returned documented 409 capture_control_busy
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
cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact (120-second observation)
```

**Notes**

Cold completion expired; 167.93 seconds total
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
cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact (preview paused)
```

**Notes**

Same 120-second cold completion failure without active preview capture; 128.96 seconds total
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
cargo test -p jj-store-server --test capture_management imports::import_acceptance_progress_and_replay_are_actual_http_contracts --locked -- --exact (240-second cold observation)
```

**Notes**

194.98 seconds; production acknowledgement remains five seconds; original receipt, boundary, replay, actor/privacy and one journal-entry assertions retained
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts -t 'cumulative unavailable observations do not disable capture controls' (before)
```

**Notes**

Normal locations exceeded 4096 cumulative unavailable observations; resource counters remain separately bounded
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts (final)
```

**Notes**

19 tests including unsigned cumulative-counter acceptance and resource/negative refusals
<!-- /run -->
<!-- /evidence-field -->

