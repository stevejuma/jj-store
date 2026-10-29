---
schema: 1
id: E-INJECTABLE-STORES-002
---

# E-INJECTABLE-STORES-002

<!-- evidence-field:id markdown -->
## Id

E-INJECTABLE-STORES-002
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
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-CLI-002
- JST-CLI-004
- JST-GOV-002
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
- JST-ARCH-006
- JST-ARCH-001
- JST-LOCAL-001
- JST-LOCAL-002
- JST-LOCAL-003
- JST-JJ-001
- JST-JJ-002
- JST-JJ-003
- JST-JJ-004
- JST-JJ-005
- JST-PUB-001
- JST-PUB-004
- JST-PUB-005
- JST-PUB-007
- JST-PUB-009
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

standalone-git-provider-extraction
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change vqsskvov; verified standalone Git extraction slice.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean working copy after committed foundation stkrmqto/1f6c155e. Exact local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc baseline passed before edits. Generic Git metadata remained in the JJ adapter and local domain persistence in service, coupled to native ObjectStore/WAL record access.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/standalone-companion-provider.md
- Cargo.lock
- Cargo.toml
- crates/jj-store-cli/tests/local_agent_provenance/fixture.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-core/src/engine/collab_targets.rs
- crates/jj-store-core/src/engine/mod.rs
- crates/jj-store-core/src/stores/commands.rs
- crates/jj-store-core/src/stores/factories.rs
- crates/jj-store-core/src/stores/mod.rs
- crates/jj-store-core/src/stores/traits.rs
- crates/jj-store-core/tests/independent_stores.rs
- crates/jj-store-git/Cargo.toml
- crates/jj-store-git/LICENSE.md
- crates/jj-store-git/NOTICE.md
- crates/jj-store-git/README.md
- crates/jj-store-git/src/domain.rs
- crates/jj-store-git/src/evidence.rs
- crates/jj-store-git/src/factories.rs
- crates/jj-store-git/src/lib.rs
- crates/jj-store-git/src/metadata/blobs.rs
- crates/jj-store-git/src/metadata/evidence_files.rs
- crates/jj-store-git/src/metadata/mod.rs
- crates/jj-store-git/src/metadata/objects.rs
- crates/jj-store-git/src/metadata/prepare.rs
- crates/jj-store-git/src/metadata/types.rs
- crates/jj-store-git/src/records.rs
- crates/jj-store-git/src/repository/actor.rs
- crates/jj-store-git/src/repository/agent_commands.rs
- crates/jj-store-git/src/repository/agent_descriptor.rs
- crates/jj-store-git/src/repository/agent_evidence.rs
- crates/jj-store-git/src/repository/agent_io.rs
- crates/jj-store-git/src/repository/agent_pages.rs
- crates/jj-store-git/src/repository/agent_provenance_pages.rs
- crates/jj-store-git/src/repository/agent_store.rs
- crates/jj-store-git/src/repository/agent_thread_binding.rs
- crates/jj-store-git/src/repository/collab_commands.rs
- crates/jj-store-git/src/repository/collab_external.rs
- crates/jj-store-git/src/repository/collab_graph.rs
- crates/jj-store-git/src/repository/collab_store.rs
- crates/jj-store-git/src/repository/collab_types.rs
- crates/jj-store-git/src/repository/errors.rs
- crates/jj-store-git/src/repository/mod.rs
- crates/jj-store-git/src/repository/review_agent_evidence.rs
- crates/jj-store-git/src/repository/review_commands.rs
- crates/jj-store-git/src/repository/review_io.rs
- crates/jj-store-git/src/repository/review_store.rs
- crates/jj-store-git/src/repository/review_types.rs
- crates/jj-store-git/src/repository/selection.rs
- crates/jj-store-git/src/repository/work.rs
- crates/jj-store-git/src/work_api.rs
- crates/jj-store-git/tests/factories.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/README.md
- crates/jj-store-jj/build.rs
- crates/jj-store-jj/src/codec.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_evidence/capture.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/src/local_evidence/provider.rs
- crates/jj-store-jj/src/local_metadata/evidence_files.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-jj/src/local_metadata/objects.rs
- crates/jj-store-jj/src/local_metadata/stack_prepared.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-jj/src/native_dto.rs
- crates/jj-store-jj/tests/local_evidence.rs
- crates/jj-store-server/src/agent_execution/workspace_sources.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/collab_mutations.rs
- crates/jj-store-service/src/local_repository/actor.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_descriptor.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/src/local_repository/agent_io.rs
- crates/jj-store-service/src/local_repository/agent_pages.rs
- crates/jj-store-service/src/local_repository/agent_provenance_pages.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository/agent_thread_binding.rs
- crates/jj-store-service/src/local_repository/browser_agents.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/local_repository/collab_external.rs
- crates/jj-store-service/src/local_repository/collab_graph.rs
- crates/jj-store-service/src/local_repository/collab_store.rs
- crates/jj-store-service/src/local_repository/collab_types.rs
- crates/jj-store-service/src/local_repository/conversion.rs
- crates/jj-store-service/src/local_repository/conversion_import.rs
- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/records.rs
- crates/jj-store-service/src/local_repository/review_commands.rs
- crates/jj-store-service/src/local_repository/review_io.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/local_repository/review_types.rs
- crates/jj-store-service/src/local_repository/selection.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/work/commands.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-wal/src/publication/collab_targets.rs
- scripts/check-boundaries.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/contracts/cli/40-public-installation.md
- spec/src/current/contracts/jj-integration/03-existing-native-read-behavior.md
- spec/src/current/contracts/local-jj/01-local-git-backed-jj-collaboration.md
- spec/src/current/contracts/mutations/18-m18-shared-capture-service-semantics.md
- spec/src/current/contracts/publication/07-selected-agentstore-reads.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M32/history/M32/index.md
- spec/src/current/progress/M32/history/M32.1/index.md
- spec/src/current/progress/M32/history/M32.2/index.md
- spec/src/current/progress/M32/index.md
- crates/jj-store-jj/src/local_metadata/blobs.rs
- crates/jj-store-jj/src/local_metadata/prepare.rs
- crates/jj-store-service/src/local_repository/review_agent_evidence.rs
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-jj/src/local_metadata/blobs.rs
- crates/jj-store-jj/src/local_metadata/objects.rs
- crates/jj-store-jj/src/local_metadata/prepare.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-service/src/local_repository/actor.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_descriptor.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/agent_io.rs
- crates/jj-store-service/src/local_repository/agent_pages.rs
- crates/jj-store-service/src/local_repository/agent_provenance_pages.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository/agent_thread_binding.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/local_repository/collab_external.rs
- crates/jj-store-service/src/local_repository/collab_graph.rs
- crates/jj-store-service/src/local_repository/collab_store.rs
- crates/jj-store-service/src/local_repository/collab_types.rs
- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/records.rs
- crates/jj-store-service/src/local_repository/review_agent_evidence.rs
- crates/jj-store-service/src/local_repository/review_commands.rs
- crates/jj-store-service/src/local_repository/review_io.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/local_repository/review_types.rs
- crates/jj-store-service/src/local_repository/selection.rs
- crates/jj-store-service/src/local_repository/work.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-git/tests/factories.rs#factories_are_read_only_on_load_and_all_five_domains_reopen_and_replay
- crates/jj-store-core/tests/independent_stores.rs#each_store_is_independently_replaceable_by_an_external_implementation
- crates/jj-store-service/tests/local_collab.rs#local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc
- crates/jj-store-jj/tests/local_evidence.rs#capsule_codec_rejects_wrong_profiles_tampering_and_missing_extra_records
- crates/jj-store-cli/tests/local_agent_provenance.rs#invalid_capsules_commits_sessions_and_dependencies_never_publish
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-INJECTABLE-STORES-002 details 2](details-02.md)
- [E-INJECTABLE-STORES-002 details 3](details-03.md)
- [E-INJECTABLE-STORES-002 details 4](details-04.md)
- [E-INJECTABLE-STORES-002 details 5](details-05.md)
