# Security and deployment

Token revocation is checked independently of index freshness. RustFS credentials
remain server-side. Local operator tooling provisions the first repository and
scoped credentials. Logs and receipts record authenticated actors, never secret
bytes or caller-supplied authorship as authentication. Search degradation does
not make authoritative storage unready.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                                                      |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-SEC-001 | RPC and HTTP MUST authenticate one principal model with revocable repository-scoped read/write/admin tokens and a separate repository-creation operator capability.                                                                                                                              |
| JST-SEC-002 | Credentials MUST remain outside checkouts; servers MUST retain token verifiers, CLI credentials MUST use the OS credential facility or automation environment, and browser tokens MUST remain in memory.                                                                                         |
| JST-SEC-003 | Authorization MUST precede every object, historical, staging, search, preview, and publication request; knowledge of an object ID MUST NOT grant access.                                                                                                                                         |
| JST-SEC-004 | Shared deployments MUST use TLS; unauthenticated operation MUST require an explicit restricted-loopback development setting.                                                                                                                                                                     |
| JST-SEC-005 | Server operations MUST NOT execute repository hooks, arbitrary programs, editors, or merge tools; paths, bodies, queries, and work queues MUST have explicit limits.                                                                                                                             |
| JST-SEC-006 | Access policy MUST preserve authenticated token semantics while LocalTrusted accepts only verified local peers, assigns stable installation principals, and protects every browser route with exact loopback authority, origin, fetch-metadata, protocol-version, method, and media-type checks. |

## Repository token foundation

Tokens contain a random public token ID and 256 bits of secret entropy.
Versioned policy documents retain only a SHA-256 verifier bound to the token ID
and exact repository (or separate operator scope), authenticated principal,
permission and optional expiry. Verifier comparison is constant time. Read,
write and admin are hierarchical repository permissions; repository creation is
an independent operator capability. Credential types have redacted Debug output
and do not implement serialization. Policy parsing and token counts are bounded.

Repository policies are immutable settings selected by the publication manifest.
Each request verifies its credential against the selected policy before reading
repository data. Publication rechecks it immediately before CAS and after every
CAS conflict, so a concurrent revocation prevents an older policy snapshot from
committing. Receipt replay also requires current authorization. Settings changes
require admin permission and cannot change the policy's repository scope. The
service guard requires an explicit semantic validator; authentication alone does
not permit client-computed history changes.

Operator policies remain separate from repository state. The authentication
library does not itself persist credentials or expose transports. CLI credential
storage, HTTP/RPC wiring and server operator administration remain M3 gates.

Repository initialization requires an operator policy and a credential with the
creation capability before storage access. Its coordinator guard rejects all
ordinary publications. Once initialized, the native service session enforces
current repository credentials and the shared native semantic policy; no
permissive policy constructor is exposed by that session.

Native RPC V1 carries bearer credentials in sensitive authorization metadata.
Unknown repositories and invalid credentials share an authorization failure.
Object IDs never select a raw storage path. Uploads validate identity before
storage and reauthorize before writing; long downloads reauthorize per chunk.
Server messages expose stable error categories rather than storage or credential
details. Work permits and total stream deadlines apply before body consumption.

Client credential accounts bind the canonical server URL and repository ID.
Missing or inaccessible OS credential facilities fail explicitly without a
plaintext fallback. Login reads secrets from stdin or a hidden terminal prompt,
verifies repository access, then saves to the OS store. Logout cannot remove a
process environment variable and reports when an environment override remains.
Local connection metadata is bounded and rejects embedded URL credentials,
unknown fields and profile mismatches before connecting.

## Provisioning and token administration

Repository creation authenticates against a fresh operator policy before any
repository storage access. The client retains the repository ID, epoch and
initial admin credential across retries; the server persists only its verifier.
The operator principal owns the initial grant. An identical creation request
recovers the same initialized repository even after later policy changes;
conflicting epochs or initial grants never overwrite existing authority.

Repository token list/grant/revoke require current admin permission. Grants
accept a client-generated credential over the authenticated channel so loss of
an acknowledgement does not lose a server-generated secret. The service derives
the policy change from the exact committed expected snapshot and preserves all
other grants. It never accepts a replacement policy from a transport. Admin
results and durable receipts contain token metadata, never bearer secrets or
verifiers. A revoked credential cannot read old receipts. In-flight creation is
authorized against the operator policy loaded for that request; later requests
load it again. No operator privilege implies repository read access.

Hosted RPC resolves a repository only through current manifest authorization.
Missing manifests and incorrect credentials return the same access-denied
category. Credential-bearing creation/grant messages redact Debug and erase
decoded plaintext after parsing. Request limits and permits apply to
administration as well as object transfers.

Server configuration rejects unknown keys, embedded secrets, invalid resource
budgets and insecure shared listeners. RustFS secrets come only from the server
environment. Operator policy and process state paths must be outside Git/JJ
checkouts. Operator policy files contain only verifiers, use bounded canonical
encoding, and reject symlinks. Local policy updates lock a stable sibling lock,
atomically replace the file and sync it; each creation request reloads policy.
The foreground process holds an exclusive local state-directory lock. Shared
storage still requires a single configured serving instance; this lock does not
claim distributed fencing or automatic failover.

Operator OS-account provisioning locks a per-endpoint file outside checkouts
before its read/create sequence. Contending commands fail without replacing a
credential; retry reuses the saved account. Lock files contain no credentials.

Repository initialization saves a generated initial admin credential to its
endpoint/repository OS account before sending any creation request. Its local
journal stores only the public credential ID. Neither a missing OS facility nor
a changed retry credential permits a plaintext fallback or new secret identity.

