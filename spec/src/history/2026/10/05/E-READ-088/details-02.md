---
schema: 1
id: E-READ-088
---

# E-READ-088 details 2

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
cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_finishes_with_snapshot_and_preserves_source_state --locked
```

**Notes**

before.log: Expected initial Pending-versus-Snapshot regression.
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
cargo check -p jj-store-jj -p jj-store-service --locked
```

**Notes**

check.log: Passed; unused Context field warning was removed, then restored as used by user-requested history mode.
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
cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_finishes_with_snapshot_and_preserves_source_state --locked
```

**Notes**

http.log: Incomplete Context removal missed one initializer; corrected.
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
pnpm contracts:generate
```

**Notes**

contracts-generate.log: Same Context compilation error; corrected before regeneration.
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

contracts-generate-fixed.log: First quick-only contracts regenerated.
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
cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_finishes_with_snapshot_and_preserves_source_state --locked
```

**Notes**

http-fixed.log: Exact regression passes.
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
pnpm web:check
```

**Notes**

web-check.log: Ran before generated attribution type was ready; one missing-field error corrected by regeneration.
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

web-check-generated.log: Zero errors/warnings after generation.
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
pnpm check:architecture
```

**Notes**

architecture.log: Directory rows exceeded the 300-line cap; extracted attribution presentation into a focused component.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_crosses_merges_and_retains_the_ancestry_bound --locked
```

**Notes**

native.log: One quick-mode regression passes; native source write fault remains unconsumed.
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
cargo test -p jj-store-cli --test local_browser managed_code:: --locked
```

**Notes**

local-managed.log: Six pass, one opt-in Linux smoke ignored.
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
cargo test -p jj-store-cli --test local_browser bounded_reads:: --locked -- --test-threads=2
```

**Notes**

local-bounded.log: Seven pass, one opt-in smoke ignored; point membership, unrelated reads, corruption behavior preserved.
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
JJ_STORE_FAST_PATH_CAPTURE=$PWD/.local/fast-path-metadata/local-contracts.json cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_finishes_with_snapshot_and_preserves_source_state --locked
```

**Notes**

http-capture-final.log: One passes; source parity and mixed provenance.
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
JJ_STORE_BROWSING_CONTRACT_CAPTURE=$PWD/.local/fast-path-metadata/native-contracts.json cargo test -p jj-store-cli --test native_lifecycle browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured --locked
```

**Notes**

native-http.log: One passes; unauthorized/stale/cursor-bound reads.
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
cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance:: --locked
```

**Notes**

native-browsing.log: Nine pass; direct native browsing/reference consumers.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked
```

**Notes**

check-final.log: Passed prior to history follow-up.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

ui-units.log: Initial 10 tests pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
```

**Notes**

directory-story.log: Three cold interaction/accessibility stories pass.
<!-- /run -->
<!-- /evidence-field -->

