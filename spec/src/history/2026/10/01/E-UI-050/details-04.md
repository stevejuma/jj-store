---
schema: 1
id: E-UI-050
---

# E-UI-050 details 4

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
pnpm spec:validate
```

**Notes**

Final 4 validators, 21 chapters, 132 requirements pass.
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
pnpm spec:first
```

**Notes**

Final spec-first gate: 23 protected files and 6 canonical chapters pass.
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
pnpm check:progress
```

**Notes**

Final evidence bookkeeping passes for 111 tasks; milestone statuses unchanged.
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
JJ_STORE_DEV_ROOT=/tmp/jj-store-three-profiles-cold JJ_STORE_DEV_PORT=7461 pnpm dev:stop; node --input-type=module
```

**Notes**

Owned isolated service stopped; verified test fixture and temporary installer prefix removed. Interactive main demo remains running.
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
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Final ledger formatting passes.
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

Three real development repository profiles, native sample links, browser sidebar flags and explicit CLI installer. Production storage/API contracts unchanged. Git-backed browser serving excluded by user choice. Rust qualification and root aggregates are out of scope; builds only supply real fixture/startup artifacts. The cold seed also fixes its source-cwd escape; complete affected seeding tests and an isolated live capture/teardown cover that harness boundary.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

One native checkout; all three projects linked to it. Sidebar flags reset on app mount. No installer script.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native manifest/WAL and Git SHA-1 companion authorities unchanged. Private profile marker version 1; browser navigation flags version 1. Enclosing Jujutsu change muputzvr.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; locked CLI, Svelte, Storybook and Vitest.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the three repository profiles and persisted sidebar in the main demo; run pnpm cli:install for an explicit user CLI installation.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Git-backed JJ browser serving remains M13.4; visual baseline approval and
  release qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/three-profiles-*.log and navigation-persistence-*.log; isolated /tmp/jj-store-three-profiles-cold and /tmp/jj-store-cli-install-review. Durable source chapter/ledger evidence retains scope; ignored .scratch/three-profiles-cold-before.json and -after.json bind the exact compared snapshots. .scratch/three-profiles-projects.jpg shows the native home and its three distinct links.
<!-- /evidence-field -->

