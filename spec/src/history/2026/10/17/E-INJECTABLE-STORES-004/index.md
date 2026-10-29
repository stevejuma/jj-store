---
schema: 1
id: E-INJECTABLE-STORES-004
---

# E-INJECTABLE-STORES-004

<!-- evidence-field:id markdown -->
## Id

E-INJECTABLE-STORES-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M32
- M32.1
- M32.2
- M32.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-002
- JST-ARCH-005
- JST-ARCH-006
- JST-PUB-001
- JST-PUB-004
- JST-PUB-005
- JST-PUB-007
- JST-PUB-009
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-store-provider
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change snvvlvon.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean parent d211429c after selected-participant composition; exact original bound-capture test passed before native edits. No native provider crate previously implemented the five public interfaces.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-native/tests/factories.rs#original_domain_receipts_survive_later_writes_checkpoint_and_reopen
- crates/jj-store-native/tests/factories.rs#prepared_bound_capture_keeps_exact_native_request_and_publishes_atomically
- crates/jj-store-native/tests/factories.rs#unknown_after_apply_retains_original_receipt_and_replays_without_live_sources
- crates/jj-store-wal/tests/bound_capture.rs#bound_capture_selects_both_roots_once_preserves_lifecycle_and_replays_after_recovery
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-INJECTABLE-STORES-004 details 2](details-02.md)
- [E-INJECTABLE-STORES-004 details 3](details-03.md)
