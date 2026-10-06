---
schema: 1
id: E-GOV-003
---

# E-GOV-003 details 7

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
pnpm exec spec-validator first --base aae49090 --head potwxpyw
```

**Notes**

Actual explicit JJ revision range passes through the registry CLI.
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
pnpm spec:first
```

**Notes**

All protected paths have same-change owner and evidence coverage.
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

175 requirements validate globally across bounded current contracts and split verification; original 173 statements remain preserved except the two approved governance changes.
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
pnpm spec:build
```

**Notes**

The actual complete mdBook builds with bounded authored sources, generated navigation and tracked repository-reference closure. The large generated search index remains ignored and non-authoritative.
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
pnpm spec:links
```

**Notes**

Generated HTML links, local assets and anchors close within the book, including compatibility anchors and transitive repository references.
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
pnpm spec:search --semantic "confirmation that protected refactor leaves contract unchanged" --limit 5 --json
```

**Notes**

Registry-based semantic current retrieval returns five existing absolute canonical source files with positive source line numbers.
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

Registry @lapismd/spec-validator 0.1.4 (^0.1.4); Node 24.10.0 consumer; pnpm 10.32.1; Deno 2.9.5 source gates; mdBook 0.5.4 local / 0.5.2 CI; QMD 2.5.3; YAML 2.9.1. Python 3.14.7 book-link audit.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version 1 Markdown evidence field markers, YAML metadata, bounded topic/index Markdown. Native object/WAL/manifest and application transport formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Split canonical reading units, globally validate verification, archive original bytes, derive task evidence histories, retain historical mappings/resolutions, validate fresh same-change confirmations and revision-pinned merge evidence.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain bounded current topics, append fresh evidence with explicit contract/milestone reviews, and regenerate histories/navigation for each qualified slice.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- "Rust builds/tests, application UI tests, service restarts and visual
  baselines: product behavior is unchanged."
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Governance adds authored-file budgets, split coverage and justified
  unchanged-contract confirmations; product contracts retain their existing
  identities and semantics.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M22 is complete after migration, registry, book, search, native-JJ
  range/merge and consumer governance qualification; all 142 original statuses,
  dependencies, completion gates and selected evidence are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:diagnostic_corrections list -->
## Diagnostic corrections

- The initial new evidence ID reused E-GOV-001; the audit caught it and the new
  record alone became E-GOV-003. Original E-GOV-001/E-GOV-002 remain unchanged.
- Formatting and direct evidence links exposed three oversized verification
  tables; rows were split between bounded files without changing their coverage
  or status.
- Source path/anchor repairs retain old chapter links and literal regex prose;
  archive hashes and original statements verify preservation.
- Initial shell wrappers used a read-only zsh variable and an unsupported
  --version flag. Final gates use direct scripts and registry package metadata.
<!-- /evidence-field -->

