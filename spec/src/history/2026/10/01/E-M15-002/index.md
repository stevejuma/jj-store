---
schema: 1
id: E-M15-002
---

# E-M15-002

<!-- evidence-field:id markdown -->
## Id

E-M15-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.1
- M15.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-memory-and-local-filesystem
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qvwrlxto
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Canonical domain slice committed at ba78eb90. Exact new WAL collab_persistence target absent before implementation. Native formats 1-4 and local companion version 1 contain no Thread/Reaction roots.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; pinned gix 0.87.1 and existing Git; image 0.25.10
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive native manifest 5; optional independent Thread/Reaction root/version fields; repository-bound CollabObject/ThreadRoot/ReactionRoot V1 records with bounded canonical-object inventories. Existing native fields omit new empty values. Local companion version 2 supports independent Thread/Reaction domains and atomic multi-domain receipts under unchanged refs/lapis/state/v1.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-jj/src/local_metadata/prepare.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-jj/tests/local_metadata.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/collab_store.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_seed.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/collab_persistence.rs
- crates/jj-store-wal/tests/publication.rs
- scripts/check-boundaries.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/replication.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_metadata.rs#additive_collaboration_batch_selects_both_domains_and_preserves_legacy_receipts
- crates/jj-store-jj/tests/local_metadata.rs#stale_batch_precondition_cannot_partially_publish_any_domain
- crates/jj-store-wal/tests/collab_persistence.rs#explicit_feature_use_publishes_format_five_without_backfill_or_code_changes
- crates/jj-store-wal/tests/collab_persistence.rs#cold_receipts_checkpoint_and_manifest_last_copy_retain_human_revision_reactions
- crates/jj-store-wal/tests/collab_persistence.rs#custom_emoji_assets_and_static_previews_survive_native_copy
- crates/jj-store-wal/tests/collab_persistence.rs#atomic_domain_selection_replays_one_receipt_and_never_partially_accepts_stale_roots
- crates/jj-store-wal/tests/collab_persistence.rs#each_atomic_domain_requires_authorization_before_any_root_is_selected
- crates/jj-store-wal/tests/collab_persistence.rs#legacy_manifest_and_domain_request_encoding_omit_all_additive_fields
- crates/jj-store-wal/tests/collab_persistence.rs#work_homes_keep_origin_ledger_frozen_while_accepting_local_threads
- crates/jj-store-wal/tests/collab_persistence.rs#replicated_code_repositories_reject_unqualified_collaboration_without_changing_authority
- crates/jj-store-wal/tests/collab_persistence.rs#existing_domain_publication_keeps_format_five_and_thread_roots_pinned
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-002 details 2](details-02.md)
- [E-M15-002 details 3](details-03.md)
