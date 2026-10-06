---
schema: 1
id: E-M17-024
---

# E-M17-024

<!-- evidence-field:id markdown -->
## Id

E-M17-024
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

independent-bounded-codex-ingress-and-exact-queued-publication-lanes
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_source/ingress.rs
- crates/jj-store-server/src/capture_source/pipeline.rs
- crates/jj-store-server/src/capture_source/recovery.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/tests/capture_pipeline.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_pipeline.rs#revoked_pending_repository_does_not_block_later_independent_ingress_and_publication
- crates/jj-store-server/tests/capture_pipeline.rs#resumed_pipeline_ingests_offline_tail_past_unknown_intent_without_rewinding_anchor
- crates/jj-store-server/tests/capture_pipeline.rs#independent_ingress_preserves_same_partition_order_across_a_b_a
- crates/jj-store-server/tests/capture_pipeline.rs#pipeline_backpressure_keeps_exact_batch_and_requires_current_write_and_same_spool
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-024 details 2](details-02.md)
- [E-M17-024 details 3](details-03.md)
