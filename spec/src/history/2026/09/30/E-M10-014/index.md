---
schema: 1
id: E-M10-014
---

# E-M10-014

<!-- evidence-field:id markdown -->
## Id

E-M10-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

minimal-replication-activation-scope-alignment
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- docs/cli/sync.md
- spec/src/cli.md
- spec/src/milestones.md
- spec/src/spec-governance.md
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
pnpm check:progress
```

**Notes**

Before this evidence entry, the checker correctly rejected the protected guide and CLI documentation changes because the enclosing change had no ledger update.
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
pnpm spec:validate
```

**Notes**

All 103 canonical requirements validate with the active single-upstream profile and explicit deferral boundary.
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
pnpm spec:first
```

**Notes**

The protected agent guide and CLI documentation changes have their mapped spec-governance and CLI chapter updates.
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
pnpm exec prettier --check AGENTS.md docs/cli/sync.md spec/src/cli.md spec/src/spec-governance.md
```

**Notes**

All alignment documents are formatted before the ledger update.
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
pnpm exec prettier --write AGENTS.md docs/cli/sync.md spec/src/cli.md spec/src/milestones.md spec/src/spec-governance.md
```

**Notes**

The final guide, command help, canonical chapters and evidence ledger are formatted together.
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
pnpm check:progress
```

**Notes**

All 70 task rows and the new scope-alignment evidence entry pass ledger validation.
<!-- /run -->

<!-- run -->
### Run 7

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

The final protected change maps the guide and CLI documentation to three updated canonical chapters.
<!-- /run -->

<!-- run -->
### Run 8

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

All 103 canonical requirements validate after the final milestone alignment.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Rust and tracked-text formatting pass. Production tests and aggregate gates were not repeated because this slice changes only guidance and status; the merged implementation passed them under E-M10-013.
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

jj 0.45.1; pnpm 10.32.1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product API, persistence or wire format change. The guide and CLI help now identify the existing format-2 single-upstream profile as active and keep M10.8 plus stock-JJ filesystem mirroring deferred.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


