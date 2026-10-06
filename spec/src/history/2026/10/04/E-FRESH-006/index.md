---
schema: 1
id: E-FRESH-006
---

# E-FRESH-006

<!-- evidence-field:id markdown -->
## Id

E-FRESH-006
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Qualified event stack integration into main and temporary workspace retirement
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; macOS ARM64; verified event tip f86c7f923f243413af5bf4b74940d0f200899968.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, API, event cursor, repository or companion format change during integration.
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
jj --no-pager st (main and event-freshness); jj --no-pager workspace list; inspect ancestry, ignored artifacts and active processes
```

**Notes**

Both working copies clean. Original main f5cd62d4 is already an ancestor of event tip f86c7f92; no main-only changes or merge conflicts. Isolated test services stopped; main development service PIDs76009/76010/76011 retained.
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
Copy .local/event-freshness and web/test-results-real into main; SHA-256 compare all source evidence files
```

**Notes**

176 verification files preserved byte-for-byte, plus browser results. source-evidence-sha256.json retained under main .local/event-freshness/integration.
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
jj rebase -r qkqknzqv -o zrmqwtuq; jj --no-pager st; jj --no-pager diff --from zrmqwtuq --to @ --summary
```

**Notes**

Advanced only the empty main working-copy change onto the qualified stack. Original main history retained; 52 added and146 modified paths, no removals. Production tree identical to event tip, confirmed before integration documentation.
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
python3 .local/event-freshness/integration/audit.py (initial comparison)
```

**Notes**

Markdown task rows differed only in formatter padding after adding E-FRESH-006. The audit was corrected to compare every trimmed table cell; no task, dependency, gate or prior evidence changed. preservation-before-normalization.log.
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
python3 .local/event-freshness/integration/audit.py (normalized table cells)
```

**Notes**

Qualified production tree and original main ancestry preserved. All470 prior evidence objects and139 task states unchanged;176 saved verification files match their original SHA-256 values. preservation.log.
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
pnpm spec:validate; pnpm check:progress; pnpm spec:first; pnpm exec prettier --check docs/implementation-plan.md spec/src/architecture.md spec/src/milestones.md
```

**Notes**

5validators/23chapters/172requirements;139task ledger;1protected file/2canonical chapters; all changed documents formatted. specification.log, progress.log, spec-first.log, format.log.
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
pnpm spec:first --base f5cd62d433aa5d525b41bb5066ef733b52a83f9d --head e985b6f07233317e31a3af82d0e04ffc6b5bf1d6
```

**Notes**

Complete landed stack passes183protected files/15canonical chapters. Explicit original-main/current-head commit range; spec-first-landed-stack.log.
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
jj workspace forget event-freshness; trash -v /Users/stevejuma/code/lapismd/jj-store-event-freshness; verify workspace list, directory absence and main process PIDs
```

**Notes**

Only event-freshness retired. Directory moved recoverably to macOS Trash; other workspaces preserved and main PIDs76009/76010/76011 unchanged. retirement.json.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check (main workspace)
```

**Notes**

Built current source-derived references and manual:9Mermaid diagrams in light/dark;139files/13,696,662bytes; links and anchors valid. documents.log. This rebuilt the documentation helper only; no development services restarted.
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

<!-- evidence-field:next markdown -->
## Next

Main contains the qualified event stack and bounded-read prerequisites; temporary event workspace retired and its evidence retained under main .local/event-freshness. M20 remains In progress while existing M15.5 full-flow qualification is open. E-FRESH-005 focused/cold evidence applies to the unchanged production tree. Development services remain running on their prior backend until an explicit rebuild/restart. Remote publication, root aggregate suites, visual baselines and wider capacity/release qualification are outside this documentation-only integration slice.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qkqknzqv
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


