---
schema: 1
id: E-CHAT-LIVE-005
---

# E-CHAT-LIVE-005 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChat*.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-appearance/*.stories.svelte (earlier run)
```

**Notes**

95/96 passed; the old lifecycle article selector did not match the new explicit answer textbox. Updated to the answer reader and retained content assertion.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChat*.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-appearance/*.stories.svelte (first continuation run)
```

**Notes**

94/96 passed; gutter and remounted fold assertions expected offscreen virtualized lines. Tests now scroll their exact line before asserting.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'CapturedLineNumberGutter|LargeCodeAndDiffMiddleFolding'
```

**Notes**

2 passed, 19 outside this exact regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChat*.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-appearance/*.stories.svelte
```

**Notes**

96 passed across 18 files including Axe. Full directly affected family required because shared composer, projection, thread controller and fixtures changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-*.test.ts web/tests/vite/agent-chat.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/appearance.test.ts web/tests/vite/thread-*.test.ts web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/local-inbox-threads.test.ts
```

**Notes**

85 passed in 15 files: exact anchors, stale/unavailable sources, partial and unknown delivery, duplicate protection, scope/parent isolation, draft retention, historical revision semantics, provider and thread consumers.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-reactions.test.ts
```

**Notes**

1 passed: answer-fragment reactions use the original capture row and owning repository reader.
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
pnpm exec tsx --test web/tests/conversation-*.test.ts
```

**Notes**

6 passed: source maps, Unicode, stream ordering and merged source identity.
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
pnpm web:check
```

**Notes**

Final 0 errors, 0 warnings. Initial extraction and emoji argument type errors corrected.
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
pnpm catalog:check
```

**Notes**

Final 0 errors, 0 warnings; 344-module catalog boundary passed. Earlier fixture command shapes and forbidden API type import corrected.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership (first final audit)
```

**Notes**

Extracted composer.css required its original Community component provenance entry.
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
pnpm check:ui-ownership
```

**Notes**

Passed after recording the inherited source path, revision and license; no external Community dependency.
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
node scripts/contracts.mjs --check
```

**Notes**

Generated public schema and TypeScript current.
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
pnpm test:contracts
```

**Notes**

7 tests plus TypeScript contract consumer check.
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
pnpm check:architecture
```

**Notes**

Changed oversized modules were split. Only scripts/dev-ui.acceptance.mjs remains at 311 counted lines; jj file show at @- and e26a177a independently confirms the same pre-existing 311. No limit/baseline was changed.
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
pnpm web:build (initial attempts)
```

**Notes**

Vite succeeded. Docs first required regenerated API pages, then source review refresh, then uv was missing from PATH. Chunk-size and mixed Tauri import warnings remain.
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
pnpm docs:generate
```

**Notes**

Generated API reference reflects committed durable contracts.
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
pnpm docs:review application "Document and Classic share durable conversation adapters, session drafts, linked agent controls, reactions, exact navigation and advertised runtime configuration; settings and architecture pages describe ownership and limitations."
```

**Notes**

Reviewed source and updated collaboration, settings, architecture, publication and transport pages; earlier relevant backend topics also reviewed.
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
PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/docs.mjs stage
```

**Notes**

Existing uv found; locked 32-package local environment; strict documentation build and 141-file link/anchor closure passed. Paired with the successful Vite/manifest phases of web:build.
<!-- /run -->
<!-- /evidence-field -->

