# Native WorkStore

The neutral jj-work crate owns project, task and comment models and command
validation. WAL owns WorkStore radix catalogs and publication; service and
transports consume owned contracts. See [native projects](../projects/index.md).

WorkStore enablement preserves and freezes an existing origin ledger. Native
repository creation initializes that ledger independently of an upstream, so its
presence cannot reject local task creation. Format-3 publication omits new M10
origins; protocol negotiation, feed, offer, closure, import and seed routes
reject the unsupported profile explicitly. The local WAL and native copy retain
complete WorkStore authority. Existing configured replication becomes
unavailable for a WorkStore home; WorkStore does not claim remote convergence.
