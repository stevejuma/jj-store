---
schema: 1
id: E-M17-001
---

# E-M17-001

<!-- evidence-field:id markdown -->
## Id

E-M17-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.0
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

agent-capture-specification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/collaboration.md
- spec/src/cli.md
- spec/src/security.md
- spec/src/publication.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/ui.md
- spec/src/adapters.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
jj workspace add /Users/stevejuma/code/lapismd/jj-store-agent-capture --name agent-capture -r 2852a3e3 -m "Specify and implement reliable agent capture"
```

**Notes**

New clean workspace; current parent includes later local serving and UI ownership changes. M16 is already Appearance, so capture uses M17.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/security.md spec/src/publication.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/ui.md spec/src/adapters.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Changed canonical files formatted.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements validated.
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
pnpm spec:first
```

**Notes**

Documentation-only scope; no protected implementation files changed.
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
pnpm check:progress
```

**Notes**

129 tasks; bookkeeping only.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/security.md spec/src/publication.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/ui.md spec/src/adapters.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All changed document formatting passes.
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

JJ 0.45.1; Node.js 26.9.0; pnpm 10.32.1; existing Codex 0.156.1, Cursor 3.22.7 and OpenCode 1.18.30 hook contracts are not new reader or real-edit qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Specification only. Fourteen new JST-COLLAB requirements; M17.0-M17.7 staged gates; no runtime, repository or public wire format change.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Canonical specification and owning chapter references only. Source reader qualification remains a runtime M17.2/M17.5 gate. Root aggregates and product suites out of scope.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


