---
schema: 1
id: E-M17-042
---

# E-M17-042

<!-- evidence-field:id markdown -->
## Id

E-M17-042
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

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/process/authority.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/running.rs
- crates/jj-store-server/src/process/tests.rs
- crates/jj-store-server/src/storage.rs
- crates/jj-store-server/src/storage/lifetime.rs
- crates/jj-store-server/src/storage/lifetime/tests.rs
- crates/jj-store-server/src/storage/lifetime/tests/attachment.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/process/tests.rs#cold_foreground_state_lock_follows_capture_storage_after_drop_and_cancel
- crates/jj-store-server/src/storage/lifetime/tests/attachment.rs#late_lock_attachment_preserves_capture_lifetime_and_releases_rejected_lock
- crates/jj-store-server/src/storage/lifetime/tests/attachment.rs#duplicate_lock_attachment_preserves_original_and_releases_rejected_lock
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-042 details 2](details-02.md)
- [E-M17-042 details 3](details-03.md)
