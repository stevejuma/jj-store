---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 12

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
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

workspace-clippy-34: exit 0, 7.49 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --test cursor_supervisor linked_cursor_products_backfill_pause_follow_final_flush_and_restart_without_duplicate_messages -- --exact
```

**Notes**

cursor-supervisor-exact-37: exit 0, 58.73 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --test cursor_supervisor
```

**Notes**

cursor-supervisor-all-37: exit 0, 20.15 seconds. 
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
pnpm dev:cargo -- test --workspace --locked --no-fail-fast -- --exact restart_overlap_hardlinks_and_replacement_retain_source_identity header_qualification_has_no_registry_side_effect_and_leased_reader_revalidates exclusive_source_lease_crosses_process_boundary_and_releases source_registry_process_probe killed_reader_releases_both_leases_and_keeps_durable_source_identity current_binding_inspection_preserves_read_only_state_and_rejects_replacement_headers linked_cursor_products_backfill_pause_follow_final_flush_and_restart_without_duplicate_messages inactive_unlinked_and_disabled_cursor_profiles_never_enroll_or_publish
```

**Notes**

workspace-leaf-repairs-38: exit 0, 54.17 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-rust-corpus.py
```

**Notes**

rust-corpus-coverage-39: exit 0, 0.2 seconds. 
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
JJ_STORE_BUILD_TOTAL_GIB=70 pnpm dev:clean -- --dry-run
```

**Notes**

cleanup-preview-40: exit 0. Preview identified only idle main workspace artifacts; live forge target skipped.
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
JJ_STORE_BUILD_TOTAL_GIB=70 pnpm dev:clean -- --apply
```

**Notes**

cleanup-apply-40: exit 0. Stricter temporary budget created runtime headroom. Freed18.9GiB(74.6-&gt;93.5GiBfree), cleaned only main workspace artifacts, preserved dependency cache/live forge/retainedrelease/fixtures/resources.
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
pnpm dev:cargo -- build --workspace --bins --examples --locked
```

**Notes**

workspace-build-18: exit 0, 121.04 seconds. 
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
pnpm dev:backend:build
```

**Notes**

backend-build-entrypoint-18: exit 0, 5.63 seconds. 
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

web-check-41: exit 0, 16.81 seconds. 
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

catalog-check-41: exit 0, 25.4 seconds. 
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
pnpm check:ui-ownership
```

**Notes**

ownership-41: exit 0, 3.45 seconds. 
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
pnpm check:architecture
```

**Notes**

architecture-41: exit 1, 0.49 seconds. 
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
pnpm check:boundaries
```

**Notes**

boundaries-41: exit 0, 0.33 seconds. 
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
pnpm contracts:check
```

**Notes**

generated-contracts-18: exit 0, 538.78 seconds. 
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

architecture-42: exit 0, 0.57 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./workflow-reload-42.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

workflow-reload-42: exit 0, 16.54 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-42: exit 0, 22.63 seconds. 
<!-- /run -->
<!-- /evidence-field -->

