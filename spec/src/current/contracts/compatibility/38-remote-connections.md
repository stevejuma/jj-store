# Remote connection compatibility

JST-COMPAT-001 retains native repository/configuration identities and published
collaboration formats. Bootstrap extensions are additive; saved connections
require matching protocol and installation identity before restoration. Local
execution journals remain independently readable. Published dependencies and
development fixtures remain retained. Linux and macOS cold qualification are
separate gates and cannot be inferred from tests on the other platform.

The operator editor adds `manage-agents` to operator policies and bounded
administration endpoints to protocol 1/OpenAPI. Existing operator grants keep
exactly their prior authority; no implicit expansion or persisted-format
migration occurs. Older binaries reject the unknown permission, so upgrade the
server before issuing it. Format-2 TOML retains the existing `remote_agents`
section and restart activation. Native Write onboarding delegates to existing
CAS-selected token receipts and keeps their exact identities. Repository,
AgentStore, directory and WAL formats remain unchanged.
