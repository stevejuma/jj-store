---
schema: 1
id: E-M17-073
---

# E-M17-073

<!-- evidence-field:id markdown -->
## Id

E-M17-073
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

opencode-private-admitted-session-recovery-index
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ltlywptw
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/source_registry/catalog/opencode_index.rs
- crates/jj-store-capture/src/source_registry/catalog.rs
- crates/jj-store-capture/src/source_registry/opencode.rs
- crates/jj-store-capture/tests/opencode_recovery_index.rs
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/opencode_recovery_index.rs#admitted_session_index_survives_exit_before_enqueue_without_paths_or_allocation
- crates/jj-store-capture/tests/opencode_recovery_index.rs#malformed_or_unbound_index_withholds_registry_and_legacy_empty_index_is_readable
- crates/jj-store-capture/src/source_registry/catalog/opencode_index.rs#full_recovery_index_refuses_without_eviction
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-073 details 2](details-02.md)
- [E-M17-073 details 3](details-03.md)
