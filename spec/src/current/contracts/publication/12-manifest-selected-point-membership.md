# Manifest-selected point membership

Manifest-selected inventory scan pages seek an exclusive radix key without
replaying earlier subtrees. Each page admits at most 256 descriptors, 1,024
catalog and descriptor reads combined, and 32 MiB of encoded control records
before decoding. Reserve at most 768 reads and 8 MiB for catalog paths and keep
descriptor admission independent, using the remaining total byte allowance. This
permits the existing 16 MiB record ceiling without changing stored formats. If
work/byte admission fills after a verified nonempty prefix, return its last
verified key as a continuation. A zero-progress admission failure remains an
error; a short window never claims complete enumeration. Validate every loaded
radix prefix, checksum, envelope and descriptor key. Decode descriptors
sequentially and retain only object identities. This read is a bounded scan of a
selected catalog, not a fresh complete-closure audit; publication, recovery and
integrity retain their complete audits. Inventory keys hash both kind and
identity, so sparse kind filters may produce empty windows with a continuation.
Never represent such a window as the complete inventory. No selected root or
authoritative format changes.

Checked snapshots may compare their immutable inventory roots within one
repository and authority epoch. Addition pages skip equal record references and
visit only changed radix paths, validating that every visited baseline entry
retains its exact descriptor reference. Leaf/branch transitions partition only
one admitted leaf; the frontier admits at most 1,024 entries. Use the same
exclusive digest anchors, page-size, read and encoded-byte limits as ordinary
inventory windows, and hydrate only newly selected descriptors. Missing or
replaced baseline entries fail closed. A nonempty admitted prefix may continue;
zero-progress exhaustion remains an error. Only complete traversal from the
start proves that a derived baseline was retained; a partial or anchored page
cannot certify an entire root. Callers retain current authorization, deadline
and epoch checks. This optimization cannot select staged objects, replace
publication/recovery closure audits or change any persisted format.

Application readers may verify one object against a checked manifest's immutable
inventory catalog root without materializing the whole repository. The radix
path and inventory record identity are verified; each dependency followed by a
reader requires its own selected membership check. A point read never enumerates
all references in a view or descendants of a tree. An existing object-store key
is never proof of committed membership. Complete closure audits remain required
by publication, recovery and explicit integrity operations. Later publications
cannot replace a retained reader's catalog root.

Explicit derived-index tasks may retain this checked root beyond the initiating
page deadline. Their execution is separately bounded and authorization remains
current; they do not reuse the expired page capability. Positive membership
memoization is a bounded optimization, not a growing authority inventory.
Eviction requires a new verified lookup at the same root and cannot admit
objects from later publications or staged storage.
