# Identity and outcomes

Git OIDs carry their hash algorithm and remain distinct from native CommitIds,
ChangeIds, representation profiles, remote identities and request/attempt IDs.
Original Git bytes are retained before acceptance; metadata validation precedes
upstream lossy decoding. Mappings preserve aliases and reject ambiguity rather
than choosing arrival order. New native children use original mapped parent
OIDs.

A frozen push includes full ref names, selected native targets, desired Git
OIDs, known-OID/known-absent expectations and destination binding version.
Unknown expectations are not pushable. The ledger records prepared,
transport-started, result-recorded and applied states, plus
unknown/partial/apply-pending outcomes. Observed desired state is satisfaction
evidence, not proof of attempt authorship. Recovery does not roll back or
silently retransmit an external update.
