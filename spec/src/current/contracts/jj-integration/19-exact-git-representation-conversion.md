# Exact Git representation conversion

Git conversion remains an owned adapter in this crate. It uses a private pinned
GitBackend as a conversion tool and translates explicit parent/tree/file IDs
into native identities. Original raw bytes and required conversion evidence
remain native durable dependencies. Imported metadata is validated before the
pinned decoder can normalize or replace it. Cache removal must not reidentify
accepted imported commits or their exported children.

The native Git RPC accepts typed records and optional native head transitions,
never an arbitrary Git command, endpoint, selected principal or raw manifest.
The service verifies conversion evidence using the pinned integration crate and
rechecks repository write access before publication. The CLI may bind a pending
Git delta to its existing request-scoped OpHeadsStore publication so normal JJ
transaction/rebase/workspace finishing performs only one acceptance.

An invocation-owned factory registry shares the native client and a scoped
pending Git delta across backend/operation/head adapters. The pending guard is
installed only around the Git command's final transaction. Snapshot and ordinary
head reconciliation remain ordinary native publications, and the guard clears on
success, failure or cancellation. No global principal or pending request is
introduced.

Git push applies only the frozen remote bookmark target and tracking state. It
never resets a local bookmark to a previously uploaded target. The invoking
transaction retains JJ operation/rebase handling and concurrent sibling heads;
causally incomparable remote observations remain durable conflicts. Native undo
can change its code view while retaining external attempt evidence in the Git
root. Private conversion rebuilds the selected closure from retained mappings
and original bodies before any transport-started event.

The native operation reducer also owns data-only verification of imported Git
bytes, mappings and frozen bookmark targets. A new imported push intent proves
its source against accepted published code history; the replica inventory alone
cannot authorize it. Verification uses isolated conversion scratch state and
never invokes Git transport or reconciles native heads.

Git recovery consumers merge the frozen native remote-ref base, current target
and confirmed result with the pinned `merge_ref_targets` algorithm. The frozen
base must match the durable Git lease; missing or ambiguous evidence refuses
apply. A later observation cannot be rolled back by repairing an older attempt.
The Git stage fault fixture exercises before/after manifest-write uncertainty
for activation, intent, started, result and applied facts through cold replay.
