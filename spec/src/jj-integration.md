# Native JJ integration

Commit ancestry, operation ancestry, evolution, and publication order are four
separate histories. Multiple operation heads are legal. CLI repository opening
can use native reconciliation, but HTTP GET cannot. Backend copy lookup and
submodule data need full coverage beyond the reference SimpleBackend.

## Requirements

| ID         | Requirement                                                                                                                                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-JJ-001 | Custom CliRunner factories MUST implement the exact pinned Backend, OpStore, and OpHeadsStore contracts over RPC, with equivalent in-process stores on the server.                   |
| JST-JJ-002 | Only guarded OpHeadsStore publication MUST advance authority; request-scoped principal, intent, snapshot, and receipt data MUST NOT use global mutable context.                      |
| JST-JJ-003 | Read sessions MUST pin explicit operations through load_at and MUST NOT reconcile heads; authorized mutation sessions MUST use native transactions and finish descendant rewrites.   |
| JST-JJ-004 | Physical checkout bookkeeping MUST stay client-local; logical workspace IDs MUST be distinct across unrelated clients and stale-workspace updates MUST preserve unsnapshotted edits. |
| JST-JJ-005 | Indexes and prefix lookup MUST be bounded, rebuildable, and versioned by native profile; normal required methods MUST NOT contain silent stubs.                                      |

## M0 extension qualification

M0 probes register a request-scoped OpHeadsStore through StoreFactories,
preserve per-principal publication IDs and exact preconditions across real
loopback gRPC, and load explicitly pinned divergent operations in a bounded
local async task. The RPC fixture is test-only and does not implement durable
publication or policy.

## Native codec boundary

The integration crate owns explicit conversions for all pinned commit, tree,
copy, view, and operation fields. File and symlink IDs hash raw bytes; the other
kinds hash the full native value. Submodule IDs are preserved as opaque external
links. Decoding validates structure before invoking native constructors that
would otherwise panic. Physical integrity alone never establishes native
semantic identity.

## Checked native object access

Native object access verifies the expected kind/ID, envelope checksum and
decoded identity before returning metadata. Immutable-create collisions load and
compare the complete existing native value. File staging and verification stream
with 64 KiB buffers and configured disk limits. A file reader is exposed only
after full checksum and BLAKE2b verification. Unknown upload outcomes remain
explicit unless a subsequent read proves the identical immutable object is
present.
