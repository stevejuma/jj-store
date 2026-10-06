# Native WorkStore

WorkStore format 3 adds an independent work root/version and optional work
domain in WAL before/after selections. The first task command publishes
enablement and its typed records atomically, preserving existing authority.
Recovery and native copy audit every historical WorkStore catalog. See
[native projects](../projects/index.md).

WorkStore service consumers use WAL-owned neutral error/type exports; this does
not alter record bytes or move upstream JJ imports across the owned boundary.

WorkStore enablement preserves and freezes an existing origin ledger. Native
repository creation initializes that ledger independently of an upstream, so its
presence cannot reject local task creation. Format-3 publication omits new M10
origins; protocol negotiation, feed, offer, closure, import and seed routes
reject the unsupported profile explicitly. The local WAL and native copy retain
complete WorkStore authority. Existing configured replication becomes
unavailable for a WorkStore home; WorkStore does not claim remote convergence.
