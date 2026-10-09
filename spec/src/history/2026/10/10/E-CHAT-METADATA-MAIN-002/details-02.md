---
schema: 1
id: E-CHAT-METADATA-MAIN-002
---

# Main metadata handoff validation

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
pnpm contracts:check
```

**Notes**

The landed additive contracts pass: 300 actual HTTP/SSE responses conform and seven schema tests pass. This wider lane completed after E-MAIN-001; /tmp/metadata-main-contracts.log. No public contract changed in the refinements.
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
pnpm dev:backend:build
cargo build --manifest-path runtime/local-agent/Cargo.toml --bin jj-store-local-agent --locked
PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:build
PATH="/Users/stevejuma/.local/bin:$PATH" pnpm dev:ui
jj workspace forget conversation-metadata
```

**Notes**

Original handoff builds succeeded. Existing stageAgentResources staged main-built control-version-3 helpers into ignored desktop/resources; unchanged ACP/tools/compute hashes and updated helper manifest were retained. The verified temporary checkout was then removed. Logs /tmp/metadata-main-build.log, helper-build.log, help-build.log, resource-stage.log and restart.log. Nine session IDs, repository identity and normalized settings hash were retained; /tmp/metadata-main-live-before.json and live-after.log.
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
Read-only live browser inspection
GET /v1/local/agents
GET /v1/local/agents/metadata?session=3aa8c81a8b5fbe2219498750ab014416
```

**Notes**

Initial native status took 13.31s and metadata 5.0s; overlapping browser reads timed out. Title lookup rematerialized five domains per row; metadata projection also repeated display-only feedback enrichment. Standalone reads succeeding did not qualify browser behavior. No agents were resumed or sent prompts.
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
cargo test -p jj-store-cli --test agent_execution --locked metadata_tests
```

**Notes**

The first cache revision overflowed the default test stack in metadata_stale_generation_and_unknown_publication_recover_without_new_inference. Boxed cache-pin futures fix the nesting without increasing thread stacks. A preceding new cache-test compile attempt also used the wrong ThreadSummary field shape; corrected to reference. Logs /tmp/metadata-main-refinement-metadata-tests.log and cache-test.log.
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
cargo test -p jj-store-service --test collab_pagination --locked materialization::metadata_pins_reuse_thread_data_and_select_new_titles_after_publication -- --exact
cargo test -p jj-store-service --test collab_pagination --locked materialization::scoped_materializations_reuse_unchanged_roots_and_refresh_envelopes -- --exact
cargo test -p jj-store-service --test collab_pagination --locked resumed_pages_reauthorize_current_tokens_and_never_store_credentials -- --exact
```

**Notes**

Three exact tests pass. Nine warm metadata pins cause zero domain rematerializations while authorization/manifest reads remain. New titles load from a new root; old pins stay immutable. /tmp/metadata-main-refinement-cache-test-pass.log, cache-existing.log and cache-auth.log.
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
cargo test -p jj-store-cli --test agent_execution --locked metadata_tests
```

**Notes**

All four metadata publication/recovery tests pass on default stacks in 157.63s after boxing. /tmp/metadata-main-refinement-metadata-tests-final.log. Real controlled local companion publication remains distinct from authenticated providers.
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
cargo test -p jj-store-service --test agent_sessions --locked
cargo check -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --locked
```

**Notes**

Nine captured session tests pass, including exact source equivalence and bounded captured-detail future size. Existing browser detail keeps enrichment. All-target boundary checks recorded in /tmp/metadata-main-refinement-capture-check.log; tests in captured-detail.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentRuntimeSettings.stories.svelte
```

**Notes**

One geometry assertion used a header title short enough to fit; enlarged the representative fixture without weakening bounds. One inbox import failed during the observed Vite optimizer reload. AgentRuntimeSettings is not a story file and was replaced by the existing settings families. /tmp/metadata-main-refinement-stories.log.
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
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

All 27 directly affected interactions/accessibility stories pass with closed defaults, retained drafts and constrained title geometry. /tmp/metadata-main-refinement-stories-closed.log. Intermediate expanded-default passes do not substitute for this final run.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown
```

```shell
pnpm catalog:check
```

**Notes**

Nine story type diagnostics came from a Playwright-only exact option passed to Testing Library. Removed that unsupported option; exact string names keep the same matching behavior. /tmp/metadata-main-refinement-catalog-complete.log.
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
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte
pnpm catalog:test catalog/controller/conversation-metadata.test.ts
```

**Notes**

Catalog recheck has zero errors/warnings and 356 owned modules. The five metadata stories pass after the type correction; five metadata state tests pass. /tmp/metadata-main-refinement-catalog-qualified.log, metadata-stories-qualified.log and state.log.
<!-- /run -->

<!-- /evidence-field -->
