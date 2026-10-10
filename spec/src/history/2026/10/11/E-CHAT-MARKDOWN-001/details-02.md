---
schema: 1
id: E-CHAT-MARKDOWN-001
---

# E-CHAT-MARKDOWN-001 details 2

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
pnpm build (cwd ../mira-mde)
```

**Notes**

8/8 tasks; upstream clean source built before diagnosing app decoration integration.
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
pnpm pack --pack-destination .local/packages (cwd ../mira-mde)
pnpm install --no-frozen-lockfile
jj restore --from @- pnpm-lock.yaml
```

**Notes**

Latest local built Mira archive used via optional pnpm hook; registry-portable lockfile retained.
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
node --test scripts/local-package-pnpmfile.test.mjs
```

**Notes**

3/3 local archive substitution tests.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/markdown-reading.test.ts
```

**Notes**

Before implementation, hyphen bullet and file-link regressions failed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/file-links.test.ts catalog/fixtures/delta-chat/markdown-reading.test.ts
```

**Notes**

4/4 after repair; Markdown-only rerun also 2/2.
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
cargo test -p jj-store-capture --test transcript_sanitization streaming_paths_are_private_at_every_chunk_boundary --locked
```

**Notes**

Before implementation, split chunks retained /Users/synthetic/project rather than $WORKSPACE.
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
cargo check -p jj-store-capture -p jj-store-server --locked
```

**Notes**

Initial shared sanitizer and legacy runtime compiled. Later fleet extraction briefly failed on a stale stream.tools reference; repaired to self.tools before final builds.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=$PWD/.local/dev-demo/local/agent-execution/privacy-cache cargo test -p jj-store-capture --test transcript_sanitization --locked
```

**Notes**

Test setup correctly refused an in-checkout cache. Policy retained; copied only compiled .vscdb files to /tmp for the final run.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=$PWD/.local/dev-demo/local/agent-execution/privacy-cache cargo test -p jj-store-server --lib agent_execution::runtime_privacy --locked
```

**Notes**

Same deliberate outside-checkout policy; rerun below uses private temporary cache.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-markdown-privacy-cache cargo test -p jj-store-capture --test transcript_sanitization --locked
```

**Notes**

6 passed, 1 fixture helper ignored in parent lane; helper spawned by the privacy test also passed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-markdown-privacy-cache cargo test -p jj-store-server --lib agent_execution::runtime_privacy --locked
```

**Notes**

2/2: ordered terminal flush and distinct message/tool boundaries.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-markdown-privacy-cache cargo test -p jj-store-server --lib agent_execution::conversation_stream --locked
```

**Notes**

1/1: interleaved fleet sessions normalize independently and retain terminal assistant text.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/MarkdownReading.stories.svelte
```

**Notes**

Mira surface defaults introduced reader padding; existing prompt/avatar alignment regression exposed it. Configured supported preview padding/margin/width variables.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/MarkdownReading.stories.svelte
```

**Notes**

24/24 after zero-inset preview configuration; no visual baseline approval.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte
```

**Notes**

3/3, including alert icon/theme, Retry geometry in a trailing column centered on the parent, and recovery without agent dispatch.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test
```

**Notes**

Escalated because optional dependency archive resolution affects the shared UI test surface. 947 passed, 2 timeouts, 1 skipped. local_preparation_preserves_the_failed_query_pin_and_rejects_mismatched_task_status; storage retains only bounded opaque keys and boolean flags.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/disclosure-layout.test.ts
```

**Notes**

Unchanged isolated rerun of the two timed-out files: 31/31; see saved /tmp/jj-store-markdown-timeouts-rerun.log for actual filenames.
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
catalog-unit focused adapter/comment rerun (original file arguments not retained across summary)
```

**Notes**

Earlier focused adapter/comment regression rerun: 18/18 in 4 files; result retained in /tmp/jj-store-markdown-units-final.log. Exact original file arguments were lost across context compaction, so no inferred filenames are claimed.
<!-- /run -->
<!-- /evidence-field -->

