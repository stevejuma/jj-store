---
schema: 1
id: E-WORKFLOW-001
---

# E-WORKFLOW-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WF-001
- JST-WF-002
- JST-WF-003
- JST-WF-004
- JST-WF-005
- JST-WF-006
- JST-WF-007
- JST-WF-008
- JST-UI-019
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-13
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- AGENTS.md
- Cargo.lock
- Cargo.toml
- catalog/components/store-workflows/StoreWorkflows.stories.svelte
- catalog/components/store-workflows/WorkflowEditor.stories.svelte
- catalog/components/store-workflows/WorkflowTriggers.stories.svelte
- catalog/fixtures/WorkflowsFixture.svelte
- catalog/fixtures/workflow-collaboration.ts
- catalog/fixtures/workflows.ts
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/main.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/server_process.rs
- crates/jj-store-cli/src/workflow_commands.rs
- crates/jj-store-cli/src/workflow_host/agent.rs
- crates/jj-store-cli/src/workflow_host/command.rs
- crates/jj-store-cli/src/workflow_host/config.rs
- crates/jj-store-cli/src/workflow_host/events.rs
- crates/jj-store-cli/src/workflow_host/journal.rs
- crates/jj-store-cli/src/workflow_host/worker.rs
- crates/jj-store-cli/src/workflow_host.rs
- crates/jj-store-cli/src/workflow_source.rs
- crates/jj-store-cli/tests/server_executable/workflows.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/workflow.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/src/work.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/process/authority.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/conversion/import.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/workflow/commands.rs
- crates/jj-store-service/src/workflow/matching.rs
- crates/jj-store-service/src/workflow/reads.rs
- crates/jj-store-service/src/workflow/source.rs
- crates/jj-store-service/src/workflow.rs
- crates/jj-store-service/tests/workflow_store.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/workflow.rs
- crates/jj-store-wal/src/publication/workflow_commands.rs
- crates/jj-store-wal/src/publication/workflow_events.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/workflow_store/fault.rs
- crates/jj-store-wal/tests/workflow_store.rs
- crates/jj-workflow/Cargo.toml
- crates/jj-workflow/src/filters.rs
- crates/jj-workflow/src/ids.rs
- crates/jj-workflow/src/lib.rs
- crates/jj-workflow/src/model.rs
- crates/jj-workflow/src/state.rs
- crates/jj-workflow/tests/coordination.rs
- crates/jj-workflow/tests/filters.rs
- docs/cli/commands.md
- docs/cli/server-run.md
- docs/cli/workflows.md
- docs/maintenance/arguments.json
- docs/maintenance/coverage.json
- docs/site/cli/index.md
- docs/site/cli/jj-agent.md
- docs/site/cli/jj-store-client.md
- docs/site/cli/jj-store.md
- docs/site/cli/jj-workflow.md
- docs/site/guides/browsing.md
- docs/site/guides/work.md
- docs/site/reference/api/types.md
- docs/site/reference/api/workflows.md
- docs/site/reference/configuration.md
- docs/site/reference/http-api.md
- docs/site/reference/transport.md
- docs/site/technical/publication.md
- docs/ui-provenance.json
- mkdocs.yml
- schemas/openapi.json
- scripts/check-boundaries.mjs
- scripts/docs/references.mjs
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/api/SUMMARY.nav.md
- spec/src/current/contracts/api/index.md
- spec/src/current/contracts/architecture/21-workflow-host.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/cli/33-workflows.md
- spec/src/current/contracts/cli/SUMMARY.nav.md
- spec/src/current/contracts/cli/index.md
- spec/src/current/contracts/compatibility/39-workflow-formats.md
- spec/src/current/contracts/compatibility/SUMMARY.nav.md
- spec/src/current/contracts/compatibility/index.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/documentation/SUMMARY.nav.md
- spec/src/current/contracts/documentation/index.md
- spec/src/current/contracts/jj-integration/19-workflow-transports.md
- spec/src/current/contracts/jj-integration/SUMMARY.nav.md
- spec/src/current/contracts/jj-integration/index.md
- spec/src/current/contracts/mutations/19-workflow-commands.md
- spec/src/current/contracts/mutations/SUMMARY.nav.md
- spec/src/current/contracts/mutations/index.md
- spec/src/current/contracts/publication/09-workflowstore.md
- spec/src/current/contracts/publication/SUMMARY.nav.md
- spec/src/current/contracts/publication/index.md
- spec/src/current/contracts/security/02-requirements.md
- spec/src/current/contracts/ui/03-requirements.md
- spec/src/current/contracts/ui/06-application-notifications.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/SUMMARY.nav.md
- spec/src/current/progress/M28/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/index.md
- spec/src/current/verification/workflows.md
- spec/src/history/2026/10/12/E-APP-RELOAD-001/SUMMARY.nav.md
- spec/src/history/2026/10/13/E-WORKFLOW-001/SUMMARY.nav.md
- spec/src/history/2026/10/13/E-WORKFLOW-001/index.md
- spec/src/history/2026/10/13/E-WORKFLOW-001/details-02.md
- spec/src/history/2026/10/13/E-WORKFLOW-001/details-03.md
- spec/src/history/2026/10/13/E-WORKFLOW-001/details-04.md
- spec/src/history/2026/10/13/SUMMARY.nav.md
- spec/src/history/2026/10/13/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec-validator.config.mjs
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/http-workflows.ts
- web/src/lib/controller/production-entity-search.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-fields.ts
- web/src/lib/controller/store-route-writer.ts
- web/src/lib/controller/workflow-types.ts
- web/src/lib/controller/workflows.svelte.ts
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/community-emoji-picker/CommunityEmojiPicker.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/application-views.ts
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-workflows/StoreWorkflows.svelte
- web/src/lib/ui/store-workflows/WorkflowAuthorPicker.svelte
- web/src/lib/ui/store-workflows/WorkflowCard.svelte
- web/src/lib/ui/store-workflows/WorkflowConditions.svelte
- web/src/lib/ui/store-workflows/WorkflowEditor.svelte
- web/src/lib/ui/store-workflows/WorkflowEditorMenu.svelte
- web/src/lib/ui/store-workflows/WorkflowFeedback.svelte
- web/src/lib/ui/store-workflows/WorkflowForm.svelte
- web/src/lib/ui/store-workflows/WorkflowHistory.svelte
- web/src/lib/ui/store-workflows/WorkflowInspector.svelte
- web/src/lib/ui/store-workflows/WorkflowReactionPicker.svelte
- web/src/lib/ui/store-workflows/WorkflowThreadPicker.svelte
- web/src/lib/ui/store-workflows/WorkflowName.svelte
- web/src/lib/ui/store-workflows/WorkflowRunDialog.svelte
- web/src/lib/ui/store-workflows/WorkflowSelect.svelte
- web/src/lib/ui/store-workflows/WorkflowSkeleton.svelte
- web/src/lib/ui/store-workflows/WorkflowStep.svelte
- web/src/lib/ui/store-workflows/WorkflowTriggerPicker.svelte
- web/src/lib/ui/store-workflows/WorkflowYaml.svelte
- web/src/lib/ui/store-workflows/workflow-editor.css
- web/src/lib/ui/store-workflows/workflow-home.ts
- web/src/lib/ui/store-workflows/workflow-presentation.ts
- web/src/lib/ui/store-workflows/workflow-sequence.css
- web/src/lib/ui/store-workflows/workflows.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-workflow/tests/coordination.rs#no_match_is_frozen_and_a_new_event_pins_the_new_definition
- crates/jj-workflow/tests/filters.rs#legacy_definitions_keep_their_canonical_bytes_and_missing_proof_fails_closed
- crates/jj-store-wal/tests/workflow_store.rs#workflow_manifest_timeouts_keep_atomic_visibility_and_original_receipts
- crates/jj-store-wal/tests/workflow_store.rs#one_event_one_run_one_owner_with_recovery
- crates/jj-store-service/tests/workflow_store.rs#workflow_grants_receipts_and_pinned_history_share_one_service
- crates/jj-store-service/tests/workflow_store.rs#message_and_reaction_events_are_scoped_pinned_and_exclude_edits_and_removals
- crates/jj-store-cli/tests/server_executable/workflows.rs#two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation


