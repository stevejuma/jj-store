---
schema: 1
id: E-M15-017
---

# E-M15-017

<!-- evidence-field:id markdown -->
## Id

E-M15-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

main-workspace-landing
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vuovmsop
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1 domain baseline; existing local file package snapshots; Svelte 5.57.1 and Vitest 4.1.11. E-M15-016 retains exact dependency provenance and qualification gaps.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, generated-contract, manifest or companion format change. Main now contains the exact committed Threads/replies/reactions tree from xrkpynxm / cea45f3a.
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
jj --no-pager st (main and threads-reactions); jj --no-pager log; jj workspace list
```

**Notes**

Before landing, both workspaces are clean. Main vuovmsop is an empty child of 590462ef; isolated workspace has committed cea45f3a on the same parent.
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
jj rebase -r @ -o xrkpynxm
```

**Notes**

Rebased the empty main working change onto the committed UI slice; added 22 and modified 45 files without conflicts. Isolated workspace is preserved.
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
jj --no-pager diff --from xrkpynxm --to @ --summary; jj --no-pager st
```

**Notes**

Before this ledger entry, main has no changes and matches the source commit exactly.
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
pnpm web:check
```

**Notes**

Main workspace: 0 errors and 0 warnings. /tmp/jj-thread-main-web-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts
```

**Notes**

Main workspace: 14 tests in 1 file pass. /tmp/jj-thread-main-unit.log.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm spec:first; pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Ledger formatting and 119-task bookkeeping pass. Specification validates 4 validators, 21 chapters and 141 requirements; spec-first confirms no protected files changed.
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
python3 (compare parent and current JSON evidence entries)
```

**Notes**

All 228 prior evidence entries remain identical; the landing entry is the only addition.
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

Local workspace landing only. Existing isolated Rust, generated-contract, unit and cold Storybook evidence in E-M15-016 remains applicable because the landed production tree is identical. No conflict resolution or production edits occurred. No root aggregates, Rust rebuild, package publication, service restart, visual approval or repository migration. Main dev server PID 66628 remains running its prior build; this entry does not claim live browser qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue released Community dependency, cold backend/browser full-flow and remaining M15 qualification. M15 and M15.5 remain In progress.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


