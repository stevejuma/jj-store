---
schema: 1
id: E-INJECTABLE-STORES-005
---

# E-INJECTABLE-STORES-005

<!-- evidence-field:id markdown -->
## Id

E-INJECTABLE-STORES-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M32
- M32.3
- M32.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-002
- JST-ARCH-005
- JST-ARCH-006
- JST-PUB-004
- JST-PUB-005
- JST-PUB-007
- JST-PUB-009
- JST-LOCAL-001
- JST-LOCAL-003
- JST-CLI-001
- JST-CLI-002
- JST-CLI-004
- JST-MUT-001
- JST-API-001
- JST-API-002
- JST-COMPAT-001
- JST-COMPAT-003
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

factory-consumer-migration
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change qsospxok.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Parent7358a980 native provider committed. Existing application services and LocalTrusted facades still called provider-specific publication directly. Native21baseline tests passed before edits; command postflight reported disk pressure.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_work.rs#factory_facade_replays_sync_receipts_and_keeps_semantic_errors
- crates/jj-store-service/tests/native_collab.rs#retained_library_handles_keep_the_session_permission_boundary
- crates/jj-store-native/tests/factories.rs#unknown_after_apply_retains_original_receipt_and_replays_without_live_sources
- crates/jj-store-server/tests/embedded_storage.rs#injectable_native_stores_replay_exact_receipts_after_cold_rustfs_restart
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-INJECTABLE-STORES-005 details 2](details-02.md)
- [E-INJECTABLE-STORES-005 details 3](details-03.md)
- [E-INJECTABLE-STORES-005 details 4](details-04.md)
