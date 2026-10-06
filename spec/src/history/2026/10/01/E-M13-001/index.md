---
schema: 1
id: E-M13-001
---

# E-M13-001

<!-- evidence-field:id markdown -->
## Id

E-M13-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

nlvotsvl
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- spec-validator.config.mjs
- spec/src/SUMMARY.md
- spec/src/local-jj.md
- spec/src/jj-integration.md
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/milestones.md
- spec/src/verification.md
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-jj/src/local_metadata/objects.rs
- crates/jj-store-jj/src/local_metadata/prepare.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-jj/tests/local_metadata.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_metadata.rs#reads_and_staged_candidates_never_initialize_or_publish
- crates/jj-store-jj/tests/local_metadata.rs#racing_initializers_adopt_winning_identity_and_do_not_replace_siblings
- crates/jj-store-jj/tests/local_metadata.rs#receipts_survive_lost_response_restart_and_later_domain_publication
- crates/jj-store-jj/tests/local_metadata.rs#stock_git_gc_retains_history_code_and_nonstandard_dependencies
- crates/jj-store-jj/tests/local_metadata.rs#lock_failures_remain_unknown_and_retry_uses_the_exact_candidate
- crates/jj-store-jj/tests/local_metadata.rs#admission_and_missing_selected_objects_fail_without_partial_reads
- crates/jj-store-jj/tests/local_metadata.rs#hard_process_exit_before_and_after_ref_publication_recovers_correctly
- crates/jj-store-jj/tests/local_metadata.rs#jj_conflict_header_terms_get_real_git_reachability
- crates/jj-store-jj/tests/local_metadata.rs#broken_nested_dependencies_and_budget_exhaustion_never_publish
- crates/jj-store-jj/tests/local_metadata.rs#stock_jj_import_gc_and_undo_preserve_companion_metadata
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-001 details 2](details-02.md)
- [E-M13-001 details 3](details-03.md)
