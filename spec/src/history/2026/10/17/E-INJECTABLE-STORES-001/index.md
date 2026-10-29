---
schema: 1
id: E-INJECTABLE-STORES-001
---

# E-INJECTABLE-STORES-001

<!-- evidence-field:id markdown -->
## Id

E-INJECTABLE-STORES-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M32
- M32.1
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-006
- JST-ARCH-001
- JST-ARCH-002
- JST-ARCH-005
- JST-PUB-001
- JST-PUB-004
- JST-PUB-005
- JST-PUB-007
- JST-PUB-009
- JST-LOCAL-001
- JST-LOCAL-003
- JST-OBJ-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

injectable-store-core-extraction
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change stkrmqto; no future commit hash is required.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean working copy at xvnuyqqw/4605a422. The exact local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc baseline passed once before source edits. Agent/Review immutable engines and control-record encoding were coupled to native WAL and no public five-store factory registry existed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/injectable-collaboration-stores.md
- Cargo.lock
- Cargo.toml
- crates/jj-store-core/Cargo.toml
- crates/jj-store-core/LICENSE.md
- crates/jj-store-core/NOTICE.md
- crates/jj-store-core/README.md
- crates/jj-store-core/src/engine/agent.rs
- crates/jj-store-core/src/engine/agent_capture.rs
- crates/jj-store-core/src/engine/agent_capture_aliases/catalog.rs
- crates/jj-store-core/src/engine/agent_capture_aliases/catalog_tests.rs
- crates/jj-store-core/src/engine/agent_capture_aliases/remap.rs
- crates/jj-store-core/src/engine/agent_capture_aliases.rs
- crates/jj-store-core/src/engine/agent_capture_binding.rs
- crates/jj-store-core/src/engine/agent_capture_projection.rs
- crates/jj-store-core/src/engine/agent_capture_records.rs
- crates/jj-store-core/src/engine/agent_capture_results.rs
- crates/jj-store-core/src/engine/agent_capture_tools.rs
- crates/jj-store-core/src/engine/agent_capture_transcript.rs
- crates/jj-store-core/src/engine/agent_catalog.rs
- crates/jj-store-core/src/engine/agent_catalog_engine.rs
- crates/jj-store-core/src/engine/agent_io.rs
- crates/jj-store-core/src/engine/agent_provenance.rs
- crates/jj-store-core/src/engine/agent_read_engine.rs
- crates/jj-store-core/src/engine/agent_reads.rs
- crates/jj-store-core/src/engine/agent_transcript_audit.rs
- crates/jj-store-core/src/engine/agent_transcript_read.rs
- crates/jj-store-core/src/engine/agent_transcript_tail.rs
- crates/jj-store-core/src/engine/mod.rs
- crates/jj-store-core/src/engine/replication_review_graph.rs
- crates/jj-store-core/src/engine/review.rs
- crates/jj-store-core/src/engine/review_audit.rs
- crates/jj-store-core/src/engine/review_audit_events.rs
- crates/jj-store-core/src/engine/review_audit_relations.rs
- crates/jj-store-core/src/engine/review_audit_replication.rs
- crates/jj-store-core/src/engine/review_catalog.rs
- crates/jj-store-core/src/engine/review_catalog_engine.rs
- crates/jj-store-core/src/engine/review_command_apply.rs
- crates/jj-store-core/src/engine/review_command_commit.rs
- crates/jj-store-core/src/engine/review_command_history.rs
- crates/jj-store-core/src/engine/review_command_relations.rs
- crates/jj-store-core/src/engine/review_command_stage.rs
- crates/jj-store-core/src/engine/review_command_update.rs
- crates/jj-store-core/src/engine/review_commands.rs
- crates/jj-store-core/src/engine/review_graph.rs
- crates/jj-store-core/src/engine/review_io.rs
- crates/jj-store-core/src/engine/review_projector.rs
- crates/jj-store-core/src/engine/review_read_engine.rs
- crates/jj-store-core/src/lib.rs
- crates/jj-store-core/src/record_format.rs
- crates/jj-store-core/src/stores/commands.rs
- crates/jj-store-core/src/stores/factories.rs
- crates/jj-store-core/src/stores/mod.rs
- crates/jj-store-core/src/stores/traits.rs
- crates/jj-store-core/tests/factories.rs
- crates/jj-store-core/tests/record_format.rs
- crates/jj-store-object/src/s3.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs
- crates/jj-store-service/src/collaboration/review_activity/tests.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/agent_io.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/review_io.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/review_mutations.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/lib.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_io.rs
- crates/jj-store-wal/src/publication/agent_reads.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication/collab_targets.rs
- crates/jj-store-wal/src/publication/replication_review_graph.rs
- crates/jj-store-wal/src/publication/review_catalog.rs
- crates/jj-store-wal/src/publication/review_command_history.rs
- crates/jj-store-wal/src/publication/review_commands.rs
- crates/jj-store-wal/src/publication/review_io.rs
- crates/jj-store-wal/src/publication/review_reads.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- crates/jj-store-wal/tests/agent_provenance.rs
- crates/jj-store-wal/tests/review_commands.rs
- crates/jj-store-wal/tests/review_projector.rs
- crates/jj-store-wal/tests/review_provider.rs
- crates/jj-store-wal/tests/support/review_provider.rs
- scripts/check-boundaries.mjs
- spec/src/current/contracts/architecture/03-requirements.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/mutations/18-m18-shared-capture-service-semantics.md
- spec/src/current/contracts/object-store/01-object-storage.md
- spec/src/current/contracts/publication/07-selected-agentstore-reads.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M32/SUMMARY.nav.md
- spec/src/current/progress/M32/history/M32/SUMMARY.nav.md
- spec/src/current/progress/M32/history/M32/index.md
- spec/src/current/progress/M32/history/M32.1/SUMMARY.nav.md
- spec/src/current/progress/M32/history/M32.1/index.md
- spec/src/current/progress/M32/history/M32.2/SUMMARY.nav.md
- spec/src/current/progress/M32/history/M32.2/index.md
- spec/src/current/progress/M32/history/M32.3/SUMMARY.nav.md
- spec/src/current/progress/M32/history/M32.3/index.md
- spec/src/current/progress/M32/history/M32.4/SUMMARY.nav.md
- spec/src/current/progress/M32/history/M32.4/index.md
- spec/src/current/progress/M32/history/SUMMARY.nav.md
- spec/src/current/progress/M32/history/index.md
- spec/src/current/progress/M32/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/arch.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog_tests.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/remap.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases.rs
- crates/jj-store-wal/src/publication/agent_capture_binding.rs
- crates/jj-store-wal/src/publication/agent_capture_projection.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/src/publication/agent_capture_results.rs
- crates/jj-store-wal/src/publication/agent_capture_tools.rs
- crates/jj-store-wal/src/publication/agent_capture_transcript.rs
- crates/jj-store-wal/src/publication/agent_catalog_engine.rs
- crates/jj-store-wal/src/publication/agent_provenance.rs
- crates/jj-store-wal/src/publication/agent_read_engine.rs
- crates/jj-store-wal/src/publication/agent_transcript_audit.rs
- crates/jj-store-wal/src/publication/agent_transcript_read.rs
- crates/jj-store-wal/src/publication/agent_transcript_tail.rs
- crates/jj-store-wal/src/publication/review_audit.rs
- crates/jj-store-wal/src/publication/review_audit_events.rs
- crates/jj-store-wal/src/publication/review_audit_relations.rs
- crates/jj-store-wal/src/publication/review_audit_replication.rs
- crates/jj-store-wal/src/publication/review_catalog_engine.rs
- crates/jj-store-wal/src/publication/review_command_apply.rs
- crates/jj-store-wal/src/publication/review_command_commit.rs
- crates/jj-store-wal/src/publication/review_command_relations.rs
- crates/jj-store-wal/src/publication/review_command_stage.rs
- crates/jj-store-wal/src/publication/review_command_update.rs
- crates/jj-store-wal/src/publication/review_graph.rs
- crates/jj-store-wal/src/publication/review_projector.rs
- crates/jj-store-wal/src/publication/review_read_engine.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-core/tests/factories.rs#external_factory_is_independent_and_loading_never_reads_or_initializes
- crates/jj-store-core/tests/factories.rs#invalid_configuration_is_rejected_before_factory_construction
- crates/jj-store-core/tests/factories.rs#provider_binding_mismatch_and_disabled_stores_fail_closed
- crates/jj-store-core/tests/record_format.rs#existing_repository_envelope_bytes_are_preserved
- crates/jj-store-core/tests/record_format.rs#oversized_untrusted_control_bytes_are_rejected_before_json_decoding
- crates/jj-store-service/tests/local_collab.rs#local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc
- crates/jj-store-service/tests/local_agent.rs#git_carrier_preserves_native_record_bytes_and_repository_bound_ids
- crates/jj-store-wal/tests/work_store.rs#uncertain_manifest_keeps_staged_work_invisible_and_retry_identity_stable
<!-- /evidence-field -->


<!-- generated-navigation -->
## Pages

- [E-INJECTABLE-STORES-001 details 2](details-02.md)
- [E-INJECTABLE-STORES-001 details 3](details-03.md)
- [E-INJECTABLE-STORES-001 details 4](details-04.md)
