---
schema: 1
id: E-M18-020
---

# E-M18-020

<!-- evidence-field:id markdown -->
## Id

E-M18-020
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ylnwrlks
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Verified Cursor and OpenCode integration and retired temporary workspace
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; existing Jujutsu workspace store; integrated configuration d269d6de, active capture b5942af7 and grouped settings 91f7be87.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product, API or repository-data change. Only cursor-opencode workspace registration removed; checkout and ignored local artifacts moved recoverably to Trash.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
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
jj st/workspace list/log in main and cursor-opencode; jj log -r "::cursor-opencode@- ~ ::default@"; jj diff --from cursor-opencode@ --to default@ --stat
```

**Notes**

Both working copies clean. No feature-only committed ancestors; settings and capture commits already in main. Main adds only the recorded review-startup evidence and subsequent grouped harness UI/spec change.
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
ps command-line inspection; lsof cwd inventory; exact workspace path/symlink checks; lsof -nP -iTCP:7460 -iTCP:7462 -sTCP:LISTEN
```

**Notes**

No persistent process using temporary checkout. Main UI PID99289 and backend PID56370 use main paths. Temporary node_modules is a symlink to main and was not traversed.
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
Recheck clean temporary checkout, unique committed ancestry and absent workspace processes; jj workspace forget cursor-opencode; trash -v /Users/stevejuma/code/lapismd/jj-store-cursor-opencode
```

**Notes**

Unregistered only cursor-opencode and moved its directory to /Users/stevejuma/.Trash/jj-store-cursor-opencode. Qualification artifacts and ignored files remain recoverable there.
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
jj workspace list; jj st; verify original directory absent and Trash copy present; verify main node_modules and fixture retained; lsof main listeners; curl -fsS -o /dev/null -w HTTP-status http://127.0.0.1:7460/
```

**Notes**

Seven other registrations retained with unchanged tips. Main clean before this evidence entry. UI returned200; UI/backend PIDs unchanged. No fixture reset or service restart.
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
Format only E-M18-020 JSON entry and compare previous evidence to jj file show -r @-; pnpm check:progress; pnpm spec:validate
```

**Notes**

Existing evidence unchanged; new entry uses Prettier JSON formatting. Progress bookkeeping passed for136 tasks; specification validation passed5 validators,22 chapters and167 requirements. Product lanes not run for workspace cleanup.
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

User-authorized verification and retirement of cursor-opencode only. Other registered workspaces, the parked OpenCode history draft and all running services retained. Product tests and builds out of scope for cleanup; prior qualification gaps unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue review in main at http://127.0.0.1:7460/; any future harness implementation starts from the current main revision.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


