---
schema: 1
id: E-M17-038
---

# E-M17-038 details 4

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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-generations-doc-format-sealed.log 2>&1
```

**Notes**

Changed canonical documents formatted after capacity requirements/evidence additions; repeated after final ledger sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-generations-spec-validate-sealed.log 2>&1
```

**Notes**

Final canonical specification validates; repeated after final ledger sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-generations-spec-first-sealed.log 2>&1
```

**Notes**

Spec-first gate passes: 23 protected files and three canonical chapters; repeated after final ledger sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-generations-progress-sealed.log 2>&1
```

**Notes**

All 129 tasks validate; previous 292 entries preserved; M17/M17.2 remain In progress. Repeated after final ledger sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-generations-doc-check.log 2>&1
```

**Notes**

Final changed canonical documents and sealed ledger pass formatting; repeated after recording this result.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wrnzmoxx
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI with native MemoryStore publication and device directory/socket admission. No live harness, real RustFS or cold configured server acceptance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive runtime ReaderSourceProgress snapshot and automatic enqueue methods, exact physical/header digest accessor and lease-bound floor refresh. Existing private source registry V1, per-repository reader journal V1, pending V1, prepared request, actor, native/WAL receipt and generic generation interpretation unchanged. Only current AgentStore remains supported; compact durable spool retry support remains independent.
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

One physical-generation recovery behavior affects shared reader selection and sanitized enqueue plus native/local direct consumers. Automatic readers use the matching physical/header progress across every retained repository partition and pending entry; channel-wide generation and sequence floors are independent maxima. Generic sanitized pending entries also occupy allocation floors. Unchanged files do not alternate new generations. Delayed archived/current appends retain verified prefixes and allocate monotonic continuation generations without replaying acknowledged bytes. Truncation and equal-length rewrites reset offsets above all known generation floors. Fresh lease acquisition and ingress refresh bounds; final automatic admission is atomic under the mutation lock. Exact already-durable retries retain prior receipt/intent interpretation. Conflicting raw frames retain their identities/bytes and cannot enqueue or leak leases; complete pre-durability allocation liveness remains an explicit next gate. No accepted batch, ID, journal, prepared bytes or receipt is renumbered or rewritten. Generic APIs retain prior accepted generation semantics. Complete shared capture tests and direct native/server/CLI cursor, spool, publication, resume and parking consumers are justified by this common allocation boundary; UI, root aggregate, whole Rust workspace and real-storage lanes remain out of scope. All previous 292 evidence entries preserved. M17/M17.2 remain In progress, with raw-allocation liveness/races, wider rotation/root rebinding and task-loss/deadline faults, configured startup/ordered storage shutdown, Cursor/OpenCode readers, authenticated controls/coverage UI and real harness/cold qualification pending. M17.3-M17.7 remain pending. Automatic enabled startup remains guarded. No installed service/binary, unrelated daemon restart, credential, checkpoint, deployment or publication changed. Initial attempted canonical patch missed wrapped context and made no change; the substantive requirement patch was applied against actual text. Incidental inspection requests for nonexistent guessed module paths/globs were corrected by scoped file inventory; no such failure is counted as product evidence. Review found and fixed an otherwise-hidden independent-handle capacity refusal in the new snapshot lookup; its actual before failure and exact after pass retain E-M17-037 diagnostics through acquisition, ingress and parking. A later canonical patch missed newly formatted context and made no change before correct reapplication. Final qualification is 95 main-process capture cases plus one fresh-process privacy helper, 55 direct native server cases and 31 direct CLI cases. The original broad server timeout remains recorded; exact isolation and two subsequent complete default-concurrency runs pass unchanged. This resolves the slice verification failure without qualifying heavier concurrent-suite pressure or weakening deadlines. All required final owning/direct-consumer compile/lint, source boundaries and changed-document gates pass. Partial JST-COLLAB-017/019 verification rows and full M17/M17.2 status remain unchanged; raw allocation liveness and the full roadmap remain required.
<!-- /evidence-field -->

