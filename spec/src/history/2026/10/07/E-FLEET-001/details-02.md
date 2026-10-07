---
schema: 1
id: E-FLEET-001
---

# E-FLEET-001 details 2

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
cargo check --locked -p jj-store-server -p jj-store-http
cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

Initial compile errors: incorrect WAL imports, moved lock guard, service reexports and native socket fixture field. Personal-home failure identified shared receipt request identity between folder registration and WorkStore project creation; separate stable identities repaired it. The next fixture failure compared /var with canonical /private/var. No prompt was sent during provisioning failure.
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
cargo check --locked -p jj-store-server -p jj-store-http
cargo test --locked -p jj-store-cli --test agent_execution
```

**Notes**

Five passed, one installed-runner test ignored at this run. Repository-free home, two participants, two isolated attachments, dirty sources preserved, exact duplicate Send and restart without replay. Warm Start creates no sessions/workspaces/prompts; Stop suppression survives restart. v2 lifecycle, setup revisions and runtime receipts remain covered. Shared CARGO_TARGET_DIR=../jj-store/target; linker compact-unwind warning retained.
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
pnpm contracts:generate
pnpm web:check
pnpm catalog:check
```

**Notes**

Generation passed; types initially failed for missing additive fixture fields and number/bigint origin target mismatch. Fixtures and exact DTO type repaired. Subsequent validation results are recorded below.
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
pnpm exec tsx --test web/tests/agent-mentions.test.ts
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

Mention tests 2 passed. catalog-unit 121 files passed, 752 tests passed and 1 skipped. Shared composer/avatar/controller changes justify this direct-consumer suite.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
pnpm exec tsx --test web/tests/production-store-mutations.test.ts web/tests/production-store-search.test.ts
```

**Notes**

115 passed; 2 files failed because tsx cannot load TanStack .svelte package modules. The same two tests fail on untouched original workspace at parent revision. No infrastructure workaround or substituted full-suite claim.
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
cargo clippy --locked -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-jj --all-targets -- -D warnings
cargo clippy --locked -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-jj --lib -- -D warnings
```

**Notes**

Blocked by existing jj-store-jj lint errors outside modified agent_workspaces code (redundant field, collapsible branches, argument count, history tests, and other findings). Focused package checks and tests remain separate; wider lint is not a pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

Two passed after initial fixture repository identifier repair; production ID validation retained. Contextual sidebar, exact autocomplete, persistent pins/reopen/unpin and general draft-only opening. Requested other component paths did not match files; wider agent stories are run separately. No screenshot/baseline updates.
<!-- /run -->

<!-- run -->
### Run 8

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

New changes exceeded existing 300-line module limits after formatting. Submission, audience, navigation, status and request builders extracted without raising any baseline.
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
cargo test --locked -p jj-store-server --lib agent_execution; cargo test --locked -p jj-store-http --lib agent_execution (initial retry)
```

**Notes**

Disk exhaustion interrupted archive/linking. The new workspace target cache was cleaned and tests rerun with the shared target directory. No source or user state was removed.
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
CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-server --lib agent_execution; CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-http --lib agent_execution
```

**Notes**

Server: 16 passed/1 ignored; HTTP admission: 5 passed. Warm v3 additions preserve local-admin and typed-input refusals.
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
CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-cli --test agent_execution (new origin assertion)
```

**Notes**

Four passed, one failed, two ignored. New test incorrectly used inbox_thread(None); companion reads require an explicit metadata pin. Corrected the test to use inbox_selection().snapshot.metadata_commit; production already used that pin.
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
CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

One passed, with canonical metadata-thread assertion and first real bootstrap/context message, two agents/two repositories, isolated edits, dirty sources and duplicate/restart retention.
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
CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-server --lib agent_execution::fleet::tests::stop_signal_is_delivered_when_the_command_queue_is_full -- --exact
```

**Notes**

Dedicated Stop signal delivered while the command queue remained full. Immediate Stop/Start generation transition passed in the CLI lifecycle family.
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
PATH=<existing adapter cache>/node_modules/.bin:$PATH JJ_STORE_AGENT_TEST_ACP=<rebuilt jj-acp> CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-cli --test agent_execution real::installed_fleet_warms_general_chat_then_reuses_an_attached_workspace -- --ignored --exact --nocapture
```

**Notes**

Installed Codex ACP 1.13.1: warm/general/attached reuse, isolated file edit, dirty source preserved, canonical publication and exact duplicate without replay; zero live permission requests.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

Zero Svelte errors/warnings; 280 catalog modules; 1298 architecture modules; 17 package boundaries; local provenance complete. Initial additive fixture/schema type errors and 301-305 line limits were repaired by exact operation DTOs and focused extraction, with no raised baseline.
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
pnpm exec tsx --test web/tests/agent-mentions.test.ts
```

**Notes**

Three passed: exact identity/member separation, adjacent-text delimiter regression, opt-in scoped removal and storage denial. Delimited IDs never consume subsequent typing.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

Final five application files: 17 interactions passed. Project/task agent family: 4 passed. Six consumer files total 21. Shared composer/avatar/status consumers justify this scope. No visual baselines updated.
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
CARGO_TARGET_DIR=../jj-store/target pnpm web:build (before manual/reference refresh)
```

**Notes**

New installation API routes required generated reference updates and local/services/lifecycle/application source review. Updated guides, generated references and coverage reasons; built closure subsequently passed. Known large-chunk warning retained.
<!-- /run -->
<!-- /evidence-field -->

