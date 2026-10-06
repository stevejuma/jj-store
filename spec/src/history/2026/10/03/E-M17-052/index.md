---
schema: 1
id: E-M17-052
---

# E-M17-052

<!-- evidence-field:id markdown -->
## Id

E-M17-052
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/capture_management.rs
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-server/src/capture_control_state.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/report.rs
- crates/jj-store-server/src/capture_management/rescan.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management/tests/rescan.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- crates/jj-store-server/src/capture_runtime/rescan.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/discovery.rs
- crates/jj-store-server/src/capture_supervisor/discovery/tests.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/src/capture_supervisor/status.rs
- crates/jj-store-server/src/capture_supervisor/tests.rs
- crates/jj-store-server/src/capture_supervisor/tests/observations.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-server/tests/capture_management/http.rs
- crates/jj-store-server/tests/capture_management/http_admission.rs
- crates/jj-store-server/tests/capture_management/rescan.rs
- crates/jj-store-server/tests/capture_runtime.rs
- crates/jj-store-server/tests/capture_runtime/rescan.rs
- crates/jj-store-service/src/capture_management.rs
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

- crates/jj-store-server/tests/capture_management/rescan.rs#rescan_is_conditional_actor_free_and_preserves_device_policy
- crates/jj-store-server/tests/capture_management/rescan.rs#rescan_denials_preserve_policy_and_never_initialize_missing_state
- crates/jj-store-server/tests/capture_runtime/rescan.rs#explicit_rescan_discovers_before_cooldown_but_preserves_activity_admission
- crates/jj-store-server/src/capture_supervisor/discovery/tests.rs#busy_rescan_preserves_progress_and_coalesces_one_followup_before_cooldown
- crates/jj-store-server/src/capture_supervisor/discovery/tests.rs#missing_root_remains_cached_until_requested_recovery
- crates/jj-store-server/src/capture_supervisor/tests/observations.rs#source_degradation_is_cached_and_clears_after_successful_recovery
- crates/jj-store-server/src/capture_management/tests/rescan.rs#cancelled_rescan_retains_both_gates_without_selecting_policy
- crates/jj-store-server/src/capture_management/tests/rescan.rs#rescan_rechecks_revocation_and_physical_policy_before_runtime_request
- crates/jj-store-http/src/contract/capture_tests.rs#capture_management_contract_is_installation_local_and_actor_free
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-052 details 2](details-02.md)
- [E-M17-052 details 3](details-03.md)
- [E-M17-052 details 4](details-04.md)
