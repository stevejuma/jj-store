---
schema: 1
id: E-UI-028
---

# E-UI-028 details 6

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
GitHub file-editor reference and existing StoreFileEditor source audit
```

**Notes**

Edit mode used a plain textarea with a Contents label, no line numbers, no filename-selected parser and a nested framed field instead of a source editor integrated with the content body.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both Svelte projects and catalog boundaries passed, while the first inline Mira integration pushed StoreFileEditor to 318 counted lines; the language-backed editor surface was extracted into its own production component.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both Svelte projects, catalog boundaries and all 260 architecture modules pass after extracting the public Design Core Mira editor surface and filename language resolver.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/components/community-projects/repository-editor-language.test.ts
```

**Notes**

All 6 focused language-selection cases pass for Markdown, TOML, Rust, Shell, TypeScript and extensionless plain text.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Four dependency-optimization attempts stopped before the story assertion on CommonJS default interop for extend, debug or highlight.js while Vite first encountered Design Core's nested Mira package. The catalog now prebundles that exact nested Mira dependency as one unit.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

The focused Chromium interaction and automatic accessibility story passes for the line-numbered Markdown-highlighted Mira editor, flush zero-border body, removed Contents label, rename, draft preview and save flow; the story remains visual-pending.
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
pnpm web:build
```

**Notes**

The production UI bundle and manifest rebuild pass with the existing Lightning CSS :global and bundle-size warnings; no Rust build ran.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first && pnpm check:progress
```

**Notes**

Spec-first required the mapped architecture chapter for the extracted editor and nested Mira optimization boundary. Progress validation rejected one parameterized test label and one Storybook story label as non-literal named tests; the source test now has a literal name and the story remains recorded as run evidence.
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
pnpm spec:first && pnpm check:progress
```

**Notes**

Spec-first passed for nine protected files and five canonical chapters. Progress validation then required the literal named regression to use its recognized test() declaration instead of it().
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
live production-CSP computed-style inspection of README.md edit mode
```

**Notes**

The rejected dynamic CodeMirror base stylesheet left the scroller in block layout, stacked the gutter above the content, collapsed the number column to ten pixels, limited content to its intrinsic width and exposed Design Core's lavender field and active-line surfaces.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

The focused Chromium interaction now asserts the flex scroller, first line and line-number alignment within one pixel, content-card background for the editor and gutter, transparent active line, zero border, language highlighting, rename, draft preview and save flow.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both Svelte projects, all 67 catalog boundaries and all 260 architecture modules pass with the CSP-resilient local editor layout and surface styles.
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
pnpm web:build
```

**Notes**

The corrected production UI bundle and manifest rebuild pass with the existing Lightning CSS :global and bundle-size warnings.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:restart
```

**Notes**

The helper unconditionally entered its embedded-RustFS and Rust binary build lanes, so it was interrupted; the preserved demo was restarted directly with the existing target/debug binaries after rebuilding only the web bundle.
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
direct existing-binary demo restart and live README.md edit-mode verification
```

**Notes**

The rebuilt UI runs at 127.0.0.1:7460 with the preserved fixture. The live gutter is 35 pixels wide, line 1 aligns within 0.2 pixels of its content line, content fills the editor width, and the editor, gutter and active line match the white content-card surface.
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
pnpm spec:first && pnpm check:progress
```

**Notes**

The protected editor styling and interaction regression retain their canonical requirement mapping and the evidence ledger passes.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
GitHub tldraw responsive repository header and current landing/editor style audit
```

**Notes**

The landing summary had no independent commit-details action, its message and fixed revision/snapshot controls competed for one row at constrained width, and the requested focused-editor and content-sized filename treatments were not yet explicit.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Both affected Svelte projects, all 67 catalog boundaries and all 260 architecture modules pass for the responsive commit summary, editor focus suppression and content-sized breadcrumb filename.
<!-- /run -->
<!-- /evidence-field -->

