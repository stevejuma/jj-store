# Remote execution authority

Under JST-MUT-001, the operator configuration editor is independent of
repository mutation authority. Fresh `manage-agents` permission guards
conditional replacement of private `remote_agents` configuration; effective
execution definitions change only on restart. Identity enrollment still requires
`manage-identities`. Neither role grants repository or agent execution access.

Explicit repository Write onboarding delegates to native token administration:
current repository Admin, an enrolled matching principal/token, an exact
expected native snapshot and the original request identity remain required.
Status reads the original receipt without resubmitting a grant. A committed
receipt may repair the derived token-candidate index; it cannot substitute for
manifest selection. Unknown publication or post-publication authorization loss
remains recoverable against that exact intent. Configuration recovery compares
saved state with the original conditional intent before any new save.

JST-API-011 retains JST-MUT-001 semantic publication policy. The service exposes
owned connection and remote-execution interfaces; inference/controller transport
does not enter storage, model or WAL. Native repository mutations and
publication use the initiating caller's current authority, never a LocalTrusted
administrator fallback. Bound read/stream continuation and receipt recovery also
reauthorize.
