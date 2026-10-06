---
schema: 1
id: E-M17-060
---

# E-M17-060 details 2

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
jj --no-pager st
```

**Notes**

oplstxvp empty atop E-M17-059; no overlap.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
jj describe -m 'Admit exact explicitly selected transcript history through shared source leases'
```
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
cargo test -p jj-store-capture --test source_registry_import --locked > /tmp/jj-store-agent-capture-m172-import-enrollment-before.log 2>&1
```

**Notes**

Exact acceptance target absent before implementation. Cargo rejects absent test binary; not a runtime functional regression.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt --all
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test source_registry_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-enrollment-exact.log 2>&1
```

**Notes**

Nine exact regressions pass on first functional run.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-enrollment-doc-format.log 2>&1
```

**Notes**

Repeated after exact-prefix restart refinement.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --all-targets --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-enrollment-capture.log 2>&1
```

**Notes**

Initial direct package suite passes. Code review then tightened import-only resume before landing; repeat final exact and complete direct suites for the refinement.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test source_registry_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-enrollment-prefix-exact.log 2>&1
```

**Notes**

Twelve exact tests pass, including fresh-process exact import marker reconciliation and changed-prefix refusal before sanitized progress. No assertion/deadline weakening.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-enrollment-progress.log 2>&1
```

**Notes**

New evidence metadata omitted required top-level formats and incorrectly listed roadmap-pending work as gaps in a passing slice. Repaired metadata: overall roadmap remains explicitly pending in scope and task rows; this verified primitive has no unresolved slice failures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --all-targets --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-enrollment-capture-final.log 2>&1
```

**Notes**

Final capture suite passes 122 top-level tests across 15 binaries; three fixture entries ignored and parent fresh-process proofs preserved.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-enrollment-spec-validate.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-enrollment-spec-first.log 2>&1
```

**Notes**

Protected mapping passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-import-enrollment-boundaries.log 2>&1
```

**Notes**

17 packages pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-import-enrollment-architecture.log 2>&1
```

**Notes**

719 modules pass.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-enrollment-progress-repaired.log 2>&1
```

**Notes**

Exact failed metadata gate repaired; 129 tasks pass.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-import-enrollment-rust-format.log 2>&1
```

**Notes**


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
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-enrollment-doc-check.log 2>&1
```

**Notes**


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
rustc --version
```

**Notes**

Rust 1.98.1.
<!-- /run -->
<!-- /evidence-field -->

