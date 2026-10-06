# Native WorkStore

WorkStore enablement preserves and freezes an existing origin ledger. Native
repository creation initializes that ledger independently of an upstream, so its
presence cannot reject local task creation. Format-3 publication omits new M10
origins; protocol negotiation, feed, offer, closure, import and seed routes
reject the unsupported profile explicitly. The local WAL and native copy retain
complete WorkStore authority. Existing configured replication becomes
unavailable for a WorkStore home; WorkStore does not claim remote convergence.
