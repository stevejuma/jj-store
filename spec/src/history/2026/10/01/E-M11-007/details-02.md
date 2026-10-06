---
schema: 1
id: E-M11-007
---

# E-M11-007 details 2

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
node --test scripts/dev-demo-repository.test.mjs scripts/dev-demo-state.test.mjs scripts/dev-demo-collaboration.test.mjs
```

**Notes**

Before state: 8/8 existing fixture tests pass. .scratch/work-seed-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/dev-demo-work.test.mjs (before implementation)
```

**Notes**

Exact new target fails at missing seeder import; no native data modified. .scratch/work-seed-regression-before.log.
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
node --test scripts/dev-demo-work.test.mjs
```

**Notes**

Initial three native command-seed regressions pass. Added interrupted task-creation regression before final focused rerun. .scratch/work-seed-regression-after.log and work-seed-unit.log.
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
pnpm exec prettier --write package.json scripts/dev-demo.mjs scripts/dev-demo-work.mjs scripts/dev-demo-work-samples.mjs scripts/dev-demo-work.test.mjs spec/src/cli.md spec/src/spec-governance.md spec/src/architecture.md web/tests/real/work-seed.spec.ts
```

**Notes**

Initial changed-path formatting. .scratch/work-seed-format.log.
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
pnpm exec prettier --write scripts/dev-demo.mjs scripts/dev-demo-work.test.mjs web/tests/real/work-seed.spec.ts
```

**Notes**

Interrupted creation and owned-process checks normalized. .scratch/work-seed-format2.log.
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
node --test scripts/dev-demo-*.test.mjs scripts/governance.test.mjs
```

**Notes**

35/35 initial direct development/governance tests pass; final 36/36 includes interrupted task creation. .scratch/work-seed-harness.log and work-seed-harness-final.log.
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
node --check scripts/dev-demo-work.mjs; node --check scripts/dev-demo-work-samples.mjs; node --check scripts/dev-demo.mjs
```

**Notes**

All three native harness modules parse.
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
node --test scripts/dev-demo-work.test.mjs (final focused target)
```

**Notes**

4/4 pass, including interrupted task creation before initial state change. .scratch/work-seed-unit-final.log.
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
pnpm exec prettier --write web/tests/real/work-seed.spec.ts
```

**Notes**

Native API reads use the production loopback Origin/Fetch Metadata headers. Seeded task heading binds its exact native short ID and task route binds the full ID. .scratch/work-seed-format3.log and work-seed-format4.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7482 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_REAL_TEST=work-seed.spec.ts pnpm web:test:real
```

**Notes**

1/1 cold real RustFS/browser seed test passes. Three native projects, nine tasks/all four states, three subtasks, six comments and three replies survive fixture preparation physical restart. Reseed publishes zero changes and leaves snapshot identical; production task thread opens with exact native route. Initial startup 1024 ms, prepared restart 870 ms, resident 171648 KiB; complete code/agent/review/work fixture preparation 382939 ms. Owned disposable service stopped and root removed. .scratch/work-seed-cold.log.
<!-- /run -->

<!-- run -->
### Run 11

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

4 validators, 20 chapters and 125 requirements pass. .scratch/work-seed-spec.log.
<!-- /run -->

<!-- run -->
### Run 12

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

6 protected files, 5 owning canonical chapters pass. .scratch/work-seed-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 13

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

96 tasks pass evidence bookkeeping. .scratch/work-seed-progress.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Interactive native diagnostic GET with only protocol header
```

**Notes**

403 from the existing LocalTrusted Origin/Fetch Metadata policy; diagnostic corrected to match production seeder headers. No mutation from that denied GET. The new cold read assertion uses the same required headers.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:seed:work
```

**Notes**

Interactive owned main service adds 3 projects, 9 tasks and 6 comments in 28 native publications, with existing repository and live service retained. .scratch/work-seed-main.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Verify interactive native WorkStore records and retained identity files
```

**Notes**

Native selected reads confirm all four states, 3 subtasks/3 replies and active home links. Exact fixture, installation and repository-registration SHA-256 bytes match pre-seed evidence; original PID 60331 retained. Work state version 28 at manifest/publication 101. .scratch/work-seed-main-verify.log and work-seed-main-snapshot.json.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:seed:work (interactive repeat)
```

**Notes**

0 new publications; existing records preserved. .scratch/work-seed-main-repeat.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CUA live native sample project and task-thread verification
```

**Notes**

All three projects render under Mine and Your projects with linked home, active/done counts and current timestamps. Native browser task shows In progress, two subtasks and three threaded comments. Exact native heading and full task route verified, runtime errors empty. Screenshots .scratch/work-seed-projects.png and work-seed-thread.png. Temporary tab closed; user tabs/routes preserved. Initial title-only heading probe had no match because native short ID is part of accessible name; final assertion includes it.
<!-- /run -->
<!-- /evidence-field -->

