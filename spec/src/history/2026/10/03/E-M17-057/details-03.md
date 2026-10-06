---
schema: 1
id: E-M17-057
---

# E-M17-057 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-spec-validate.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-spec-first.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-check-progress.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-import-format-rust-check.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-doc-check.log 2>&1
```
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-seal-format.log 2>&1
```

**Notes**

Final metadata sealing; repeated once after recording this command.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI 0.156.1 metadata and real private filesystem; no real-model/storage qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New installation-private import catalog version 1, max 4096 receipts within 2 MiB, state.json/import.lock/.capture-import- layout. Exact installation and existing control generation, caller/principal/request/policy/selector plus four opaque source/boundary digests and complete offset/count. Lowercase fixed hex digests. Independent journal ordinals; control state layout/version remains unchanged at 1 MiB.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zmssxsys
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

Current AgentStore only. Implements durable intent/replay primitive, not enrollment, authorization, source progress, HTTP/CLI import, publication or a worker. Complete byte/receipt bounds refuse new selection without eviction; replay ignores offered replacement metadata. Private synchronized state checks reject missing/corrupt/foreign/rebound state without repair. Fault after replacement is unknown, exact retry re-synchronizes receipt. Callback retains journal gate and detects byte-identical selected-file replacement. Shared filesystem extraction affects both import and pause policy plus manager/foreground handoff: concrete escalation to complete server library (including cold policy factory), private control/import integration, management, runtime/supervisor and direct cold capture CLI; server/CLI all-target check/strict lint. Rust suites run serially with original deadlines and within-scenario concurrency. Initial full-library invocation queued behind the exact unit build; no concurrent scenario deadline changed. No public HTTP DTO/route or response runner changed: generated contracts, UI/root aggregates, unrelated suites and real services are out of scope. Preserve all 311 prior parsed evidence entries. Inactive history admission/scheduling, API/CLI/browser controls, remaining readers/startup and real harness/storage/cold gates remain pending; M17.3-M17.7 Planned, full goal active. No install/publication/migration/restart or subagents. Removed helper paths are registered for historical evidence continuity. Final direct acceptance passes: 49 server library, nine control integration, eight import integration, ten management, ten runtime, 27 supervisor and ten cold capture CLI tests (123 total), plus server/CLI all-target check and strict lint. Canonical, spec-first, boundary, architecture, progress, Rust format and changed-doc checks all pass. All 311 prior parsed entries remain unchanged, total 312. Exact import suite rerun after unused fixture cleanup passes; no warning suppressed. Current private journal slice complete; executable import and remaining roadmap stay pending. Final sealed metadata repeats the existing document-consumer commands without changing production or widening test scope.
<!-- /evidence-field -->

