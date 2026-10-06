---
schema: 1
id: E-M17-028
---

# E-M17-028 details 3

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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-supervisor-lint-qualified.log 2>&1
```

**Notes**

All affected/direct-consumer targets lint after private branch and fixture update corrections.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-supervisor-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; no dependency or wire format change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor enabled_supervisor_backfills_new_start_without_ingesting_inactive_history --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-exact-final.log 2>&1
```

**Notes**

Original exact connected case still passes after shared journal FIFO and resource tests.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked -- --exact --nocapture > /tmp/jj-store-agent-capture-m172-supervisor-blocked-fifo-detail.log 2>&1
```

**Notes**

Static diagnostic reproduces remaining source registry acquisition contention; removed diagnostic after shared acquisition FIFO and compact parked metadata probes were added.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-supervisor-idle-final.log 2>&1
```

**Notes**

All eight connected cases pass, including retained stalled ownership/B delivery, fair long A/B proof turns and bounded multi-step inactive probes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-supervisor-runtime-qualified.log 2>&1
```

**Notes**

Full capture runtime passes again after source acquisition FIFO and parked metadata-only probes: 84 parent cases and launched privacy child.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_activation --test capture_parking --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-supervisor-server-qualified.log 2>&1
```

**Notes**

All 42 directly affected native server cases pass, including all eight supervisor cases. Synthetic workspace verification over actual native MemoryStore service/WAL/AgentStore is distinct from filesystem binding and whole-server/cold qualification.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-supervisor-lint-idle.log 2>&1
```

**Notes**

All affected/direct-consumer targets lint after metadata-only idle and source registry FIFO composition.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-supervisor-check-qualified.log 2>&1
```

**Notes**

All affected/direct-consumer targets compile after pure configured/spool budget agreement check.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-supervisor-connected-qualified.log 2>&1
```

**Notes**

Eight connected cases pass after pure budget validation, including refusal of a mismatched prepared spool capacity.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-supervisor-format.log 2>&1
```

**Notes**

Affected Rust formatting passes; modified production modules remain below 300 lines.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor enabled_supervisor_backfills_new_start_without_ingesting_inactive_history --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-exact-qualified.log 2>&1
```

**Notes**

Final exact original connected regression passes after all implementation changes.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-supervisor-lint-budget.log 2>&1
```

**Notes**

Final strict all-target lint passes for capture/server and CLI direct consumers.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test local_agent_provenance --test local_agent_provenance_cli --locked > /tmp/jj-store-agent-capture-m172-supervisor-cli-qualified.log 2>&1
```

**Notes**

All 43 direct CLI cases pass again after shared registry FIFO, metadata-only parked probes and pure supervisor budget agreement.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-supervisor-doc-format.log 2>&1
```

**Notes**

All owning canonical chapters, verification and ledger formatted.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-supervisor-spec-validate.log 2>&1
```

**Notes**

All five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-supervisor-spec-first.log 2>&1
```

**Notes**

All 28 protected files map to same-change owning canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-supervisor-progress.log 2>&1
```

**Notes**

All 129 tasks pass bookkeeping; M17/M17.2 remain In progress and evidence count is 283.
<!-- /run -->
<!-- /evidence-field -->

