---
schema: 1
id: E-UI-073
---

# E-UI-073

<!-- evidence-field:id markdown -->
## Id

E-UI-073
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

aligned repository directory description columns
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rqpmklxo
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-WORK-008
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual approval remains separate. Root aggregates and Rust lanes are out of
  scope for owned UI layout.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared repository list rows in landing and selected-project directories reserve a consistent name column and remaining description width. Narrow list rows stack copy and metadata without overflow; equal-height card layout and full-width descriptions remain. Community source inspected first and Design Core public Button second; no upstream dependency defect or new copied source.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-work/community-work-directory.css
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- spec/src/ui.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;existing Design Core registry primitives.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

CSS layout only; no routes, object/receipt encodings or storage contracts changed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue authorized top-row search and Community structural Focus sheets.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-repository-description-*.log; /tmp/jj-repository-description-aligned.png.
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
pnpm test:storybook catalog/components/store-work/ProjectsDirectory.stories.svelte -t Repository descriptions align
```

**Notes**

Initial spaced title filter skipped all 6 stories; not acceptance evidence.
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
pnpm test:storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

Two preliminary harness failures: incorrect list label Repositories and readonly New project trigger. Corrected against existing source; other 5 stories pass.
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
pnpm test:storybook catalog/components/store-work/ProjectsDirectory.stories.svelte -t RepositoryDescriptionsAlign
```

**Notes**

Final before regression: narrow row overflow 283px in269px. Mandatory List selection and exact alignment assertions preserved.
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
pnpm test:storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

6/6 after including directory/project alignment at980/620/300px, overflow, equal-height cards and retained metadata.
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
pnpm catalog:check
```

**Notes**

Unsupported Testing Library getByRole exact option removed; name strings already match exactly. No product error.
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
CUA live read-only repository directory geometry
```

**Notes**

All three description left positions equal517.7578125px. Screenshot /tmp/jj-repository-description-aligned.png.
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
pnpm check:progress
```

**Notes**

Invalid run result pending for the skipped preliminary filter command. Corrected to pass with explicit skipped/nonacceptance note; evidence overall remains pending for separate visual approval.
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
pnpm check:progress; pnpm check:ui-ownership
```

**Notes**

Final corrected ledger and repository-owned presentation pass. Visual baseline approval remains separate.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


