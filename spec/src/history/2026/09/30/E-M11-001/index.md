---
schema: 1
id: E-M11-001
---

# E-M11-001

<!-- evidence-field:id markdown -->
## Id

E-M11-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-workstore
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-cli/jj-lib 0.45.1; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest 3 with optional WorkStore selections in WAL domains; typed repository-bound WorkRootV1 and WorkObjectV1 envelopes; existing format-1/2 canonical bytes preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication/application.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/publication/replication_seed.rs
- crates/jj-store-wal/src/publication/work.rs
- crates/jj-store-wal/src/publication/work_commands.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/work_store.rs
- crates/jj-work/Cargo.toml
- crates/jj-work/src/apply.rs
- crates/jj-work/src/commands.rs
- crates/jj-work/src/ids.rs
- crates/jj-work/src/lib.rs
- crates/jj-work/src/state.rs
- crates/jj-work/src/validation.rs
- crates/jj-work/tests/commands.rs
- scripts/check-boundaries.mjs
- spec/src/SUMMARY.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/replication.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/work_store.rs#workstore_receipt_survives_checkpoint_and_reconstruction
- crates/jj-store-wal/tests/work_store.rs#linked_tasks_comments_pins_and_native_copy
- crates/jj-store-wal/tests/work_store.rs#workstore_and_origin_replication_are_mutually_excluded
- crates/jj-store-wal/tests/work_store.rs#uncertain_manifest_keeps_staged_work_invisible_and_retry_identity_stable
- crates/jj-work/tests/commands.rs#rejects_cross_project_hierarchy_cross_task_replies_and_invalid_admission
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-001 details 2](details-02.md)
- [E-M11-001 details 3](details-03.md)
