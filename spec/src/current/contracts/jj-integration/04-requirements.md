# Requirements

| ID         | Requirement                                                                                                                                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-JJ-001 | Custom CliRunner factories MUST implement the exact pinned Backend, OpStore, and OpHeadsStore contracts over RPC, with equivalent in-process stores on the server.                   |
| JST-JJ-002 | Only guarded OpHeadsStore publication MUST advance authority; request-scoped principal, intent, snapshot, and receipt data MUST NOT use global mutable context.                      |
| JST-JJ-003 | Read sessions MUST pin explicit operations through load_at and MUST NOT reconcile heads; authorized mutation sessions MUST use native transactions and finish descendant rewrites.   |
| JST-JJ-004 | Physical checkout bookkeeping MUST stay client-local; logical workspace IDs MUST be distinct across unrelated clients and stale-workspace updates MUST preserve unsnapshotted edits. |
| JST-JJ-005 | Indexes and prefix lookup MUST be bounded, rebuildable, and versioned by native profile; normal required methods MUST NOT contain silent stubs.                                      |
