---
schema: 1
id: E-M17-014
---

# E-M17-014

<!-- evidence-field:id markdown -->
## Id

E-M17-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

durable-source-registration-and-exclusive-reader-leases
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/source_registry/catalog.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/src/source_registry/files.rs
- crates/jj-store-capture/tests/source_registry.rs
- crates/jj-store-capture/tests/source_registry_policy.rs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/source_registry.rs#restart_overlap_hardlinks_and_replacement_retain_source_identity
- crates/jj-store-capture/tests/source_registry.rs#header_qualification_has_no_registry_side_effect_and_leased_reader_revalidates
- crates/jj-store-capture/tests/source_registry.rs#exclusive_source_lease_crosses_process_boundary_and_releases
- crates/jj-store-capture/tests/source_registry.rs#source_registry_process_probe
- crates/jj-store-capture/tests/source_registry.rs#killed_reader_releases_both_leases_and_keeps_durable_source_identity
- crates/jj-store-capture/tests/source_registry_policy.rs#capacity_conflicting_aliases_and_installation_mismatch_preserve_registry
- crates/jj-store-capture/tests/source_registry_policy.rs#unsupported_or_changed_header_cannot_allocate_or_change_channel
- crates/jj-store-capture/tests/source_registry_policy.rs#private_state_rejects_checkouts_symlinks_permissions_and_rebound_leases
- crates/jj-store-capture/tests/source_registry_policy.rs#registry_recovery_is_bounded_and_does_not_evict_history_or_unknown_files
- crates/jj-store-capture/tests/source_registry_policy.rs#unknown_alias_of_changed_header_waits_for_known_location_rediscovery
- crates/jj-store-capture/tests/source_registry_policy.rs#missing_catalog_and_rebound_directory_never_reallocate_source_ids
- crates/jj-store-capture/tests/source_registry_policy.rs#header_rewrite_requires_new_registration_before_returning_batches
- crates/jj-store-capture/src/source_registry/catalog.rs#retained_alias_budget_has_an_exact_bound_without_eviction
- crates/jj-store-capture/src/source_registry/files.rs#reader_lease_explicitly_unlocks_inherited_description
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-014 details 2](details-02.md)
- [E-M17-014 details 3](details-03.md)
