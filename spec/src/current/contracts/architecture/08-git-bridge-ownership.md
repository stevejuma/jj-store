# Git bridge ownership

[Explicit Git interoperability](../git-interop/index.md) adds CLI-side transport
and a native durable evidence domain. Core storage and publication consume owned
records only. No persistent writable mirror or additional transport daemon is
introduced. WorkStore origin replication retains its separately deferred gate.

The exact converter adds gix 0.87.1 to the JJ integration boundary while keeping
JJ 0.45.1 and the existing resolved Git dependency pin. Its temporary GitBackend
is a conversion tool; no Git dependency enters WAL, model or object storage.

Git transport supervision uses the already resolved rustix 1.1.5 process API
inside the CLI only. Non-reaping wait retains the owned group leader's PID until
cleanup completes, preventing a cleanup signal from targeting a recycled PID. No
transport or process dependency enters native authority layers.

The Projects hub preserves native model/service ownership and adds only an
installation UI orchestration layer and read-only HTTP summary contract.
Production build qualification uses the generated Vite dependency manifest to
serve application-reachable chunks, styles and fonts plus declared agent icons.
Unused package-emitted workers are excluded, while server asset admission limits
remain unchanged. The manifest traversal regression is wired into the existing
governance test lane.

The development harness includes native local, native server-only and ordinary
Git-backed JJ fixtures. All three original projects keep the native metadata
home and link distinct repository identities. The Git-backed fixture is CLI-only
until local serving is qualified; discovery never substitutes a native copy for
its storage profile. `cli:install` builds the owned CLI package and copies the
server and custom JJ tools under distinct names without replacing system JJ.

Native Thread/Reaction roots select bounded canonical-object-to-envelope
catalogs. This retains shared domain content identities while reusing
repository-bound immutable WAL records. Complete source closures include
external AgentStore, ReviewStore and native code edges. Local companion batches
retain independently versioned domains through one owned ref and a versioned
receipt envelope.

Filesystem-only composition omits native object storage, RPC, search and
replication workers. The HTTP host is optional; native requests fail explicitly
when absent. Installation-owned bindings and the Git-profile reader use the same
HTTP admission and query-worker pool as native serving.

Device-bound collaboration facades carry the admitted local repository identity
into the existing WorkStore provider. Identity is rechecked on publication
retries, while reads validate independently selected companion history. Native
manifest authority and local Git ref authority remain separate.

Folder initialization composition persists the proposed identity before
publication and reuses the same initialization receipt. Reconnect changes only a
validated device binding; companion metadata and linked project identities
remain authoritative and unchanged.
