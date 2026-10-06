---
schema: 1
id: E-UI-079
---

# E-UI-079 details 3

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
pnpm test:storybook catalog/components/store-work (per-instance storage)
```

**Notes**

55/55 pass with original assertions and isolated fixture storage. /tmp/jj-search-controls-consumers-isolated-storage.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate; pnpm check:progress (initial final)
```

**Notes**

0typeerrors/0warnings;140module boundaries. All focused governance lanes passed. Final fixture/evidence checks follow.
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
Live browser: Activity filter and search controls
```

**Notes**

One Filters for activity button; one dialog combines scope and native categories. Search control x251/y59/32x32 before/after, Clear24x24, original themed outline; Clear retains focused open input. Inspected /tmp/jj-search-controls-fixed.png and /tmp/jj-search-controls-activity-final.png. No data mutations.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Design Core wider checks and compare (independent source)
```

**Notes**

InputGroup12/12;715unit;643Storybook;14workspace;35Shadcn pass with1configured skip. Initial checks shell10pass/1readinessfailure; fresh dev/static exact retries also failed. AI1pass/2readiness failures plus overlapping output artifact loss. Subsequent shell11/11 and AI3/3 pass against owned9259preview; complete checks retry passed cold shell11/11 but failed cold AI1pass/2readinesstimeouts. Visual compare of5InputGroupstories failed canonical cacheEACCES before screenshots. No baseline changed. Source verification retains exact logs and later outcomes.
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
Temporary source preview cleanup
```

**Notes**

Stopped only owned Python9259server and later owned Storybook9259preview; closed temporary source browser tab. Vite7460/backend7462 retained.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Read-only discovery and unsupported guessed paths
```

**Notes**

Some rg/cat discovery used absent paths or a zsh glob with no matches. Retried using rg --files. No source state changed by failed reads.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (initial final evidence)
```

**Notes**

Rejected three descriptive tests references and pass with remaining qualification gaps. Removed descriptive references from tests and retained them in run notes; entry remains pending for separate visual/source qualification. Checker and assertions unchanged.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate; pnpm check:progress; targeted Prettier (fixture and final evidence)
```

**Notes**

Final web/catalog0errors0warnings,140catalog boundaries; ownership/spec-first/spec-validate/progress120tasks pass. All nine changed files formatted and format-checked. Logs /tmp/jj-search-controls-web-final.log; /tmp/jj-search-controls-catalog-final.log; /tmp/jj-search-controls-governance-final.log; /tmp/jj-search-controls-progress-final.log; /tmp/jj-search-controls-format-final.log.
<!-- /run -->
<!-- /evidence-field -->

