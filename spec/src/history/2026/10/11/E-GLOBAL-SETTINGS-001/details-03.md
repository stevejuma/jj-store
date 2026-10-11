---
schema: 1
id: E-GLOBAL-SETTINGS-001
---

# E-GLOBAL-SETTINGS-001 details 3

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
pnpm check:ui-ownership
```

**Notes**

UI ownership passed. Local log: .local/global-settings/ownership-final.log.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries passed. Local log: .local/global-settings/boundaries-final.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

Parent reproduces five E-NPM-REGISTRY-001 diagnostics: missing .pnpmfile.cjs and prose pseudo-file; invalid named test, run result and result. Local log: .local/global-settings/progress-parent.log.
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
pnpm exec tsx --test web/tests/numeric-setting.test.ts web/tests/repository-settings-navigation.test.ts
```

**Notes**

9 tests passed; exact canonical numerical values and settings history/back/profile defaults.
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
pnpm test:storybook catalog/components/store-application/AgentChatGpt.stories.svelte
```

**Notes**

2 account stories passed after removing the doubled header/footer divider. .local/global-settings/chatgpt-divider-final.log.
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
pnpm catalog:check
```

**Notes**

Final 0 errors/warnings, 398 catalog module boundary check passed. .local/global-settings/catalog-delivery-final.log.
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
pnpm docs:review application 'Global Settings shared presentation, consolidated Preferences with metadata disclosure, joined numerical controls, consistent notice spacing and single header dividers; existing navigation, authorization and saves retained'
```

**Notes**

Refreshed application documentation fingerprint after final presentation changes. .local/global-settings/docs-review-verified.log.
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
jj workspace forget global-settings-baseline
```

**Notes**

Forgot then removed the isolated parent-baseline checkout; retained original workspace and development fixture.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial check overlapped formatting and reported working-copy-changed; rerun on stable source.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial new evidence omitted 12 changed milestone/history pages. Added those paths; five E-NPM-REGISTRY-001 diagnostics inherited.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Same omitted evidence paths plus five inherited diagnostics; own evidence coverage repaired.
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
pnpm docs:check
```

**Notes**

141-file documentation closure and links/anchors passed; nine Mermaid diagrams rendered in both themes.
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
pnpm spec:links
```

**Notes**

2558 HTML files and 96461 local references passed.
<!-- /run -->

<!-- run -->
### Run 14

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

Verified stable-source result contains only the five inherited E-NPM-REGISTRY-001 diagnostics, reproduced at parent. All new evidence coverage errors resolved.
<!-- /run -->

<!-- run -->
### Run 15

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

Verified stable-source result contains only the five inherited E-NPM-REGISTRY-001 diagnostics, reproduced at parent. All new evidence coverage errors resolved.
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
pnpm exec prettier --write <changed files returned by jj diff --name-only>
```

**Notes**

78 changed files formatted, including all owning chapters, evidence and milestone reviews. File list is the enclosing change.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Focused implementation passes are separate from one inherited composer story
  and five inherited E-NPM-REGISTRY-001 governance diagnostics. No full-suite
  pass is claimed.
- Visual baselines remain unapproved; no reset, migration, dependency override,
  native build, platform or release qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0, pnpm 10.32.1, JJ 0.45.1, Storybook 10.5.7, Vitest 4.1.11, Playwright 1.63.0. Released Design Core ^0.3.1 and Mira ~0.0.3 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, persisted configuration, authorization, repository, route or credential format changes. Global categories remain transient. Optional Compute memory retains null/automatic; canonical numerical values and Save/Apply semantics remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Global Settings adopts shared cards, divided rows, tinted icon headers, joined numerical inputs, bounded right-aligned controls and action footers. Fixed headings and line subtabs sit outside ScrollArea. Appearance has Theme/Preferences and sibling preview/layout cards. Launch/Storage inventory is grouped with exact path copy. Runtime commands share rows with descriptions. Preferences combines conversation/device controls and a metadata disclosure retaining the mounted editor. Shared banners receive 1rem vertical clearance and inset card sides. User-directed application CSS is scoped; unrelated consumers keep defaults. Community then released Design Core inspected before presentation work; no source copied. Full UI form guidance updated in canonical requirements and UI AGENTS.md. Verification follows repository targeted policy: no root aggregate or Rust lanes.
<!-- /evidence-field -->

<!-- evidence-field:live_acceptance markdown -->
## Live acceptance

CUA localhost cold tabs and HMR: Compute Models/Resources/Sharing Save gaps measured 16px; registration banner 16px insets; standalone disabled Capture banner has 16px before and after. Runtime commands align right and descriptions expand. Metadata model is 320px inside a 488px column with trailing offset 0. Harness separators are 1px with no empty gaps. Preferences metadata starts closed, shows its description, has a 1px top divider and accessible description; ChatGPT empty footer has 0px top border against 1px header divider. Earlier 390px check found no document overflow, wrapped actions and fixed navigation. Grouped Launch/Storage inventories visually checked. No live settings saved, accounts connected or runtimes changed. Screenshots are ignored under .local/global-settings; independent visual-baseline approval not inferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified UI slice with Jujutsu; retain composer and governance issues with their existing owners. Remaining platform, release and visual gates unchanged.
<!-- /evidence-field -->