Token administration separates the authenticating admin credential from the
grant credential. Local journals retain only the grant ID and a domain-separated
fingerprint of its high-entropy bytes. They never retain bearer secrets. Grant
status/retry requires the original credential through the same protected input
channels and authenticates the admin before reading server policy or receipts.

The standalone TLS fixture rejects untrusted certificates, mismatched hostnames
and plaintext connections before repository requests. The positive deployment
fixture uses a generated localhost certificate and explicit trust file; it
qualifies the production TLS path on an isolated loopback interface, not
external DNS, certificate automation, load balancers or additional serving
topologies.

Graph and directory cursor signatures bind traversal kind and inputs and
retained read snapshots; they do not grant repository access. Continuation
requests authenticate before cursor lookup or historical reads, including cached
pages. The signing key is generated for the serving process, remains outside
serialized DTOs and has redacted Debug output. Restart or expiry requires
explicit snapshot reselection.

HTTP accepts exactly one Authorization Bearer header, never URL credentials or
cookies. Browser Origin headers must match the configured public origin; no
cross-origin credential policy is enabled. Repository binding precedes source
reads, including task/cursor lookup. Unknown repositories and invalid repository
tokens share access-denied behavior. API responses disable caching and MIME
sniffing and use a restrictive content security policy. Request/response
admission also covers slow or unpolled bodies, independently of native worker
slots.

The production listener negotiates HTTP/2 and HTTP/1.1 through one TLS acceptor.
Connection admission covers both transports, including incomplete handshakes.
Handshake concurrency and duration are bounded; idle sockets expire even before
request headers arrive. HTTP/2 stream and header limits supplement application
admission. Native RPC and HTTP keep separate bounded request pools. A socket
close never proves an in-flight publication failed; native receipt recovery
continues to resolve unknown outcomes.

SSE authenticates before subscription and before every queued event or
heartbeat. Resume IDs are public positions, not capabilities. A new request must
present a current scoped credential, including after reconnect or server
restart. Event streams have dedicated admission so connected browsers do not
consume every ordinary HTTP read slot. Slow/unpolled streams expire and release
their bound session and credential with the response body.

The browser clears in-memory credentials and data on disconnect or authorization
failure, aborts old work before repository changes, and ignores stale async
completions. It uses same-origin fetch with omitted cookies and manual redirect
rejection. Source data is always text. Static serving has an explicit bounded
asset allowlist, rejects symlinks and path escapes, and never falls back to the
app for unknown API paths. Browser assets contain no deployment credentials.

The shared listener returns HTTP 404 for unknown ordinary routes. Unknown native
gRPC services retain the protocol-specific UNIMPLEMENTED status.

Conflict uploads use a bounded raw binary endpoint with current write permission
before body consumption and an exact snapshot/principal binding before remote
writes and completion. Body errors, cancellation and revocation leave no
publication. Server-computed files and canonical staging records are immutable;
records are addressed within repository/epoch/principal namespaces and checked
again for the exact commit/path/fingerprint on use. Native RPC's typed object
keys cannot address staging records. Missing/wrong-context handles return the
same invalid_upload category. No handle grants repository read access.

Upload admission has a separate bounded pool (two streams by default), a 16 MiB
per-file default, a hard 256 MiB ceiling and the session/HTTP deadline.
Temporary spools can briefly use three times the file cap per admitted upload;
concurrent streams bound that local disk usage. Uploaded files in one resolution
are capped at 256 MiB in total and also consume existing native read/publication
budgets. Records are bounded to 16 KiB. Staging is not a mutable repository side
table and cannot advance history. Persisted orphan staging is retained;
automated cleanup and destructive GC remain disabled.

Adapter source/control reads reauthorize the bound repository session before and
after each storage request and before returning buffered content. A captured
source epoch cannot cross revocation or an epoch change. Projection checkpoints
contain identity and durability metadata, never credentials. Local writer locks
and checkpoint CAS do not provide distributed destination fencing.

Local search readers recheck current repository permission and epoch before and
after query work. A pinned generation does not convey access. Source bodies are
indexed but omitted from hits; native content retrieval remains separately
authorized. Search text uses a bounded literal token conjunction and exact
canonical native IDs, without accepting regular-expression or query-language
programs. Local index roots have private permissions and exclusive ownership.

Search extraction binds authenticated actors to selected receipt principals,
without treating native usernames or attributes as authentication. Explicit
source scopes and prefix exclusions precede file-content reads. Native
projection readers accept only the selected inventory and expose no write
capability.

Selected-view search filters retain no credential or live session. Their checked
repository and epoch must match the pinned index, and a new request must supply
current repository authorization before and after execution. Neither a filter
nor a historical search mode bypasses token revocation.

Search cursors contain signed identity/position/query digests and retain no
bearer credential. Serving-instance keys are generated securely, kept in memory
and zeroized on drop. Authentication precedes cursor parsing and lookup; a valid
cursor cannot bypass repository or epoch checks. Bounded waits reauthorize while
polling, and expiry maintenance releases idle generation pins independently of
further requests. Response-size failures release any newly retained pin.

A restart with changed search source exclusions cannot temporarily serve the
previous configuration's files during recovery. A configured extractor rejects
mislabeled identities before runner startup, and incompatible recovered
artifacts remain unserved until a compatible rebuild succeeds.

Configured search runners use dedicated repository tokens supplied through named
environment variables. Config output includes variable names, never their
values; missing/invalid secrets produce fixed errors. Index files inherit
private state directory access. A revoked runner credential degrades indexing
without changing repository serving permission or bypassing fresh authorization
for search users.
