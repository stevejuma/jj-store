---
schema: 1
id: E-M17-066
---

# E-M17-066 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management::tests::native_import::reader_binding::accepted_import_selects_a_durable_reader_association_before_returning_its_pipeline -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-before.log 2>&1
```

**Notes**

Genuine native source enrollment returned an imported pipeline without durable reader association.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management::tests::native_import::reader_binding::accepted_import_selects_a_durable_reader_association_before_returning_its_pipeline -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-after.log 2>&1
```

**Notes**

Initial implementation compile errors: use the existing service source identity type and copy ordinal before moving catalog reader entries. No new dependency or contract substitution.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management::tests::native_import::reader_binding:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-exact.log 2>&1
```

**Notes**

Original native regression plus early reopen/admission regressions pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_import_state::tests::reader:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-journal.log 2>&1
```

**Notes**

One fixture issue emitted two compiler diagnostics: non-debug private file stamp cannot use assert_ne. Compare the same equality without adding metadata Debug or weakening assertion.
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

Repeated after implementation and exact regression edits.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_import_state::tests::reader:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-journal-after.log 2>&1
```

**Notes**

Four journal tests pass including separately exited process and exact held stamp replacement.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management::tests::native_import::reader_binding:: -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-native-final.log 2>&1
```

**Notes**

Four native association regressions pass; exact original absent-association failure goes one to zero.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test codex_import --test source_registry_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-capture-binding.log 2>&1
```

**Notes**

21 top-level tests passed across 2 binaries
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-server-library.log 2>&1
```

**Notes**

91 top-level tests passed across 1 binaries
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_admission --test capture_authority --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_activation --test capture_management --test capture_import_state --test capture_control_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-reader-server-direct.log 2>&1
```

**Notes**

106 top-level tests passed across 15 binaries
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-import-reader-check.log 2>&1
```

**Notes**

Affected packages and direct CLI compilation consumers.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-import-reader-lint.log 2>&1
```

**Notes**

Affected packages and direct CLI compilation consumers.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-reader-doc-format.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-reader-canonical.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-reader-spec-first.log 2>&1
```
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-import-reader-boundaries.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-import-reader-architecture.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-reader-progress.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-import-reader-rust-format.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 20

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-reader-doc-check.log 2>&1
```
<!-- /run -->
<!-- /evidence-field -->

