---
schema: 1
id: E-M21-058
---

# E-M21-058 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
detail: New Vite PID 77542 on 7476; backend PID 42570 on 7478 and
  .local/dev-demo preserved. No Rust builds, backend restart, reset or unrelated
  process changes.
hasNote: false
noteEncoding: markdown

```

```shell
Owned Vite-only restart using inspectRuntimeProcesses/start/ready helpers
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
detail: With only registry reads held, live main shows Loading stack workspace
  busy skeleton and no unavailable copy. Continued held reads and cleared raw
  interception, then verified loaded Revision graph. Native WIP/merge bookmark
  chips measure 256px; labels 241px contain 592/607px text, with complete hover
  titles. Temporary network state restored. Proof
  .local/stacks-startup-chips/live-loading.png and live-chips.png.
hasNote: false
noteEncoding: markdown

```

```shell
CUA held discovery refresh, loaded JJ Log chip geometry and screenshots
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
detail: 1 passed against existing Vite with mocked API and existing ignored
  native response fixtures. Updated only the chip semantic wrapper assertion;
  working-copy, Snapshot and operation-head decorations pass. The ignored config
  disables the normal Rust preview launcher.
  /tmp/stack-startup-browser-read.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/stacks-startup-chips/browser.config.ts reads.spec.ts -g 'branch targets scope native history and expose repository decorations'
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
detail: Application fingerprint reviewed against the browsing guide; production
  and test files formatted. Native API, commands and TOML remain unchanged.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm docs:review application <startup and bounded history label review>; pnpm exec prettier --write <changed files>
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
detail: 70 passed in 10 files in a fresh browser process after the tab-fit fix;
  includes all original startup/chip regressions and direct consumer coverage.
  /tmp/stack-startup-all-final-stories.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <10 affected Loading, Graph, Stacks/Application and ordinary RepositoryNavigation story files>
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
detail: "Final: 0 errors/0 warnings; 244 catalog modules.
  /tmp/stack-startup-all-final-web.log and catalog.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
detail: "Final ownership: 1 validator/23 chapters/173 requirements;
  architecture: 1140 modules and original line budgets respected.
  /tmp/stack-startup-all-final-ownership.log and architecture.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership; pnpm check:architecture
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
detail: Offline light/dark Mermaid render, documentation closure, links and
  anchors pass. /tmp/stack-startup-docs-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm docs:check
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
detail: 19 protected files/4 canonical chapters; 5 validators/23 chapters/173
  requirements. /tmp/stack-startup-first-final.log and spec-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm spec:validate
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
detail: Final Vite PID 91981 on 7476, backend PID 42570 unchanged. Cold normal
  reload shows the Loading stack workspace skeleton without unavailable copy;
  discovery and loaded history proof retained. Network latency zero and raw
  interception cleared; user tabs untouched.
  /tmp/stack-startup-cold-restart-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
Final owned Vite-only restart and CUA cold reload
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
detail: All changed files formatted; progress validates 142 tasks. All 596
  previous evidence entries remain identical as parsed JSON data.
  /tmp/stack-startup-format-final.log and progress-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <23 changed files>; pnpm check:progress
```
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

Keep the existing live Vite/backend available. The verified UI slice is committed independently; broader release/provider gates and visual baselines remain separate.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Native labels preserve complete values while long chip text truncates within 16rem/available width. Startup no longer interprets absent controllers or binding-only snapshots as unavailable/empty workspace content. Discovery is independently reactive, first successful reads unlock content and later refreshes retain it. Failed reads have scoped notifications and neutral retry presentation. A related exact overflow regression exposed redundant tab fitting remounting the More trigger; equivalent fits now preserve its identity.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Exact before/after chip and startup regressions; all directly affected workspace controller units; affected Stacks/application/history/navigation stories, web/catalog types, UI ownership, architecture, one existing mocked native browser regression and cold live Vite verification. Additional repository-navigation stories are included because the More fix affects ordinary and focused tab hosts. Consumer root aggregates, unrelated UI families, Rust builds/tests, generated transport contracts, source package checks/publication, screenshot comparisons and baseline updates are out of scope under targeted UI policy; no milestone promotion or native repository mutations.
<!-- /evidence-field -->

