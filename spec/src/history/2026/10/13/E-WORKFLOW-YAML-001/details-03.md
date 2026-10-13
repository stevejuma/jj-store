---
schema: 1
id: E-WORKFLOW-YAML-001
---

# E-WORKFLOW-YAML-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Final zero errors/warnings and 429 modules. /tmp/jj-store-workflow-yaml-catalog-final-2.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Final 1830 modules pass. /tmp/jj-store-workflow-yaml-architecture-final.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
failures: 1
result: fail
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Initial missing mapped API chapter for protected web changes. Added explicit unchanged JST-WF-006 confirmation to the workflow API topic; no contract or validator weakening. /tmp/jj-store-workflow-yaml-first.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Pass: 153 tasks, 783 evidence records. M28 remains In progress. /tmp/jj-store-workflow-yaml-progress.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Pass: 7 validators, 3741 chapters and 196 requirements before final record additions. Navigation regenerated after final evidence; rerun confirms closure. /tmp/jj-store-workflow-yaml-spec.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check [changed authored files]
```

**Notes**

Pass: all changed authored files formatted; evidence paths are formatter-ignored.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review contracts
```

**Notes**

Refreshed after the unchanged API confirmation required by the web mapping.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Final gate passes: 5 protected files, 3 canonical chapters. /tmp/jj-store-workflow-yaml-first-final.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Final pass: 153 tasks, 783 evidence records. Enclosing change zqulysou retains the M28 unchanged review. /tmp/jj-store-workflow-yaml-progress-final.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Final pass: 7 validators, 3742 chapters, 196 requirements. Complete evidence navigation regenerated and validated again before commit. /tmp/jj-store-workflow-yaml-spec-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check
```

**Notes**

Final pass after unchanged API confirmation: strict site build, local light/dark diagrams and 143-file closure. /tmp/jj-store-workflow-yaml-docs-final.log.
<!-- /run -->
<!-- /evidence-field -->

