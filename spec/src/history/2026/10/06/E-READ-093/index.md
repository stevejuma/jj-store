---
schema: 1
id: E-READ-093
---

# E-READ-093

<!-- evidence-field:id markdown -->
## Id

E-READ-093
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Blame UI integrated in main with active Stacks work retained and live preview verified
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tmotwyknlwly
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Same UI dependencies and server processes as E-READ-092; macOS ARM64; pnpm 10.32.1; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, generated contract, persisted repository or dependency changes. Existing file_view presentation contract retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

User explicitly authorized main integration after the overlap warning. Main held active Stacks change lmpkknlw on empty change 5103af31. Blame UI was committed as pvlzsxrt/a5d00c50. The open port7470 Vite process served the clean codex-preview workspace at e955df8a, not main; its backend was PID80426.
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
jj --no-pager st; jj --no-pager log; record immutable main commit and changed paths under .local/blame-integration
```

**Notes**

Inspected both workspaces and recorded main-before.txt/main-before-files.txt before rebase. Main contained active Stacks changes; explicit user integration instruction governs this overlap.
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
jj bookmark create codex/main-before-blame-ui-20261006 -r @; jj rebase -s @ -d pvlzsxrt; jj bookmark delete codex/main-before-blame-ui-20261006
```

**Notes**

Blame becomes main parent; original uncommitted Stacks change ID retained. Temporary bookmark followed the rewrite and was deleted; original immutable commit ID and JJ operation history remain the recovery record. One append conflict in store-route-history.test.ts was resolved by keeping both the Blame display test and the Stacks live/pinned test.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj resolve --list
```

**Notes**

After explicit conflict resolution, installed JJ returned No conflicts found at this revision with nonzero exit; this confirms no unresolved conflict rather than a failed product check.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/blame-query.test.ts
```

**Notes**

Main .local/blame-integration/routing-tests.log:3 files,36 tests pass on integrated source, including both independently added history tests.
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
pnpm web:check
```

**Notes**

Main .local/blame-integration/web-check.log:zero errors and warnings for integrated UI.
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
Compare main-before immutable file bytes against rebased files outside the Blame diff; jj log -r "pvlzsxrt & ancestors(@)"
```

**Notes**

preservation.json:36 original nonoverlapping paths identical; five were concurrently updated by continuing Stacks work and were not overwritten. a5d00c50 verified in main ancestry. Both fileDisplay and stackHistory fields remain.
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
jj new pvlzsxrt -m "Preview committed Blame UI" (codex-preview workspace); jj --no-pager st
```

**Notes**

Clean preview workspace advanced to the committed UI slice; it remains empty at1228e447. Existing Vite picked up source changes. Development fixture and backend were not restarted or reset.
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
CUA reload existing port7470 Linux file tab, inspect DOM and capture visible file controls
```

**Notes**

The pinned include/crypto/acompress.h page showed Source, disabled Blame and Working copy, with the unavailable explanation. Subsequent user navigation to CREDITS also shows those controls. Screenshot:main .local/blame-integration/blame-ui-live.jpg. The original operation pin remains. No production annotation provider is enabled.
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

<!-- evidence-field:out_of_scope list -->
## Out of scope

- E-READ-091 live annotation comparison/cancellation gate and public-provider
  qualification remain blocked; C syntax highlighting remains as recorded in
  E-READ-092.
- Rust, Stacks feature qualification, new browser suites, root aggregates,
  dependency updates, server restarts and release/visual-baseline approval were
  not part of this integration.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain bounded-reads for follow-up. Production Blame remains disabled until the separately scoped backend feasibility gate is resolved.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


