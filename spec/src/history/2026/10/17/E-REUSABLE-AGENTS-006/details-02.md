---
schema: 1
id: E-REUSABLE-AGENTS-006
---

# E-REUSABLE-AGENTS-006 details 2

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
pnpm exec tsx --test web/tests/agent-mentions.test.ts
```

**Notes**

Before extraction:3pass. After extraction:3pass, exact ID and audience preference behavior unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/queued-agent-messages.test.ts
```

**Notes**

Before and after:2files/22pass; exact queue/unknown recipient behavior preserved.
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
pnpm client:test
```

**Notes**

8independent runtime tests and public type fixture pass.
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
node --test scripts/conversation-client.test.mjs
```

**Notes**

Initial archive fixture used CJS resolution against ESM-only public exports and rejected the archive. Changed fixture to import.meta.resolve with canonical macOS temp paths; package exports unchanged.
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
node --test scripts/conversation-client.test.mjs
```

**Notes**

Final:3pass, actual independent archive runtime/types and cold ordinary workspace install. Final digest recorded above.
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
pnpm test:governance
```

**Notes**

80pass, including independent SDK acceptance and shared package size boundary.
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
node --test scripts/ci*.test.mjs scripts/docs.test.mjs scripts/spec/*.test.mjs
```

**Notes**

Final explicit selection:86CI/docs/spec regression cases pass. New package source selects web-only; root shared inputs stay conservative.
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

0errors/0warnings.
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

0errors/0warnings;472module boundaries pass.
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
pnpm web:test
```

**Notes**

180pass.
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
pnpm catalog:test
```

**Notes**

189files;1206pass/1existing skipped.
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
pnpm build-storybook
```

**Notes**

Cold static build passes.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Only3inherited size failures, reproduced from parent: StoreStackWorkspaceApplication.stories.svelte401/400, StoreApplicationMain.svelte313/300, StoreStackWorkspace.svelte310/300. Limits and unrelated code unchanged.
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
pnpm check:ui-ownership
```

**Notes**

Pass after accurate Community design reference and retained app adapter attribution. SDK moves no Community UI or Nostr implementation.
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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Actual changed authored-file inventory from jj diff is checked by Prettier. Initial check found7formatting issues; formatted exactly those files and final check passes.
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
node scripts/docs.mjs render
```

**Notes**

Final render-only strict manual passes after stale remaining-client text was corrected:144files/26679329bytes; valid links/anchors and9offline diagrams in both themes. No Rust compilation for this UI/client slice.
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
pnpm test:storybook
```

**Notes**

Cold aggregate attempt produced7passing stories and1failing ClosedDetailsStaysAbsentAfterRepositorySettings assertion, then made no further progress with an active renderer. Interrupted the owned test process with SIGINT after more than4minutes (exit130). No full-suite pass claimed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte --reporter=verbose
```

**Notes**

Clean parent165041c2 in a disposable JJ workspace reproduces the same assertion:7pass/1fail. Initial external-node_modules symlink could not serve its setup module; a local copy-on-write dependency tree then ran the real stories. Comparison workspace/dependencies retired after clean status; no production sources or live services changed.
<!-- /run -->
<!-- /evidence-field -->