<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live admitted agent provider dispatch and actual Forgejo wrapper execution.
- Unchanged external Forgejo Runner ConnectRPC registration/polling compatibility.
- Approved visual baselines, additional platforms and Thread/Reaction origin reduction.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; JJ 0.45.1; Git 2.55.0; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Tool versions were read during this enclosing change; package lock owns exact dependencies.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native manifest format 6 adds WorkflowStore root/state. Typed workflow definitions, host journal, request/receipt/evidence and HTTP/native RPC documents use version 1. Optional conditions and conversation references preserve old canonical encodings when omitted; code pins remain native identities. No downgrade or origin reducer is added.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native metadata home stores immutable workflow definitions, evaluations, runs and reports through existing WAL/receipts/CAS. The optional operator-configured host polls pinned source events and uses the custom CLI for durable command/agent reporting. Scope includes manual/code/bookmark/thread-message/reaction triggers, bounded source filters, one-owner ordered actions, cancellation, explicit retry, secret redaction, pinned exports and browser authoring/history. Buzz is a licensed design/reference source, with maintained owned components and released foundational dependencies.

The narrow acceptance target is two-host real-storage execution and exact unknown-outcome recovery, plus scoped committed conversation triggers and the workflow editor interaction/accessibility surface. Source schemas and runtime policy changed; provider protocol and approved visual gates remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified slice in enclosing Jujutsu change xnsuxmul. Continue the independent M28 provider/Forgejo/visual gates; no publication, installed CLI replacement, migration or destructive reset.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Add JST-WF-001 through JST-WF-008 WorkflowStore/host/UI contracts and strengthen JST-UI-019 shared toast feedback. Owning publication, mutations, API/RPC/CLI, compatibility, UI, documentation, governance and security chapters are updated. Manifest CAS, native identity, frozen home origins, exact unknown request retention, current grants and source ownership remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. E-WORKFLOW-001 qualifies the implemented native command/source/service/editor slice including source-scoped thread selection, author avatars and reaction emoji selection. Live provider, actual Forgejo runner and visual approval gaps remain in current/progress/M28/index.md; compilation or screenshots do not complete those gates.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [Focused validation](details-02.md)
- [Diagnostics, limits and remaining gates](details-03.md)
- [Workflow selector verification](details-04.md)
