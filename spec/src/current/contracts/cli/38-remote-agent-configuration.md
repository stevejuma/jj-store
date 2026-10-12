# Remote execution configuration

JST-SEC-009/JST-API-011 add opt-in operator-owned remote agent configuration
with stable setup IDs, existing runner/provider settings and principal
allowlists. Changes apply at restart. Executable and credential references
remain private host configuration. Native repositories and WorkStore homes must
already exist. No setup, credential, runtime installation or fleet-wide remote
administration is implied. Device connection metadata lives outside checkouts;
secret sessions are memory-only and must be reacquired after restart.

E-REMOTE-AGENTS-002 qualifies the unchanged configuration and CLI admission
contracts using private disposable format-2 installations, synthetic independent
operator/repository credentials and real native repository creation. The gateway
freshness correction supplies the already-required registered remote Origin; it
introduces no new flag, configuration field or installation action.
