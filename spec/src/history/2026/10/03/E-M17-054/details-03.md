---
schema: 1
id: E-M17-054
---

# E-M17-054 details 3

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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-cli-validate.log 2>&1
```

**Notes**

CLI-only canonical formatting, ownership boundaries, architecture/module budget and specification/progress checks completed; Rust formatting passes.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-cli-first.log 2>&1
```

**Notes**

CLI-only canonical formatting, ownership boundaries, architecture/module budget and specification/progress checks completed; Rust formatting passes.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-cli-progress.log 2>&1
```

**Notes**

CLI-only canonical formatting, ownership boundaries, architecture/module budget and specification/progress checks completed; Rust formatting passes.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-cli-rust-format.log 2>&1
```

**Notes**

CLI-only canonical formatting, ownership boundaries, architecture/module budget and specification/progress checks completed; Rust formatting passes.
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
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-cli-docs-check.log 2>&1
```

**Notes**

Final sealed canonical specification, protected-change/progress and changed-document formatting checks pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; existing locked reqwest 0.12.28 moves from CLI dev-only to production dependency. Real private device policy journal and admitted loopback HTTP; native MemoryStore host has no repository publication.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

All 88 HTTP routes, source/spool/journal/native/repository formats and shared DTOs unchanged. Explicit CLI status/pause/resume/rescan uses current version-1 local Admin HTTP contracts; literal loopback HTTP only, no proxy/redirect/credential lookup/application or transport retry; responses bounded to 64 KiB.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nstuqwky
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

One CLI behavior plus its command types, bounded HTTP adapter, direct cold process fixtures, owning specification and ledger. Additive command dispatch occurs before Backend/workspace/integration state; --state-dir and actor selection are rejected. Existing service/HTTP/runtime authority and schema are unchanged. Concrete verification scope: exact failing command and acknowledgement targets, complete direct capture_cli suite, owning CLI library and direct agent_setup/local_agent parser/backend consumers, CLI all-target compile/strict lint, formatting and boundary/architecture/canonical/progress checks. No shared runner or generated contract changed, so response-runner/web/server/supervisor/journal/RustFS/root aggregate/full workspace/UI lanes are out of scope. Source coverage/selectors/history import, implicit installation endpoint discovery, browser controls, other readers/faults and enabled startup/real JJ/storage/live harness/cold full-flow qualification remain pending; M17.3-M17.7 stay Planned. Full goal remains active, current AgentStore only. No installation, migration, publication, service restart or subagents. Cold binary links repeat the baseline macOS ld __eh_frame size warning; this is recorded and not suppressed. Tests and compiler checks do not qualify installed-service operation or a real product edit. Final production acceptance: original command and mismatched acknowledgement failures each one to zero; seven direct cold executable capture tests, ten owning library, six setup and two local backend entries pass serially with original assertions/timeouts; CLI all-target compilation/strict lint and metadata lanes pass. All 308 prior parsed entries remain unchanged; total 309. Implicit endpoint discovery and all broader M17 release gates remain pending. One sealing document-text assertion missed formatter-wrapped prose after successfully updating the ledger; the verification paragraph remained unchanged until read-based repair. This was a documentation edit failure, not a product gate failure.
<!-- /evidence-field -->

