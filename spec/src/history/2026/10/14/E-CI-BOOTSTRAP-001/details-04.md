---
schema: 1
id: E-CI-BOOTSTRAP-001
---

# E-CI-BOOTSTRAP-001 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions run 38621777373: node scripts/ci-build.mjs heavy and ordinary
```

**Notes**

Both build cohorts failed with nine identical Linux u64 identity-cast lints in physical stat stamps and the local execution guard. Same field values are widened through i128 before retaining the original u64 bit patterns; no lint suppression.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 458
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions run 38621777373: pnpm exec prettier --check .
```

**Notes**

Formatting failed on 458 tracked files. Formatted authored code/docs and current navigation/history tables; immutable historical evidence and the byte-exact native association fixture were preserved.
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
GitHub Actions run 38621777373: native ARM Linux build
```

**Notes**

libdbus-sys build could not find the credential-store development headers. Composite setup now installs pkg-config and libdbus-1-dev on Linux.
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
GitHub Pages run 38621781516: node scripts/docs.mjs render
```

**Notes**

Stale publication documentation review stopped the build. Reviewed every affected topic and refreshed source fingerprints; installation and capture documentation now describe public executable names and released scanner/cache policy.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --all-targets --locked --target-dir .local/build-targets/public-release -- -D warnings
```

**Notes**

Complete affected native package/all-target strict Clippy passed on macOS ARM64/Rust 1.99. Linux qualification remains the hosted PR gate.
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
pnpm dev:cargo -- test -p jj-store-jj --test working_tree_current --locked --target-dir .local/build-targets/public-release -- --test-threads=1
```

**Notes**

All nine working-tree/current identity, bounds, policy race and authorization regressions passed.
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
node --test scripts/docs.test.mjs
```

**Notes**

Seven tests passed; staging fixture still created jj after the public client was renamed. Updated its fixture name without weakening closure/replacement assertions.
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
node --test scripts/docs.test.mjs
```

**Notes**

All eight documentation tests passed after the fixture-name correction.
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
pnpm test:governance
```

**Notes**

All 68 governance tests passed.
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
pnpm test:spec
```

**Notes**

All 26 specification tooling tests passed.
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
pnpm release:check
```

**Notes**

Eleven release/CI tests passed, including the checksum-pinned JJ matrix/rejection test; package metadata checks passed for 20 crates.
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
node scripts/docs.mjs render
```

**Notes**

Nine diagrams rendered in both themes, strict MkDocs build passed, and the generated closure audit passed.
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
pnpm exec prettier --check .
```

**Notes**

All format-checked tracked files passed after repair. New JJ installer will be format-checked again before commit.
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
pnpm format:rust:check
```

**Notes**

Both Rust workspaces formatted correctly.
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
actionlint
```

**Notes**

Workflow/action syntax passed.
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
pnpm web:check
```

**Notes**

Zero errors/warnings after controller/application source formatting.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings and all 442 catalog module boundaries passed.
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
node scripts/ci-setup-jj.mjs with isolated RUNNER_TEMP and GITHUB_PATH
```

**Notes**

Actual checksum-verified upstream binary includes the official commit suffix in its version string. Exact official 0.45.1/7c41cdeb identity is now accepted; arbitrary versions remain rejected.
<!-- /run -->
<!-- /evidence-field -->

