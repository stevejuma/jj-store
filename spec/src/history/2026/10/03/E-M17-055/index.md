---
schema: 1
id: E-M17-055
---

# E-M17-055

<!-- evidence-field:id markdown -->
## Id

E-M17-055
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

cached-source-pages-and-cli
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_commands/capture.rs
- crates/jj-store-cli/src/agent_commands/capture/client.rs
- crates/jj-store-cli/src/agent_commands/capture/sources.rs
- crates/jj-store-cli/tests/capture_cli.rs
- crates/jj-store-cli/tests/capture_cli/sources.rs
- crates/jj-store-cli/tests/capture_cli/transport.rs
- crates/jj-store-http/src/capture_management.rs
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/report.rs
- crates/jj-store-server/src/capture_management/sources.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management/tests/sources.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- crates/jj-store-server/src/capture_runtime/sources.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/sources.rs
- crates/jj-store-server/src/capture_supervisor/status.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-server/tests/capture_management/http_admission.rs
- crates/jj-store-server/tests/capture_management/sources.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/capture_management/sources.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_management/sources.rs#source_pages_are_bounded_cached_and_hold_no_admission_proofs
- crates/jj-store-server/tests/capture_management/sources.rs#source_pages_recheck_authority_installation_configuration_and_physical_journal
- crates/jj-store-server/src/capture_management/tests/sources.rs#cancelled_source_read_retains_gates_until_owned_observation_finishes
- crates/jj-store-server/src/capture_management/tests/sources.rs#source_read_rechecks_revocation_disable_and_physical_policy_under_gate
- crates/jj-store-server/src/capture_runtime/sources.rs#source_producer_loss_withholds_nonterminal_cache_but_preserves_stopped_cache
- crates/jj-store-server/src/capture_supervisor/sources.rs#selector_generations_change_and_ordinals_never_wrap_or_reuse
- crates/jj-store-cli/tests/capture_cli/sources.rs#disabled_source_listing_requires_no_repository_or_capture_state
- crates/jj-store-cli/tests/capture_cli/sources.rs#source_listing_uses_one_encoded_request_and_safe_cached_output
- crates/jj-store-cli/tests/capture_cli/sources.rs#source_listing_rejects_stale_and_malformed_pages_without_private_output
- crates/jj-store-http/src/contract/capture_tests.rs#capture_management_contract_is_installation_local_and_actor_free
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-055 details 2](details-02.md)
- [E-M17-055 details 3](details-03.md)
- [E-M17-055 details 4](details-04.md)
