---
schema: 1
id: E-M17-023
---

# E-M17-023 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-schedule-doc-format.log 2>&1
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic sanitized hook spool and direct native MemoryStore service consumers with the existing pinned Kingfisher cache. No real harness edit, whole-server cold restart or real-RustFS claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing pending/cursor/prepared request/receipt/catalog formats are unchanged. The scheduler holds only one volatile repository/integration/source turn cursor and borrows validated pending entries. Exact publication reports and CLI reexports remain compatible.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oylyqxln
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One native queue selection behavior rotates bounded exact partition heads, advancing before delivery so withheld/failed/cancelled/unknown outcomes cannot reserve a later turn. The 4096-entry bound and complete input validation precede cursor mutation. Duplicate IDs fail closed; empty/all-local selections retain the cursor. Local prepared intent excludes its entire partition. Exact drain rechecks live generation/source order and local exclusion under the drain lease, including earlier enqueues after selection and already-prepared unknown intent, without changing any entry or calling a publisher on refusal. Another partition does not block delivery. Each selected batch still requires independent exact device proof, current native directory/socket/catalog admission and Write authority; this metadata selector grants none. The exact before failure is retained and fixed. The existing exact-entry unrelated-batch regression now uses a genuinely foreign repository rather than qualifying same-partition reordering. Complete direct publication, shared spool/local recovery and native server consumers, affected/direct-consumer check/lint/format and boundary checks qualify this changed shared runtime behavior. Production modules remain below 300 lines. M17.2 and aggregate M17 remain In progress. Independent bounded ingress, staged worker composition, temporal verification scheduling, blocking-I/O ownership, deadlines/backpressure/pause/restart, legacy absent-root initialization, Cursor/OpenCode readers, controls/UI and M17.3-M17.7 remain pending; automatic startup stays guarded. No installed service, code heads, harness process, credentials, publication, deployment or release changed. Root aggregates, unrelated UI/reader suites and wider real-storage qualification remain out of scope under the targeted policy.
<!-- /evidence-field -->

