---
schema: 1
id: E-M17-031
---

# E-M17-031

<!-- evidence-field:id markdown -->
## Id

E-M17-031
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

qualification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/tests/capture_supervisor/backlog.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_supervisor/backlog.rs#full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart
- crates/jj-store-server/tests/capture_supervisor/backlog.rs#captured_preparation_growth_withholds_exact_intent_until_explicit_capacity_relief
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor backlog::full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-backlog-before.log 2>&1
```

**Notes**

Before: named durable backlog target was absent; zero tests ran. This is missing-coverage evidence, not acceptance.
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
cargo fmt -p jj-store-server
```

**Notes**

Affected Rust test formatting.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor backlog::full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-backlog-exact.log 2>&1
```

**Notes**

New fixture initially imported WAL directly from the server test, which has no such dependency. Corrected to the owned service transport types without adding a dependency.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor backlog::full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-backlog-owned-boundary.log 2>&1
```

**Notes**

Original exact full prepared backlog target passes: actual native commit then simulated lost acknowledgement; full byte-bound queue, A revocation, B ingress refusal, pause/restart and exact A receipt recovery before B delivery.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor backlog::captured_preparation_growth_withholds_exact_intent_until_explicit_capacity_relief --locked -- --exact > /tmp/jj-store-agent-capture-m172-backlog-captured-growth.log 2>&1
```

**Notes**

Captured preparation growth under exact full capacity retains original captured bytes/cursor and selects no native root. Explicit configured capacity relief permits guarded publication. This proves the current fail-closed profile, not automatic headroom.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-backlog-supervisor.log 2>&1
```

**Notes**

All 12 connected supervisor cases pass, including both new backlog cases and prior pressure, handoff and stalled draining ownership.
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
cargo check -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-backlog-check.log 2>&1
```

**Notes**

Focused owning package/test target compiles.
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
cargo clippy -p jj-store-server --test capture_supervisor --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-backlog-clippy.log 2>&1
```

**Notes**

Focused owning package/test target passes strict lint.
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
cargo fmt -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-backlog-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes; no production code changed.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-backlog-doc-format.log 2>&1
```

**Notes**

Canonical chapter formatting.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-backlog-ledger-format.log 2>&1
```

**Notes**

Ledger formatting.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-backlog-doc-check.log 2>&1
```

**Notes**

All changed documents pass formatting.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-backlog-spec-validate.log 2>&1
```

**Notes**

5 validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-backlog-spec-first.log 2>&1
```

**Notes**

Same-change gate passes for the protected test files and canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-backlog-progress.log 2>&1
```

**Notes**

129 tasks pass bookkeeping; all prior 285 evidence entries preserved, total 286; M17/M17.2 remain In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI 0.156.1 transcript over actual native MemoryStore service/WAL/AgentStore, synthetic workspace/socket admission and pinned Kingfisher cache. A fixture reports unknown only after the real native publisher commits; this is not an actual network timeout, RustFS, live harness or whole-server cold startup proof.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, dependency, native/source/admission/cursor/pending/prepared request/actor/receipt or wire format change. Existing exact prepared native bytes fill the configured physical byte budget; snapshots verify no mutation across revocation/pause/restart. Latest sanitized source progress remains unchanged on rejected ingress. Captured preparation overflow retains the current legacy hard-bound encoding unchanged until explicit configured capacity relief.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-031 details 2](details-02.md)
