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
