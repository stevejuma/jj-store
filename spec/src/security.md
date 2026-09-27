# Security and deployment

Token revocation is checked independently of index freshness. RustFS credentials
remain server-side. Local operator tooling provisions the first repository and
scoped credentials. Logs and receipts record authenticated actors, never secret
bytes or caller-supplied authorship as authentication. Search degradation does
not make authoritative storage unready.

## Requirements

| ID          | Requirement                                                                                                                                                                                              |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-SEC-001 | RPC and HTTP MUST authenticate one principal model with revocable repository-scoped read/write/admin tokens and a separate repository-creation operator capability.                                      |
| JST-SEC-002 | Credentials MUST remain outside checkouts; servers MUST retain token verifiers, CLI credentials MUST use the OS credential facility or automation environment, and browser tokens MUST remain in memory. |
| JST-SEC-003 | Authorization MUST precede every object, historical, staging, search, preview, and publication request; knowledge of an object ID MUST NOT grant access.                                                 |
| JST-SEC-004 | Shared deployments MUST use TLS; unauthenticated operation MUST require an explicit restricted-loopback development setting.                                                                             |
| JST-SEC-005 | Server operations MUST NOT execute repository hooks, arbitrary programs, editors, or merge tools; paths, bodies, queries, and work queues MUST have explicit limits.                                     |

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

Graph cursor signatures bind traversal inputs and retained read snapshots; they
do not grant repository access. Continuation requests authenticate before cursor
lookup or historical reads, including cached pages. The signing key is generated
for the serving process, remains outside serialized DTOs and has redacted Debug
output. Restart or expiry requires explicit snapshot reselection.
