---
schema: 1
id: E-M17-025
---

# E-M17-025 details 3

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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-worker-first.log 2>&1
```

**Notes**

Final protected changes include their owning canonical chapters.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-worker-doc-format.log 2>&1
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic Codex CLI 0.156.1 native service/WAL/AgentStore over MemoryStore with the existing pinned Kingfisher cache. New test binaries/runtimes exercise real owned pipeline tasks and source leases; deterministic private fake-I/O gates separately qualify executor ownership and timing. No full cold automatic server startup, live model edit or real-RustFS claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive in-memory capture worker command/phase/poll APIs and private owned executor. Raw device/source/proof/spool state has no Debug or serialization. Existing source/catalog/cursor/pending/prepared request/actor/receipt and native/wire formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kruvryxv
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One explicit leased-pipeline ownership surface moves bounded synchronous reader/sanitizer/spool work to one owned blocking task and runs existing asynchronous native checks/publication through the current runtime. The same pipeline, two source leases, private spool, authority and admission context remain in exactly one idle/in-flight owner. Dispatch deadline is positive and at most 300 seconds; expired queued work is never started. No second step or replacement can race a running/overdue/draining owner. Pause cancels asynchronous waits and prevents dispatch; resume cannot bypass draining or current native Write. Synchronous OS work may outlive a deadline, so phase/poll remain responsive while original resources stay retained; a known late completion retains its meaning. Actual five-second shutdown timeout keeps the owner for later settlement; successful retry releases resources. Dropped controllers request cancellation while running blocking work keeps ownership until settlement. Task loss closes that owner and requires renewed qualification. No deadline/cancellation invents a conflict or discards pending/prepared intent. Eight single-thread/deterministic executor cases are distinct from two actual native facade cases and the complete direct capture/spool/publication/recovery consumers. Exact before API failure is retained and fixed; affected/direct-consumer compile/lint/format, package boundaries and canonical checks qualify the slice. Production modules remain below 300 lines. This per-pipeline owner does not allocate additional readers, manage discovery/activity/resume probes, schedule partitions globally, enable a configured server or implement typed authenticated management controls. M17.2 and aggregate M17 stay In progress. Complete fair bounded supervisor composition including preparation I/O, legacy missing-root initialization, selected-reader coverage, deadlines/backpressure/pause/restart controls, full cold server/harness acceptance, UI and M17.3-M17.7 remain pending. Enabled startup stays guarded; server composition must drain owners before authoritative storage shutdown. No installed daemon, service process integration, code heads, credentials, harness process, publication, deployment or release changed. Root aggregate and unrelated UI/reader/model suites remain out of scope under targeted policy.
<!-- /evidence-field -->

