# Durable tree-changing checkout inputs

Tree-changing plans MUST be selected in a separately bounded
installation-private store before their original journal reference is selected.
The store binds the complete physical scope and authenticated request to
canonical plan identity and retained code/archive/recovery linkage. Open and
lookup do not initialize state; exact duplicate selection syncs the retained
bytes instead of replacing them. Conflicting reuse, unsafe/replaced files and
exhausted capacity fail without discarding historical plans. A selected plan
with a missing journal reference is an interrupted preparation, not a published
command or an invitation to execute.

The command journal's versioned plan reference MUST match the original
principal, intent, preparation, native candidate, physical guard,
source/candidate trees and exact affected paths. Plan selection precedes journal
selection under one documented lock order. Unknown acknowledgements remain
unknown; exact retry reuses the same retained plan/reference. No new reference
may be attached to a terminal request. Tree-changing publication admission
requires its saved reference; loading and provider revalidation remain mandatory
before execution. Neither the store nor journal reference is a
checkout-completion proof.

The plan catalog admits at most 128 immutable records and 64 MiB encoded
aggregate, with canonical plan bodies bounded to 32 MiB. Its private lock and
temporary namespace are separate from the 8-MiB command journal. The lock order
is command journal then plan catalog; plan acknowledgement and sync precede the
journal reference. Version-4 journal catalogs add only a digest and encoded
length, mutually exclusive with the unchanged-tree witness. Legacy versions 1–3
retain their original serialization and validation. Saved inputs never expand
the journal admission budget or evict an old request.
