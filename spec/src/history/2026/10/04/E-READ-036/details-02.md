---
schema: 1
id: E-READ-036
---

# E-READ-036 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts -t 'native directory reads compact' (before)
```

**Notes**

Existing card reader calls task browse and fails the no-summary-hydration regression. work-activity-ui-before-036.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_workstore_pages_commands_permissions_and_contract_capture -- --nocapture (before)
```

**Notes**

New body-free route returns unknown_route/404. work-activity-http-before-036.log.
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
cargo test -p jj-store-service --lib --locked work:: -- --nocapture (initial and final)
```

**Notes**

2pass: 301-task complete group, 101 ordered groups, bounded deep slice, invalid offsets/limits and selected-window browse. work-activity-units-036.log/work-activity-units-final-036.log.
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
JJ_STORE_WORK_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/performance/work-activity-native-capture-036.json cargo test -p jj-store-cli --test native_lifecycle --locked http_workstore_pages_commands_permissions_and_contract_capture -- --nocapture
```

**Notes**

1pass/0.40s. Actual native first/resumed group pages retain root across publication, reject changed limits and browse cursors, exclude task text and preserve command/access behavior. work-activity-native-036.log.
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
cargo test -p jj-store-service --test work_store --locked -- --nocapture
```

**Notes**

5pass/0.13s: warm task groups share browse/detail root data without domain reads, fresh inventory limits apply to both initial/resumed groups and historical data survives later publication. work-activity-native-service-036.log.
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
cargo test -p jj-store-service --test local_work --locked -- --nocapture
```

**Notes**

11pass/3.32s: pinned complete groups, current roots, changed limits/commits, foreign/staged selection rejection and existing local WorkStore consumers. work-activity-local-service-036.log.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/performance/work-activity-local-capture-036.json cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture (initial)
```

**Notes**

New test assumed CLI request identity was its project identity and the second task received 404. Corrected fixture to use the actual project receipt and deterministic identity ordering. work-activity-local-http-036.log.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/performance/work-activity-local-capture-036.json cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture (corrected)
```

**Notes**

1pass/13.28s. Actual local explicit companion pin, first/resumed signed pages, query bounds, hostile Origin rejection and existing filesystem registration/read consumers. work-activity-local-http-corrected-036.log.
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
cargo test -p jj-store-http --lib --locked contract::tests:: -- --nocapture (initial)
```

**Notes**

10pass/1fail. Inherited inventory assertion expected 103 paths/30 local; parent generated schema already contains 113/40. Initial +2 adjustment was insufficient. Parent/current schema comparison proves only the two activity routes are newly added. work-activity-contract-shapes-036.log/work-activity-inventory-baseline-036.log.
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
pnpm contracts:generate (initial and final); pnpm exec tsc -p scripts/tsconfig.contracts.json (initial and final); node --input-type=module affected-fixture responseValidator capture validation
```

**Notes**

Offline Swagger/type generation and generated contract type checks pass. Current schemas validate 50 actual affected-fixture responses, including 4 native/local first/resumed activity pages; finite query limits/page lengths are encoded. work-activity-generate-036.log/work-activity-generate-final-036.log/work-activity-contract-types-036.log/work-activity-contract-types-final-036.log/work-activity-responses-036.log. Complete registered capture inventory remains deferred.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts (initial and extended)
```

**Notes**

12pass then13pass after adding local aggregate/partial-state coverage. work-activity-ui-036.log/work-activity-ui-final-036.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts
```

**Notes**

23pass/3.09s across directory/fixture consumers, including complete cross-home/project counts without task summary loading, viewport cancellation, revoked/foreign snapshots and unloaded-group partial state. work-activity-ui-consumers-036.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

20pass/44.57s: directly affected repository/project card, navigation, membership and directory interaction/accessibility families. Fresh Storybook test process; production cold-browser qualification and visual baselines remain deferred. work-activity-stories-036.log.
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
pnpm web:check; pnpm catalog:check (ownership-corrected final)
```

**Notes**

Web and final catalog0errors/0warnings;190module catalog boundary pass. work-activity-web-check-036.log/work-activity-catalog-check-corrected-036.log.
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
pnpm check:architecture; pnpm check:ui-ownership; cargo fmt -p jj-store-service -p jj-store-http -p jj-store-cli; pnpm exec prettier --write changed controller/UI/fixture/test and canonical documents; jj --no-pager diff --git
```

**Notes**

972-module architecture and UI ownership pass; affected formatting and initial diff review pass. work-activity-architecture-036.log/work-activity-ownership-036.log and work-activity-*-format*-036.log/work-activity-review-036.patch. Root aggregates, full contract capture, scale/RSS/release/cold production/RustFS/registry/Help/visual qualification not run under user delivery split.
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
pnpm catalog:check (initial)
```

**Notes**

One fixture type error: union browse snapshot was not narrowed to native before returning an activity page. Fixed by explicit epoch/profile narrowing. work-activity-catalog-check-036.log.
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
pnpm catalog:check (type-corrected)
```

**Notes**

Type check0errors/0warnings, then catalog boundary rejects direct API type import in fixture. Replaced it with the owned controller WorkActivityPage type. work-activity-catalog-check-final-036.log.
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
cargo test -p jj-store-http --lib --locked contract::tests:: -- --nocapture (final)
```

**Notes**

11pass/0.24s. Exact body-free shape, separate native/local snapshots, strict local body and current executable115path inventory. work-activity-contract-shapes-final-036.log.
<!-- /run -->
<!-- /evidence-field -->

