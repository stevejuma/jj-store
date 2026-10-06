---
schema: 1
id: E-M17-022
---

# E-M17-022 details 3

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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-resume-first.log 2>&1
```

**Notes**

All final protected implementation and owning chapters pass spec-first.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-resume-doc-format.log 2>&1
```

**Notes**

Changed canonical document formatting passes.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1 and independently qualified Desktop 0.159.2 JSONL. Actual native service/WAL/AgentStore over MemoryStore with the existing pinned Kingfisher cache. Fresh OS process qualifies read-only retained channel lookup and completed offline context reconstruction; no fresh whole-server startup, live model edit or real-RustFS claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive SourceRegistry read-only admitted-channel lookup and source-ID reexport, memory-only current-complete-context probe and one-use staged resume. Capture privacy roots may clone in memory, without Debug or serialization. Existing source catalog/marker, cursor, pending/prepared request, native alias, sanitizer, receipt and model/wire formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vkosuuuw
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One supervisor preparation surface reconstructs the current context from an exact qualified stable file before leases or rotated-alias enrollment, at most 4 MiB per step and 1 MiB partial bytes. Metadata changes restart bounded reconstruction; invalid/repeated headers, source/root replacement and partial EOF have explicit context semantics. Only read-only unambiguous retained native-session/channel lookup supplies eligibility; generic registration, a known location or arbitrary growth cannot import history. Native directory/socket/catalog and Write admission precede acquisition, including known same-UUID rotation; current file, eligible channel and private spool bindings are checked around acquisition and after reader construction. Returning a source consumes this one-use preparation. A revoked original A header no longer blocks an admitted current B tail, whose exact queued B batch is independently recovered and published. This preparation neither acknowledges cursors nor grants body or queued publication permission through its prior/header anchor; every driver batch and durable entry retains independent proof and sanitation requirements. Header-only convenience acquisition remains explicit; the eventual supervisor uses staged current-context preparation. Fresh-process context evidence plus all 76 runtime and 19 direct native consumer cases, affected/direct-consumer compile/lint/format and dependency boundaries qualify this surface. Initial absent API and incorrect import failures are retained with fixes. M17.2 and aggregate M17 remain In progress; fair bounded blocking-I/O supervision, deadlines/backpressure/pause/restart and controls, legacy absent-domain initialization, Cursor/OpenCode readers, real harness edit and cold full-flow, UI and full M17.3-M17.7 roadmap remain pending. The enabled-startup guard stays in place. No browser lifecycle, native/WAL implementation, installed service, credentials, release, publication or deployment changed; root aggregate and unrelated wider lanes are out of scope.
<!-- /evidence-field -->

