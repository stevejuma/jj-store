---
schema: 1
id: E-M17-020
---

# E-M17-020 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source changed_spool_binding_never_clears_a_retained_source_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-source-binding-before.log 2>&1
```

**Notes**

A concrete context switch to another spool incorrectly cleared the retained pending proof; exact new regression establishes one assertion failure.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source changed_spool_binding_never_clears_a_retained_source_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-source-binding-after.log 2>&1
```

**Notes**

Exact spool-switch regression passes after a read-only canonical/physical binding; reopening the original unchanged root still publishes its exact queued batch.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --locked > /tmp/jj-store-agent-capture-m172-source-bound-final.log 2>&1
```

**Notes**

All four final pipeline cases pass; extended binding case additionally replaces the physical directory at the same canonical name and preserves the old queue.
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
cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-source-spool-consumers.log 2>&1
```

**Notes**

All 25 directly affected shared spool/native publication/local recovery cases pass after the additive binding API; two subprocess helpers remain invoked by parent tests.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-source-bound-check.log 2>&1
```

**Notes**

Final spool-bound affected/direct-consumer all-target checks pass.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-source-bound-clippy.log 2>&1
```

**Notes**

Final spool-bound strict all-target lint passes.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-source-bound-format.log 2>&1
```

**Notes**

Final spool-bound Rust and embedded-runtime formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --locked > /tmp/jj-store-agent-capture-m172-source-bound-admission.log 2>&1
```

**Notes**

All four pipeline cases pass after spool binding is pinned before source-progress reconstruction and checked again before returning the reader.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-source-bound-admission-clippy.log 2>&1
```

**Notes**

Strict lint passes after constructor binding checks.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --locked > /tmp/jj-store-agent-capture-m172-source-bound-ack.log 2>&1
```

**Notes**

All four final cases pass after source steps repeat spool binding around sanitized enqueue and before acknowledgement.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-source-bound-ack-clippy.log 2>&1
```

**Notes**

Final strict all-target lint passes after acknowledgement-boundary binding checks.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-source-doc-write.log 2>&1
```

**Notes**

Initial canonical document formatting before the additional binding regression.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-source-progress.log 2>&1
```

**Notes**

Initial progress bookkeeping passes before added binding evidence.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-source-spec.log 2>&1
```

**Notes**

Initial canonical validators pass before added binding evidence.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-source-first.log 2>&1
```

**Notes**

Initial owning-chapter gate passes before added binding evidence.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-source-doc-format.log 2>&1
```

**Notes**

Initial changed canonical document formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-source-bound-doc-write.log 2>&1
```

**Notes**

Final expanded canonical documents formatted.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-source-bound-progress.log 2>&1
```

**Notes**

129 tasks validate; 275 evidence structures preserve all 274 parent entries.
<!-- /run -->
<!-- /evidence-field -->

