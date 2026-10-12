# Remote execution configuration

JST-SEC-009/JST-API-011 add opt-in operator-owned remote agent configuration
with stable setup IDs, existing runner/provider settings and principal
allowlists. Changes apply at restart. Executable and credential references
remain private host configuration. Native repositories and WorkStore homes must
already exist. Runtime installation, credential provisioning and fleet-wide
remote administration remain outside this profile. Device connection metadata
lives outside checkouts; secret sessions are memory-only and must be reacquired
after restart.

Operator agent administration is a separate authenticated API, requiring the
independent `manage-agents` operator capability. Repository Admin,
`create-repository`, execution allowlists and `manage-identities` do not imply
it. Its bounded editor conditionally updates only `remote_agents` in the
existing private server configuration, preserving other sections. Saved
definitions and allowlists take effect at restart; the editor shows saved and
running state and never restarts the server or installs tools. Executable paths
and provider credential references are operator input, never execution-caller
input.

Access onboarding uses existing identity and repository token administration.
Enrollment requires `manage-identities`; a repository grant requires its
independent current Admin credential. No onboarding action grants both execution
and repository authority implicitly. Credentials remain session memory and
unknown outcomes retain the original intent for explicit recovery.

E-REMOTE-AGENTS-002 qualifies the unchanged configuration and CLI admission
contracts using private disposable format-2 installations, synthetic independent
operator/repository credentials and real native repository creation. The gateway
freshness correction supplies the already-required registered remote Origin; it
introduces no new flag, configuration field or installation action.
