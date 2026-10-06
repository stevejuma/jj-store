---
schema: 1
id: E-READ-048
---

# E-READ-048

<!-- evidence-field:id markdown -->
## Id

E-READ-048
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pqoyvznw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Cold native receipt proof outside the namespace under independent quota ownership
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib/jj-cli pinned0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/index6/receipt/companion/WAL, public DTO/schema, RPC or signed cursor format changes. Bounded name/type/size admission transfers immutable data into an actual-size reservation or pins an existing charged shared slot before namespace release. Pending checksums are verified before native loading/allocating decode. Private checksum damage discards owned data and rebuilds through fresh construction admission; shared damage fails without resident fallback, unlink or lost charge. Budget/cancellation/deadline errors and failed proof state remain explicit. Coarse metadata rehashes before loading and before namespace reacquisition; stamp-free warm residents are not retained. Final installation uses bounded metadata checks. Explicit filter upgrades reserve the full writable allowance before growth. Failed admission before transfer preserves existing files and receipt.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs
- crates/jj-store-jj/src/query/cache/proof.rs
- crates/jj-store-jj/src/query/cache/proof_tests.rs
- crates/jj-store-jj/src/query/cache/recovery.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/mod.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Integrate accumulated bounded-read changes into the current main workspace and verify the combined changes at the user's request; keep the temporary workspace open for different embedded operations and deep/difference/search/candidate/RPC delivery. Complete large-scale qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private native cold cache ownership/proof and direct graph/content/preparation/HTTP consumers. Exact regressions, focused cache consumers and package boundaries; unrelated full integration, UI, contracts, large fixtures, release/RustFS and visual qualification remain deferred for this delivery slice.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/cache/proof_tests.rs#cold_cache_admission_does_not_charge_full_file_hashing_under_namespace
- crates/jj-store-jj/src/query/cache/proof_tests.rs#cold_checksum_damage_rebuilds_private_files_and_preserves_shared_files
- crates/jj-store-jj/src/query/cache/proof_tests.rs#cold_proof_budget_and_cancellation_do_not_become_rebuild_success
- crates/jj-store-jj/src/query/cache/proof_tests.rs#coarse_metadata_rehashes_before_namespace_reacquisition_and_detects_damage
- crates/jj-store-jj/src/query/cache/proof_tests.rs#cold_filter_upgrade_reserves_writable_construction_before_growth
- crates/jj-store-jj/src/query/cache/proof_tests.rs#failed_cold_recovery_admission_preserves_existing_files_and_receipt
- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs#existing_disk_projection_recovery_releases_namespace_and_preserves_generations
- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs#canceled_disk_projection_recovery_cleans_up_while_namespace_is_busy
- crates/jj-store-cli/tests/native_lifecycle/filter_cache.rs#native_filter_upgrade_and_pinned_cold_recovery_preserve_exact_generation
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-048 details 2](details-02.md)
- [E-READ-048 details 3](details-03.md)
