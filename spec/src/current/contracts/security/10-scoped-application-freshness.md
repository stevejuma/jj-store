# Scoped application freshness

Application freshness uses bounded process-local semantic notices, separate from
repository authority and the existing durable native WAL event cursors. Shared
monitors compare committed manifest/domain witnesses and external local heads
and companion references, never reconstruct history or reconcile during reads.
Current authorization gates subscription and delivery; internal monitor state
never grants access. A restart, overflow or authorization-scope mismatch resets
continuity. Unchanged observations and stream renewal produce no data reload.
