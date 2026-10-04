import type { SyncSettings } from "../../web/src/lib/controller/sync-settings.js";
export function syncWorker(): NonNullable<
  NonNullable<SyncSettings["value"]>["worker"]
> {
  return {
    version: 1,
    repository: "ab".repeat(16),
    local_ready: true,
    offline_ready: true,
    phase: "running",
    local_replica: null,
    peer_replica: null,
    local_frontier: null,
    peer_frontier: null,
    pushed: null,
    pulled: null,
    local_pending_positions: 2,
    peer_pending_positions: 3,
    checkpoint_revision: 7,
    operation_heads: 2,
    reconciliation_required: true,
    push: {
      consecutive_failures: 1,
      circuit_open: false,
      last_fault: "transient_endpoint",
    },
    pull: { consecutive_failures: 0, circuit_open: false, last_fault: null },
    search: {
      configured: true,
      repository_position: 9,
      indexed_position: 7,
      degraded: false,
    },
  };
}
