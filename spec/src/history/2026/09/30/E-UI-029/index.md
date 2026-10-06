---
schema: 1
id: E-UI-029
---

# E-UI-029

<!-- evidence-field:id markdown -->
## Id

E-UI-029
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

merged-file-preview-story-assertion-repair
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- spec/src/ui.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/reads.spec.ts#native_graphs_files_and_source_rendering_are_accessible
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
pnpm test
```

**Notes**

The first merged aggregate run passed 87 of 88 Storybook interactions; RepositoryFiles still expected the file path inside the source Code Block.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t RepositoryFiles
```

**Notes**

The exact case reproduced the stale Code Block text assertion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t RepositoryFiles
```

**Notes**

The story now verifies the selected path in the labelled preview header while retaining native Rust language and source-token checks.
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
pnpm test:storybook
```

**Notes**

A first full rerun passed RepositoryFiles but observed an unrelated transient CommandSearchNavigation active-tab timing failure; its isolated retry passed without code changes.
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
pnpm test:storybook
```

**Notes**

The unchanged search story and repaired file story pass with all 88 interactions across 22 Storybook files.
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
pnpm test
```

**Notes**

The complete repository test lane passes after the story repair, including cold catalog browser and Rust workspace acceptance.
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
pnpm build
```

**Notes**

The complete repository build lane passes with the repaired story.
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
pnpm spec:check
```

**Notes**

All 18 specification lanes pass with the repaired file story and its canonical UI chapter.
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

<!-- evidence-field:versions markdown -->
## Versions

pnpm 10.32.1; Storybook 10.5.7; Vitest browser 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product API, persistence, wire format or visual baseline change; this repairs the browser-story assertion for the existing file preview layout.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


