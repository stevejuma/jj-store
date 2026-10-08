---
schema: 1
id: E-COMPUTE-008
---

# E-COMPUTE-008 details 3

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
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Initial landing record passed all three after fixing evidence test-reference syntax: 146 tasks, 669 records; seven validators and 181 requirements. Final label changes receive updated coverage below.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte -t AddAgentDiscoversUnsavedProvidersAndUsesSettingsRows
```

**Notes**

Before label correction: exactly one story failed because Application default (This machine) disappeared after an explicit Anthropic selection; two stories intentionally skipped.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte
```

**Notes**

After correction: fresh process, seven interaction/Axe stories passed across Add/Edit provider and defaults surfaces. The exact failing story passes as part of this complete direct-consumer lane.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final label fix: zero errors/warnings; 305 catalog modules.
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
pnpm docs:review application "Reviewed combined agent context and provider integration; the default provider option now names actual inherited configuration independently of explicit draft selection."; pnpm exec prettier --write docs/maintenance/coverage.json; env PATH="$PATH:/Users/stevejuma/.local/bin" pnpm docs:check
```

**Notes**

Final application coverage refreshed; strict documentation build and link closure passed for 141 files (17,817,680 bytes).
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
CUA main preview provider menu inspection after explicit Anthropic selection
```

**Notes**

Application default (This machine) remains correctly labeled. Restored that inherited option in the unsaved Add agent draft and left main preview ready for review.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Final label record passed progress (146 tasks, 669 records) and all seven spec validators (3236 chapters, 181 requirements). Spec-first required the mapped API chapter for the web presentation change. Added an explicit unchanged API/schema/authorization/persistence clarification describing how existing metadata projects the inherited provider label.
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
pnpm exec prettier --write spec/src/current/contracts/api/42-compute-and-agent-settings.md spec/src/history/2026/10/08/E-COMPUTE-008/*.md; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Final owning-chapter correction passed formatting, progress (146 tasks, 669 evidence records), all seven specification validators (3236 chapters, 181 requirements), and the spec-first gate (four protected files, three canonical chapters). No remaining conflict in lptotxop..@; main preview responds HTTP 200.
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

JJ 0.45.1; Rust 1.98.1; Node 24.10.0 (main shell); pnpm 10.32.1; Vitest 4.1.11; unchanged Buzz pin and existing Design Core resolution from E-COMPUTE-007.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No integration-time API, native storage, WAL, manifest, AgentStore or WorkStore format change. Retained format-2 LocalTrusted fixture and version-1 fixture metadata.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Main preview is ready for user review. Continue independent real-provider, paired inference and distribution qualification from E-COMPUTE-007; its existing development-script architecture limit remains outside this landing scope.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Clarified JST-UI-012 and JST-DOC-001/JST-DOC-002 default-provider label
  truthfulness in the owning UI/documentation chapters. The mapped API chapter
  confirms projection from existing settings and metadata under JST-API-001.
  JST-COLLAB-034/JST-API-001 launch inheritance, authorization, credential
  custody, storage and publication contracts are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M25 remains In progress. Source integration and a restarted development
  preview do not close real external-provider, paired inference, signed
  macOS/WKWebView, Linux or independent M24 gates.

```
<!-- /evidence-field -->

