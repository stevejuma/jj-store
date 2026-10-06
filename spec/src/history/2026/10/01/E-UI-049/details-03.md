---
schema: 1
id: E-UI-049
---

# E-UI-049 details 3

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
pnpm exec prettier --write catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/source-highlight-assertions.ts catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte spec/src/api.md spec/src/ui.md web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte web/src/lib/ui/community-projects/StoreFileEditor.svelte web/src/lib/ui/community-projects/repository-files.css web/src/lib/ui/community-projects/repository-log-diff.css web/src/lib/ui/store-application/store-application.css web/src/lib/ui/store-collaboration/community-native-review.css web/src/lib/ui/store-collaboration/store-collaboration.css web/src/lib/ui/store-conflicts/store-conflicts.css web/src/lib/ui/store-settings/StoreSettings.css web/src/lib/ui/store-work/community-work-layout.css web/src/lib/ui/store-work/community-work-tasks.css web/src/style.css web/tests/browser/editor-focus.spec.ts
```

**Notes**

Initial changed-file formatting. Subsequent source-helper, story, catalog theme and mobile test edits were individually formatted before tests.
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
cua_repl: cold rebuilt demo reload; recorded README Source; Edit/draft/Source/Cancel; header geometry and Settings navigation
```

**Notes**

Recorded and draft Markdown headings rgb(91,33,182), distinct from foreground lab(2.65138 5.82402 -12.6171), with zero injected CodeBlock styles after the source fix. Test draft cancelled without publication. Repository tabs, section header and breadcrumb use 8px leading padding; Settings Back inset 8px and title remains on the right. Ignored source-highlight-final.jpg, source-highlight-draft-final.jpg and settings-spacing-final.jpg.
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
pnpm exec prettier --write .storybook/preview.css catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/column-canvas-story-assertions.ts catalog/components/store-application/source-highlight-assertions.ts catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte spec/src/api.md spec/src/milestones.md spec/src/ui.md web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte web/src/lib/ui/community-projects/StoreFileEditor.svelte web/src/lib/ui/community-projects/repository-files.css web/src/lib/ui/community-projects/repository-log-diff.css web/src/lib/ui/store-application/store-application.css web/src/lib/ui/store-collaboration/community-native-review.css web/src/lib/ui/store-collaboration/store-collaboration.css web/src/lib/ui/store-conflicts/store-conflicts.css web/src/lib/ui/store-settings/StoreSettings.css web/src/lib/ui/store-work/community-work-layout.css web/src/lib/ui/store-work/community-work-tasks.css web/src/style.css web/tests/browser/editor-focus.spec.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

All final changed files and evidence formatted.
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
pnpm spec:first
```

**Notes**

27 protected files and 3 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters, 132 requirements pass.
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
pnpm check:progress
```

**Notes**

111 tasks; evidence bookkeeping passes. Milestone status unchanged.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
cwd: ../design-core
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --project storybook src/shared/shadcn/code-block/CodeBlock.stories.svelte
```

**Notes**

Owning-source baseline: 8 pass, 1 fail after injected styles are disabled.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
cwd: ../design-core
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --project storybook src/shared/shadcn/code-block/CodeBlock.stories.svelte src/shared/diff/file-diff/FileDiff.stories.svelte src/shared/diff/merge-editor/MergeEditor.stories.svelte
```

**Notes**

Source static CSS fix: all 32 direct stories pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
cwd: ../design-core
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks
```

**Notes**

Initial source gate stops at three existing DC-AI-004 prose diagnostics; condense those two bullets without changing behavior.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
cwd: ../design-core
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks
```

**Notes**

Second source run: governance and 715 units pass, existing verification-table formatting stops the aggregate. Update its evidence row and format table.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
cwd: ../design-core
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks
```

**Notes**

Final source-required checks pass: governance/format/types, 715 units, 3 Turbo tests, static Storybook build and metadata audit, 639 stories, 14 Workspace/11 Shell/3 AI/35 shadcn browser tests; one existing shadcn skip. Durable source evidence records artifacts.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
cwd: ../design-core
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Final source canonical gate passes all three lanes.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
cwd: ../design-core
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
STORYBOOK_PORT=9040 pnpm test:visual:affected
```

**Notes**

Isolated Docker build passes, then cache publication fails with EACCES under /visual-delta/canonical-build-cache/entries before image comparison. A dirty sibling Visual Delta global input selects full-suite fallback; preserve that work. Visual comparison is non-blocking under parent guidance, no baseline changed and no pixel acceptance claimed.
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
pnpm exec prettier --check .storybook/preview.css catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/column-canvas-story-assertions.ts catalog/components/store-application/source-highlight-assertions.ts catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte spec/src/api.md spec/src/milestones.md spec/src/ui.md web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte web/src/lib/ui/community-projects/StoreFileEditor.svelte web/src/lib/ui/community-projects/repository-files.css web/src/lib/ui/community-projects/repository-log-diff.css web/src/lib/ui/store-application/store-application.css web/src/lib/ui/store-collaboration/community-native-review.css web/src/lib/ui/store-collaboration/store-collaboration.css web/src/lib/ui/store-conflicts/store-conflicts.css web/src/lib/ui/store-settings/StoreSettings.css web/src/lib/ui/store-work/community-work-layout.css web/src/lib/ui/store-work/community-work-tasks.css web/src/style.css web/tests/browser/editor-focus.spec.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Final complete changed-file formatting passes.
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
pnpm spec:first
```

**Notes**

Final passing evidence: 27 protected files, 3 canonical chapters.
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
pnpm check:progress
```

**Notes**

Final passing evidence: 111 tasks, no milestone status change.
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

<!-- evidence-field:scope markdown -->
## Scope

UI-only header spacing, recorded/draft source presentation and their complete direct consumers. Source Design Core owns the static CSS fix and its separate required verification; no public prop/schema, Rust, WAL, native authority or persistence change. jj-store root aggregates, unrelated UI and Rust qualification are out of scope under the targeted policy. No milestone status changes.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Headers retain 12-16px leading padding/margins. Repository Source supplies the Rust-only tokenizer for every language, suppressing built-in highlighting. Span styles still depend on runtime injection blocked by strict production CSP.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing public CodeBlock tokenizer/highlightMode and Column Canvas padding tokens. Filename language detection and source/draft bytes unchanged; unknown languages stay plain text. No security-policy relaxation. Revision is enclosing Jujutsu change omyyllum; source dependency fix is enclosing Design Core change ummolzql. Verified source dependency committed as c433fa75 (Design Core change ummolzql).
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Design Core source package 0.1.2 via documented local-package hook, released range and registry lockfile unchanged; locked Svelte/Lucide/Storybook/Vitest/Playwright.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review compact headers and highlighted recorded Source in the owned local demo.
<!-- /evidence-field -->

