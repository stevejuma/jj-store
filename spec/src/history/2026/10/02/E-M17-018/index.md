---
schema: 1
id: E-M17-018
---

# E-M17-018

<!-- evidence-field:id markdown -->
## Id

E-M17-018
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

bounded-native-repository-source-partition-publication
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_publication.rs
- crates/jj-store-capture/src/agent_publication/drain.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_publication.rs#bounded_partition_drain_retains_other_repositories_sources_and_unknown_intent
- crates/jj-store-cli/tests/agent_capture_publication.rs#partition_preserves_foreign_local_intent_and_rejects_selected_local_before_preparation
- crates/jj-store-cli/tests/agent_capture_publication.rs#partition_mismatched_receipt_retains_exact_intent_without_touching_other_source
- crates/jj-store-cli/tests/agent_capture_publication.rs#cancelled_partition_releases_drain_lease_and_retries_exact_prepared_request
- crates/jj-store-capture/tests/reader_progress.rs#native_and_local_prepared_intents_recover_missing_progress_and_keep_it_after_completion
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-018 details 2](details-02.md)
- [E-M17-018 details 3](details-03.md)
