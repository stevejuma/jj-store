---
schema: 1
id: E-NUMERIC-SETTINGS-001
---

# E-NUMERIC-SETTINGS-001 details 3

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
pnpm catalog:check
```

**Notes**

Final 0 errors/warnings, 396 catalog modules; preceding qualified and conditional-reset checks also pass. /tmp/jj-numeric-final-catalog.log.
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
pnpm check:ui-ownership
```

**Notes**

Final ownership check passes; released Design Core/Mira retained, no Community runtime/source dependency or catalog import. /tmp/jj-numeric-final-ownership.log.
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
pnpm check:boundaries
```

**Notes**

All 17 package boundaries pass. /tmp/jj-numeric-final-boundaries.log.
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
pnpm docs:review
```

**Notes**

Initial invocation required an application source review reason. The subsequent application review recorded numeric entry and unchanged configuration semantics.
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
pnpm docs:review application "Reviewed shared numeric entry for Search budget, worker limits and Sync; exact unit conversions, conditional icon reset, hover and joined focus styling. Consolidated app form guidance in the UI guide and spec; existing API formats, permissions and Save/Apply or restart semantics remain unchanged."
```

**Notes**

Application fingerprint refreshed after the final source edit. /tmp/jj-numeric-final-doc-review.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 32
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial evidence story names were not recognized by the executable-test reference parser (27 diagnostics); retained the executed story names in the actual run description and use named unit/controller tests in references. Five inherited E-NPM-REGISTRY-001 diagnostics are separate. /tmp/jj-numeric-final-progress.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 32
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Same initial evidence reference diagnostics and five inherited E-NPM-REGISTRY-001 errors. /tmp/jj-numeric-final-spec-first.log.
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
pnpm docs:check
```

**Notes**

Final strict manual build renders nine Mermaid diagrams; 141 files, 22567259 bytes, links and anchors valid. Earlier docs checks also pass. /tmp/jj-numeric-final-docs.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Final record has no numeric-settings diagnostics. Only inherited E-NPM-REGISTRY-001 remains: missing .pnpmfile.cjs, invalid pseudo-file/test reference, invalid run result and invalid result. /tmp/jj-numeric-qualified-progress.log
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Final record has no numeric-settings diagnostics. Only inherited E-NPM-REGISTRY-001 remains: missing .pnpmfile.cjs, invalid pseudo-file/test reference, invalid run result and invalid result. /tmp/jj-numeric-qualified-spec-first.log
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
pnpm exec prettier --check catalog/components/store-application/StoreNumericSetting.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/fixtures/NumericSettingFixture.svelte docs/maintenance/coverage.json docs/site/guides/settings.md spec/src/current/contracts/api/30-installation-configuration-editing.md spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md spec/src/current/progress/M14/history/M14.4/index.md spec/src/current/progress/M14/history/M14.6/index.md spec/src/current/progress/M14/index.md spec/src/current/progress/M19/history/M19/index.md spec/src/current/progress/M19/index.md spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/SUMMARY.nav.md spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/details-02.md spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/details-03.md spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/index.md spec/src/history/2026/10/11/SUMMARY.nav.md spec/src/history/2026/10/11/index.md web/src/lib/ui/AGENTS.md web/src/lib/ui/store-settings/StoreNumericSettingNumberRow.svelte web/src/lib/ui/store-settings/StoreNumericSettingRow.svelte web/src/lib/ui/store-settings/StoreSearchAdvanced.svelte web/src/lib/ui/store-settings/StoreSearchFields.svelte web/src/lib/ui/store-settings/StoreSyncConfiguration.svelte web/src/lib/ui/store-settings/numeric-setting.css web/src/lib/ui/store-settings/numeric-setting.ts web/src/lib/ui/store-settings/search-numeric-fields.ts web/src/lib/ui/store-settings/sync-numeric-fields.ts web/tests/numeric-setting.test.ts
```

**Notes**

Scoped formatting check of the verified slice. Authored changed spec pages also fit 400 physical lines and 32768 bytes. /tmp/jj-numeric-final-format.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Focused numeric-settings acceptance passes. Inherited E-NPM-REGISTRY-001
  governance diagnostics remain separately tracked; no broader suite pass is
  claimed.
- Existing architecture-budget diagnostics, exploratory catalog failures, visual
  baseline approval and packaged-release gates are outside this presentation
  slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.0; published @lapismd/design-core ^0.3.1 and @lapismd/mira ~0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, persisted configuration, identity, publication, authorization or route format changes. Values remain canonical integer milliseconds, seconds, bytes or counts. Global Settings keeps its existing presentation.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared application-owned numeric row, exact decimal unit helpers, Sync numeric-draft adapter, all repository Search budget/worker limits and all eleven Sync limits. Joined unit dropdown and conditional icon-only Reset have theme hover, complete right border and one focus ring. Redundant default-state captions removed; exact conversion helpers remain. All form conventions are consolidated in the UI AGENTS guide and canonical JST-UI-016 chapter. Community and released Design Core inspected; no Community source copied or package override added. Narrow target plus directly affected consumers only: no root aggregate checks, Rust builds/tests, resets/migrations, package publication or baseline updates.
<!-- /evidence-field -->

<!-- evidence-field:live_acceptance markdown -->
## Live acceptance

CUA inspection of http://127.0.0.1:7460 in a clean temporary tab: Search budget 512MiB -&gt; 0.5GiB leaves Save disabled and Reset absent. Changed budget shows Reset; reset restores the default and input focus. Sync draft exposes eleven numeric limits; 5000 -&gt; 2500ms adds exactly one Reset; reset restores 5000 and removes it. Search worker defaults have no Reset; editing 1000 -&gt; 2500ms adds one. Final default/changed captions absent. Joined group has a neutral 1px right border, unit addon sits 1px inside its edge, and focused group has a single 3px theme ring with no input shadow. Earlier live unit/save-disabled, refresh draft retention, subtab round-trip, reset and 390px containment checks pass. Browser-only test drafts were cancelled; no live configuration was saved. Screenshot: .local/numeric-settings/worker-limits-final.png (local, ignored).
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified UI slice with Jujutsu; resolve inherited governance/release diagnostics separately. Broader global Settings adoption remains a later task.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-016 presentation guidance now defines shared exact-unit numeric
  fields, attached conditional icon reset, hover/focus/border treatment and
  compact rows without redundant state captions. Existing canonical
  values/ranges/defaults, Save/Apply versus restart, permission filters,
  routes/controllers, pending identities and lifecycle remain unchanged under
  JST-API-001/004. JST-DOC-001/002/003 source-review and delivery gates remain
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M14.4 remains Done, M14.6 In progress, and M19 Partial. This
  presentation/form-guidance slice advances no sync platform, packaged-release
  or visual gate. The enclosing Jujutsu change is the evidence revision.

```
<!-- /evidence-field -->

