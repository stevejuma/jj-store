---
schema: 1
id: E-EXEC-020
---

# E-EXEC-020 details 3

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
cargo test --locked -p jj-store-server --lib agent_execution::
cargo check --locked -p jj-store-service -p jj-store-server -p jj-store-http
```

**Notes**

Final nine focused Rust tests pass, including forged stored projection clearing, legacy journal loading, distinct pre-first-turn workspace operation, native discussion target identity, restart/delivery and presence. Final three affected packages check passes. Existing macOS large __eh_frame linker warning.
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
pnpm web:build
```

**Notes**

After API reference generation and services/configuration/contracts/application documentation review, production web/docs build passes: nine diagrams, 141-file closure and valid links/anchors. Existing chunk-size warning unchanged.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
pnpm spec:first
pnpm check:progress
```

**Notes**

Seven-validator spec validation passed. Progress/spec-first required new ledger file coverage and actual runs in the newly created unchanged-contract record. Refreshed exact file lists and recorded already completed authority/ownership runs; pending final broad stories and final document gates remain explicit.
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
pnpm spec:first
```

**Notes**

Fresh evidence coverage was corrected; shared service/generated contracts also require the mapped API/CLI/compatibility/mutation chapters. Added explicit additive-read and unchanged-command/native-format confirmations in their existing interactive execution chapters, without changing the governance map.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

Second expanded run again had 103 pass and one copy-story failure. Scoped portal queries and clipboard mocking to the story owner document/window; complete pointer and keyboard paths retain exact original JSON and two-write assertions. No production menu change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Final owner-document-scoped regression: all nine stories pass, including both exact-body clipboard writes and accessibility.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

Final cold Vitest run: all 104 stories in 18 directly affected families pass, including blocking accessibility. No functional or interaction failures remain; visual baseline approval remains separate.
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
pnpm spec:first
pnpm check:progress
pnpm check:ui-ownership
```

**Notes**

After mapped chapters and exact coverage were updated: spec-first passes 33 protected files and six canonical chapters; progress passes 144 tasks and 644 evidence records; ownership passes one validator, 3120 chapters and 178 requirements. Final post-bookkeeping document validation follows.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Initial scoped formatting found the copy story and two recently updated API/CLI chapters. Formatting only was corrected; production behavior unchanged.
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
pnpm exec prettier --write catalog/components/store-application/AgentPromptMarkup.stories.svelte spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/cli/35-interactive-agent-execution.md
```

**Notes**

Formatted the three reported files.
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
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Every changed Svelte/TypeScript/JavaScript/Markdown/JSON file accepted by the configured scoped formatter. Generated evidence remains within its separately enforced physical budgets.
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
cargo fmt -p jj-store-service -p jj-store-server -- --check
```

**Notes**

Affected Rust packages pass formatting.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
pnpm docs:check
```

**Notes**

Final post-story bookkeeping gates pass: 144 tasks and 644 evidence records; seven validators, 3120 chapters and 178 requirements; 33 protected files and six canonical chapters; nine documentation diagrams and 141-file closure with valid links/anchors.
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

Enclosing Jujutsu change lowkpvrl; macOS ARM64, Node 24.10.0, pnpm 10.32.1, Vitest 4.1.11. Existing owned Community/Design Core components reused.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive version-1 LocalTrusted execution read fields: recorded_session_ids, workspace_name and workspace_operation. Old private journals default these fields; read clears stored projections and admitted status recomputes them. Canonical AgentStore/ThreadStore/ReactionStore bytes and IDs unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain exact admitted identity and workspace presentation. Visual baselines and native code-publication qualification remain separately gated. Root aggregate Rust/web suites are out of scope under targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 retains its original Done qualification. This maintenance slice
  joins admitted installation identities to exact recorded sessions and
  workspace proofs; it does not qualify another harness, native code publication
  or visual baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-020
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-COLLAB-031
- JST-COLLAB-033
- JST-API-001
- JST-API-006
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository agent rows and selected responses use captured identities via exact repository/normalizer aliases; workspace choices use captured avatars via exact ready names. Last activity appears before lifecycle state independently of first time. Inbox tooltips name the repository and navigate to exact workspace proof operation, including before the first turn. Before: Controller name/avatar, start-only timing, folder-only workspace choices and a repository link without workspace selection. No execution is dispatched by these controls.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated JST-UI-015 and JST-COLLAB-031/JST-COLLAB-033 owning chapters;
  documented additive JST-API-001/JST-API-006 local read projections and
  regenerated OpenAPI, TypeScript and API type reference. Exact native
  identities remain normalizer-owned; no client hash/name matching, republishing
  or canonical format migration.

```
<!-- /evidence-field -->

