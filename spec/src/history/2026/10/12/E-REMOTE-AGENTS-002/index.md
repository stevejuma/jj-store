---
schema: 1
id: E-REMOTE-AGENTS-002
---

# E-REMOTE-AGENTS-002

<!-- evidence-field:id markdown -->
## Id

E-REMOTE-AGENTS-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-SEC-009
- JST-API-011
- JST-UI-022
- JST-DOC-001
- JST-GOV-002
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

sswpvmvq
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/connections/event_tests.rs
- crates/jj-store-server/src/connections/events.rs
- crates/jj-store-server/src/connections/mod.rs
- scripts/remote-agents-acceptance.py
- scripts/remote-agents-browser.mjs
- docs/agent-execution.md
- spec/src/current/contracts/security/12-remote-connections-and-agents.md
- spec/src/current/progress/M27/index.md
- spec/src/current/progress/M27/history/M27/index.md
- spec/src/current/verification/sec.md
- spec/src/current/verification/api.md
- spec/src/current/verification/ui-04.md
- docs/maintenance/coverage.json
- docs/site/reference/configuration.md
- docs/site/guides/agent-execution.md
- spec/src/current/contracts/cli/38-remote-agent-configuration.md
- spec/src/current/contracts/collaboration/44-authenticated-agent-admission.md
- spec/src/current/contracts/documentation/09-remote-agent-connections.md
- spec/src/current/governance/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Qualify the narrow macOS production gateway and authenticated native-agent integration slice against two isolated real RustFS installations. Parent mulvtstp contains the implementation but only permissive gateway/source fixtures and MemoryStore execution evidence. The first cold built-App run against actual API admission exposed a missing remote WebSocket Origin, which invalidated successful sign-in. Add a focused failing regression, send the registered remote Origin and preserve existing admission. Repeatable private disposable installations exercise actual discovery, allowlists, profile/avatar isolation, live freshness, history/reload, delegated execution and publication, independent code-grant and directory revocation, same-endpoint installation replacement, and restart/receipt recovery. Published dependencies, original working copy and development services/data are retained.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/connections/event_tests.rs#gateway_freshness_uses_registered_origin_and_session_authentication
- scripts/remote-agents-acceptance.py#qualify
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust/Cargo 1.98.1; custom JJ 0.45.1; Node 26.9.0; pinned RustFS 1.0.0 runtime reused read-only from the default workspace (SHA-256 407cd42c2fa1fc4a41303865075b8377419aa80f33981f34b7818ad52667a3b0). Current workspace server/custom-JJ binaries and production web bundle were freshly built. Released foundational dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, TOML format-2, grant, native object, WAL, manifest CAS, AgentStore, WorkStore or connection preference migration. Gateway credentials/handles remain memory-only. Test credentials are generated in memory, supplied over process environment/stdin and never persisted in checkout or logs. Cache input is copied into the disposable root; no shared writable Cargo target or storage volume.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-REMOTE-AGENTS-002 details 2](details-02.md)
- [E-REMOTE-AGENTS-002 details 3](details-03.md)
