---
schema: 1
id: E-FORGE-STACKS-001
---

# E-FORGE-STACKS-001

<!-- evidence-field:id markdown -->
## Id

E-FORGE-STACKS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-FORGE-001
- JST-FORGE-002
- JST-FORGE-003
- JST-FORGE-004
- JST-FORGE-005
- JST-FORGE-006
- JST-FORGE-007
- JST-LOCAL-005
- JST-CLI-001
- JST-UI-020
- JST-GOV-002
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Jujutsu change xltkslny (enclosing change)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- catalog/components/store-forge/StoreForge.stories.svelte
- catalog/fixtures/ForgeFixture.svelte
- crates/jj-forge/Cargo.toml
- crates/jj-forge/src/client.rs
- crates/jj-forge/src/engine.rs
- crates/jj-forge/src/journal.rs
- crates/jj-forge/src/lib.rs
- crates/jj-forge/src/planner.rs
- crates/jj-forge/src/providers/checks.rs
- crates/jj-forge/src/providers/merge.rs
- crates/jj-forge/src/providers/mod.rs
- crates/jj-forge/src/providers/pull_requests.rs
- crates/jj-forge/src/types.rs
- crates/jj-forge/tests/planner.rs
- crates/jj-forge/tests/providers.rs
- crates/jj-forge/tests/submission.rs
- crates/jj-store-cli/help/stacks.md
- crates/jj-store-cli/src/stack_commands/args.rs
- crates/jj-store-cli/src/stack_commands/forge.rs
- crates/jj-store-cli/src/stack_commands.rs
- crates/jj-store-cli/tests/forge_cli.rs
- crates/jj-store-http/src/local_browser/forge.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/tests/forge_admission.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/local_stacks/forge/restack.rs
- crates/jj-store-jj/src/local_stacks/forge/templates.rs
- crates/jj-store-jj/src/local_stacks/forge/tests.rs
- crates/jj-store-jj/src/local_stacks/forge.rs
- crates/jj-store-jj/src/local_stacks/graph.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/operation_description.rs
- crates/jj-store-jj/src/local_stacks/preview.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/lib.rs
- docs/maintenance/coverage.json
- docs/site/cli/jj-store.md
- docs/site/guides/git.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/check-boundaries.mjs
- scripts/forge-boundaries.test.mjs
- spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/cli/34-server-free-local-stacks-cli.md
- spec/src/current/contracts/compatibility/01-compatibility-and-release.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/28-ordinary-local-stacks-engine.md
- spec/src/current/contracts/local-jj/31-complete-native-stack-discovery-and-boundaries.md
- spec/src/current/contracts/mutations/21-local-stacks-protocol-ownership.md
- spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M31/SUMMARY.nav.md
- spec/src/current/progress/M31/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/forge.md
- spec/src/current/verification/index.md
- spec/src/history/2026/10/15/E-FORGE-STACKS-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-FORGE-STACKS-001/comparison-and-rollout.md
- spec/src/history/2026/10/15/E-FORGE-STACKS-001/details-02.md
- spec/src/history/2026/10/15/E-FORGE-STACKS-001/details-03.md
- spec/src/history/2026/10/15/E-FORGE-STACKS-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec-validator.config.mjs
- web/src/lib/api/generated.ts
- web/src/lib/controller/forge/controller.svelte.ts
- web/src/lib/controller/forge/effects.svelte.ts
- web/src/lib/controller/forge/polling.svelte.ts
- web/src/lib/controller/forge/types.ts
- web/src/lib/controller/forge/view-lifecycle.ts
- web/src/lib/controller/production-stacks.svelte.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-forge/StoreForgeButton.svelte
- web/src/lib/ui/store-forge/StoreForgeChecks.svelte
- web/src/lib/ui/store-forge/StoreForgePanel.svelte
- web/src/lib/ui/store-forge/StoreForgePullRequest.svelte
- web/src/lib/ui/store-forge/StoreForgeSetup.svelte
- web/src/lib/ui/store-forge/store-forge.css
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/tests/vite/forge.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-forge/tests/planner.rs#rewritten_branch_needs_both_push_and_retarget
- crates/jj-forge/tests/submission.rs#unknown_creation_is_observed_and_never_duplicated
- crates/jj-forge/tests/providers.rs#native_stack_requires_exact_order_and_no_extra_members
- crates/jj-store-jj/src/local_stacks/forge/tests.rs#frozen_local_chain_pushes_only_members_and_restacks_after_squash
- crates/jj-store-cli/tests/forge_cli.rs#configure_all_forges_preserves_operation_and_keeps_tokens_out_of_device_records
- crates/jj-store-http/tests/forge_admission.rs#forge_endpoint_rejects_foreign_origin_before_credentials_or_device_state
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [Comparison and rollout](comparison-and-rollout.md)
- [details-02 ](details-02.md)
- [details-03 ](details-03.md)
