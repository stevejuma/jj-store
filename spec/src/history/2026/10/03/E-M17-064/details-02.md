---
schema: 1
id: E-M17-064
---

# E-M17-064 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-before.log 2>&1
```

**Notes**

Initial compile diagnostics: new progress API absent; fixture module naming also collided. Not 17 independent product failures.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-exact.log 2>&1
```

**Notes**

New leaf cannot use parent-private retained_progress; made helper crate-private without changing behavior.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-exact-fixed.log 2>&1
```

**Notes**

Test-only tokio attribute required an undeclared runtime. Use an immediate synthetic publisher polled through std; no new dependency or production runtime change.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-exact-ready.log 2>&1
```

**Notes**

Original cross-repository restart regression passed.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-expanded.log 2>&1
```

**Notes**

Seven durability, no-false-completion, exact retry and privacy entries passed.
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
cargo test -p jj-store-capture --locked --lib reader_publication -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-bounds.log 2>&1
```

**Notes**

Two internal bounds/binding tests; capacity retains all 4096 holes and exact folding releases one slot.
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
cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-final-exact.log 2>&1
```

**Notes**

Ten integration entries including separate-process exit and private catalog paths passed before adding ingress refusal regression.
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
cargo test -p jj-store-capture --test reader_publication --locked paths::ingress_capacity_refusal_does_not_select_a_publication_requirement -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-capacity-before.log 2>&1
```

**Notes**

Exact genuine regression: refused body capacity left a selected metadata requirement.
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
cargo test -p jj-store-capture --test reader_publication --locked paths::ingress_capacity_refusal_does_not_select_a_publication_requirement -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-capacity-after.log 2>&1
```

**Notes**

Preflight the unchanged full captured/prepared reservation under the same mutation lock before metadata selection; exact assertion one to zero.
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

Repeated after source/test edits. No formatting suppression.
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
cargo test -p jj-store-capture --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-capture-suite.log 2>&1
```

**Notes**

Early owner-suite invocation passed on pre-final source snapshot; uncached sanitizer scenarios took 260 seconds. Final exact/owner/direct qualification follows final quota repair and linear frontier folding, using existing configured sanitizer cache.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-publication-precheck.log 2>&1
```

**Notes**

Precheck passed; final direct runner compiles final source.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery --locked multiple_queued_partitions_recover_in_source_order_before_new_reading -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-mixed-before.log 2>&1
```

**Notes**

Existing exact server recovery reproduces the newly found mixed-reader gap; no assertions or deadlines changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery --locked multiple_queued_partitions_recover_in_source_order_before_new_reading -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-mixed-after.log 2>&1
```

**Notes**

Shared reader metadata now extends only an already tracked channel before acknowledging its cursor; same existing recovery target one to zero.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-publication-doc-format-final.log 2>&1
```

**Notes**

Repeated after canonical/ledger edits; historical JSON bytes preserved.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-publication-progress.log 2>&1
```

**Notes**

Ledger schema uses remaining for gaps within the evidence slice, not the wider roadmap. Source coverage is verified; broader M17 work remains explicit in scope/status/next action. Failed output preserved in /tmp/jj-store-agent-capture-m172-publication-progress-ledger-schema-before.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-exact-complete.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-exact-complete.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked --lib reader_publication -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-bounds-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-bounds-final.log
<!-- /run -->
<!-- /evidence-field -->

