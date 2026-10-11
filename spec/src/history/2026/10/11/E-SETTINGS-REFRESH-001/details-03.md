---
schema: 1
id: E-SETTINGS-REFRESH-001
---

# E-SETTINGS-REFRESH-001 details 3

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
pnpm test:storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte
```

**Notes**

8 stories pass again after final underline and overflow styling.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=80d8731bbc255ca6373c797e8b9eb4b4 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/repository-settings-presentation.spec.ts
```

**Notes**

New combined selected-tab/More locator also matched hidden measurement probes; narrowed to visible controls. No production behavior or assertion thresholds changed.
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
pnpm check:progress; pnpm spec:first
```

**Notes**

Initial evidence had bare story/test paths and stale changed-file coverage. Corrected to named executable tests and refreshed file coverage. Inherited E-NPM-REGISTRY-001 missing .pnpmfile.cjs/invalid result/reference errors remain separately recorded.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Final tree retains the same 18 inherited source-budget diagnostics. No changed presentation file exceeds its budget.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=80d8731bbc255ca6373c797e8b9eb4b4 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/repository-settings-presentation.spec.ts
```

**Notes**

Final desktop/mobile presentation tests both pass: all seven destinations, exact 3px active underline overlapping divider, selected More state, fixed navigation during real keyboard scrolling, no header/first-row gap, single-line copy values and icons, full-width rows, right-aligned actions, light/dark, no overflow and Axe.
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
pnpm test:storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
```

**Notes**

Final 5 identifier stories pass after full-value hover hint; exact copy, keyboard, independent Open, truncation, centred 24px controls and stale clipboard feedback retained.
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
pnpm docs:check
```

**Notes**

Application documentation fingerprint became stale after the UI change. Updated Settings guide and configuration reference and reviewed the source before final check.
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
pnpm docs:review application; pnpm docs:check
```

**Notes**

Review reason records settings presentation, fixed navigation, Scroll Area, exact copy and unchanged behavior. Final strict build renders 9 Mermaid diagrams and validates 141 files, links and anchors (22,565,408 bytes).
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
pnpm check:progress; pnpm spec:first
```

**Notes**

After correcting this record, only inherited E-NPM-REGISTRY-001 diagnostics remain: missing .pnpmfile.cjs, missing pseudo-file origin/version checks, invalid test reference, invalid run result, invalid result. No settings evidence errors or uncovered changed files remain.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Focused UI and documentation acceptance is complete. Exploratory full-catalog
  failures remain untriaged; this record does not claim a full-suite pass.
- Inherited E-NPM-REGISTRY-001 governance and 18 source-budget diagnostics
  remain separate. Visual approval and packaged release qualification are not
  advanced.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.0; Svelte 5/Vite 8; published Design Core 0.3.1 and Mira 0.0.3 retained. No release or dependency change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No route, API, authorization, configuration-file, manifest, WAL or receipt format changes. Opt-in presentation props and context only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository settings presentation and action placement across General, Local folders, Search, Storage, Sync and Advanced. Shared cards/headers/rows/footers preserve global defaults. New/refreshed settings and forms across the app follow the documented convention; global adoption is deferred. Copyable values truncate per latest user direction; descriptive values wrap. Primary and secondary navigation stay above the bounded released Scroll Area. User explicitly directed application CSS for line-tab presentation after upstream defect diagnosis; no package patch, alias, local override or publication. The prepared Design Core source patch is archived locally and its checkout restored clean. Focused live acceptance precedes further test edits, as requested. A prior full catalog exploration was stopped at user direction; wider failures remain separately unqualified. UI-only: no Rust/RustFS rebuild, migration/reset, visual baseline or root aggregate check/test/build/spec:check.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

The UI slice is verified for live desktop/mobile use and committed with Jujutsu. Qualify wider catalog failures and inherited governance diagnostics separately; no package publication is needed.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012/016 presentation requirements updated for repository cards,
  scoped line-tab CSS, fixed navigation with bounded Scroll Area, single-line
  copyable values, row icons and shared settings/form action placement.
  Navigation/history, permissions, controller owners, drafts, stale-response
  rejection, exact pending identities, WorkStore restrictions, authorization and
  persisted formats remain unchanged. JST-GOV-002/008 ownership retains released
  dependencies with the explicit user-authorized repository tab styling
  exception.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Reviewed M4.18, M14.1, M14.2, M14.4, M14.5 and M14.6. Existing Done/In
  progress statuses remain; this presentation change promotes no visual, sync,
  platform or release gate. The enclosing Jujutsu change owns the evidence, not
  a predicted future commit hash. Reviewed M19 as Partial without changing its
  packaged-release gates; Settings/manual locations and application source
  review are current.

```
<!-- /evidence-field -->

