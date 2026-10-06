---
schema: 1
id: E-M17-018
---

# E-M17-018 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-partition-doc-format.log 2>&1
```

**Notes**

All changed canonical documents pass formatting.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic sanitized native-hook captures and exact native/local prepared requests. Publication fixtures and fresh handles/subprocesses, no live harness or RustFS execution.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive metadata-only repository/integration/source selector and one-through-256 batch bound. Existing CaptureDrainReport counts the entire shared queue; pending entries, reader progress, prepared request/receipt encodings, request identities, sanitization and native/local completion semantics are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tvywvvwv
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One native spool selection behavior extracts the exact-intent publication/completion loop for reuse by strict whole-spool CLI and bounded shared-spool partition drains. Invalid bounds fail before the drain lease; matching LocalPrepared requests anywhere in the selected partition fail before preparation even outside the current bound. Filtering, ordering and truncation leave foreign native/local partitions untouched. Unknown outcomes stop only this call with durable exact intent; mismatch/cancellation retain requests, release the global drain lease and retry without repreparation. Other processes may still enqueue while publication waits; existing cursor completion repairs durable reader progress before removal. Reports count all remaining entries and empty selections invoke no publisher. Selection grants no directory admission, write authority or scheduling fairness; the supervisor remains responsible for each. Exact regression, complete direct CLI publication/spool/local-recovery binaries, direct reader-progress cases, affected/direct-consumer all-target check/lint, formatting and architectural checks qualify this slice. M17.2 and aggregate M17 remain In progress; supervised automatic Codex composition, per-publication current admission, fair scheduling/deadlines/backpressure/controls, absent-domain initialization, pending-source recovery, Cursor/OpenCode readers, real harness/cold full-flow and the full M17.3-M17.7 roadmap remain pending. Startup guard remains in place. No model/RPC/WAL schema, browser lifecycle, credential, installed service, deployment or release behavior changed; root aggregate and unrelated wider suites are out of scope.
<!-- /evidence-field -->

