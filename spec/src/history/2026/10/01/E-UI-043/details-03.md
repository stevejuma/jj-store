---
schema: 1
id: E-UI-043
---

# E-UI-043 details 3

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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds. Log .scratch/file-editor-navigation-spec-build.log.
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
pnpm exec prettier --check catalog/components/store-application/StoreFileEditor.stories.svelte spec/src/api.md spec/src/milestones.md spec/src/ui.md spec/src/verification.md web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

All 7 changed production, catalog, regression and canonical files pass formatting.
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
pnpm exec prettier --check spec/src/milestones.md spec/src/verification.md
```

**Notes**

Appending final evidence requires normalizing the new JSON entry indentation to the Markdown formatter output; milestones formatting check fails. No product code changed.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final JSON evidence indentation normalized.
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
pnpm exec prettier --check spec/src/milestones.md spec/src/verification.md
```

**Notes**

Final canonical ledger and matrix formatting passes after normalization.
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

- Changed stories remain visual-pending; visual baseline approval is separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated UI suites; Rust builds/tests, contract generation, real-RustFS and
  shared release qualification are out of scope under the targeted UI-only
  policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing dependency setup unchanged; no sibling or package manifest edits.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

File presentation requires a nonempty selected native path before honoring a requested edit path. Directory/root transitions end edit presentation and release the existing workspace navigation guard. Explicit landing-document editing and typed edit/rename submission retain existing behavior. No controller, read DTO, native identity, object, manifest, WAL, receipt or credential format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain exact root-navigation, workspace-guard and explicit-edit regressions; visual approval remains separate.
<!-- /evidence-field -->

