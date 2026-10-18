# Filesystem browser reads

The ordinary Git-backed reader uses only server defaults and a private device
binding. It checks bounded operation ancestry before `RepoLoader::load_at`, then
checks the selected ancestry and predecessor closure before exposing code. Git
SHA-1 objects and the virtual root retain their source identities. Content reads
prove file membership through selected commit trees. Queries run in admitted
cancelable workers with limits on metadata, work, bytes and time. Repository
aliases, executable settings and content-predicate revsets are not loaded.
Ancestry traversal uses upstream parent identifiers directly, retaining
membership and work limits. Missing, unrelated and ambiguous selections fail
explicitly. Reads may rebuild derived JJ indexes but cannot publish heads, refs
or checkout snapshots.

The local query transport serializes the same owned request variants and output
records used by the qualified reader. Server composition shares query workers
with native reads; transport continuations cannot supply unchecked raw anchors.

Prepared core-index reads admit native file-path predicates only when every
retained commit has upstream changed-path metadata. A work/byte/memory-bounded
preflight prevents a fallback to native content/recursive merge evaluation.
Missing coverage requests private derived preparation at the original pin;
file-body and cross-operation predicates stay disabled. Tauri performs that
preparation on existing workers with current scope/cancellation and cache
admission. This enables exact file revision history without modifying source
files or source indexes (JST-JJ-003/005, JST-LOCAL-004).

Tauri annotation uses the same bounded native blame implementation at a retained
operation and commit, with native-owned handle scope for continuations. Author,
origin commit and one-based source ranges retain their actual JJ attribution.
Read admission, source binding, revocation and drained closure remain mandatory;
no renderer-provided path is physical-source authority.

Trusted working-copy status composes the qualified physical observation with a
sole-parent pinned read. Private bounded bytes support direct final-tree line
counts and never enter baseline/observation frames. Ignore/sparse projection and
complete post-count revalidation remain mandatory. This adds no source-index
repair, snapshot, reconciliation, publication or command execution authority
(JST-JJ-003/005, JST-LOCAL-004/005; E-WORKING-COPY-STATUS-001).
