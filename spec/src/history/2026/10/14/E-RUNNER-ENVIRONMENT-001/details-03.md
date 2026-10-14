---
schema: 1
id: E-RUNNER-ENVIRONMENT-001
---

# E-RUNNER-ENVIRONMENT-001 details 3

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
pnpm spec:first
pnpm check:progress
```

**Notes**

Initial evidence collection excluded deleted files and newly updated M27/M28 indexes. Corrected files coverage to include removed_files and all changed owning milestone pages; no production/spec requirement relaxation. spec-first.log/progress-first.log.
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
pnpm spec:first
pnpm spec:validate
pnpm check:progress
```

**Notes**

After evidence-path correction, full spec validation and progress pass. Spec-first additionally required the mapped Change map owning chapter for the changed UI provenance; added the explicit JST-GOV-002/008 owned-source review without weakening mappings. spec-first-corrected.log/spec-validate.log/progress-corrected.log.
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
pnpm contracts:check
```

**Notes**

Generated OpenAPI/TypeScript match; 300 actual HTTP/SSE responses conform; eight contract tests and consumer TypeScript compile pass. Includes the existing exact capture fixtures required by contract generation; no root aggregate test lane. contracts-check.log.
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
pnpm spec:first
pnpm spec:validate
pnpm check:progress
node scripts/docs.mjs review
pnpm format:rust:check
```

**Notes**

Spec-first: 40 protected files/12 canonical chapters. Seven validators, 3764 chapters, 196 requirements; progress 153 tasks/788 evidence records. Source fingerprints current, Rust formatting pass; spec-first-final.log/spec-validate.log/progress-final.log/docs-review-final.log/rust-format-check.log. Evidence remains pending only for independent explicitly retained milestone gates and inherited CSS size limit.
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
pnpm exec prettier --check catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/fixtures/ServerSettingsFixture.svelte catalog/fixtures/server-settings.ts catalog/fixtures/workflow-host-settings.ts docs/cli/workflows.md docs/maintenance/coverage.json docs/site/cli/jj-workflow.md docs/site/guides/settings.md docs/site/reference/api/installation.md docs/site/reference/api/types.md docs/site/reference/configuration.md docs/site/reference/transport.md docs/site/technical/architecture.md docs/ui-provenance.json schemas/openapi.json spec/src/current/contracts/api/30-installation-configuration-editing.md spec/src/current/contracts/api/32-workflows.md spec/src/current/contracts/architecture/21-workflow-host.md spec/src/current/contracts/cli/23-app-managed-operational-configuration.md spec/src/current/contracts/cli/33-workflows.md spec/src/current/contracts/compatibility/39-workflow-formats.md spec/src/current/contracts/documentation/10-workflows.md spec/src/current/contracts/jj-integration/19-workflow-transports.md spec/src/current/contracts/mutations/19-workflow-commands.md spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md spec/src/current/contracts/ui/41-workflows.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M27/index.md spec/src/current/progress/M28/index.md spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/SUMMARY.nav.md spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/details-02.md spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/details-03.md spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/index.md spec/src/history/2026/10/14/SUMMARY.nav.md spec/src/history/2026/10/14/index.md spec/src/history/2026/10/SUMMARY.nav.md spec/src/history/2026/10/index.md web/src/lib/api/generated.ts web/src/lib/controller/workflow-host-configuration.ts web/src/lib/controller/workflow-runner-environment.ts web/src/lib/ui/store-settings/StoreWorkflowHostFields.svelte web/src/lib/ui/store-settings/StoreWorkflowHostSettings.svelte web/src/lib/ui/store-settings/WorkflowRunnerAccess.svelte web/src/lib/ui/store-settings/WorkflowRunnerAdvanced.svelte web/src/lib/ui/store-settings/WorkflowRunnerEnvironmentFields.svelte web/src/lib/ui/store-settings/global-settings-skeleton.ts web/tests/vite/workflow-host-settings.test.ts
```

**Notes**

All changed authored/generated Markdown, JSON, TypeScript and Svelte files formatted after the targeted --write pass over the same paths. format-write.log/format-check.log. No unrelated file formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/src/workflow_host.rs#managed_environment_requires_no_repository_fields
- crates/jj-store-service/src/workflow_host.rs#managed_environment_defaults_and_bounds_preserve_authority
- crates/jj-store-cli/src/workflow_host/discovery.rs#registered_source_requires_exact_identity_socket_and_ready_native_link
- crates/jj-store-cli/src/workflow_host/discovery.rs#changed_or_missing_native_binding_is_not_a_source
- crates/jj-store-cli/src/workflow_host/discovery.rs#removed_home_cannot_hide_an_uncertain_attempt_journal
- crates/jj-store-server/tests/capture_management/server_settings.rs#managed_runner_environment_save_needs_no_repo_mapping_or_identity_input
- crates/jj-store-cli/tests/server_executable/workflows.rs#two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
<!-- /evidence-field -->

