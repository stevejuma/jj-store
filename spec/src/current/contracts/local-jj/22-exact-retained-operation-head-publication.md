# Exact retained operation-head publication

A separate trusted original-request recovery path MAY publish the retained
prepared operation after complete source operation/view, extra and index
preflight. It MUST retain foreground admission and acquire the actual upstream
head-store lock nonblockingly through held no-follow descriptors. Restore and
validate exact immutable dependencies before selecting any head; never prepare a
replacement command, import refs or load a latest reconciled operation.

Publication uses the exact retained candidate whose sole operation parent is the
saved original. Preserve every concurrent sibling head. When another workspace
has consumed the original head, bounded exact operation ancestry must prove the
current heads descend from the original before adding the candidate. If the
candidate is already selected or reachable through later heads, exact retry
acknowledges that publication without reannouncing an old head or rewinding
current state. Unrelated or incomplete ancestry fails explicitly. Previously
witnessed sibling heads and their proven descendants remain admitted without
replacing them. Bound the proof to 4096 operations, 64 parents per operation, 4
MiB per source operation record and 16 MiB aggregate. Verify native content
identities while decoding; source substitution cannot prove ancestry.

Sync the create-only candidate marker before removing only the saved original
marker, under the upstream head lock. Authorization loss, cancellation or
process loss after candidate selection may leave both heads; original-request
retry verifies and completes this transition. Revalidate the original checkout,
sparse/ignore policy, installation binding and lock/directory identities.
Metadata head observation may ignore only bounded safe native coordination locks
and exact recovery staging files. This is code publication only: physical files,
checkout bookkeeping and companion/combination selection remain unchanged and
require their separately qualified guards and durable receipts. Runtime writable
capabilities remain unavailable until that complete protocol is composed. Head
enumeration admits at most 64 markers and 128 total entries; lock and exact
recovery staging names must refer to empty no-follow singly linked regular
files. Insufficient intermediate marker capacity fails before selection.

The provider-only publication proof MUST bind the original retained preparation
key and recovery witness digest, alongside the exact operation/view; it cannot
serve as a physical-checkout completion receipt.
