---
schema: 1
id: E-UI-103
---

# E-UI-103 details 3

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
pnpm spec:validate
```

**Notes**

Seven validators, 3023 chapters and 175 requirements; .local/projects-feedback-spec-validate.log.
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
pnpm check:progress
```

**Notes**

143 tasks, 620 evidence records; evidence bookkeeping only.
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
pnpm spec:first
```

**Notes**

11 protected files, four canonical chapters.
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
pnpm spec:validate
```

**Notes**

Final seven validators, 3023 chapters, 175 requirements.
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
node --input-type=module (Prettier check on jj diff paths, plus changed-file architecture counts)
```

**Notes**

All changed files formatted; final feedback story 184/400, fixture 162/300, ProjectsHub story 397/400 and StoreProjects 290/300.
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

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; existing Design Core notification dependency.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Session-only condition notifications and stable per-domain/home keys. Existing authorization, independent snapshot/cursor reads, command identities and public API unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only six-tab Projects hub and selected-project Tasks read feedback. Existing unsupported Empty explanations, contextual access/scope notices, form validation, exact mutation recovery and drafts remain. Shared hidden-tab recovery follows the application/controller lifetime. Rust/root aggregate/release/package/visual-baseline lanes are out of scope under the targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused regression gates. M11.4 shared-package qualification/visual review and M4.31 released Design Core acceptance remain pending.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: UI notification and Projects hub topics require toast placement,
  explicit retry and hidden recovery; API owner confirms unchanged
  reads/authorization; documentation owner covers the refreshed source review.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M11.4 and M4.31 remain In progress. Focused UI acceptance is complete;
  it does not satisfy pending package qualification or
  visual/released-dependency gates.

```
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- catalog/components/store-work/ProjectsFeedback.stories.svelte#Projects
  failures use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Tasks failures
  use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Reviews failures
  use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Agents failures
  use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Agent scope
  failures use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Activity
  failures use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Repository
  failures use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Dismissal
  survives retry and tabs until hidden recovery
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Selected project
  task failures use retry toasts
- catalog/components/store-work/ProjectsFeedback.stories.svelte#Activity
  association failures use warning toasts
- catalog/components/store-work/ProjectsHub.stories.svelte#Review load failure
  retains retry
- catalog/components/store-work/ProjectsHub.stories.svelte#Failed home retains
  successful results
- catalog/components/store-work/ProjectsHub.stories.svelte#Unsupported reviews
  empty state
- catalog/components/store-application/StoreNotifications.stories.svelte#Dismissal
  recovery and navigation
<!-- /evidence-field -->

