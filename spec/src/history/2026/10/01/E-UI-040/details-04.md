---
schema: 1
id: E-UI-040
---

# E-UI-040 details 4

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

Final badge-width refinement: zero errors/warnings and 75-module boundary audit passes. Log .scratch/commit-picker-badge-width-catalog-check.log.
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
pnpm check:architecture
```

**Notes**

Final badge-width refinement: 291-module architecture audit passes.
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
pnpm web:build
```

**Notes**

Final badge-width refinement: fresh build passes with existing nonfatal minification/bundle-size warnings. Log .scratch/commit-picker-badge-width-build.log.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Final built demo ready at 127.0.0.1:7460; PID 49017; existing explicit binaries and data preserved.
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
CUA live built-UI verification
```

**Notes**

Final hidden tab 11: all 12 loaded options have 24px GIT and JJ badges and zero identifier-code left-edge difference. Title/time/chips and stacked Git/JJ remain visible in menu and button. Screenshot .scratch/commit-picker-final.png saved; tab closed and user tabs preserved. No mutation preview/apply.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final ledger formatted.
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
pnpm exec prettier --check web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/ui.md spec/src/api.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All six final changed files pass formatting.
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
pnpm spec:validate
```

**Notes**

Final layout: 4 validators, 18 chapters and 107 requirements pass.
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
pnpm spec:first
```

**Notes**

Final layout: 2 protected files and 3 canonical chapters pass same-change gate.
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
pnpm check:progress
```

**Notes**

Final layout: 80 tasks pass evidence bookkeeping audit.
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
pnpm spec:build
```

**Notes**

Final canonical mdBook HTML builds.
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

<!-- evidence-field:deferred list -->
## Deferred

- Changed stories remain visual-pending; human baseline approval is separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated UI suites; Rust, contract generation and real-RustFS qualification
  are out of scope under targeted UI-only policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing dependency resolution unchanged; no sibling or package manifest edits.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation only. Selectable commit rows put relative time and bookmark/workspace chips together directly below title and stack Git above JJ in a fixed-width column, reserving a missing JJ row. GIT/JJ badges share a 1.5rem width to align identifier text; existing 10px Community chip token and oval indicator shape retained. Fixed copyable targets retain their metadata placement. Full bound native IDs, search, copy controls, mutation commands, HTTP/native object/manifest/WAL/receipt formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused picker and direct mutation-form acceptance; visual baseline approval remains separate.
<!-- /evidence-field -->

