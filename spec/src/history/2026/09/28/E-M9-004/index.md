---
schema: 1
id: E-M9-004
---

# E-M9-004

<!-- evidence-field:id markdown -->
## Id

E-M9-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- scripts/agent-capture-sanitizer.mjs
- scripts/agent-capture-sanitizer.test.mjs
- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/agent-capture-sanitizer.test.mjs#sanitizes before returning a
  capture-ready observation
- scripts/agent-capture-sanitizer.test.mjs#fails closed without touching hostile
  or unsupported capture input
- scripts/agent-capture-sanitizer.test.mjs#accepts only the initial qualified
  integration identifiers
- scripts/local-package-pnpmfile.test.mjs#local package hook uses available
  agent redaction source
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/agent-capture-sanitizer.test.mjs
```

**Notes**

The exact regression failed because jj-store had no packaged capture sanitizer boundary.
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
node --test scripts/agent-capture-sanitizer.test.mjs
```

**Notes**

Three focused tests prove whole-record sanitization before field access, fail-closed hostile inputs, input immutability, path and secret removal, allowlisted output and the three initial integration identifiers.
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
node --test scripts/local-package-pnpmfile.test.mjs
```

**Notes**

Three focused tests prove sibling resolution for Design Core and Agent Redaction while absent siblings preserve declared ranges.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm exec prettier --check <changed files>
```

**Notes**

The mapped gate required the new protected capture script in spec-governance.md, and formatting required one collaboration paragraph reflow.
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
pnpm test:governance; pnpm check:architecture; pnpm check:boundaries; pnpm spec:validate; pnpm check:progress
```

**Notes**

All 26 governance tests, 198 web architecture modules, 12 package boundaries, 84 canonical requirements and 60 task records pass; the sanitizer implementation is 80 lines against the 300-line production budget.
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
pnpm spec:first; pnpm exec prettier --check <changed files>
```

**Notes**

The mapped canonical chapters and all changed JavaScript, JSON, YAML and Markdown files pass their focused gates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Checkpoint and code-association schemas with native dependency retention
- AgentStore root-pinned read/staging facade and typed publication deltas
- Version-bound Codex, Cursor and OpenCode fixtures and normalizers
- Released registry acceptance for @lapismd/agent-redaction
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; @lapismd/agent-redaction 0.1.0 at sibling revision 9583b512; Codex CLI 0.156.1; Cursor 3.22.7; OpenCode 1.18.30.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The Node adapter returns only an allowlisted capture-ready DTO plus the agent-redaction policy identity and sanitization summary. It admits the three initial integration IDs and the source-kind vocabulary owned by SanitizedObservationV1.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


