---
schema: 1
id: E-CHAT-LIVE-007
---

# E-CHAT-LIVE-007 validation 1

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
cargo test -p jj-store-service --test agent_sessions session_detail_resolves_current_stable_identity_and_exact_links --locked
```

**Notes**

Before heap boxing: one failure; future is 71,232 bytes. Live crash report jj-store-2026-10-09-015739.ips supports the nested stack finding.
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
cargo test -p jj-store-service --test agent_sessions --locked
```

**Notes**

Exact repaired test and all 9 reader tests pass; cargo check -p jj-store-service --locked also passes.
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
cargo test -p jj-store-cli --test agent_execution providers::managed_catalog_does_not_start_compute_or_mutate_the_journal --locked
```

**Notes**

New regression fails because discovery creates compute preparation state.
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
cargo test -p jj-store-cli --test agent_execution providers:: --locked
```

**Notes**

Both provider tests pass after discovery stops preparing compute; 90.60 seconds including privacy preparation.
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
cargo test -p jj-store-cli --test agent_execution conversations:: --locked
```

**Notes**

Repository-free chat reuse and restart/no-replay regression passes; 125.51 seconds. Existing linker warning remains.
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
cargo check -p jj-store-server --locked
```

**Notes**

Affected server and direct dependencies compile.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts -t "dispatching retains"
```

**Notes**

Initial exact request test fails; pending was cleared before acknowledgement. Fixed controller retains it.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/conversation-projection.test.ts -t "live acknowledgement|committed comments-only"
```

**Notes**

Before fixes, both regressions fail. Intermediate test fixture allowed an overlapping showGeneral refresh; made the fixture explicit. Intermediate web check exposed two unknown payload accesses; use the owned object helper.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/conversation-projection.test.ts
```

**Notes**

Final 19 tests pass. A separate attempted invocation used nonexistent web/vitest.config.ts and failed before tests; corrected to the owned catalog configuration.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte
```

**Notes**

28 interaction/accessibility stories pass in three affected files. Separate from the prior 96-story acceptance record.
<!-- /run -->

<!-- /evidence-field -->
