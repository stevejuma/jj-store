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
