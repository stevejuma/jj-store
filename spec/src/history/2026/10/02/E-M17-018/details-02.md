---
schema: 1
id: E-M17-018
---

# E-M17-018 details 2

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
cargo test -p jj-store-cli --test agent_capture_publication bounded_partition_drain_retains_other_repositories_sources_and_unknown_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-partition-before.log 2>&1
```

**Notes**

Inherited exact regression establishes one unresolved-import error for the two missing APIs.
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
cargo fmt -p jj-store-capture -p jj-store-cli
```

**Notes**

Repeated focused formatting after module extraction, regressions and lint-fixture correction.
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
cargo test -p jj-store-cli --test agent_capture_publication bounded_partition_drain_retains_other_repositories_sources_and_unknown_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-partition-after.log 2>&1
```

**Notes**

Exact missing-interface regression now passes; mixed-repository whole-spool refusal and bounded independent partitions retain unknown exact intent across reopen.
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
cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-partition-cli.log 2>&1
```

**Notes**

Additional regression incorrectly called a private lease helper; replaced with a live competing drainer fixture rather than exposing the helper.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-partition-check.log 2>&1
```

**Notes**

Same private-helper fixture error; corrected before final verification.
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
cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-partition-cli-after.log 2>&1
```

**Notes**

25 direct publication/spool/local-recovery cases pass; two ignored subprocess helpers are invoked by parent tests. Four new partition cases cover unknown retry, untouched local/native partitions, selected-local refusal before preparation, mismatch, cancellation, empty selection and bounds.
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
cargo test -p jj-store-capture --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-partition-progress.log 2>&1
```

**Notes**

All 11 direct cursor journal, native/local completion-repair, partitioned recovery, resource and regression cases pass.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-partition-check-after.log 2>&1
```

**Notes**

Affected runtime and both direct Rust consumers pass all-target checks.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-partition-clippy.log 2>&1
```

**Notes**

New regression retained a lexical mutex guard across awaits despite explicit drop; narrowed its block without suppression.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-partition-clippy-after.log 2>&1
```

**Notes**

Strict affected/direct-consumer all-target lint passes.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-partition-format.log 2>&1
```

**Notes**

Rust and embedded-runtime formatting pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-partition-boundaries.log 2>&1
```

**Notes**

17-package dependency boundaries pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-partition-architecture.log 2>&1
```

**Notes**

719-module web architecture audit passes; presentation remains untouched.
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
cargo test -p jj-store-cli --test agent_capture_publication bounded_partition_drain_retains_other_repositories_sources_and_unknown_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-partition-final.log 2>&1
```

**Notes**

Exact regression passes after the lexical fixture-lifetime correction.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-partition-doc-write.log 2>&1
```

**Notes**

Changed canonical documents formatted.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-partition-ledger.log 2>&1
```

**Notes**

129 tasks validate; 273 evidence structures preserve all 272 parent entries.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-partition-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-partition-first.log 2>&1
```

**Notes**

Three protected files and three canonical chapters pass spec-first.
<!-- /run -->
<!-- /evidence-field -->

