---
schema: 1
id: E-COMPUTE-007
---

# E-COMPUTE-007 details 4

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
pnpm exec prettier --write docs/maintenance/coverage.json <changed current spec Markdown paths>
```

**Notes**

Formatted changed authored documents; historical evidence remains generator-owned.
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
CUA inspection of focused Advanced before and after user screenshot correction
```

**Notes**

Before: keyboard-focused trigger radius 0px and external 3px ring clipped by parent border. After: collapsed radius 13px; expanded radii 13px 13px 0px 0px; transparent trigger border, no box shadow, visible 2px theme-colored outline inset by 3px. Inspected both keyboard-focused states and ordinary expanded state in the real isolated preview.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

First focused-outline regression read the animated outline offset before transition completion. The interrupted dialog left a subsequent defaults story blocked by pointer events. Added waitFor for resolved outline and corner transitions; assertions retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

After transition-aware assertions: fresh process, three interaction/Axe stories pass, including visible inset focus, rounded corners in both states and retained drafts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/{AgentProviderConfiguration,AgentDefaults,AgentFleet,AgentMentionSettings,LocalCompute,ComputeModelPicker}.stories.svelte
```

**Notes**

Final shared-disclosure consumer lane: fresh process, six story families, 31 interaction/Axe tests passed. No screenshots promoted to baselines.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final disclosure changes: zero errors/warnings for both surfaces, 303 catalog modules.
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
pnpm docs:review application "Reviewed rounded Add/Edit Advanced group and the shared disclosure focus treatment: rounded corners follow expanded state, inset keyboard outline avoids clipping, drafts and focus remain retained."; pnpm exec prettier --write docs/maintenance/coverage.json; env PATH="$PATH:/Users/stevejuma/.local/bin" pnpm docs:check
```

**Notes**

Updated final documentation fingerprint, formatted generated review metadata and rebuilt strict documentation; all 141 files and their links/anchors passed.
<!-- /run -->

<!-- run -->
### Run 8

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

A concurrent evidence write invalidated revision-pinned bookkeeping validation. Reran after file edits settled; no contract or test failure.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Final source and evidence coverage passed: 146 tasks, 667 records; spec-first 52 protected files and nine canonical chapters. M25 remains In progress.
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
pnpm exec prettier --check <all changed Markdown/JSON/TypeScript/Svelte paths excluding generator-owned history>
```

**Notes**

All matched files conform after the final disclosure change and document formatting.
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
pnpm check:ui-ownership
```

**Notes**

Final ownership validation passed, including the added harness composition and provenance.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Existing unrelated scripts/dev-ui.acceptance.mjs architecture limit violation
  (311 counted lines versus 300), reproduced in unchanged parent. No baseline or
  unrelated source change.
- Authenticated external-provider calls, real paired inference with this
  selector, signed macOS/WKWebView and Linux distribution gates remain
  unqualified.
- Preview is Vite HMR; production CSP/bundled release qualification was not
  repeated for final presentation follow-ups.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be; existing parent Design Core dependency resolution retained, no upstream/package changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive version-1 draft catalog POST and Runner providers/Catalog status. Legacy options remain accepted. No native object, WAL, manifest, AgentStore or WorkStore format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the completed provider configuration slice; independently resolve the existing development-script architecture limit and qualify real authenticated providers, paired inference and platform distributions.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated owning
  API/collaboration/UI/CLI/compatibility/mutations/architecture/documentation/provenance
  chapters; native identity and manifest-only authority unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M25 remains In progress. Source/fixture provider selection does not
  qualify authenticated services or platform distributions; M24 independent
  gates remain unchanged.

```
<!-- /evidence-field -->

