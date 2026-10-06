---
schema: 1
id: E-GOV-001
---

# E-GOV-001

<!-- evidence-field:id markdown -->
## Id

E-GOV-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

targeted-verification-scope-guidance
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The tracked guide already preferred targeted verification, but canonical governance deferred to the parent ladder and its escalation wording could still treat one shared UI component with several stories as full-suite impact. That ambiguity contributed to root aggregate runs for a small Settings presentation fix.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/verification.md
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
pnpm exec prettier --write AGENTS.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md; pnpm spec:validate; pnpm spec:first; pnpm spec:build
```

**Notes**

The four guidance files are formatted, all 105 requirements validate, the protected guide maps to the two changed canonical chapters, and mdBook builds.
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
pnpm exec prettier --check AGENTS.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build
```

**Notes**

Final targeted documentation verification passes without running root product aggregates: formatting, 105 requirements, same-change mapping, all 71 task rows and mdBook are valid.
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

jj 0.45.1; pnpm 10.32.1; spec-validator current workspace dependency.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Documentation and agent-guidance change only. No product code, API, schema, generated contract, persistence, wire format, test implementation or runtime behavior changes. Root aggregate product gates are out of scope under JST-GOV-007.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


