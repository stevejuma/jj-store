---
schema: 1
id: E-EXEC-001
---

# E-EXEC-001 details 4

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
cargo test -p jj-agent --test controller --locked controller_multiline_tool_ids_preserve_exact_identity_in_valid_aliases (before)
```

**Notes**

Invalid agent alias: installed Cursor opaque tool IDs contain newlines. Exact full identity was retained; the projection alias needed reversible escaping.
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
cargo test -p jj-agent --test controller --locked
```

**Notes**

5/5 including newline-bearing exact IDs, bounded titles, cumulative tools, stable turn identity and unknown outcomes.
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
JJ_STORE_TRANSCRIPT_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test -p jj-agent -p jj-store-capture --locked
```

**Notes**

Full directly affected domain/capture suites. The earlier uncached run was interrupted during repeated fixture rule compilation; it is not a pass.
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
cargo test -p jj-store-jj --test agent_workspaces --locked
```

**Notes**

Real custom JJ: isolated destination, dirty source preservation, verified original selection and retained cleanup revision.
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
JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test -p jj-store-cli --test agent_execution --locked lifecycle_deduplicates_permissions_publishes_and_resumes_without_prompt_replay -- --exact
```

**Notes**

Final lifecycle fixture: permissions/dedup, edits, publication, open/end settlement, incomplete-workspace refusal, explicit resume without prompt, interrupted unknown outcome and restart without replay. Exact queued cancellation usability was fixed and fully verified in jj-acp change mkmlwqzt, E-M11-003.
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
cargo test -p jj-store-http --locked (before strict contract inventory update)
```

**Notes**

New routes needed explicit inventory shapes; strict assertions were preserved.
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
cargo test -p jj-store-http --locked
```

**Notes**

31 library tests and 9 assets tests; one unrelated explicit capture test ignored.
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
pnpm contracts:generate
```

**Notes**

Generated strict HTTP OpenAPI and TypeScript models.
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
pnpm contracts:check
```

**Notes**

Generated comparison, captured response validation and seven contract tests.
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
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 JJ_STORE_AGENT_TEST_ACP=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp cargo test -p jj-store-cli --test agent_execution --locked real::installed_codex_and_cursor_edit_isolated_workspaces_and_publish_history -- --ignored --exact
```

**Notes**

Both installed adapters edited ordinary registered Git-backed JJ isolated workspaces; source dirty files remained unchanged and sanitized history published. 0 live permission requests.
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
pnpm web:check
```

**Notes**

0 errors and warnings.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (before story role-selector type correction)
```

**Notes**

Story testing-library ByRoleOptions does not accept the Playwright exact option; an anchored role-name expression preserves the intended assertion. Inherited counted-line helper extraction is separately recorded in E-EXEC-002.
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
pnpm catalog:check
```

**Notes**

0 diagnostics; 256 catalog modules.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts
```

**Notes**

3 presentation regressions for queued cancellation, configOptions and exact modes.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte (before draft and dismissal corrections)
```

**Notes**

Draft retention and permission story isolation failed. Corrected production draft ownership and explicit story teardown; assertions retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte
```

**Notes**

3/3 with retained setup/conversation drafts, exact permission options and unknown outcomes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```

**Notes**

Final directly affected interaction/accessibility lane: 10 files, 70/70. An earlier selected 10-file lane passed 69/69; neither result is an aggregate repository Storybook pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-execution-controller.test.ts catalog/controller/store-controller.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

Final 51/51 in seven directly affected files, including exact reconnect regression. Earlier selected six-file evidence passed 57/57; no aggregate catalog-suite claim.
<!-- /run -->
<!-- /evidence-field -->

