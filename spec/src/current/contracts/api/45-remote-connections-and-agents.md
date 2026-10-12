# Remote application connections

## Requirements

| ID          | Requirement                                                                                                                                                                                                                  |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-API-011 | Authenticated agent APIs MUST enforce a configured agent/principal allowlist, caller-owned execution identities and independent current native repository permissions, including streaming, queued dispatch and publication. |

JST-API-011 adds installation identity and connection/execution capabilities to
bootstrap. LocalTrusted connection APIs expose a bounded saved registry,
memory-only connection sessions and a fixed-origin public API gateway, including
binary transfers and authenticated activity streams. These capabilities never
make local-management routes remotely callable.

Authenticated agent routes project allowed agent summaries and caller-owned
conversations, commands, original outcomes and bounded activity. Existing
LocalTrusted routes retain their contracts. Client-supplied principals, paths,
executables, secrets and runtime definitions cannot select execution authority.
Current installation identity admission, per-agent permission and independent
repository grants govern every operation. Unknown outcomes retain their request.
