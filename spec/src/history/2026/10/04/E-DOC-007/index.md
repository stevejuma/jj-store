---
schema: 1
id: E-DOC-007
---

# E-DOC-007

<!-- evidence-field:id markdown -->
## Id

E-DOC-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M19
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vrsptlyk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Validated documentation workspace retirement
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Jujutsu 0.45.1; macOS ARM64; merged application documentation 73f579fb.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Operational cleanup and evidence bookkeeping only; no product or state format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

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
jj --no-pager st in both workspaces; jj log -r "soumsxws & ::zpoqtoun"; jj workspace list; ps/lsof ownership and listener checks
```

**Notes**

Both workspaces clean after validated merge 73f579fb; complete documentation history is an ancestor of main. Verified documentation preview PID 43922 and Python PID 45991 by command and exact working directory before signaling only those processes. Server and its RustFS child exited; listeners 7480/7484 are closed.
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
Copy and SHA-256 compare documentation .local, .scratch, .pnpmfile.cjs, docs/build and web/test-results into main .local/documentation-integration/retired-workspace; write retirement-manifest.json
```

**Notes**

1705 evidence/fixture files totaling 29920405 bytes preserved with equal pre-copy, post-copy and destination inventories. Includes original screenshots, logs, generated manual and partial fixture; no fixture reset or live-state migration. Generated caches, dependencies and binaries remain rebuildable.
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
jj workspace forget documentation; remove exact verified /Users/stevejuma/code/lapismd/jj-store-documentation directory; jj workspace list; jj --no-pager st
```

**Notes**

Temporary workspace registration and directory removed after merged product validation and evidence preservation. Main clean before this ledger entry. Other workspace registrations and services left intact.
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
pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check .local/documentation-integration/retirement-entry.md; final parent-ledger and path checks
```

**Notes**

138 tasks, five validators, 23 chapters and 172 requirements; new evidence formatting passes. Both parent evidence sets remain JSON-identical, retired directory is absent and the copied Mermaid dependency resolves entirely within main.
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

<!-- evidence-field:scope markdown -->
## Scope

Documentation workspace retirement only. Product checks are recorded in E-DOC-006 and are not repeated for ledger-only cleanup. M19 remains Partial with foundational registry-only and packaged native-host release gates unchanged. Retained fixture files are archival evidence, not a running relocated installation.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue documentation and Help development in the main jj-store workspace; retired preview URLs are stopped.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


