---
schema: 1
id: E-M17-074
---

# E-M17-074 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_supervisor --locked -- --test-threads=1
```

**Notes**

Inherited exact before: API profile rejected by Codex-only root selection. /tmp/jj-store-opencode-supervisor-before.log. Test-only Api fixture used distinct endpoint/api fields; targeted dead-code annotations document separate consumers.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_supervisor --locked -- --test-threads=1
```

**Notes**

Initial composition compile: four private helper visibility errors and one incorrect IntegrationId path. Fixed at the owning module/reexport. /tmp/jj-store-opencode-supervisor-after.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_supervisor --locked -- --test-threads=1
```

**Notes**

Original two automatic runtime regressions pass after composition. /tmp/jj-store-opencode-supervisor-after-2.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_supervisor --test opencode_source --locked -- --test-threads=1
```

**Notes**

Nine selected tests: six source and three supervisor; includes one-reader Codex/API fairness, idle compaction and revoked partition yielding. /tmp/jj-store-opencode-supervision-regressions.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_supervisor::opencode::tests -- --test-threads=1
```

**Notes**

New test used a nonexistent Config installation_path method and an unused import. Corrected to the existing isolated installation path. /tmp/jj-store-opencode-supervision-owned.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_supervisor::opencode::tests -- --test-threads=1
```

**Notes**

Both blocked-publication/drain and independent exact-proof/lease tests pass. macOS linker warns about large DWARF unwind section; functional tests pass without suppression. /tmp/jj-store-opencode-supervision-owned-after.log.
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
cargo check -p jj-store-server --all-targets --locked
```

**Notes**

Affected package and all typed direct consumers compile. /tmp/jj-store-opencode-supervision-check.log.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

One unnecessary unwrap diagnostic; changed only the equivalent optional-source branch to if-let. /tmp/jj-store-opencode-supervision-clippy.log.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Final strict lint; no production lint suppression. /tmp/jj-store-opencode-supervision-clippy-final.log.
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
cargo fmt --all
```

**Notes**

Run after each production/test edit and final structural lint fix.
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
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Owning canonical chapters and qualification narrative formatted.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_runtime --test capture_source --test opencode_read_api --test capture_management --locked -- --test-threads=1
```

**Notes**

All 59 directly affected existing integration tests pass: supervisor 27, runtime 10, source 4, API 5, management 13. Existing cold storage lifetime and durable pause/import/loss/pressure/receipt consumers remain qualified in their stated fixture lanes. /tmp/jj-store-opencode-supervision-consumers.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked -- --test-threads=1
```

**Notes**

Complete owning server library: 105 tests pass, including new two API owned-lane tests and existing cold foreground/control/storage consumers. Runtime 310.12s. This required library lane predates only the equivalent if-let lint cleanup, whose final API regressions and all-target strict check follow. macOS test linker emits the recorded unwind-size warning. /tmp/jj-store-opencode-supervision-library.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_supervisor --test opencode_source --locked -- --test-threads=1
```

**Notes**

Final exact set after all behavioral changes and equivalent lint cleanup: six source plus three supervisor tests pass. /tmp/jj-store-opencode-supervision-final.log.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters, 160 requirements.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries.
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
pnpm check:architecture
```

**Notes**

732 web modules; new Rust modules below 300 lines and tests below 500.
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
pnpm check:progress
```

**Notes**

129 tasks; evidence bookkeeping only; M17/M17.2 remain In progress.
<!-- /run -->
<!-- /evidence-field -->

