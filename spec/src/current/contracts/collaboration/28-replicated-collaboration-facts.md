# Replicated collaboration facts

Authored AgentStore and ReviewStore publications retain their validated typed
deltas in the origin envelope. Import reads the referenced authored root to
prove every replacement exists in the transferred closure, then applies those
accepted facts to the destination's current pinned root. Import never retries a
capture parser, review command or other user action.

AgentStore uses deterministic catalog-specific reduction. Independent stable
keys union. Session snapshots union disjoint evidence, events, transcripts and
tools. A terminal tool state dominates a nonterminal observation; incompatible
terminal results remain an explicit `AgentCatalogConflictV1`. Checkpoint and
association versions follow their declared supersession edges, and source states
advance only when one receipt history is a prefix of the other. Conflicting
alias bindings remain explicit. Relation and projection selections are canonical
unions. An ordinary authored delta rejects direct conflict-object selection so
only the internal replication reducer can construct one.

ReviewStore represents concurrent state with `ReviewCommentConflictV1`,
`ReviewEventGraphV1` and `ReviewThreadMergeV1` while preserving the existing
catalog reference kinds and prior canonical object bytes. Event nodes retain
immutable event identities and causal parents. Independent replies survive;
concurrent edits retain every maximal revision. A resolve that did not observe
later activity cannot close the merged thread, and a reopen concurrent with a
resolve keeps it open. A later resolve whose graph parents cover all activity
can close it.

`ResolveCommentConflict` requires the complete strictly sorted set of currently
selected alternative revision IDs. The resulting visible revision has no single
previous-revision winner and instead names every superseded revision. The
conflict-resolution event retains the same set. Stale or incomplete sets fail
without publication. Relation and status pages are rebuilt atomically with the
reduced thread and comment catalogs.

Root-pinned service reads expose comment alternatives separately, include each
event's causal parents and never present one conflicting revision as canonical.
Normal reply and workflow commands can continue from a merged thread graph.
Native copy and cold recovery retain both AgentStore and ReviewStore conflict
objects and all alternatives. The restricted import capability cannot invoke
ordinary publication, domain-command or native-transition authorization and has
no controller, tool, hook, message or external-adapter dispatch interface.
