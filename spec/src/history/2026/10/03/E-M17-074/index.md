---
schema: 1
id: E-M17-074
---

# E-M17-074

<!-- evidence-field:id markdown -->
## Id

E-M17-074
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

opencode-retained-library-supervision-independent-publication
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change txyzzyku
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_supervisor/delivery.rs
- crates/jj-store-server/src/capture_supervisor/lifetime.rs
- crates/jj-store-server/src/capture_supervisor/opencode/delivery.rs
- crates/jj-store-server/src/capture_supervisor/opencode/discovery.rs
- crates/jj-store-server/src/capture_supervisor/opencode/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/opencode/source.rs
- crates/jj-store-server/src/capture_supervisor/opencode/tests.rs
- crates/jj-store-server/src/capture_supervisor/opencode.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/sources.rs
- crates/jj-store-server/src/capture_supervisor/status/harness.rs
- crates/jj-store-server/src/capture_supervisor/status.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/opencode_read_api.rs
- crates/jj-store-server/src/opencode_source/ingress.rs
- crates/jj-store-server/src/opencode_source/publication.rs
- crates/jj-store-server/src/opencode_source/snapshot.rs
- crates/jj-store-server/src/opencode_source.rs
- crates/jj-store-server/tests/opencode_source/fixture.rs
- crates/jj-store-server/tests/opencode_source/scheduling.rs
- crates/jj-store-server/tests/opencode_source.rs
- crates/jj-store-server/tests/opencode_supervisor.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/opencode_supervisor.rs#api_runtime_automatically_backfills_follows_pause_resume_and_offline_restart
- crates/jj-store-server/tests/opencode_supervisor.rs#inactive_api_history_waits_for_activity_and_paused_preparation_has_no_acquisition
- crates/jj-store-server/tests/opencode_supervisor.rs#codex_and_api_share_one_reader_without_starving_either_harness
- crates/jj-store-server/tests/opencode_source/scheduling.rs#settled_api_releases_acquisition_and_revoked_partition_yields_without_reordering
- crates/jj-store-server/src/capture_supervisor/opencode/tests.rs#blocked_api_publication_retains_drain_ownership_while_ingress_continues
- crates/jj-store-server/src/capture_supervisor/opencode/tests.rs#independent_api_proof_retains_lease_and_refuses_changed_sanitized_entry
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-074 details 2](details-02.md)
- [E-M17-074 details 3](details-03.md)
