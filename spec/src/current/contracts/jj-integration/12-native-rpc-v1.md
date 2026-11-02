# Native RPC V1

The additive InventoryPage RPC pins a checked manifest inventory root, epoch,
publication position, code-state version and exact head set. Signed
continuations bind repository, object kind and scan size; they retain data only,
never a credential or request. Each page rechecks current permission and epoch
before and after its bounded scan. At most64 process-local pins live for five
minutes without expiry renewal; expiry, restart and epoch change require
reselection. The request scan size bounds inspected descriptors, not matching
IDs. Empty filtered windows retain their continuation. Responses report
inspected count and the original snapshot. Legacy Inventory framing remains
supported with its existing complete-result bound. NativeClient's
complete-result convenience uses the server's single selected complete-kind
lookup, benefiting from exact source-scoped reuse and additions advancement. It
rejects oversized, malformed, duplicate or unordered identities and never
returns a truncated success. Its caller count limit, server-owned
two-million-descriptor scan limit and whole-call monotonic deadline remain
explicit. Progressive callers continue to use the unchanged paged contract.

Legacy RPC Inventory and in-process native operation/copy enumeration use the
same selected descriptor windows, retaining only matching identities before
growth. Their caller and trusted session bounds limit the complete matching
result; an independent two-million-descriptor scan ceiling and session deadline
limit total work. Each window rechecks permission and epoch while retaining the
original manifest root. Return an error on exhaustion, never a truncated prefix
or copy match. These ordinary reads rely on publication/recovery closure
certification; semantic publication still performs its complete inventory audit.
Both paged and complete inventory paths enforce a monotonic deadline before
work, between scan windows and before returning data, as well as canceling
pending I/O at that deadline. Ready I/O or synchronous decoding cannot produce a
successful result after the admitted time budget.

Complete native kind projections contain only exact matching IDs and checked
immutable manifest data. Scope reuse to the native source capability,
repository, epoch, inventory root and operation/copy kind. Current
authorization, smaller caller/session limits and monotonic deadlines precede
response delivery even on a warm root. Complete changed-path additions advance a
verified baseline without reading unchanged descriptors. Fixed root/TTL/global
allocation and concurrency bounds apply; eviction cannot release an active
owner's charge. Cold scans remain explicit bounded work and do not retain
unrelated object identities. No legacy wire field, paged inspected-count
semantics, authoritative format or mutation policy changes.

The native gRPC boundary accepts typed repository/object IDs, immutable object
uploads, committed ID queries, and head transitions. It exposes no raw keys,
manifest CAS, policy writes, or caller-selected principal. The service verifies
complete envelope and native identity before immutable storage, preventing a
malformed upload from poisoning a content-addressed key. Remote factory adapters
reuse the same native codecs and runtime binding as in-process factories.

Each RPC authenticates current repository permissions. Uploads recheck write
permission after bounded streaming and validation; downloads recheck read
permission as chunks are emitted. Streams have bounded chunks, total bytes,
concurrency and elapsed time, and drop owned spools when canceled. Publication
uses the shared native semantic policy and deterministic receipt identity.
Ambiguous transport failures preserve unknown publication outcomes. An explicit
insecure-loopback client option is required for local plaintext; shared channels
require TLS. Executable command and administration integration remain separate.

AgentStore capture adds a bounded two-phase service to this same native RPC. A
prepare call carries canonical supported-profile and sanitized-batch JSON plus
stable request/publication IDs; the server pins the current AgentStore root and
returns an exact validated `AgentDeltaRequest` with its admitted peer principal.
The publish call accepts only that prepared request and returns either the full
committed receipt or the original unknown identity. The client cannot select an
AgentStore root or actor, and operation IDs never cross this domain boundary as
commit or capture identities.

Writable replication adds versioned service-owned status, frontier, finite
barrier, wait and search-freshness DTOs plus corresponding `NativeClient`
methods. Authenticated native RPC exposes status, now, pause, resume, barrier
capture, bounded wait and replica inspection without importing upstream JJ
types. Status and inspection read local runtime and durable checkpoint state
only; they do not contact the upstream, advance a frontier or reconcile native
heads. Reconciliation remains an explicit application mutation using the same
guarded command service and durable receipt policy.

The executable installs lazy native factory closures: a native cache's bounded
connection record selects the endpoint and repository, while credentials are
resolved independently. The shared factory instance within one invocation keeps
staged object inventories consistent across backend/operation/head stores.
Ordinary Git-backed workspaces continue to use upstream factories without
resolving native credentials or connecting to the server.

TLS entry points retain a host-installed Rustls crypto provider. When none is
installed, the RPC implementation selects its declared Ring provider once before
configuring Tonic, including when another dependency enables the AWS-LC backend
too. Plaintext/Unix transports do not initialize TLS policy. Deployment CA,
hostname verification, endpoint restrictions and bearer authorization remain
unchanged; crypto implementation choice never becomes request/principal state.
