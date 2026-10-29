---
schema: 1
id: E-JJ46-DOCS-001
---

# E-JJ46-DOCS-001

<!-- evidence-field:id markdown -->
## Id

E-JJ46-DOCS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/releases.md
- docs/site/technical/local.md
- spec/src/current/progress/M30/index.md
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
pnpm docs:review
```

**Notes**

Refreshed the capture, local, search, and CLI source fingerprints. The Stacks page names jj-lib 0.46.0. Pending-hook filenames stay lowercase SHA-256 hex. Tantivy still copies existing segment files. The recorded JJ 0.45.1 qualification is unchanged.
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

jj-lib 0.46.0. Documentation review fingerprints only; no package version change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Acknowledge the jj-lib 0.46 compile fix in the capture, local, search, and CLI manuals. The Stacks page names the pinned library. Qualification gates stay at JJ 0.45.1.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Required CI must pass on this head. M30 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-DOC-001 and JST-DOC-002 keep the same manuals. The review records
  the renamed jj-lib 0.46 interfaces and the unchanged JJ 0.45.1 qualification.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. Documentation review does not complete release
  qualification.

```
<!-- /evidence-field -->

