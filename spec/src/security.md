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
