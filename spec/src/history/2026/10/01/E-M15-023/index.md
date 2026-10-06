---
schema: 1
id: E-M15-023
---

# E-M15-023

<!-- evidence-field:id markdown -->
## Id

E-M15-023
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
- M15.6
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

enclosing Jujutsu change yqsrnpmp
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main default workspace is clean at ozomqkpp / 2a6ce3ef with parent vkutkopr / c347df2d. Rebased Threads msszrzpn / 02f94c13 has completed integration qualification in E-M15-022 and includes that main parent. Main development service PID 23243 remains running independently.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; same owned UI provenance, local development foundation snapshot, generated contracts and binaries qualified in E-M15-022. No package install, publication or installed binary replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Fast-forward main checkout to the verified descendant msszrzpn / 02f94c13; no redundant merge parent. Production files and native/local contracts are identical to the qualified isolated tree. Only this landing evidence is new.
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
jj new msszrzpn -m "Record Threads alignment landing in main workspace"
```

**Notes**

Preflight confirms clean main, exact c347df2d parent, main ancestor in msszrzpn and no conflicts; switched default checkout to verified tree. Preserved original threads-reactions workspace.
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
jj --no-pager diff --from msszrzpn --to @ --summary
```

**Notes**

Immediately after checkout landing, trees match exactly.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm spec:first; pnpm check:ui-ownership
```

**Notes**

Documentation-only landing formatting, progress/specification/ownership checks. Product verification is the unchanged tree qualified in E-M15-022; no redundant product suites.
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
python3 compare ledger and production tree using jj file show -r msszrzpn and jj diff --from msszrzpn --to @ --name-only
```

**Notes**

All 244 preceding evidence objects remain identical, with this entry appended; only milestones.md differs from the qualified Threads tree.
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

User-authorized local rebase, conflict resolution and main landing only. Main development service remains untouched. M15/M15.5 remain In progress; visual baselines, M15.7 real RustFS/runner gates and registry-only Design Core foundations remain separately unqualified. Root aggregates and full Rust workspace are out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue from the landed main source tree. Rebuild/restart the existing main development server only on explicit user request; preserve the isolated Threads workspace and independent qualification gates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


