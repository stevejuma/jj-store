# Original full physical inventory for checkout recovery

Tree-changing installation MUST retain the complete original physical inventory
before publication or physical writes, not only its final digest. Original
sorted file fingerprints and full directory stamps must reproduce the existing
scope-bound scanner fingerprint. Empty directories, ignored/untracked paths,
symlinks and unaffected editor changes remain in that input. Canonical inventory
encoding admits at most 4097 total directory/file records and 8 MiB; source
scanning retains its existing two-pass entry/byte/depth/deadline bounds. Missing
parents and the exact planned effects remain separately bound by the tree plan.

The service-owned physical guard MAY capture this data under its existing
registered access, shared scanner admission and current authorization. It MUST
verify the original repository/generation/physical identity and complete
expected fingerprint, keep admission until canceled workers exit, and preserve
ordinary validation behavior. A guard without this capability fails explicitly.
Reopening canonical inventory data is not a completion proof or permission to
normalize new editor changes. Durable linkage, exact provider-owned installation
progress, restricted normalization of only planned transitions and verified
after-state remain required before interrupted physical checkout can resume.

The private plan catalog MUST select the original complete inventory immutably
before the command journal selects its reference, using journal-then-plan lock
order. The reference binds inventory digest/encoded length and exact saved plan
digest; the body must reproduce that plan's original physical fingerprint and
the journal's original repository/generation/workspace binding. Authenticate the
original principal/request/intent/prepared selection before inventory storage
access. Exact replay cannot replace an inventory, reconstruct a missing selected
body from caller input, or attach a new reference after Prepared. An orphan
inventory is interrupted preparation, not permission to publish or install.

Catalog version 2 adds optional saved inventory to immutable plan records;
inventory-free version-1 catalogs retain canonical bytes. The existing
128-record and 64-MiB aggregate limits include the inventory body, whose own
bound remains 8 MiB. Journal version 5 adds only the inventory reference and
requires its existing tree-plan reference. Versions 1–4 retain absent-field
serialization and read behavior. New tree-changing publication admission MUST
require both saved references. Historical records lacking inventory remain
readable and cannot gain new execution input after Prepared. No capacity
eviction, physical write or completion proof follows from this selection.
