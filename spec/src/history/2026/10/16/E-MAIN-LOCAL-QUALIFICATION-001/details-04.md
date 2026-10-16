---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 4

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
pnpm install --frozen-lockfile --ignore-pnpmfile
```

**Notes**

Matched the merged Design Core 0.5.0 lockfile; main initially had 0.3.2 installed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-concurrency=1 scripts/*.test.mjs scripts/spec/*.test.mjs
```

**Notes**

219 tests: 211 passed, eight failed. Seven stale fixture/inventory failures; migration verification rejected concurrent working-copy edits during the baseline, requiring a stable-tree rerun.
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
node --test scripts/dev-ui.test.mjs scripts/dev-agent-runtime.test.mjs
```

**Notes**

12 passed after repairing the fake ACP reply/EOF handshake; production preflight remains unchanged.
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
node --test scripts/ci-selection.test.mjs scripts/view-module-urls.test.mjs scripts/install-cli.test.mjs
```

**Notes**

16 passed. Inventories now include the forge crate and two added views; installer fixture uses jj-store-jj.
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
node scripts/package-check.mjs
```

**Notes**

21 packages have contained compilation inputs and complete README/license/notices.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:test
```
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
pnpm catalog:test
```

**Notes**

184 files: 1170 passed, one declared skip.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm test:storage-harness
```
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
node --test --test-name-pattern=migration_preserves_original_evidence_tasks_requirements_and_byte_archives scripts/spec/progress-regression.test.mjs
```

**Notes**

Exact migration audit passed on a stable working tree; earlier concurrent-edit rejection is retained.
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
pnpm test:rustfs
```

**Notes**

Pinned disposable external RustFS contract passed; .scratch/rustfs-contract.json retains topology, counts, image digest and limitations. Docker was initially stopped, started for the test, and returned to stopped afterward.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm desktop:check
```
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
pnpm desktop:test
```

**Notes**

Interrupted scheduling attempt, not a test assertion result. Defer remaining heavy builds to two-job serial execution on this 16-GiB host; rerun required.
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
pnpm build:embedded-rustfs
```

**Notes**

Interrupted compilation to reduce concurrent resource pressure; no runtime test result established.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

Three collapsible-if lints in forge engine and GitHub current-check filtering. Equivalent short-circuit let chains retain original guards. Recheck required.
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
pnpm web:check
```

**Notes**

Zero errors and warnings before comparison listbox accessibility labels; recheck needed for subsequent UI edits.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:rust:check
```
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
pnpm check:boundaries
```

**Notes**

20 root Rust workspace packages.
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
pnpm check:architecture
```

**Notes**

1990 modules.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Full Rust and Prettier checks passed at this intermediate source; later changed files retain their focused formatter run.
<!-- /run -->
<!-- /evidence-field -->

