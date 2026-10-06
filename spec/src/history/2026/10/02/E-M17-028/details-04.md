---
schema: 1
id: E-M17-028
---

# E-M17-028 details 4

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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-supervisor-ledger-format.log 2>&1
```

**Notes**

Final recorded command ledger formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-supervisor-status-final.log 2>&1
```

**Notes**

All eight connected cases pass after task-loss caught-up invalidation and conservative in-flight delivery frame reservation reporting.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-supervisor-status-lint.log 2>&1
```

**Notes**

Final capture/server/CLI all-target lint passes after status-only correction.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-supervisor-doc-check.log 2>&1
```

**Notes**

All changed canonical documents pass formatting.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-supervisor-progress-final.log 2>&1
```

**Notes**

Final evidence/task bookkeeping passes after all recorded results.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-supervisor-spec-first-final.log 2>&1
```

**Notes**

Final same-change canonical map passes.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-supervisor-format-final.log 2>&1
```

**Notes**

Final affected Rust formatting passes.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI 0.156.1 files, actual native service/WAL/AgentStore over MemoryStore, synthetic workspace/socket binding verification and existing pinned Kingfisher cache. Fresh test processes/fixtures qualify explicit supervisor composition; no live harness edit, real directory JJ binding, RustFS or whole-server cold startup claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Opaque in-memory physical/header fingerprint, read-only exact queued recovery, compact EOF/metadata probes, independent forked spool handles and retained generic owned executors. Configured spool capacity must match prepared handle. Local FIFO turns retain existing authoritative filesystem journal and registry/physical/channel leases. Raw paths, proofs, parked reader state and source bodies have no Debug or serialization. Source/native admission/cursor/pending/prepared request/actor/receipt/native/wire formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change umzmlxzt
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One bounded explicit Codex supervisor surface composes configured root resolution, one-candidate owned discovery, opaque overlap/hardlink deduplication, genuine activity/current-context admission, parked independent ingress and exact native delivery. It tracks the configured source bound, caps concurrently occupied source owners at one through four, separately caps noncompact source frames at twice that bound and caps retained independent partition recovery frames at twice that bound. EOF compaction releases partial allocation without losing cutoffs, hashes, active proof or exact unacknowledged batches; unchanged compact readers perform metadata/current-admission/progress checks without re-leasing. Noncompact multi-step proofs survive turns. Independent retained blocking owners make ordinary runtime/status/pause responsive while stalled synchronous I/O stays owned; cancellation retains reservations until settlement, and shutdown has one five-second bound. B delivery progresses while A directory I/O is stalled or A exact long proof is withheld; same-partition order and exact prepared retry remain existing authority. A stale queue snapshot, in-flight work or a non-EOF partial scan cannot report caught-up. Shared in-process journal and source-acquisition FIFO prevents deterministic local lock-collision starvation while every operation still obeys OS leases and current Write. Disabled preparation does no source I/O. All before/expanded fixture, lifecycle and lint failures are retained and fixed; temporary static diagnostics were removed. Shared spool/gate behavior affects both native server and CLI consumers, justifying the complete capture runtime and direct publication/spool/provenance/recovery lanes; unrelated UI/model/whole-Rust-workspace and root aggregate lanes remain out of scope. M17.2 and aggregate M17 stay In progress. Pending gates include saturated backpressure/revocation/requalification and source rotation/retirement pressure/fault coverage, revoked parked ingress-anchor/current-directory handoff, deadline/drain faults across all supervisor lanes, configured process startup and ordered storage shutdown, legacy missing AgentStore initialization, Cursor/OpenCode readers, authenticated controls, coverage/backlog UI and real harness/cold acceptance; M17.3-M17.7 remain pending. Automatic enabled startup stays guarded. No installed service, daemon restart, code heads, credentials, harness process, checkpoints, release, deployment or publication changed. In-flight delivery status reports the reserved frame upper bound until settlement, stopped owners report zero frames and lost source owners invalidate cached caught-up. Full source task-loss requalification remains an explicit future fault gate.
<!-- /evidence-field -->

