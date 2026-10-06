# Bounded application graphs

Core browsing uses the pinned `jj-lib::DefaultIndexStore` and its standard
incremental builder in server-owned storage. One repository/profile/authority
namespace contains JJ's per-operation associations; bookmarks share its graph.
Reuse an indexed ancestor and read only newly needed commit bodies. Native
backend and operation-store adapters preserve committed membership and current
read authorization. Missing core indexes use the existing preparation task
contract; metadata, bookmark pages and scoped content do not require an index.
Core history must not depend on custom visibility, operation, substring or copy
projections. Those advanced optimisations retain their explicit bounded paths
and are outside this simplification's acceptance target.

Local warm reuse copies compatible source index files under existing disk,
file-count, read, memory and cancellation bounds before opening the private copy
with JJ. Only the managed copy may be repaired. The existing read-only Git
commit decoder remains mandatory for any incremental construction. Standard JJ
construction is subject to owned admission; if it cannot meet core acceptance,
report its specific limitation without introducing another custom writer.

Prepared graph visibility and divergence flags use exact visible change groups
from the selected admitted projection, with logarithmic point membership. They
MUST NOT enumerate hidden incarnations merely to describe one selected commit.
Cold readers retain upstream resolution; full prefix/offset contracts are
unchanged.

Prepared evolution pages may reuse the admitted selected operation topology and
retain at most32 evictable data-only predecessor-walk checkpoints per
generation, keyed by exact starting commit and exclusive anchor. Preserve JJ
0.45.1 operation order, within-operation topological sorting, duplicate handling
and the first legacy predecessor boundary. Retain only IDs, pending/queued walk
state and operation ranks; no store, stream, credential or request survives.
Checkpoints require proof that the anchor has exactly one known
predecessor-metadata occurrence in the captured operation closure; unknown or
possibly repeated anchors use bounded replay. The owned disposable
operation-history format2 captures a checksum-bound conservative repeated-ID
bitmap during the existing operation pass. A positive bitmap result only
disables caching; false positives cannot alter results. Format1 remains readable
with checkpoint reuse disabled. Admit copied state and traversal scratch before
growth and charge work/cancellation. Reused anchors and selected rows require
fresh source/operation admission; absent and known-empty predecessor records
remain distinct. Probe continuation using IDs without loading a commit body.
Missing/evicted checkpoints use bounded body-free replay; arbitrary filtered log
continuations remain separate delivery work.

Prepared evolution MUST use an admitted exact commit-to-operation occurrence
lookup to skip unrelated operation bodies. Seek the earliest relevant captured
operation at or after the walk's current rank, considering every pending commit
and the first legacy predecessor-metadata boundary. Preserve repeated IDs,
known-empty records, pending order and within-operation splice/topology
behavior. The disposable operation-history format3 checksum covers map-presence
flags and sorted occurrence identities; derive ranked lookup under resident/work
admission. Fresh readers validate and hydrate only relevant operations and
selected commits. Formats1/2 remain readable through the existing bounded replay
path. No source, credential, request capability or predecessor body is retained
by the lookup.

Prepared default `all()` log pages MUST retain the selected view's exact JJ
visible order and compact identity-to-rank lookup during generation admission.
Deep continuations seek a visible anchor and hydrate only the selected rows;
they MUST NOT replay preceding graph entries or load a body for lookahead.
Preserve ordered direct parent edges for this ancestor-closed set, current
authorization, selected membership, annotations and end-of-history semantics.
Absent or hidden anchors fail explicitly. Admit all retained arrays and sort
work before growth; retain no source, credential, stream or request capability.
Other revsets keep bounded upstream traversal until separately qualified
continuation delivery. Evolution uses the separately governed prepared walker
above; this default-log optimization does not claim general graph seeking.

The graph executor loads only the service-selected operation through load_at,
uses a read-only authority and permits physical object access only to the
captured committed inventory. It runs upstream revset, operation-walk and
evolution algorithms without a server checkout. Graph DTOs retain native IDs,
ordered parents, direct/indirect/missing commit edges, local bookmark conflict
annotations, remote bookmark names with their remote identity, logical
workspaces, visibility and divergence. Local and remote bookmark decorations
remain distinct even when they share a bookmark name. Evolution distinguishes
absent predecessor records from known-empty records.

Requests have explicit page, object-byte, output-byte, index-size, traversal,
revset-size and elapsed-time bounds. Native futures run on a caller-owned
multithread reactor inside bounded blocking jobs; cancellation retains the job
permit until worker exit. Query caches bind upstream revision, native profile,
repository, epoch and operation. Cache construction stages a complete directory
before rename, and a local lock rejects competing cache writers. Losing the
cache causes reconstruction from committed native objects.

Operation graph metadata retains snapshot flags, logical workspace names and
attributes alongside timestamps, user, host and description. A corrupt cache
profile or backend type fails explicitly; removing the derived directory allows
reconstruction. Managed index-file corruption recovery is described below. Large
rebuild task reporting remains separate qualification work.

The current work/deadline checks bound admission, I/O and returned rows. Native
synchronous CPU algorithms cannot be preempted by a Tokio timeout; these limits
do not yet establish production CPU/memory qualification. That qualification
must precede shared HTTP query exposure; managed cache retention is bounded
independently.

Native object DTOs reuse the explicit frozen V1 field schema after native
integrity verification. Exporting those owned types does not change persisted
bytes. Content readers apply committed membership and current authorization
before root handling, metadata reads and stream preparation. Native directory
queries run with read-only backend authority and no signer or physical checkout;
trivial merge semantics are delegated to the pinned library.

Operation-diff reads compare decoded native view fields and traverse
commit-parent links for visible-set changes. They do not infer visibility from
predecessor retention, bookmarks or storage inventories. Immediate
operation-parent selection is explicit; read capabilities and bounded workers
remain mandatory.

The application content layer uses native merged-tree diff streams and conflict
traversal. It shares a read-only Store and native merge options with directory
reads. Copy sources and unresolved terms are preserved without heuristic renames
or writes. Native line diffing runs only after conservative text/line-work
admission checks in the bounded query worker.

Copy ancestry queries preserve placeholder parent fields but never attempt to
load a placeholder object. The application diff layer treats placeholders as
lineage boundaries, retaining complete structural deltas and all source choices.
It does not invoke the pinned experimental copy-history diff helper, whose graph
walker requires every parent (including placeholders) to be a stored history.
