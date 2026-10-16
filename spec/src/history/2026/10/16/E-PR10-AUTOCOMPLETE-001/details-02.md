---
schema: 1
id: E-PR10-AUTOCOMPLETE-001
---

# E-PR10-AUTOCOMPLETE-001 details 2

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
pnpm test:storybook --shard=1/3 --reporter=default --reporter=junit --outputFile=.release/ci-results/storybook.xml
```

**Notes**

Hosted jj-store run 38923026289, job 117054034068, head 08dccc761720f85dcfe23d810f3654cead032de5. StoreWorkflows/CreateDraftSwitchesModesAndEntersRunnerChips expected linux and received inux. Other shards and all non-Storybook standard checks passed; 1,021 of 1,022 cases passed and required correctly failed.
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
pnpm exec vitest run --project storybook src/shared/forms/autocomplete-input/AutocompleteInput.stories.svelte --reporter=default
```

**Notes**

design-core-autocomplete-before-98: exit 1, 4.15 seconds. Original five cases pass; new deterministic typing regression fails: linux becomes inux.
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
pnpm exec vitest run --project storybook src/shared/forms/autocomplete-input/AutocompleteInput.stories.svelte --reporter=default
```

**Notes**

design-core-autocomplete-before-caret-98: exit 1, 8.08 seconds. Five original cases pass; both typing and keyboard-caret regressions fail.
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
pnpm exec vitest run --project storybook src/shared/forms/autocomplete-input/AutocompleteInput.stories.svelte src/shared/forms/autocomplete-input/AutocompleteInput.variations.stories.svelte src/shared/forms/chip-autocomplete/ChipAutocomplete.stories.svelte --reporter=default
```

**Notes**

design-core-autocomplete-after-98: exit 0, 4.27 seconds. All 11 autocomplete/chip interaction and accessibility cases pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks
```

**Notes**

design-core-checks-98: exit 1, 47.738 seconds. Initial aggregate correctly failed one new story callback return type. Fixed the story to return void; replacement full aggregate is running.
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
pnpm package:local
```

**Notes**

design-core-pack-98: exit 0. Prepared artifact bde5301be4c9e8939310ac97c394a4e6819737a4c59d1a48775468ee248d1f02; local qualification only, no publication.
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
pnpm web:check
```

**Notes**

packed-consumer-web-check-98: exit 0. Temporary consumer with the packed fix; 0 errors/warnings.
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
pnpm catalog:check
```

**Notes**

packed-consumer-catalog-check-98: exit 0. Temporary consumer with the packed fix; 0 errors/warnings, 466 modules.
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
pnpm --dir .local/jj-store-packed-consumer-98 web:test
```

**Notes**

packed-consumer-web-units-98: exit 0, 3.04 seconds. 
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=/Users/stevejuma/code/lapismd/jj-store/.scratch/qualification-working-tree.json pnpm --dir .local/jj-store-packed-consumer-98 catalog:test
```

**Notes**

packed-consumer-catalog-units-98: exit 0, 23.55 seconds. 
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
pnpm checks
```

**Notes**

design-core-checks-fixed-98: exit 0. Full source-owner aggregate: 746 units, 762 stories, 15 workspace + 11 shell + 3 AI + 89 shadcn browser cases; one declared platform skip. Types, specification, formatting, prepared-source consumers, static build and metadata audit pass. No visual baselines updated.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte --reporter=default --reporter=junit --outputFile=/Users/stevejuma/code/lapismd/jj-store/.local/main-qualification-2026-10-16/packed-consumer-workflows-98.xml
```

**Notes**

packed-consumer-workflows-98: exit 0, 40.2 seconds. Temporary packed consumer: all 10 original workflow stories passed, including the hosted failing case. Active checkout remains on released 0.5.0.
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
pnpm spec:check
```

**Notes**

design-core-final-docs-98: exit 0. Source verification record and canonical review pass validation, mdBook and spec-first.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm --dir .local/jj-store-packed-consumer-98 test:storybook --shard=1/3 --reporter=default --reporter=junit --outputFile=/Users/stevejuma/code/lapismd/jj-store/.local/main-qualification-2026-10-16/packed-consumer-storybook-1-99.xml
```

**Notes**

packed-consumer-storybook-1-99: exit 0, 330.2 seconds. 
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm --dir .local/jj-store-packed-consumer-98 test:storybook --shard=2/3 --reporter=default --reporter=junit --outputFile=/Users/stevejuma/code/lapismd/jj-store/.local/main-qualification-2026-10-16/packed-consumer-storybook-2-99.xml
```

**Notes**

packed-consumer-storybook-2-99: exit 0, 327.54 seconds. 
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
gh run view 38928902165 --repo lapismd/design-core --json status,conclusion,jobs
```

**Notes**

design-core-source-ci-99: exit 0. All 11 protected source PR jobs pass; PR #29 squash merged as 41efbc9b68652c03ad92d15353e8c13b4f827492 without bypass.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm --dir .local/jj-store-packed-consumer-98 test:storybook --shard=3/3 --reporter=default --reporter=junit --outputFile=/Users/stevejuma/code/lapismd/jj-store/.local/main-qualification-2026-10-16/packed-consumer-storybook-3-99.xml
```

**Notes**

packed-consumer-storybook-3-99: exit 0, 356.87 seconds. 
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
python3 .local/main-qualification-2026-10-16/packed-consumer-inventory-99.py
```

**Notes**

packed-consumer-inventory-99: exit 0. Exact hosted inventory: 1,022 cases / 211 files, each once, no failures/errors/skips.
<!-- /run -->
<!-- /evidence-field -->

