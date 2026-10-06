---
schema: 1
id: E-M13-020
---

# E-M13-020

<!-- evidence-field:id markdown -->
## Id

E-M13-020
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-003
- JST-LOCAL-004
- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

integration
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-threads-integration.ts
- web/tests/vite/local-browser-merge.test.ts
- schemas/openapi.json
- web/src/lib/api/generated.ts
- web/src/lib/controller/production-store-view.ts
- web/tests/real/local-mixed.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/local-browser-merge.test.ts#mixed_thread_navigation_selects_a_native_home
- web/tests/vite/local-browser-merge.test.ts#local_profile_hides_threads_and_discards_previous_native_thread_state
- web/tests/vite/local-browser-merge.test.ts#profile_route_reconnects_before_restoring_threads_without_a_code_operation
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- web/tests/real/local-mixed.spec.ts#developer_experience_opens_linked_git_repository_and_preserves_mixed_navigation
- web/tests/vite/local-browser-merge.test.ts#local_profile_switch_preserves_unknown_native_thread_intent
- web/tests/real/local-mixed.spec.ts#merged_native_threads_create_send_and_reload_independent_of_local_browsing
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-020 details 2](details-02.md)
- [E-M13-020 details 3](details-03.md)
- [E-M13-020 details 4](details-04.md)
