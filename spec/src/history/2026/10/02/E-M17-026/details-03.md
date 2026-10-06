---
schema: 1
id: E-M17-026
---

# E-M17-026 details 3

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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-parking-progress-check.log 2>&1
```

**Notes**

129 tasks validate; 281 entries preserve all 280 parent structures.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-parking-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-parking-first.log 2>&1
```

**Notes**

All protected changes include their owning canonical chapters.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-parking-doc-format.log 2>&1
```

**Notes**

Changed document formatting passes.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic Codex CLI 0.156.1 transcript files and real native service/WAL/AgentStore over MemoryStore with the existing pinned Kingfisher cache. Existing fresh-process registry/progress consumers qualify cross-process leases; new tests prove device-memory handoff only. No full cold automatic server startup, live model edit or real-RustFS claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive opaque device-memory parked reader/pipeline APIs and an actual acknowledged source checkpoint. Raw paths, partial/pending bodies, admission and recovery state have no Debug or serialization. Existing source catalog/cursor/pending/prepared request/actor/receipt/native/wire formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ynnklqvo
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One admitted-reader handoff surface preserves partial scan, multi-step prefix/context verification, exact raw pending batch and queued recovery while releasing physical and channel leases. Parking is infallible and snapshots the reader own acknowledged checkpoint; it cannot borrow later external spool progress, acknowledge bytes or persist raw state. A failed resume retains the frame; success consumes it once. Current retained eligibility, exact spool, prior anchor directory/socket/catalog and native Write checks precede reacquisition, with admission/progress checked after leases. Original root, profile, physical identity and header remain qualified. Foreign, rebound, revoked and concurrently leased sources withhold. Unchanged durable cursor reuses the frame without restarting verification; another owner advanced sanitized progress causes a newly opened reader at that cursor. Missing, regressive or changed same-position progress fails closed. Actual spool capacity refusal retains the original batch across append and handoff, with sanitized body checks. Source replacement cannot reuse it. New regression before API failure, wrong import and fixture payload assertion failures are retained and corrected. Eight new handoff cases, 80 full shared-runtime cases, 30 direct native server and 31 direct CLI cases plus affected/direct-consumer check/lint/format and boundaries qualify this slice. Production modules remain below 300 lines. M17.2 and aggregate M17 remain In progress. The full supervisor still must bound resident frames and raw buffers separately from reader leases, compose discovery/activity/current-context fresh admission/staged resume/ingress/exact publisher under owned blocking-I/O deadlines and pause/restart, and integrate guarded startup. Parking alone does not admit a fresh changed-directory session or bypass a revoked prior anchor. Legacy absent-domain initialization, Cursor/OpenCode readers, controls/UI, full cold server/harness and M17.3-M17.7 remain pending. No installed daemon, service startup, code heads, credentials, harness process, publication, deployment or release changed. Root aggregates and unrelated UI/model/real-storage lanes remain out of scope under targeted policy.
<!-- /evidence-field -->

