# Native commit backend

The commit backend uses checked native codecs for trees, commits, symlinks and
copy histories, and the verified streaming spool for files. It authorizes each
read or write, including virtual roots. A bounded semaphore limits outstanding
object operations. An unknown upload is an error, never an acknowledged native
write. Existing signatures are preserved; requesting a second signature on a
signed commit is rejected. New signatures cover the domain-separated V1 unsigned
commit payload, with the exact signed bytes retained in the native signature.

Copy enumeration uses only the committed inventory plus this session's
acknowledged writes. Queries enforce node, work, retained-byte and deadline
budgets, fail explicitly instead of truncating, and return deterministic reverse
topological order. Copy records cover the DAG range excluding all ancestors of
the lower bound. A history first introduced at a target is traced to its nearest
ancestor source on each lineage, including intermediate histories created within
one commit. Inherited histories do not generate repeated copy events. Conflicted
trees simplify signed terms per path before retaining positive file
alternatives, so canceled terms cannot reintroduce deleted files. Path filtering
applies to targets. Placeholders have no persisted history and generate no
inferred copy events. A file created and copied entirely within one commit has
no ancestor copy record. This integration does not infer renames from equal file
contents.

A disposable immutable copy relation index may retain owned copy IDs and
distinct parent/child adjacency derived from verified descriptors. Construction
admits node, edge, work and conservative allocation bounds before growth;
sealing rejects missing parents and cycles. Family lookup walks the requested
IDs' ancestors and all descendants of those ancestors through retained
adjacency, without scanning unrelated nodes or hydrating copy bodies. Parents of
a merge descendant do not themselves become ancestors of the original request.
Each lookup admits visited identities, scratch bytes and edge work
independently, checks cancellation and monotonic deadlines, and fails rather
than returning a truncated family. This data-only primitive retains no source,
credentials or publication capability. Source scope, global allocation leases,
checked-root selection, current authorization, acknowledged staged copies and
deterministic ordered-body delivery remain the integrating provider's
responsibilities.

Native selected copy-family providers retain only descriptor relations, scoped
to the process-local source, repository, epoch and checked inventory root. Share
the existing 64 MiB allocation pool and eight work permits with complete kind
projections. At most 32 fixed five-minute root entries retain checked selection
metadata; active owners keep their charges after eviction. Coalesce construction
for the same root without retaining authentication. Scan selected descriptors
under the existing window and two-million-descriptor cold-work limits. Complete
addition traversal can reuse an unchanged graph, or extend only copy relations;
removed or replaced baselines restart within the original remaining work/time.
Matching-family result/work/scratch bounds stay independent of construction.
Fresh permission, epoch and deadline checks precede all returned data.

The upstream backend combines that committed family with an append-only snapshot
of this session's acknowledged copy-parent relations. Retain bounded parent and
reverse-child identities after a successful upload only, admitting node, edge
and byte growth before upload and again before insertion. A captured sequence
excludes later writes without cloning the whole staged graph. Traverse staged
ancestors, resolve their selected seeds and follow staged descendants; do not
hydrate unrelated committed or staged bodies. Required unstaged seeds must be
selected. Duplicate acknowledged writes remain idempotent, unknown writes add no
staged relation, and complete ordered-body delivery retains deterministic
children-before-parents order. Compatibility providers keep their existing
bounded complete-inventory path. No native, RPC or persisted format changes.

Native factories, authenticated transport wiring and full CLI/workspace
lifecycle acceptance remain separate M3 gates.
