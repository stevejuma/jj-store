---
schema: 1
id: E-M11-006
---

# E-M11-006 details 2

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
jj --no-pager st; jj --no-pager log; jj workspace list; inspect main-only diff and owned development runtime
```

**Notes**

Both integration tips are committed and main is clean. Main-only slice has seven files. Runtime ownership, persistent fixture and restart script checked; other workspaces and Storybook preserved.
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
jj new psszmplx nuwmlxxk -m 'Merge Projects hub into main workspace'
```

**Notes**

Two-parent merge. Production code merges automatically; ledger and verification matrix conflicts resolved by retaining Projects qualification, adding main E-UI-043 and M4.30, and combining verification references. Empty-path editor guard and asynchronous document-tab connectivity check both retained.
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
pnpm install --offline --lockfile=false --ignore-scripts
```

**Notes**

Documented ignored local package hook refreshes main Community snapshot. Composer and edit-context bytes match qualified projects-hub and owning source output. Semver manifest and tracked lockfile unchanged. Log .scratch/projects-main-install.log.
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

Zero errors and warnings. Log .scratch/projects-main-web-check.log.
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
pnpm catalog:check
```

**Notes**

Zero errors and warnings; 87-module catalog boundary passes. Log .scratch/projects-main-catalog-check.log.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md
```

**Notes**

Merged ledger and verification table normalized. Log .scratch/projects-main-format.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

19/19 interaction/accessibility stories pass across four directly affected files on a fresh browser process. .scratch/projects-main-stories.log. Visual baselines remain pending.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/production-work.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

32/32 controller and route tests pass in six direct-consumer files. .scratch/projects-main-controller.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

2/2 cold built-bundle breadcrumb/history regressions pass in an isolated fresh UI preview; no native mutation. .scratch/projects-main-browser.log.
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
pnpm spec:validate
```

**Notes**

4 validators, 20 chapters and 125 requirements pass. .scratch/projects-main-spec.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Checker assumes a single @- and rejects the two-parent merge; no trustworthy default change set. Retry with explicit native commit identities. .scratch/projects-main-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base rkqwtxnl --head @
```

**Notes**

Explicit-base mode uses read-only Git objects and therefore rejects JJ change aliases; use full native commit IDs. .scratch/projects-main-spec-first-explicit.log.
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
pnpm spec:first --base <full rkqwtxnl commit ID> --head <full zllovtln commit ID>
```

**Notes**

Gate passes with the complete common-ancestor-to-merge diff, using full commit IDs obtained by JJ. Exact resolved command and results in .scratch/projects-main-spec-first-sha.log; final ledger rerun follows.
<!-- /run -->

<!-- run -->
### Run 14

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

96 tasks pass evidence bookkeeping. .scratch/projects-main-progress.log.
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
pnpm check:architecture
```

**Notes**

377 production/catalog modules pass source-size and boundary audit. .scratch/projects-main-architecture.log.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md
```

**Notes**

Integration evidence and merged matrix formatted. .scratch/projects-main-ledger-format.log.
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
pnpm dev:restart
```

**Notes**

Owned main checkout script rebuilds web/asset manifest, embedded RustFS and jj-store-cli bins, then opens persisted sample. Ready at 127.0.0.1:7460, PID 60331, search ready, 13332 ms physical restart. .scratch/projects-main-restart.log.
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
Verify persistent fingerprints, HTTP entry assets and owned listeners
```

**Notes**

Fixture, installation identity and registrations retain exact pre-restart SHA-256 bytes; repository ID and runtime root unchanged; fresh PID 60331 serves all 20 HTML entry assets. Storybook remains original PID 27372 on port 6046. .scratch/projects-main-before.json and projects-main-readiness.json.
<!-- /run -->
<!-- /evidence-field -->

