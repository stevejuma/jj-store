---
schema: 1
id: E-M17-039
---

# E-M17-039 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-reservations-cli.log 2>&1
```

**Notes**

31 direct native/local CLI cases pass; two process-entry helpers remain ignored in parent runs.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor raw_handoff:: --locked > /tmp/jj-store-agent-capture-m172-raw-handoff-final.log 2>&1
```

**Notes**

Exact final raw admission fault passes with an explicit renewed quantum after restored catalog admission; original physical/channel and deadline assertions unchanged.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-reservations-supervisor-final.log 2>&1
```

**Notes**

All final 27 registered supervisor cases pass at default concurrency. Combined with the previous complete direct native lane, 58 distinct final server cases qualify this slice.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-reservations-lint-final.log 2>&1
```

**Notes**

Final server targets pass strict lint after the added explicit resumed quantum; runtime and CLI unchanged since their full passing lint lane.
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
cargo fmt --all -- --check > /tmp/jj-store-agent-capture-m172-reservations-rust-format-check.log 2>&1
```

**Notes**

Final Rust source formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-reservations-doc-format.log 2>&1
```

**Notes**

Changed canonical chapters, verification and ledger formatted; repeated for final sealed evidence.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-reservations-spec-validate.log 2>&1
```

**Notes**

Canonical specification passes five validators over 21 chapters and 160 requirements.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-reservations-spec-first.log 2>&1
```

**Notes**

Spec-first passes for 14 protected files and three canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-reservations-progress.log 2>&1
```

**Notes**

All 129 tasks pass bookkeeping. Prior 293 entries preserved, total 294; M17/M17.2 remain In progress.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-reservations-doc-check.log 2>&1
```

**Notes**

Final sealed canonical chapters and ledger pass formatting; repeated after recording this result. Canonical validation, spec-first and progress were also repeated after sealing and passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI with native MemoryStore publication and current device admission. No live harness or real RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive opaque automatic parking and physical-only reserved acquisition. No lock descriptor cloning or raw-state encoding. Existing source registry V1, reader progress/pending V1, prepared request and durable receipt formats unchanged. Only current AgentStore is supported; compact durable spool retries remain independently supported.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lxpwunyn
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Automatic raw allocations share one parking/physical-reacquisition boundary across the capture runtime, server and direct CLI consumers, requiring complete capture tests, direct native parking/resume/pipeline/supervisor tests and CLI spool/publication/recovery acceptance. Generic parking remains lease-free. Automatic parking retains only the raw batch channel within the existing bounded noncompact frame allocation and releases physical ownership. Retained channel/registry/lock/profile/header and current admission checks precede exact-once handoff. Failed or cancelled physical acquisition retains raw Arc/cursor/batch/context identity; uncoordinated foreign allocation remains withheld without renumbering. Same-limit native queue relief publishes sanitized current B evidence without borrowing revoked A authority; shutdown releases ephemeral ownership and fresh retained-session recovery backfills unacknowledged bytes. No durable evidence is evicted or migrated. Previous 293 evidence entries are preserved. M17/M17.2 remain In progress; broader allocation/rotation/root-rebinding/task-loss/deadline matrices, configured startup and ordered storage shutdown, Cursor/OpenCode readers, authenticated controls/coverage UI and real harness/cold qualification remain pending; M17.3-M17.7 remain pending. Automatic enabled startup stays guarded. Root aggregates, unrelated UI/model/whole Rust workspace and real-storage qualification are out of scope. No installed binaries/services, unrelated daemon restart, credentials, checkpoints, harness execution, deployment or publication changed. Final qualification completes this narrow parking behavior: 102 main-process capture tests plus one fresh-process privacy helper, 58 distinct direct native server tests and 31 CLI tests, owning/direct consumer compile and strict lint, Rust formatting, boundaries, architecture, canonical validation, spec-first and ledger gates all pass. The complete supervisor lane ran after final fault-test registration; no whole-workspace or release qualification is claimed. Incidental inspection guesses for absent module directories were corrected using the actual owned files and are not product evidence. All fixture failures and the actual before exclusion failure remain recorded; no assertions, supported formats or deadlines were weakened.
<!-- /evidence-field -->

