---
schema: 1
id: E-UI-098
---

# E-UI-098

<!-- evidence-field:id markdown -->
## Id

E-UI-098
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Align and scope Settings identifier copy actions
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zksxxqxv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean at mompnpzq/aad84020 after notification integration. All StoreReadOnlyRow values expose copy controls; rows without an Open action have their copy icon one button width too far right.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0; pnpm 10.32.1; JJ 0.45.1; locked Svelte 5, Storybook 10.5.7 and Vitest 4.1.11. Existing foundation source-validation setup inherited unchanged; no dependency changes or publication.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned UI composition and presentation only. No HTTP, native identity, storage, WAL, receipt, companion metadata or persistence changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/fixtures/ReadOnlyRowFixture.svelte
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreStorageOperation.svelte
- web/src/lib/ui/store-settings/StoreStoragePosition.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncWait.svelte
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
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
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Live browser comparison of Thread ID and Revision copy button geometry
```

**Notes**

Before: linked Thread ID copy x1186, copy-only Revision x1218; each button32px wide. One missing link slot causes a 32px misalignment.
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
pnpm test:storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte; pnpm web:check
```

**Notes**

Initial alignment-only change:7/7 interactions/accessibility, web0 errors/0 warnings. .local/copy-align-stories.log and .local/copy-align-web.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreSyncSettings.stories.svelte
```

**Notes**

4 passed/1 failed after adding the requested eligibility regression: ordinary Push target through 9 still exposes a copy button. .local/copy-scope-before.log.
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
pnpm test:storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

36/36 directly affected copy, Thread, Repository, Service, Search, Storage, Sync and navigation interactions/accessibility. Existing identifier story now proves a destination enables copy even without explicit copyable opt-in; Sync count remains visible and has no copy action. .local/copy-scope-stories.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings;166 catalog modules. .local/copy-scope-web.log and .local/copy-scope-catalog.log.
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
pnpm exec prettier --write changed files
```

**Notes**

Changed production, catalog and canonical files formatted. .local/copy-align-format.log and .local/copy-scope-format.log.
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
Live in-app browser verification on 127.0.0.1:7460
```

**Notes**

After: Thread ID and Revision copy x976, width32. Repository ID/Selected operation/Operation head and copy-only Epoch all x964. Local service has zero copy buttons. Repository name, permission, counts, profile, capabilities and limits have no copy controls; IDs and hashes retain their actions. CSS reserves two equally sized action columns without a dummy control. Screenshots .local/copy-row-alignment.png and .local/copy-row-scope.png. Temporary tab10 closed; user tab and HMR preserved; no saved mutations.
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
pnpm exec prettier --check changed files; pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

15 changed files formatted; 859 architecture modules; UI ownership passes; spec-first covers 12 protected files and 3 canonical chapters. 5 validators, 22 chapters, 153 requirements and 126-task evidence bookkeeping pass. Logs .local/copy-scope-{format-final,architecture,ownership,spec-first,spec-validate,progress}.log.
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

Copyability defaults off and is explicitly enabled for identifiers/hashes or supplied internal destinations. Plain statuses/counts/descriptions stay non-interactive. Grid action slots preserve copy alignment without an empty focusable button. UI-only targeted policy: focused affected stories, types, browser geometry and governance checks; root aggregates, Rust lanes and built-bundle qualification out of scope. Visual baselines and existing registry release qualification remain separate; no milestone status promoted.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the focused copy eligibility and interaction regressions; main HMR stays available for review.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


