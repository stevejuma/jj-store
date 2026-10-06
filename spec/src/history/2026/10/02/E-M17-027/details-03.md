---
schema: 1
id: E-M17-027
---

# E-M17-027 details 3

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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-activation-spec.log 2>&1
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-activation-first.log 2>&1
```

**Notes**

Protected changes include their owning canonical chapters.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-activation-doc-format.log 2>&1
```

**Notes**

Changed document formatting passes.
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
rustc --version
```

**Notes**

Rust 1.98.1 (48a229cea 2026-09-01) verified.
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
jj --version
```

**Notes**

JJ 0.45.1 verified.
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
uname -m
```

**Notes**

ARM64 host verified.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI 0.156.1 files and native service/WAL/AgentStore over MemoryStore with synthetic workspace/socket binding verification and the existing pinned Kingfisher cache. New fixture process qualifies explicit staged APIs; no whole-server automatic startup, live harness edit, real filesystem JJ binding or RustFS claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive cloneable opaque activity proof and read-only qualified getter; fresh prepare_active API shares existing bounded current-context resume. Private eligibility distinguishes retained native channel from genuine fresh activity snapshot. Raw paths, proofs, context and source bodies have no Debug or encoding. Existing source catalog/native admission/cursor/pending/prepared request/actor/receipt/native/wire formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vlqssnyw
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One staged current-context eligibility surface accepts a current unforgeable genuinely new-start proof before first retained native admission. Preparation remains read-only and cursor-free; it does not allocate identity/alias/lease or copy a body. Complete current directory or separately authorized prior/header anchor passes current native directory/socket/catalog and Write before fresh enrollment and leases. A start observed in A can select current linked B after A revocation without authorizing historical A bodies through B: independently admitted ingress emits safe A omissions under authorized B and sanitized B body, then exact publication follows existing same-partition order. Fresh proof stays bound to exact physical/header/file snapshot throughout staged scanning and acquisition; stale proof is rejected before enrollment. The original activity probe may verify its prefix and renew the genuine start against current metadata; rearming over historical starts yields no proof. Stable large snapshots require multiple bounded steps, partial final context cannot rebind, absent Write/unbound directory/changed spool withhold without enrollment or ack. Retained admitted resume keeps its existing lookup semantics; shared eligibility acquisition and final channel checks preserve one-use preparation and recheck channel/probe around reader construction. Original absent API failure, private enum lint and unordered fixture publication mistake are retained and corrected; no ordering assertion or publisher guard weakened. Four new cases, all 34 direct native and 27 direct runtime cases, affected/direct-consumer check/lint/format and boundaries qualify the slice. Canonical patch was retried against formatter-wrapped context without implementation effect. Production modules remain below 300 lines. M17.2 and aggregate M17 stay In progress. Full fair bounded supervisor composition, resident raw-frame allocation independent of lease count, discovery/probe renewal/preparation blocking-I/O ownership and deadlines, pause/restart/backpressure, actual server startup and cold harness qualification, legacy absent-AgentStore initialization, Cursor/OpenCode readers, controls/UI and M17.3-M17.7 remain pending. Automatic startup stays guarded. No installed daemon, service process, native/WAL implementation, code heads, harness process, credentials, checkpoint, publication, deployment or release changed. CLI runtime publication behavior, unrelated reader/sanitizer/model/UI suites, real storage and root aggregates are out of scope under targeted policy; direct CLI consumers compile and lint.
<!-- /evidence-field -->

