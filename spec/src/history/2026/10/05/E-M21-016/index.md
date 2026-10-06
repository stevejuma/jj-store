---
schema: 1
id: E-M21-016
---

# E-M21-016

<!-- evidence-field:id markdown -->
## Id

E-M21-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yqurlkvt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Complete exact local Stacks operation ancestry and real reword recovery input
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1 Git SHA-1 profile; real stock JJ and Git GC in isolated disposable macOS ARM64 fixtures; no installed executable/service changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New domain-separated version-1 OperationArchiveV1 preserves full operation ancestry, every exact view and historical code/evolution records. New version-2 reserved retention record adds a history descriptor/link in the unchanged refs/lapis/stacks/prepared/v1 namespace alongside the byte-compatible selected capsule; original capsule-only version-1 records remain readable and immutable. Archive admission: 4096 operations, 100000 code records, 16 MiB encoded/raw operation-view bytes and 30-second deadline with stricter limits supported. Existing companion/native/WAL/manifest, journal and HTTP/client formats unchanged. PreparedCode additionally returns exact complete history.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/budget.rs
- crates/jj-store-jj/src/local_browser/git_read/extras.rs
- crates/jj-store-jj/src/local_browser/git_read.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/working_tree/current/physical.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/src/local_evidence/operation_archive/capture.rs
- crates/jj-store-jj/src/local_evidence/operation_archive/materialize.rs
- crates/jj-store-jj/src/local_evidence/operation_archive.rs
- crates/jj-store-jj/src/local_execution/reword/report.rs
- crates/jj-store-jj/src/local_execution/reword.rs
- crates/jj-store-jj/src/local_metadata/stack_prepared/objects.rs
- crates/jj-store-jj/src/local_metadata/stack_prepared.rs
- crates/jj-store-jj/tests/stack_operation_archive/adversarial.rs
- crates/jj-store-jj/tests/stack_operation_archive/history.rs
- crates/jj-store-jj/tests/stack_operation_archive.rs
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs
- crates/jj-store-jj/tests/stack_reword_preparation.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Archive/retention regression initially reports eight missing API diagnostics. A separate exact real reword regression demonstrates implicit original-extra import rather than rejection before candidate writes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One owning local-JJ archive/retention/preparation boundary. Bounded descriptor-relative source operation/view reads, strict original extras through the owned read-only decoder, complete exact ancestry/closure validation, private upstream materialization, immutable legacy/version-2 reserved ref selection and exact replay. Real reword captures strict history before transaction work and returns verified complete candidate history, preserving every original operation/view/code record. Complete local reader consumers run because extra lookup is shared; other runtime/UI layers remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_operation_archive.rs#retained_operation_ancestry_reconstructs_exactly_after_source_loss_and_gc
- crates/jj-store-jj/tests/stack_operation_archive.rs#archive_limits_and_missing_source_ancestry_fail_without_publication
- crates/jj-store-jj/tests/stack_operation_archive/adversarial.rs#complete_slots_replay_exactly_and_legacy_slots_are_immutable
- crates/jj-store-jj/tests/stack_operation_archive/adversarial.rs#archive_codec_rejects_missing_unrelated_tampered_and_noncanonical_records
- crates/jj-store-jj/tests/stack_operation_archive/adversarial.rs#nofollow_operation_and_view_reads_reject_links_and_wrong_source_profile
- crates/jj-store-jj/tests/stack_operation_archive/history.rs#historical_only_code_remains_reachable_after_gc_and_packed_ref_replay
- crates/jj-store-jj/tests/stack_operation_archive/history.rs#archive_capture_never_imports_missing_extras_or_creates_nogc_refs
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs#missing_original_extras_fail_before_reword_import_or_candidate_write
- crates/jj-store-jj/tests/stack_reword_preparation.rs#actual_prepared_reword_is_compatible_with_reserved_ref_retention
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#ordinary_point_reads_do_not_import_missing_extra_metadata
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-016 details 2](details-02.md)
- [E-M21-016 details 3](details-03.md)
