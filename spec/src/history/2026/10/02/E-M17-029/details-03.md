---
schema: 1
id: E-M17-029
---

# E-M17-029 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-reanchor-connected-final.log 2>&1
```

**Notes**

All seven handoff and nine supervisor cases pass again after lazy probe boxing, including four race boundaries, exact backpressure and live independently withheld A/B intent.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor current_directory_handoff_preserves_the_exact_backpressured_batch_after_old_anchor_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-reanchor-exact-final.log 2>&1
```

**Notes**

Final original exact regression passes after all production changes.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-reanchor-ledger-format.log 2>&1
```

**Notes**

Formatted new ledger entry while preserving all 283 parent entries.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-reanchor-spec-validate.log 2>&1
```

**Notes**

All five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-reanchor-spec-first.log 2>&1
```

**Notes**

All 11 protected files map to same-change owning canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-reanchor-ledger-check.log 2>&1
```

**Notes**

All 129 tasks pass bookkeeping with 284 evidence entries; M17 and M17.2 remain In progress.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-reanchor-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes; all modified production modules remain below 300 lines.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-reanchor-ledger-format-final.log 2>&1
```

**Notes**

Final qualified ledger formatted.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-reanchor-doc-check.log 2>&1
```

**Notes**

All four changed canonical documents pass formatting.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-reanchor-progress-final.log 2>&1
```

**Notes**

Final qualified evidence bookkeeping passes; 129 tasks and 284 evidence entries.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-reanchor-spec-first-final.log 2>&1
```

**Notes**

Final same-change canonical map passes.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI 0.156.1 files over actual native MemoryStore service/WAL/AgentStore with synthetic workspace/socket verifier and pinned Kingfisher cache. Fresh fixtures/processes are distinct from real native JJ directory binding, live harness, RustFS and cold whole-server acceptance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Opaque memory-only parked reader plus at most one lazily allocated bounded current-context probe. Stable EOF releases partial line allocation and retains directory/snapshot proof. No Debug or serialization of raw state/paths/bodies. Source identity, native admission, cursor, pending batch, prepared request, actor, receipt, native and wire formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nopupqvy
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Parked retained-session handoff can independently admit current B after original ingress A is revoked without discarding exact unacknowledged batches, partial scans or unfinished reader proofs. Preparation checks original spool/physical/header/channel and durable progress before bounded context inspection and rechecks snapshot/channel/progress around fresh Write-authorized acquisition. Failures retain frame and release temporary acquisitions. Advanced sanitized progress reopens at that cursor. A metadata-only supervisor failure requests a reserved body frame; unfinished preparation cannot be reported caught-up or compacted away. Old A bodies may become constant safe omissions under separately authorized B; existing queued A entries retain independent exact authority and remain unchanged while B delivers. Restored A admission permits exact queued publication. Complete-context compaction retains metadata and renews by rebuilding after append/rewrite. New regression fixture failures and lint failure are recorded and fixed; all prior evidence entries remain unchanged. Full shared capture-runtime and direct native capture-server binaries are required by retained-reader/context/supervisor impact. Direct CLI consumers compile/lint; unrelated CLI runtime, UI, whole-Rust-workspace and root aggregate lanes are out of scope. M17/M17.2 remain In progress. Remaining M17.2 gates include saturation/rotation/requalification/task-loss/deadline faults, configured process startup and ordered storage shutdown, legacy missing AgentStore initialization, Cursor/OpenCode readers, typed authenticated controls, coverage/backlog UI and live harness/cold acceptance. M17.3-M17.7 remain pending. Automatic enabled startup stays guarded. No installed binary/service, daemon, harness execution, code head, checkpoint, credential, deployment or publication changed.
<!-- /evidence-field -->

