---
schema: 1
id: E-M17-023
---

# E-M17-023

<!-- evidence-field:id markdown -->
## Id

E-M17-023
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

bounded-native-round-robin-exact-partition-head-selection-and-live-order-recheck
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_publication/exact.rs
- crates/jj-store-capture/src/agent_publication/schedule.rs
- crates/jj-store-capture/src/agent_publication.rs
- crates/jj-store-cli/tests/agent_capture_publication/scheduling.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_publication.rs#exact_entry_cannot_bypass_an_earlier_same_partition_intent
- crates/jj-store-cli/tests/agent_capture_publication.rs#exact_queued_entry_drain_preserves_other_batches_and_prepared_retry
- crates/jj-store-cli/tests/agent_capture_publication/scheduling.rs#scheduler_rotates_unknown_partition_and_retries_exact_prepared_head
- crates/jj-store-cli/tests/agent_capture_publication/scheduling.rs#scheduler_withheld_turns_dynamic_partitions_and_invalid_input_preserve_order
- crates/jj-store-cli/tests/agent_capture_publication/scheduling.rs#local_prepared_intent_excludes_its_complete_native_partition
- crates/jj-store-cli/tests/agent_capture_publication/scheduling.rs#scheduler_heads_keep_generation_sequence_and_integration_partitions_distinct
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-023 details 2](details-02.md)
- [E-M17-023 details 3](details-03.md)
