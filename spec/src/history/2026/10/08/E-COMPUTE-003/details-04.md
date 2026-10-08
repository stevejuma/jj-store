---
schema: 1
id: E-COMPUTE-003
---

# E-COMPUTE-003 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

After filling evidence, protected mappings additionally required compatibility, documentation, mutations and governance topic updates. Added the actual additive capability, draft-only service contract, manual/reference behavior and extracted-component provenance boundaries; no contract exemption.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Final gate passes: 52 protected files / 10 canonical chapters, 7 validators / 3,177 chapters / 181 requirements, and 146 tasks / 652 evidence records. M25 remains In progress.
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
node -
```

**Notes**

Final 7480 proxy probes: Compute 200 with model_file_picker true/supported true; runtime inventory 200 includes jj-agent Ready with no installer. Codex honestly reports Adapter needed; Cursor Ready. New picker rejects extra script body with 400 before any OS dialog. Bundled logo returns 200 image/svg+xml. Backend PID 97934, retained Vite PID 9640; workspace fixture preserved.
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
pnpm exec prettier --check catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/fixtures/AgentFleetApplicationFixture.svelte catalog/fixtures/AgentRuntimesFixture.svelte catalog/fixtures/LocalComputeFixture.svelte docs/maintenance/coverage.json docs/site/guides/agent-execution.md docs/site/reference/api/installation.md docs/site/reference/api/types.md docs/site/reference/transport.md docs/ui-provenance.json schemas/openapi.json spec/src/current/contracts/api/42-compute-and-agent-settings.md spec/src/current/contracts/cli/13-native-folder-selection-for-local-settings.md spec/src/current/contracts/collaboration/40-local-compute-and-agent-configuration.md spec/src/current/contracts/compatibility/14-local-inference-runtime.md spec/src/current/contracts/documentation/08-interactive-agent-execution.md spec/src/current/contracts/mutations/23-installation-agent-configuration.md spec/src/current/contracts/ui/09-transient-store-settings.md spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M25/index.md web/public/agent-icons/SOURCES.md web/src/lib/api/generated.ts web/src/lib/controller/agent-compute.svelte.ts web/src/lib/ui/community-agent-editor/CommunityAgentEditorDialog.svelte web/src/lib/ui/community-agent-editor/CommunityAgentEditorHarnessIcon.svelte web/src/lib/ui/community-agent-editor/CommunityAgentEditorModelField.svelte web/src/lib/ui/community-agent-editor/CommunityAgentEditorModelPicker.svelte web/src/lib/ui/community-agent-editor/CommunityAgentEditorSavedPreferences.svelte web/src/lib/ui/store-agent-execution/StoreAgentSetupEditor.svelte web/src/lib/ui/store-agent-execution/harness-catalog.ts web/src/lib/ui/store-settings/StoreAgentDefaults.svelte web/src/lib/ui/store-settings/StoreAgentRuntimeSummary.svelte web/src/lib/ui/store-settings/StoreAgentRuntimes.svelte web/src/lib/ui/store-settings/StoreAgentSettings.svelte web/src/lib/ui/store-settings/StoreComputeModels.svelte web/src/lib/ui/store-settings/StoreComputePeers.svelte web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte web/src/lib/ui/store-settings/StoreConfigurationFields.svelte web/src/lib/ui/store-settings/StoreModelFilePicker.svelte web/src/lib/ui/store-settings/StoreSettings.css web/src/lib/ui/store-settings/StoreSettingsPage.svelte web/src/lib/ui/store-settings/StoreSyncConfiguration.svelte web/tests/vite/agent-compute-picker.test.ts
```

**Notes**

Selected changed authored UI, generated client/contracts and documentation files pass; generated evidence/task history excluded from authored formatting.
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

JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vite 8.3.1; Vitest 4.1.11; Storybook 10.5.7; Design Core released ^0.2.0; macOS ARM64; native AppleScript compiled with /usr/bin/osacompile.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive installation HTTP model picker endpoint, version-1 ModelFileSelection, ComputeStatus.model_file_picker defaulting false; regenerated OpenAPI/TS/docs. Stable local runner identity, executable names, agent journal, compute command bodies, save revisions, request identities, credential references and repository formats retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Requested Settings spacing and joined model/configuration/Sync/defaults tables; named padded sections with defaults last, full-width shared searchable harness/model picker, installed/live/remembered choices and explicit retry action row. Fixed 10rem wrapping run buttons beneath description; shared JSON CodeBlock for own invitation; peer invitation stays editable. Add fixed service-machine GGUF native picker with guarded capability, manual fallback, cancellation and draft-only selection. Expose already-vendored Buzz-derived local agent as jj-agent using existing native app artwork and honest companion readiness. Cross-layer steering justifies focused UI/controller, native picker, HTTP and supervisor checks; full shared Settings/model editor browser closure selected because headers/styles/compositions affect multiple direct consumers. Unrelated root aggregate, storage/RustFS qualification, package publication, installed controller, model inference and signed/WKWebView/Linux distribution lanes are out of scope. No OS dialog automated or baseline approved.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-API-001 and JST-UI-012/JST-UI-016 additive picker, shared catalog
  and requested presentation changes documented in API 42, UI 09/28/35 and CLI
  13. JST-COLLAB-034 local harness label documented in collaboration 40 with
  stable identity. Explicit unchanged-contract confirmation: JST-MUT-005
  repository mutation/authorization/CAS and no arbitrary command endpoint
  unchanged; picker selects only a draft through fixed scripts. JST-CLI-004 uses
  explicit development paths, preserves installed tools/unrelated services.
  JST-DOC-002 production API/reference generation and source fingerprint review
  retained. Credentials, publication and unknown-save fences unchanged;
  extracted owned dialog summary retains pinned source/license."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M25 remains In progress. This slice passes its focused implementation
  checks; native interactive dialog review and visual acceptance are separate,
  and signed macOS/WKWebView/Linux distribution gates remain unchanged. Existing
  installed inference/transport evidence is not rerun or claimed by this slice.

```
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep local-compute workspace and 7480 preview open for review/follow-up; native OS dialog and independent M25 distribution qualification remain separate.
<!-- /evidence-field -->

