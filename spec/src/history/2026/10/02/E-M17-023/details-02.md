---
schema: 1
id: E-M17-023
---

# E-M17-023 details 2

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
cargo test -p jj-store-cli --test agent_capture_publication exact_entry_cannot_bypass_an_earlier_same_partition_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-schedule-before.log 2>&1
```

**Notes**

Original exact regression fails: a later selected batch publishes despite an earlier batch enqueued through another handle in the same partition.
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
cargo test -p jj-store-cli --test agent_capture_publication exact_entry_cannot_bypass_an_earlier_same_partition_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-schedule-after.log 2>&1
```

**Notes**

Original exact regression passes after the live partition-head check under the drain lease.
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
cargo fmt -p jj-store-capture -p jj-store-cli
```

**Notes**

Affected formatting passes; cargo fmt -p jj-store-cli repeated after expanding prepared-head assertions.
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
cargo test -p jj-store-cli --test agent_capture_publication --locked > /tmp/jj-store-agent-capture-m172-schedule-publication.log 2>&1
```

**Notes**

18 primary cases pass; both process fixtures are invoked by their parents. Four scheduler cases cover unknown prepared retry/reopen, withheld turns, dynamic partitions, empty/invalid/duplicate/bounded inputs, local-partition exclusion and generation/integration ordering.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-schedule-check.log 2>&1
```

**Notes**

Affected capture runtime and direct server/CLI consumers compile.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-schedule-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-schedule-server-direct.log 2>&1
```

**Notes**

All 19 direct native cases pass; queued proof/current context, staged resume, current admission and prepared identity semantics remain covered.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-schedule-clippy.log 2>&1
```

**Notes**

Affected/direct-consumer strict lint passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-schedule-cli-direct.log 2>&1
```

**Notes**

Nine shared spool and four direct local recovery cases pass; foreign local intent and durable progress remain unchanged.
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
cargo fmt -p jj-store-capture -p jj-store-cli -- --check > /tmp/jj-store-agent-capture-m172-schedule-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
cargo test -p jj-store-cli --test agent_capture_publication --locked > /tmp/jj-store-agent-capture-m172-schedule-publication-final.log 2>&1
```

**Notes**

All 18 primary publication cases pass after expanding the original regression to retain an earlier exact prepared unknown request.
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
cargo test -p jj-store-cli --test agent_capture_publication exact_entry_cannot_bypass_an_earlier_same_partition_intent --locked -- --exact > /tmp/jj-store-agent-capture-m172-schedule-exact-final.log 2>&1
```

**Notes**

Original exact regression rerun passes, including captured and prepared earlier heads; no publisher call or queue change on refusal.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-schedule-clippy-final.log 2>&1
```

**Notes**

Final affected/direct-consumer strict lint includes the expanded prepared-head fixture.
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
cargo fmt -p jj-store-capture -p jj-store-cli -- --check > /tmp/jj-store-agent-capture-m172-schedule-rust-format-final.log 2>&1
```

**Notes**

Final affected formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-schedule-doc-write.log 2>&1
```

**Notes**

Final canonical chapters, verification and evidence formatting passes.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-schedule-progress.log 2>&1
```

**Notes**

129 tasks validate; 278 evidence entries preserve all 277 parent structures.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-schedule-spec.log 2>&1
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-schedule-first.log 2>&1
```

**Notes**

Final protected changes include their owning canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

