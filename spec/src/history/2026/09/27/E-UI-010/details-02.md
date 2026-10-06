---
schema: 1
id: E-UI-010
---

# E-UI-010 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Nine focused interaction/accessibility stories pass with five-character prefixes, two-dot separators and eight-character expanded prefixes.
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
pnpm catalog:test
```

**Notes**

All 23 catalog unit tests pass including updated abbreviation boundary/invalid-input tests.
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
pnpm catalog:test:browser
```

**Notes**

Fresh static build and cold_catalog_contract pass. Bookmark overlap is resolved by wrapping at the graph metadata basis. Native pointer/touch copying, independent feedback, constant outer width, unclipped suffixes and reduced motion pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All normal checks pass, including contracts, architecture, Svelte diagnostics and locked Rust clippy.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

42 of 44 stories pass; OpLog and ShellAndGraphAlignment encounter multiple status roles because the new clipboard live region also rendered inside legacy operation-ID controls.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte -t 'OpLog|ShellAndGraphAlignment'
```

**Notes**

Both exact regressions pass after scoping the clipboard live region to the Git/JJ control; legacy operation-ID presentation stays unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Full suite passes after the Op Log correction: 23 catalog unit and 44 interaction/accessibility stories, cold native-pointer/touch catalog acceptance, web/governance/contracts/storage harness and full Rust workspace. This run used the intermediate five-character labels; final four-character at-rest refinement is verified below.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rust and repository-wide Prettier formatting pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Final requested display: all nine focused stories pass with only four characters at rest and prefix/dots/suffix revealed by hover or keyboard focus.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Nine stories pass after reclaiming reserved width: four characters at rest, six-character expanded prefix with two dots and suffix, and compact independent check/warning feedback.
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
pnpm catalog:test:browser
```

**Notes**

Intermediate reclaimed-width control passed cold pointer/touch acceptance.
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
pnpm catalog:test:browser
```

**Notes**

After making the inactive half chip-only, the geometry helper waited for an absent code element. The helper now inspects all visible chip, identifier and feedback boxes, and preserves minimum trailing padding assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

All nine final stories pass with chip, tick and Copied feedback in the expanded half, four-character resting prefixes and five-character expanded prefixes with dots/suffix.
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
pnpm catalog:test:browser
```

**Notes**

Fresh build and cold native-pointer/touch acceptance pass: default width &lt;=122px, height &lt;=27px, constant outer width, and at least 4px measured edge clearance for every visible chip/text/feedback box.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Final component has zero Svelte errors/warnings and passes the 110-module architecture boundary.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Latest single-identifier refinement passes all nine focused interaction/accessibility stories. Git-only and JJ-only now assert the five-character prefix, two dots and two-character suffix immediately at rest, before interaction; paired controls retain four-character resting labels.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Final source passes all normal checks: 11-package boundaries, 110-module architecture, contracts, progress, rustfmt, zero web/catalog Svelte errors or warnings, and locked workspace clippy.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Final source passes all 23 catalog unit tests, 44 Storybook interaction/accessibility stories, fresh-build cold_catalog_contract pointer/touch/geometry acceptance, web/governance/contracts/storage-harness tests, and the complete Rust workspace. Environment-gated ignored RustFS tests were not executed; no new live-service claim is made.
<!-- /run -->
<!-- /evidence-field -->

