# Installation configuration editing

JST-API-001 includes actor-free version-1 `GET /v1/local/settings` and
`POST /v1/local/settings`, advertised by `local_settings_management` only for
LocalTrusted administrators. Typed section writes cover service resource limits,
installation-wide protection rules and configured replication entries. They
require an exact saved-file revision, current installation identity and
unchanged authority infrastructure; the shared settings-file lock, bounded
regular-file read and synchronized atomic replacement apply across Search,
Agents and these sections. Only the selected section is replaced. Saved and
startup-effective values are distinct; saving starts no worker, contacts no peer
and publishes no repository state. Lost acknowledgement remains unknown until
fresh observation. Credential references are names or paths; credential values
are never read or accepted. Deployment/identity/storage paths and deferred
backup configuration are exposed as read-only inventory with their lifecycle or
migration reason. They are not generic TOML or filesystem editing capabilities.

Search configuration also includes bounded polling, timeout and snapshot
interval fields. Its existing conditional save and explicit Apply worker
replacement activate these fields with the other search configuration.

E-SETTINGS-REFRESH-001 retains JST-API-001/004 configuration, admission and
authorization contracts. Repository cards, action footers, tab styling and the
bounded content Scroll Area are client presentation only. Existing conditional
saves, Apply/restart requirements, stale-response rejection and exact pending
request identities are unchanged; no API or persisted format is added.

E-NUMERIC-SETTINGS-001 retains JST-API-001/004 canonical integer units, ranges,
conditional save and authorization contracts. Repository numeric units and
per-field Reset modify draft presentation only. Search Save/Apply and Sync
save/next-startup semantics, request identities and configuration formats remain
unchanged; invalid converted drafts cannot be silently rounded into valid saves.
