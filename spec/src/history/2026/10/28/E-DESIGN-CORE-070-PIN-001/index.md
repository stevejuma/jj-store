---
schema: 1
id: E-DESIGN-CORE-070-PIN-001
---

# E-DESIGN-CORE-070-PIN-001

<!-- evidence-field:id markdown -->
## Id

E-DESIGN-CORE-070-PIN-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-005
- JST-GOV-002
- JST-ARCH-005
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- spec/src/current/contracts/architecture/06-browser-presentation-ownership.md
- spec/src/history/2026/10/28/E-DESIGN-CORE-070-PIN-001/index.md
- spec/src/history/2026/10/28/E-DESIGN-CORE-070-PIN-001/SUMMARY.nav.md
- spec/src/history/2026/10/28/index.md
- spec/src/history/2026/10/28/SUMMARY.nav.md
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
pnpm install
pnpm web:check
```

**Notes**

Pinned published Design Core 0.7.0 and Mira 0.1.1 in package.json and the
registry lockfile. Updated the browser presentation ownership chapter to the
same ranges. After removing a stale validation lease container, web:check
reported svelte-check found 0 errors and 0 warnings. Open Dependabot PRs 19,
18, 17, 16, and 4 remain separate.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 40790866843 --repo stevejuma/jj-store --log-failed
```

**Notes**

Hosted tip ffd024ba Progress evidence gate failed while parsing this leaf:
commands list entries starting with backticks are invalid YAML plain values.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

After rewriting to runs/result shape, the gate still required formats,
justified contract review, and explicit milestone review fields.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Added formats, contract_review, and milestone_review fields matching the
2026-10-28 sibling evidence shape.
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

pnpm 10.32.1; Design Core pin moved from ^0.5.1 to ^0.7.0 (lockfile 0.7.0);
Mira pin moved from ~0.0.3 to ~0.1.1 (lockfile 0.1.1).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Registry lockfile and
manifest dependency ranges for Design Core and Mira changed to the published
0.7.0 / 0.1.1 contracts.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Unblock jj-store PR 20 web type-checking against published Design Core AppShell
attached APIs and Mira composer lineNumbers by consuming registry 0.7.0 / 0.1.1
instead of the older published ranges.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land green required checks on PR 20, then merge.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-API-005 browser UI and JST-ARCH-005 packaging consume published
  Design Core ^0.7.0 and Mira ~0.1.1. The browser presentation ownership chapter
  and lockfile now require those ranges for TagList and AppShell-attached
  surfaces; host domain mapping ownership is unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 remains at its existing browser presentation gates. Consuming
  published Design Core 0.7.0 promotes no new provider, publication, or visual
  gate beyond the already-open PR 20 required lanes.

```
<!-- /evidence-field -->
