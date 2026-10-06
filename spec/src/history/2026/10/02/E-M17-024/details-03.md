---
schema: 1
id: E-M17-024
---

# E-M17-024 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-pipeline-doc-format.log 2>&1
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1 JSONL with actual native service/WAL/AgentStore over MemoryStore and the existing pinned Kingfisher cache. Unknown publication is injected while real preparation and exact native retry remain qualified. Object/reader reopen is not fresh whole-server startup, live harness editing or real-RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Memory-only consuming CodexCapturePipeline and separate bounded ingress/exact delivery APIs. No Debug or serialization for raw device state; no accumulating batch-to-directory map. Existing pending, source-wide cursor, prepared request, actor, receipt and native catalog/wire shapes remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lulylzuk
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One admitted-source scheduling surface extracts existing ingress and exact proof recovery, preserves the conservative serial step convenience and adds a consuming pipeline with independent bounded ingress and selected-entry delivery. New sanitized batches retain current directory/socket/catalog/Write checks and durable spool-before-ack semantics while old captured/prepared/unknown entries remain unresolved. The source-wide cursor, exact unacknowledged batch and existing capacity bound remain authoritative. Delivery reconstructs each selected range independently with one bounded probe; changing selections restarts that proof and exact drain enforces live same-partition order. Publishing old intent cannot rewind the current ingress directory anchor or reader; staged admitted resume can ingest an offline B tail while original A is revoked and prepared A remains exact. After B commits, restored A retries its original prepared delta and receipt identity; constant later omissions remain anchored in current B. A/B/A delivery rejects later A before earlier A, while B independently commits. Capacity refusal leaves cursor/intent unchanged and a later append cannot change the retained exact pending B batch. Switched spools and lost current native Write withhold both lanes. The exact absent-API failure and two corrected fixture errors are retained; complete directly affected native and CLI consumers, compile/lint/format and boundaries qualify the extraction. Production modules remain below 300 lines. M17.2 and aggregate M17 stay In progress. Temporal fairness for multi-step verification, independent blocking-I/O ownership, deadlines/backpressure/pause/restart controls, discovery/activity/resume/pipeline supervisor composition, legacy absent AgentStore-root initialization, Cursor/OpenCode readers, UI and M17.3-M17.7 remain pending; enabled startup stays guarded. No installed daemon, code heads, harness process, credentials, release, publication or deployment changed. Root aggregates, unrelated raw-reader/model/UI suites and whole-server real-storage/lifecycle gates are out of scope under the targeted policy.
<!-- /evidence-field -->

