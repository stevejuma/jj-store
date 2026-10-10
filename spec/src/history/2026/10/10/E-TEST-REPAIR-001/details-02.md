---
schema: 1
id: E-TEST-REPAIR-001
---

# E-TEST-REPAIR-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/production-store-mutations.test.ts web/tests/production-store-search.test.ts
```

**Notes**

Baseline: timestamp assertion plus two Node modules unable to load TanStack HydrationBoundary.svelte.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/conversation-create.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

Baseline: 3 passed, 5 stale expectations failed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Baseline: 9 passed, 5 failed. Exact width float, lazy avatar/rail readiness, operation-status and header geometry.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

Baseline: 12 passed, 8 failed across four families.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Baseline: 10 deeper legacy reply cases failed; untouched in completed quick slice.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/conversation-ordering.test.ts web/tests/production-store-mutations.test.ts web/tests/production-store-search.test.ts
```

**Notes**

Intermediate: 19 passed; new timestamp regression contained an off-by-one expected offset. Corrected to actual literal UTF-16 length, not production range logic.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/conversation-create.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

Intermediate: new directory spy used obsolete method name threads. Corrected to transport.list.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/conversation-ordering.test.ts web/tests/production-store-mutations.test.ts web/tests/production-store-search.test.ts
```

**Notes**

20 passed, including invalid observation and per-segment regressions.
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
pnpm catalog:test web/tests/vite/conversation-create.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

10 passed, preserving authorization and unknown receipt recovery.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Intermediate: 12 passed; isolated obsolete inline operation feedback and compact table specificity.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Intermediate: 2 passed; fixture supplies no binary classification, so binary-specific empty copy was stale.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Intermediate: 19 passed; all four production Stacks flows pass after explicit route fix; retained selection, destination label and unavailable copy repairs remained.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Temporary Classic-only fixture experiment: all 10 still failed. Experiment reverted; changing the default did not restore the removed reply-panel surface.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
```

**Notes**

16 passed in complete Application, Forms and Presentation families.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Intermediate: 13 passed; cold Files import exceeded default one-second semantic find wait. Its shared family helper now uses a bounded ten-second wait at this import boundary only.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Final: 14 passed in complete Spacer, Setup, Identity and RepositoryReview families. Fresh Vitest browser process; geometry, interaction and Axe checks retained.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors and warnings; 370 catalog modules satisfy boundaries.
<!-- /run -->
<!-- /evidence-field -->

