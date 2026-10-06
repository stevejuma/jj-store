---
schema: 1
id: E-M17-020
---

# E-M17-020 details 4

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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-source-bound-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-source-bound-first.log 2>&1
```

**Notes**

Nine protected files and three canonical chapters pass spec-first.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-source-bound-doc-format.log 2>&1
```

**Notes**

All final changed canonical document formatting passes.
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
cargo fmt -p jj-store-server
```

**Notes**

Focused test formatting was also run after correcting decoded-payload and message-chunk-count fixtures.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1 source and real pinned Kingfisher transcript-paths/v1 sanitizer; actual native service/WAL/AgentStore reads and receipts over MemoryStore. Reopened admitted reader resumes offline completion after resolved queues; no fresh OS process, real RustFS, live model execution or full server lifecycle proof.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive profile accessors and an explicit memory-only admitted-source driver/context/privacy roots with no Debug/serialization. Metadata-only source-step reports reuse the existing drain report; no model/wire, pending/prepared request, cursor journal, native alias, sanitizer, receipt or source-registry encoding changes. Native transcript record_count remains the selected message-chunk count; lifecycle events independently advance source/activity. A read-only opaque private spool binding retains canonical path and physical directory identity only in memory, without Debug/serialization or durable files; source steps reject another/rebound root while accepting fresh handles/capacity changes for the same root.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vyokrppm
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One explicit admitted-source data flow connects current event-directory and native Write authorization before source enrollment, exact active/retained native-session eligibility, source/physical leases, latest source-wide sanitized cursor, bounded Codex native reading, workspace/home/temporary redaction and pinned scanner sanitation, durable enqueue/reader acknowledgement and one-batch guarded publication. Unknown/unbound contexts retain only constant safe gaps under a separately revalidated writable prior anchor; transient admission withholds. Sanitizer workspace changes follow newly verified per-batch context. Revalidation follows sanitation before durability; queue refusal retains exact reader intent and no cursor advances. One uncommitted batch blocks further reading; only that exact singleton partition entry can use its in-memory publication proof. Unexpected/legacy pending entries or a restarted unresolved queue require separately qualified proof recovery; no header proof is substituted for old queued content. Real native read projections demonstrate repository-separated messages, safe omissions, path/secret redaction, unchanged code heads, backfill and exact admitted offline final flush, with one new message chunk, two new cursor rows and terminal Idle state. The write-before-enrollment regression preserves private registry bytes and lock count when only catalog membership exists. All four final pipeline cases plus 39 direct runtime, six server admission and 25 direct CLI spool/publication/recovery cases pass; focused all-target checks/lint, formatting and dependency boundaries qualify this slice. Cold-rule attempts exposed fixture mistakes and one obsolete concurrent attempt was terminated; final focused sanitizer evidence uses the existing pinned compiled cache and does not qualify cold whole-server startup. M17.2 and aggregate M17 remain In progress. Fair bounded blocking-I/O ownership/supervision, deadlines/backpressure/pause/restart and controls, unresolved pending-proof recovery, legacy absent AgentStore-root initialization, Cursor/OpenCode readers, Desktop/live harness and cold full-flow, UI and complete M17.3-M17.7 roadmap remain pending. Enabled startup guard remains. A concrete switched-spool pending-intent defect is fixed by binding source ownership to its original canonical/physical private root before progress reconstruction, at each step and around enqueue before ack; physical replacement at the same name also withholds. No browser lifecycle, native/WAL implementation, installed daemon, credentials, release or deployment changes; root aggregate and unrelated wider lanes are out of scope.
<!-- /evidence-field -->

