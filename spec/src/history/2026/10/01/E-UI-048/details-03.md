---
schema: 1
id: E-UI-048
---

# E-UI-048 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Three direct directory/graph refinement stories pass. repository-overview-consumer-final.log.
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
pnpm dev:restart
```

**Notes**

Owned demo rebuilt and restarted with repository 1bdb89c409c1b891ac7f180c01a265d5, installation identity and RustFS volume retained; Search ready. Native builds are only the requested development restart, not Rust qualification.
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
CUA live review on isolated localhost tab
```

**Notes**

Editor: 978px pane, 8px gutter inset, 905px content, 8px right gap, zero overflow. Review header stays at y=49 while its body scrolls to 377.5px; Copy submenu exposes five complete native values. No contents edited or publication performed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Ten directly affected Overview/editor/connected-app stories pass after final directory frame fix.
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
pnpm web:build
```

**Notes**

Final production CSS builds; dependency warnings retained in repository-polish-build-final.log.
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

Final catalog assertions type-check with zero errors/warnings; 111 modules pass.
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
pnpm spec:first
```

**Notes**

18 protected files, 3 canonical chapters.
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

21 chapters, 132 requirements.
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

Evidence was temporarily marked pass with final bookkeeping listed as remaining. Removed that gap after completed checks; no product failure.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/editor-focus.spec.ts
```

**Notes**

Final fresh output after directory frame fix: seven cold scenarios pass.
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
pnpm check:progress
```

**Notes**

Final evidence checker passes for 111 tasks; no status change.
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
pnpm exec prettier --write catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte spec/src/api.md spec/src/ui.md web/src/lib/ui/community-projects/repository-files.css web/src/lib/ui/community-projects/store-file-editor.css web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/store-application.css web/src/lib/ui/store-collaboration/StoreRecordMenu.svelte web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte web/src/lib/ui/store-collaboration/community-native-review.css web/src/lib/ui/store-collaboration/record-copy-icon.ts web/tests/browser/editor-focus.spec.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts spec/src/milestones.md
```

**Notes**

Changed code, tests, chapters and ledger formatted.
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
pnpm exec prettier --check catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte spec/src/api.md spec/src/ui.md web/src/lib/ui/community-projects/repository-files.css web/src/lib/ui/community-projects/store-file-editor.css web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/store-application.css web/src/lib/ui/store-collaboration/StoreRecordMenu.svelte web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte web/src/lib/ui/store-collaboration/community-native-review.css web/src/lib/ui/store-collaboration/record-copy-icon.ts web/tests/browser/editor-focus.spec.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts spec/src/milestones.md
```

**Notes**

Final changed-file formatting passes.
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

UI-only production layouts and shared native record menus, their directly affected catalog families and three browser specs. Root aggregates, unrelated stories, Rust qualification and generated contracts are out of scope under targeted verification. No milestone statuses change.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Overview had no outer body inset; Mira centered a readable-width column; detail headers scrolled with content; native copy values were flattened in menus.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing public Design Core ScrollArea, DropdownMenu submenus and Mira file-margin/line-width tokens; no route, wire, configuration, native identity or publication changes. Revision is enclosing Jujutsu change pwxxypmm.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; locked Design Core, Svelte, Lucide, Vitest, Storybook and Playwright.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused regressions; implement separately requested tighter header insets and source syntax highlighting.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending; shared-package release and other
  milestone gates unchanged.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/repository-*.log, .scratch/repository-editor-review.jpg, .scratch/repository-review-copy.jpg and web/test-results.
<!-- /evidence-field -->

