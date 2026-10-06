---
schema: 1
id: E-READ-035
---

# E-READ-035 details 2

<!-- evidence-field:remaining list -->
## Remaining

- Existing verified primary disk indexes without a retained resident still
  load/decode/build their projection under the namespace lease. Separating this
  recovery path remains native performance delivery work; this slice covers
  missing-generation construction.
- Lightweight repository summaries, local WorkStore/root reuse, local
  operation/prefix/comparison/deep/predecessor/copy/search and paged native RPC
  remain implementation work.
- Complete combined integration/contracts, Linux/native scale fixtures, isolated
  RSS/release latency, cold production browser,
  restart/revocation/publication/fault/real-RustFS, registry-only and
  Help/source-review qualification remain deferred. No milestone promotion.
  Main/original dev services/Linux source/visual baselines unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue high-priority lightweight/root-bound metadata reads, remaining native disk-projection recovery and local operation/filtered/deep lookup performance delivery; complete combined qualification later.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Keep missing native primary construction outside the namespace while preserving shared quotas, immutable generations, current authorization and canceled-worker reservation cleanup.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#cold_primary_construction_releases_namespace_and_preserves_other_generations
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#canceled_primary_installation_releases_its_reservation_while_namespace_is_busy
- crates/jj-store-jj/src/query/cache/constructed.rs#private_output_requires_exact_operation_link_and_segment_checksum
- crates/jj-store-jj/src/query/cache/reservation.rs#reservations_share_native_repository_total_and_lifetime_admission
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
- crates/jj-store-jj/src/query/cache.rs#unretained_primary_cleanup_removes_private_and_renamed_generations
<!-- /evidence-field -->

