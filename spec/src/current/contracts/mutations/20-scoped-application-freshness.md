# Scoped application freshness

Application freshness uses bounded process-local semantic notices, separate from
repository authority and the existing durable native WAL event cursors. Shared
monitors compare committed manifest/domain witnesses and external local heads
and companion references, never reconstruct history or reconcile during reads.
Current authorization gates subscription and delivery; internal monitor state
never grants access. A restart, overflow or authorization-scope mismatch resets
continuity. Unchanged observations and stream renewal produce no data reload.

The owned agent manager may expose a bounded subscription for installation-local
execution and settings settlement. Consumers reopen authorized status/default
reads; notices contain no command body and never retry a mutation. JST-MUT-001
canonical intent, snapshot preconditions and durable receipt semantics remain
unchanged. Agent owner notices do not publish repository state.

Local prepared-query task lookup selects a reference request's embedded source
operation. Before using that generation or reporting its cold rebuild, the owned
query executor proves the retained outer/source relation through bounded parent
metadata. Preparation stays explicit and operation-bound; task availability
never establishes authorization or broadens the read selection.
