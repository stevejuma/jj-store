---
schema: 1
id: E-M17-058
---

# E-M17-058 details 3

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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-probe-check-progress.log 2>&1
```

**Notes**

Repeated after final metadata sealing.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-import-probe-format-rust-check.log 2>&1
```

**Notes**

Repeated after final metadata sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI JSONL 0.156.1 and desktop JSONL 0.159.2; private filesystem only, no live harness/storage qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Read-only Codex import binding v1 preparation: SHA-256 physical/header/session and complete-prefix digests, exact profile/complete byte offset/count. Prefix scanning at most 4 MiB per step plus fixed bounded headers; existing 1-MiB record/context buffer. Stable accepted prefix with current complete-record context, partial EOF allocation released. No native/source/spool/journal/control or public HTTP format change.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change sxnwmnns
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

Current AgentStore only. New read-only leaf preparation in capture crate; no existing reader, registry, resume or context algorithm is changed. No consumer calls the new primitive yet. Exact complete new integration target plus capture-package all-target check/strict lint is the complete direct Rust impact surface; existing server/CLI/Rust workspace, root aggregates, generated HTTP, UI and real services are out of scope under targeted policy. Canonical requirements specify independent header checks, domain-separated digests, stable complete-record boundary and restart reconciliation without retargeting. Preparation allocates no source/session/alias/lease/progress/intent/native state and grants no Admin/Write authority; source enrollment and scheduling remain the next separate slice. All 312 prior parsed entries must remain unchanged. M17.2 In progress; API/CLI/browser import, other readers/startup and real harness/storage/cold gates remain pending; M17.3-M17.7 Planned, full goal active. No install, migration, publication, restart or subagents. One documentation patch missed formatter-wrapped prose and applied no document mutation; repaired against current lines. Verification prose now clearly separates historical specification-only E-M17-056 from executable journal E-M17-057 and this preparation evidence. Final acceptance: eight focused import integrations, exact repaired desktop test, all-target capture package check and strict lint pass. Six canonical/boundary/architecture/progress/Rust-format consumers plus changed-document formatting pass. Source modules are below 300 lines. All 312 prior parsed entries unchanged, total 313. Read-only preparation slice complete; current Admin/Write admission, durable intent before enrollment and supervised scheduling remain next.
<!-- /evidence-field -->

