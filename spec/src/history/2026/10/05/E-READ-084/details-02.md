---
schema: 1
id: E-READ-084
---

# E-READ-084 details 2

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
cargo test -p jj-store-cli --test native_lifecycle content_diff_long_file_sparse_edits_remain_text --locked
```

**Notes**

Initial harness used a single-thread Tokio runtime; corrected to the native fixture's required four-thread runtime before establishing the product baseline.
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
cargo test -p jj-store-cli --test native_lifecycle content_diff_long_file_sparse_edits_remain_text --locked
```

**Notes**

Corrected before run: text comparison returned Complexity.
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
cargo test -p jj-store-cli --test local_browser local_diff_long_file_sparse_edits_remain_text --locked
```

**Notes**

Before: the local reader returned Complexity.
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
pnpm exec tsx --test --test-name-pattern='selected patch preserves' web/tests/production-store-loaders.test.ts
```

**Notes**

Before: unavailable reason was discarded; duplicate focused run reproduced the same failure.
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
cargo test -p jj-store-jj --lib line_diff --locked
```

**Notes**

5 shared admission/default-hunk/byte-preservation/cumulative-limit/cancellation tests passed.
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
cargo test -p jj-store-cli --test native_lifecycle content_diff_long_file_sparse_edits_remain_text --locked
```

**Notes**

After: 1 passed.
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
cargo test -p jj-store-cli --test local_browser local_diff_long_file_sparse_edits_remain_text --locked
```

**Notes**

After: 1 passed.
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
cargo test -p jj-store-cli --test native_lifecycle content_acceptance::diff_acceptance --locked
```

**Notes**

21 direct diff/conflict subtree tests passed; no full native-lifecycle claim.
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
cargo test -p jj-store-cli --test local_browser scoped_differences --locked
```

**Notes**

2 selected local comparison regressions passed.
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
cargo check -p jj-store-jj -p jj-store-cli --locked
```

**Notes**

Affected Rust packages and compiled direct dependencies passed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

All diagnostics are in untouched modules; the exact prior revision reproduces the same 17 diagnostics.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo clippy --manifest-path /tmp/jj-store-diff-lint-baseline/Cargo.toml -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

Clean disposable workspace at uklvrtou/1dde77e0. Diagnostic headers and all 17 owning source files are byte-identical. The workspace was forgotten, its own empty change abandoned and its directory removed.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**


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
pnpm exec tsx --test --test-name-pattern='selected patch preserves' web/tests/production-store-loaders.test.ts
```

**Notes**

After: 1 passed.
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts web/tests/production-inspection-pages.test.ts
```

**Notes**

13 passed.
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
pnpm exec tsx --test web/tests/repository-diff.test.ts web/tests/review-comment-anchor.test.ts
```

**Notes**

4 passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-patch-reader.test.ts
```

**Notes**

3 passed, repeated after adding stale-patch removal; pending, obsolete selection and failed-request state covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
```

**Notes**

Fresh browser runner: initially 6, finally 7 stories passed, including stacked operation and standalone comparison. Existing accessibility setup retained; no baseline changes.
<!-- /run -->
<!-- /evidence-field -->

