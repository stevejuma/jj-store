---
schema: 1
id: E-UI-053
---

# E-UI-053

<!-- evidence-field:id markdown -->
## Id

E-UI-053
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

appearance-local-main-landing
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change krqtopsq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main is clean at krylzqny/0c75eb0c; Appearance rmwvnkzr is qualified on 59f1cfe8, before the direct settings, aligned headers and targeted reaction-metadata commits. Foundation yovkqwtu is qualified and both default and source workspaces are clean. User explicitly requests landing in main.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
- spec/src/verification.md
- docs/reference/appearance-provenance.md
- spec/src/architecture.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/appearance.spec.ts#cold device appearance restores under the
  production CSP and propagates across tabs
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
jj rebase -s rmwvnkzr -o krylzqny; jj squash --into rmwvnkzr --use-destination-message
```

**Notes**

Resolved UI/ledger/Thread-story conflicts by preserving all 251 prior main evidence entries and current Thread requirements/stories, then adding the Appearance requirement/evidence/draft scenario. No unresolved conflicts. Source change identity rmwvnkzr retained; final implementation 68a3dbdc.
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
npm view @lapismd/design-core version dist-tags --json; npm pack @lapismd/design-core@0.2.0 --pack-destination /tmp/jj-appearance-registry --json
```

**Notes**

Registry latest is 0.2.0. Read-only tarball inspection confirms chat type scale and root native scrolling extensions are absent. This proves the remaining release gate; no publication.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t AppearanceFocus
```

**Notes**

Rebased Appearance workspace: exact draft/independent details rail acceptance 1/1, 6 filtered skips; /tmp/jj-appearance-main-exact.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/production-settings.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/store-shell-layout.test.ts
```

**Notes**

Initial subset 33/33 across four matching files. production-settings.test.ts does not exist; actual three Settings files are included in the complete focused rerun below.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/store-shell-layout.test.ts
```

**Notes**

Complete focused surface: 46/46 across seven files; /tmp/jj-appearance-main-complete-unit.log.
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
pnpm exec tsx scripts/appearance-palettes.mts --check
```

**Notes**

All 62 generated palette files match pinned source bytes; /tmp/jj-appearance-main-palettes.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-threads catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

21/21 across seven files including axe, Appearance controls, newer targeted main/reply reaction feedback, Thread settings/alignment and retained Focus/Split drafts; /tmp/jj-appearance-main-stories.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Rebased source workspace: zero type errors/warnings; 125 catalog modules; 714 architecture modules; ownership passes. /tmp/jj-appearance-main-*-check.log, ownership.log and architecture.log.
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
pnpm web:build; pnpm exec playwright test -c .scratch/appearance.playwright.config.ts
```

**Notes**

Rebased source workspace: production build passes with inherited large-chunk advisory; cold healthy-directory Appearance flow 1/1, desktop/mobile axe, OS mode/reload/cross-tab and scoped CSP/no external requests. /tmp/jj-appearance-main-build.log and cold.log.
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
jj rebase -r @ -o yovkqwtu (Design Core default); jj rebase -r @ -o rmwvnkzr (jj-store default); pnpm install --no-frozen-lockfile --lockfile=false
```

**Notes**

Both clean default workspaces advance locally. Main installation uses its existing ignored .pnpmfile.cjs source hook; no tracked manifest/lock rewrite. Byte comparison of installed chat.css, chat.tokens.ts and scroll-area.svelte matches landed Design Core yovkqwtu. /tmp/jj-appearance-land-main-install.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t AppearanceFocus; pnpm web:check; pnpm web:build; pnpm exec playwright test -c .scratch/appearance-land.playwright.config.ts
```

**Notes**

Actual main workspace: draft/rail regression 1/1 with 6 filtered skips, zero type errors/warnings, fresh bundle passes with existing size warning, and isolated port7491 cold Appearance flow 1/1. /tmp/jj-appearance-land-main-exact.log, web-check.log, build.log and cold.log.
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
pnpm spec:first
```

**Notes**

Landing provenance documentation maps to the architecture chapter; the initial evidence-only change omitted that chapter. Added the explicit local-source/registry qualification boundary in the owning chapter; no gate suppression. /tmp/jj-appearance-land-main-spec-first.log.
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
pnpm spec:validate; pnpm check:progress
```

**Notes**

Five validators, 21 chapters, 146 requirements and 120 tasks. Evidence bookkeeping only. /tmp/jj-appearance-land-main-spec-validate.log and progress.log.
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
pnpm spec:first; pnpm exec prettier --check spec/src/architecture.md spec/src/milestones.md spec/src/verification.md docs/reference/appearance-provenance.md
```

**Notes**

Repaired exact governance target passes: one protected file and two canonical chapters. Four changed documentation files are formatted. /tmp/jj-appearance-land-main-spec-first-repaired.log and format.log. Final evidence is consumed again before commit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Compatible Design Core registry release containing yovkqwtu and registry-only
  consumer qualification
- Inherited broad Storybook accessibility/mobile-draft and Bits teardown
  failures, mounted Mira/CodeMirror strict-CSP qualification, and separate
  visual baseline approval
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Local main landing is complete. UI-only rebase/integration verification covers current main Thread changes and Appearance direct consumers. Earlier full relevant source qualification remains E-UI-052; inherited broad-suite failures remain explicit. Root aggregate checks/tests/builds, Rust lanes, live-service/browser full flows and wire contract generation are out of scope. No shared test infrastructure changes. No package publication, remote push, installed CLI change or service restart.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be; Shiki 4.0.2; Inter 5.2.8; JJ 0.45.1; pnpm 10.32.1; installed source Design Core 0.1.2/yovkqwtu; portable registry manifest/lock ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing device-local jj-store.appearance.v1 version1, exact repository/conversation session draft keys and native operation/receipt/WAL formats unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-053 details 2](details-02.md)
