---
schema: 1
id: E-M17-050
---

# E-M17-050

<!-- evidence-field:id markdown -->
## Id

E-M17-050
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/capture/selection.rs
- crates/jj-store-server/src/process/tests/capture_factory.rs
- crates/jj-store-server/src/process/tests/capture_factory/controls.rs
- crates/jj-store-server/src/process/tests/capture_factory/foreground.rs
- spec/src/collaboration.md
- spec/src/cli.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/process/tests/capture_factory/controls.rs#cold_control_policy_is_revalidated_before_foreground_dispatch
- crates/jj-store-server/src/process/tests/capture_factory.rs#cold_capture_factory_prepares_private_resources_and_retains_ordered_lifetime
<!-- /evidence-field -->

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
cargo fmt --all
```

**Notes**

Pinned formatting before/after cold matrix and production composition.
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
cargo test -p jj-store-server --lib process::tests::capture_factory::controls::cold_control_policy_is_revalidated_before_foreground_dispatch --locked > /tmp/jj-store-agent-capture-m172-control-factory-before.log 2>&1
```

**Notes**

One E0609 fixture field mismatch; use existing status.tracked with unchanged zero-source assertion.
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
cargo test -p jj-store-server --lib process::tests::capture_factory::controls::cold_control_policy_is_revalidated_before_foreground_dispatch --locked > /tmp/jj-store-agent-capture-m172-control-factory-before2.log 2>&1
```

**Notes**

One genuine persisted-pause functional failure in fresh subprocess: factory ignores selected pause and observes Ready rather than Paused. Parent matrix stops at this original failed case.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --lib process::tests::capture_factory::controls::cold_control_policy_is_revalidated_before_foreground_dispatch --locked > /tmp/jj-store-agent-capture-m172-control-factory-after.log 2>&1
```

**Notes**

Three private-type compile errors and one private_interfaces warning after ready-slot extraction; broaden ReadyCapture only to its owning process module, without exposing a public entrypoint.
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
cargo test -p jj-store-server --lib process::tests::capture_factory::controls::cold_control_policy_is_revalidated_before_foreground_dispatch --locked > /tmp/jj-store-agent-capture-m172-control-factory-after2.log 2>&1
```

**Notes**

Exact original matrix passes in 1.90 seconds: ten fresh subprocesses qualify persisted/latest pause/resume, missing state and generation/installation/physical-binding refusal. Missing policy is not reset; no transcript enrollment/publication/scanner during pause.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-control-factory-consumers.log 2>&1
```

**Notes**

All 40 direct entries pass: 31 owning library in 109.37 seconds and nine runtime in 22.80 seconds. Includes 16 existing cold factory/foreground modes and ten new control modes. No-op helper entries are not independent behavior tests. Existing active native MemoryStore sanitized capture, original paused handoff and ownership/drop/drain cases pass.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-control-factory-check.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-control-factory-clippy.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-control-factory-doc-format.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-control-factory-format.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-control-factory-rustfmt.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-control-factory-boundaries.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-control-factory-architecture.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-control-factory-spec-validate.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-control-factory-spec-first.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-control-factory-progress.log 2>&1
```

**Notes**

Direct gate passes; document/canonical/progress gates repeated after final sealing.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-050 details 2](details-02.md)
