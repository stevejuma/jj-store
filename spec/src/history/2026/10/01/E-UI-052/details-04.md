---
schema: 1
id: E-UI-052
---

# E-UI-052 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t AppearanceFocus
```

**Notes**

After rebase onto the newly committed 59f1cfe8, exact Focus acceptance passes 1/1 with 3 skips. Drafts survive, only the standard Thread pane is hidden, and independently open repository details retain their state.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/settings.test.ts web/tests/vite/threads.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/store-shell-layout.test.ts
```

**Notes**

After rebase, 31 tests across three matching files: Appearance, production Threads and independent shell layouts. Legacy settings.test.ts/threads.test.ts filter names do not exist; the complete unit run below includes actual production-* Settings coverage.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

Final base includes the new Thread regressions: 212/212 across 35 files, including all actual Settings and Thread model files; /tmp/jj-appearance-rebased-all-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-threads catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

After rebase: 18/18 across seven files; complete Appearance controls, native Thread presentation/reactions/replies, independent rail/Focus draft retention, agent discussion, Split review and project directory acceptance, including axe. /tmp/jj-appearance-rebased-stories.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

After rebase: zero type errors/warnings, 125 catalog modules, 715 owned source modules and ownership pass. Root aggregates and Rust lanes remain out of scope; /tmp/jj-appearance-rebased-*.log.
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
pnpm web:build
```

**Notes**

Final production bundle on 59f1cfe8 built; existing large-chunk warning is retained; /tmp/jj-appearance-rebased-build.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test -c .scratch/appearance.playwright.config.ts
```

**Notes**

Initial rebased cold run and diagnostic retry both reached the final CSP assertion: two blocked CodeMirror style elements. The generic fixture omitted Threads API, causing the new directory error composer to mount Mira. Diagnostic DOM ancestry identifies community-chat-composer__mira; no CSP or console diagnostics were suppressed. Healthy Threads fixture is now supplied, and mounted composer CSP qualification remains separately explicit.
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
pnpm exec playwright test -c .scratch/appearance.playwright.config.ts
```

**Notes**

Final built healthy empty-directory/Appearance lane passes 1/1: desktop/mobile axe, all controls, reload, cross-tab, OS mode, root geometry and reduced motion, zero CSP violations/external requests in this flow. /tmp/jj-appearance-rebased-cold-final.log.
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
pnpm spec:validate
```

**Notes**

After the supported verification status correction, five validators, 21 chapters and 146 requirements passed before final rebase/evidence completion.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check --ignore-unknown <all changed paths>
```

**Notes**

Final rebased evidence checks: 136 protected files/six owning chapters; five validators/21 chapters/146 requirements; 120 ledger tasks; all 143 changed paths formatted. Logs /tmp/jj-appearance-final-*.log. This final evidence-only annotation is then consumed again before commit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Release containing yovkqwtu within the consumer registry range and
  registry-only qualification
- Inherited broad Storybook failures/teardown errors and mounted Mira/CodeMirror
  strict-CSP qualification remain open
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only. Full relevant Storybook is justified by app-wide palette/type tokens and shared virtualizer/scrollbar lifecycle across native Threads, review, task and agent surfaces. Rust checks/builds/tests, wire contract generation, root aggregates and unrelated live service lanes are out of scope under targeted verification policy. Main source, its active landing evidence, installed CLI and running services are untouched by this task. The Appearance workspace alone rebases onto the latest committed Thread layout at 59f1cfe8.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

New appearance workspace is based on 0915f828 with Community-owned components and native Threads already landed. Baseline has transient service/repository/search/sync Settings but no complete Appearance page or device-wide policy.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Device-local jj-store.appearance.v1 JSON preferences, version 1; session-only exact repository/conversation draft keys; existing route/snapshot/native/WAL/API authority unchanged. Source budget preserved with 62 generated palette sheets.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be; Shiki 4.0.2; Inter Variable 5.2.8; pnpm 10.32.1; JJ 0.45.1; locked Svelte/Storybook/Vitest/Playwright. Temporary source Design Core 0.1.2 in yovkqwtu; portable manifest/lock continue to declare registry ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review rmwvnkzr and the source yovkqwtu commit. Complete a separately authorized compatible foundation release and registry-only acceptance before main integration; retain unrelated accessibility, composer CSP and visual gates.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending; screenshots are manual QA, not
  approval.
- Foundation release/registry-only validation, main landing, publication and
  unrelated service restart are not authorized.
- Inherited Overview bookmark Bits Command CSP violations remain outside the
  Appearance lane.
- Untouched-main accessibility and mobile-filter draft failures remain outside
  the Appearance implementation; no global diagnostic or axe suppression.
- Inherited native-association-v1.json formatting remains unchanged.
- Mounted Mira/CodeMirror composers inject inline styles under the unchanged
  strict CSP; the healthy empty-directory Appearance lane does not mount a
  composer. This is not a global editor/CSP pass.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/appearance-preview.mjs and appearance.playwright.config.ts use built web/dist, fixture APIs, isolated port7489 and exact production STRICT_CSP; owned server closes automatically. Logs in /tmp/jj-appearance-*.log and /tmp/dc-appearance-*.log; desktop/mobile screenshots /tmp/jj-store-appearance*.png.
<!-- /evidence-field -->

<!-- evidence-field:integrationBase markdown -->
## IntegrationBase

59f1cfe8: latest committed Thread presentation and separate repository/details rail changes; active main landing evidence is user-owned and untouched.
<!-- /evidence-field -->

<!-- evidence-field:mainWork markdown -->
## MainWork

Default workspace subsequently started active uncommitted Thread-menu/reaction edits in wytvknwz. No active main files are copied, overwritten or landed here. Consumer qualification is pinned to the committed 59f1cfe8 Thread layout, including its separate details rail; review/rebase after concurrent work completes before any main integration.
<!-- /evidence-field -->

