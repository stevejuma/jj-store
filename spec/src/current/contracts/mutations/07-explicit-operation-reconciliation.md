# Explicit operation reconciliation

ReconcileOperations names an explicit base operation from the exact expected
current head set. All current heads participate; the base is the first merge
parent and the comparison view for reported bookmark/workspace changes. It is
not a caller-selected merge ancestor. Pinned JJ computes common ancestors,
including recursive merges, and finishes descendant rebases. Single-head,
missing-base and stale/subset contexts fail without publication. A changed base
under an already committed request ID is conflicting intent.

Preview stages the native merged operation without publishing. Apply uses the
same strict guarded receipt path and emits HeadsNormalized after manifest
selection. Native command/report persistence retains its existing V1 fields;
adding the command variant does not change canonical bytes for older receipts.
Reconciliation shares command worker and metadata/staging/deadline limits. Its
reachable operation graph is preflighted and a conservative operation-pair/head
work budget is reserved before native recursive merge execution.
