---
schema: 1
id: E-DEPENDENCY-AUTOMATION-005
---

# E-DEPENDENCY-AUTOMATION-005

<!-- evidence-field:id markdown -->
## Id

E-DEPENDENCY-AUTOMATION-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Documentation-review correction after E-DEPENDENCY-AUTOMATION-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- spec/src/history/2026/11/04/E-DEPENDENCY-AUTOMATION-005/index.md
- spec/src/history/2026/11/04/index.md
- spec/src/history/2026/11/04/SUMMARY.nav.md
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
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
GitHub Actions run 41929542660: UI acceptance / UI acceptance inputs
```

**Notes**

The production application bundle completed, then documentation staging failed
closed with `Documentation review required: application`. The review fingerprint
predated the branch-command active-item accessibility repair; no compile,
manifest or product-test assertion failed.
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
npx --yes pnpm@10.32.1 docs:review application "Reviewed Design Core 0.10.1 branch-command active-item cleanup: keyboard and pointer selection keep valid combobox focus while preserving documented browsing, editing, settings and conversion behavior."
npx --yes pnpm@10.32.1 docs:review
PATH=<workspace Rust 1.99 and pinned uv> node scripts/docs.mjs stage
```

**Notes**

The application review now covers the exact final `web/src` fingerprint after
reading its four mapped guides. The production bundle passes; the workspace
Rust stage renders nine Mermaid diagrams offline, builds 144 documentation
files strictly, and validates the 26,736,832-byte closure, links and anchors.
The first direct local rerun reached the machine's unconfigured `mise` Cargo
shim after bundling; explicit workspace-owned Rust and pinned `uv` paths prove
the remaining stage without changing repository behavior.
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
npx --yes pnpm@10.32.1 exec prettier --check docs/maintenance/coverage.json spec/src/history/2026/11/04/E-DEPENDENCY-AUTOMATION-005/index.md
npx --yes pnpm@10.32.1 exec spec-validator validate --only governance,summary,fileLimits,verification --json
npx --yes pnpm@10.32.1 exec spec-validator first --base @- --head @ --json
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

Formatting, bounded authored-file limits, evidence shape, exact changed-file
coverage and milestone bookkeeping pass after the correction record is added.
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

Design Core 0.10.1; pnpm 10.32.1; Rust 1.99.0; uv 0.9.28.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Only the application documentation-review fingerprint and reason change. No
storage, API, route, publication or persisted browser format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Application documentation-review closure for the branch-command accessibility
repair already recorded by E-DEPENDENCY-AUTOMATION-004.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require fresh hosted CI for the rewritten pull-request head.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The four mapped application guides already describe the preserved
  browsing, editing, settings and conversion behavior. The refreshed review
  acknowledges the exact source fingerprint without changing documentation.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This correction closes one application review
  gate and does not qualify publication, provider, platform or visual gates.
```
<!-- /evidence-field -->
