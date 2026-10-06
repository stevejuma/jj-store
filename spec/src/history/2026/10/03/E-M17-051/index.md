---
schema: 1
id: E-M17-051
---

# E-M17-051

<!-- evidence-field:id markdown -->
## Id

E-M17-051
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
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-server/src/capture_control_state.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/authority.rs
- crates/jj-store-server/src/capture_management/report.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process/capture/selection.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/tests/capture_factory/management.rs
- crates/jj-store-server/src/process/tests/capture_factory.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-server/tests/capture_management/http.rs
- crates/jj-store-server/tests/capture_management/http_admission.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/lib.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec/src/milestones.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/capture_tests.rs#capture_management_contract_is_installation_local_and_actor_free
- crates/jj-store-server/tests/capture_management/http.rs#local_capture_http_derives_actor_revalidates_authority_and_captures_contracts
- crates/jj-store-server/tests/capture_management/http_admission.rs#installation_management_enforces_transport_admin_and_bounded_input
- crates/jj-store-server/tests/capture_management.rs#old_receipt_replay_applies_latest_policy_and_cannot_restart_a_stopped_owner
- crates/jj-store-server/src/capture_management/tests.rs#caller_cancellation_retains_journal_gate_and_operation_until_owned_application_finishes
- crates/jj-store-server/src/capture_management/tests.rs#revocation_after_selection_returns_unknown_and_exact_retry_recovers_receipt
- crates/jj-store-server/src/capture_management/tests.rs#application_task_panic_preserves_selected_receipt_for_exact_replay
- crates/jj-store-server/src/capture_management/tests.rs#policy_directory_rebinding_after_selection_withholds_application_until_exact_replay
- crates/jj-store-server/src/process/tests/capture_factory/management.rs#cold_foreground_advertises_authenticated_disabled_capture_without_creating_policy
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-051 details 2](details-02.md)
- [E-M17-051 details 3](details-03.md)
- [E-M17-051 details 4](details-04.md)
