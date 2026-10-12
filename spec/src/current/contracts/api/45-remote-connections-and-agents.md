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

Separate `/v1/operator/access` and `/v1/operator/agents` endpoints discover
operator capabilities and read or conditionally save agent configuration. They
use freshly loaded operator policy and never infer operator privilege from
repository or directory membership. Configuration replies expose saved/running
definitions only to `manage-agents`; the enrolled-principal picker exposes
principal IDs and authorized public profiles, not token verifiers or secrets.
Saves bind the exact file revision, preserve unrelated sections and report
restart requirements. Unknown saves retain the original revision and intent;
recovery reads saved state before any further save.

Repository `/agent-access` and `/agent-access/status` expose the existing native
token administration for explicit Write onboarding. They require current
repository Admin, an enrolled credential bound to the specified principal, the
exact expected native snapshot and retained request identity. Status uses the
original complete intent. Publication remains manifest CAS; registry candidates
are updated only after a committed receipt. No token bytes enter replies,
receipts, URLs or persistent browser preferences.
