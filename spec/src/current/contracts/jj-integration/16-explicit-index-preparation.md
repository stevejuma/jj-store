# Explicit index preparation

Explicit native kind preparation retains a checked source/repository/epoch/root,
exclusive cursor and matching identities independently of an ordinary GET's
two-million-descriptor ceiling. Each advance inspects at most 256 descriptors
under its own call deadline, while original total descriptor, record-read,
encoded-byte, matching-ID, allocation and lifetime limits never renew. Reserve
the bounded window's work before I/O; unknown or canceled windows retain that
charge and leave their cursor/identities unchanged. Advance cursor/identities
only after successful verified window reads and fresh authorization/epoch
checks. Partial progress grants no complete IDs or cache certification. Only
terminal traversal from the start promotes a complete kind projection. Shared
allocation leases precede growth and follow active progress/data owners beyond
cache eviction. Source capabilities and credentials stay caller-owned; later
publication cannot replace a preparation root. The source primitive changes no
authoritative format. Supervised native kind tasks expose bounded metadata
through their separate HTTP contract and share the serving instance's reserved
preparation lease with native/local graph preparation. The lease follows actual
execution through cancel/expiry/teardown; interactive capacity remains
available.

Graph execution has a separate inline cold-build threshold (1000 indexed commits
by default) within the hard index-admission budget. A missing or damaged cache
stops with IndexRebuildRequired as soon as verified commit reads reach that
threshold, and discards incomplete staging. A zero threshold rejects before
construction. A verified warm cache remains usable. An explicit preparation job
loads the exact selected operation and builds/checks the native index without
evaluating a user revset or publishing repository state. Its report binds the
operation and local cache outcome; it does not pin that cache against later
eviction or loss.

Preparation uses the same byte, shape, work, index-count and elapsed bounds as
graph execution, with its reserved shared background slot. It can bypass only
the inline threshold. An owned cancellation signal stops pending waits and is
checked around synchronous native steps; awaiting preparation returns only after
the worker exits. The service retains task identity/status and fresh request
authorization separately.

A verified file may select one exact byte range before its first chunk.
Selection uses the already-verified spool, retains authorization/deadline/worker
ownership, and cannot seek again or reset a consumed stream. HTTP range
normalization stays in the transport; native content verification is unchanged.

Owned native metadata and graph/content DTOs supply schema traits to the HTTP
contract generator through service exports. Native metadata schema names are
prefixed to distinguish them from query presentation types. Schema derives do
not reorder fields, change Serde attributes or modify native payload identity.
No upstream JJ type enters generated application contracts.

Typed command preparation owns native transactions in this crate. It stages an
unpublished operation under a bounded authority that cannot publish heads.
New/Describe and bookmark updates use pinned native helpers; descendant rewrites
and logical workspace/bookmark effects are reported from the resulting view and
operation predecessor map. Physical working copies and user configuration do not
participate. Read-only query stores retain their unconditional write denial;
command staging adds only its own immutable native objects to read membership.

Prepared command reports can be decoded from manifest-selected result receipts;
these DTOs carry no native session or publication capability. Command shape
validation is shared with canonical intent construction before serialization.
Native execution still runs anew for a noncommitted apply and stages only
immutable objects until guarded publication.

EditFile loads the exact ordinary source-file term from one visible commit named
by a single-target local bookmark. It preserves the native executable bit and
copy identity, rejects a rename over an existing destination, writes the
replacement tree, creates one child commit and moves only that bookmark in the
same unpublished transaction. It does not rewrite descendants, update a physical
working copy or publish the staged operation. Current authorization is rechecked
around immutable object writes, and the guarded mutation layer remains the only
publication boundary.

Explicit typed reconciliation uses RepoLoader::merge_operations on the exact
current heads, with the requested base first and the other heads sorted by full
ID. Native common-ancestor discovery, recursive ancestor merges and descendant
rebase semantics remain upstream-owned. The returned operation remains
unpublished. Report comparison is against the explicit base view. Preflight
loads the bounded reachable operation ancestry and reserves its squared node
count times head count from the command work budget before native merging.
Requests exceeding these conservative bounds fail; limits are not throughput or
production CPU qualification.

Typed abandon follows the pinned abandon command's native descendant-transform
and RewriteRefsOptions behavior. Target visibility comes from the native index;
hidden committed targets are skipped and absent targets fail. Native reference
rewrites create replacement working-copy commits and update conflicted
bookmarks. Reports retain exact abandoned/hidden IDs without deleting native
objects. New optional empty-skipped report fields preserve older V1 receipt
canonical bytes.

Typed conflict resolution shares read-side native term construction and
fingerprinting. A FilesMatcher selects exact conflict leaves; a complete
preflight validates every path before MergedTreeBuilder changes the tree. Native
rewrite_commit/rebase_descendants maintain change identity, references and
evolution. Ordered term selection retains file/copy/mode, symlink, submodule and
directory values. It never invokes external conflict tools.

UploadedFile resolution imports only service-checked upload files into its
private command read inventory. The executor verifies their complete native
content before tree writes and uses explicit executable/copy-term choices.
Ordinary read inventories remain committed-only. Mixed uploaded/term/deletion
batches share one original tree preflight, native descendant rewrite and guarded
publication. No upstream trait or native codec changes are needed.

Conflict-resolution impact reports enumerate native remaining paths on every
resulting rewritten or created commit under the same command budget and current
authorization. Sorted target/selected-path and per-result conflict data remain
owned DTOs. An optional empty-skipped report extension preserves canonical bytes
for older Prepared/StoredResult V1 receipts.
