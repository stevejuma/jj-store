---
schema: 1
id: E-UI-103
---

# E-UI-103 details 2

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
jj --no-pager st; jj workspace list
```

**Notes**

Started clean at b63e5cf8 above Help change 36eb7f13. All work stays in default/main workspace.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFeedback.stories.svelte
```

**Notes**

Before implementation: 7 failed, 1 passed; each failure detected an inline alert outside the notification host.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFeedback.stories.svelte
```

**Notes**

First after run: eight assertions passed but one unhandled fixture error used the wrong reviews.load signature. Corrected the fixture; not treated as acceptance.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial fixture check: wrong reviews.load arguments plus four unsupported Testing Library exact options. Fixed the fixture and assertions; production web diagnostics remained zero.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFeedback.stories.svelte
```

**Notes**

8/8 after fixture correction. Hidden recovery explicitly succeeds and fails again while Reviews is unmounted.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work catalog/components/store-application/StoreNotifications.stories.svelte
```

**Notes**

71/71 across 16 files; interaction/accessibility and notification/modal consumers. Fresh Storybook/Vite process and Chromium contexts qualify the cold lifecycle lane. .local/projects-feedback-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFeedback.stories.svelte
```

**Notes**

Added two cases: 9/10 passed; warning assertion expected alert while Design Core correctly uses status. Corrected the assertion to require a warning toast with role=status.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFeedback.stories.svelte
```

**Notes**

Final 10/10, including selected-project Tasks and incomplete Activity agent-context warnings. .local/projects-feedback-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/notification-service.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/projects-freshness.test.ts
```

**Notes**

19/19 across four files; .local/projects-feedback-unit.log.
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
pnpm web:check
```

**Notes**

Zero errors/warnings; .local/projects-feedback-web-check.log.
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
pnpm catalog:check
```

**Notes**

Final fixture diagnostics zero; 253 catalog modules pass boundaries. .local/projects-feedback-catalog-final.log.
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
pnpm check:ui-ownership
```

**Notes**

Public foundational dependency/owned Community boundary unchanged.
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
node --input-type=module (changed-file countedLines/fileKind/limits audit)
```

**Notes**

Changed source/story files stay within their limits; ProjectsHub story 397/400 and StoreProjects 290/300. The unrelated pre-existing StoreStackWorkspaceConflicts story overage is outside this slice; the global architecture lane was not rerun.
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
pnpm docs:review application 'Reviewed Projects home, reviews, agents and activity read feedback: recoverable errors use retry toasts, unsupported capabilities and validation remain contextual, and existing browsing guidance remains accurate.'
```

**Notes**

Application source fingerprint refreshed; user guides remain accurate.
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
pnpm docs:check
```

**Notes**

Nine Mermaid diagrams; 139-file self-contained Help closure and valid links/anchors. .local/projects-feedback-docs-check.log.
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
CUA live 7460 Projects hub: Reviews Retry, Activity, Projects, Tasks, Agents, Repositories, and Reviews reload
```

**Notes**

Reviews and Agents real unavailable errors appear in application toasts with retry. Repositories retains its existing metadata toast; other tabs retain rows. Reviews reload also uses a toast without an inline alert. Screenshot .local/projects-feedback-live.png. Existing HMR/backend/fixture preserved; no repository mutation.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Expected incomplete-evidence gate before this fresh record and milestone histories were written; final governance checks recorded below.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first (initial populated evidence)
```

**Notes**

Evidence test-reference parser accepts test()/fn/def declarations, not Storybook names or bare unit paths. Moved named stories to an explicit stories field and retained real unit declarations in tests; added generated milestone files to fresh coverage. No tests or validation policy weakened.
<!-- /run -->
<!-- /evidence-field -->

