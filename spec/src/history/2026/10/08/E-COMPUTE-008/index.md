---
schema: 1
id: E-COMPUTE-008
---

# E-COMPUTE-008

<!-- evidence-field:id markdown -->
## Id

E-COMPUTE-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-UI-012
- JST-API-001
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Provider configuration landing and main development review
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change otuywnoy after tnyoxvlv rebased onto lptotxop (6d3ee2e9)
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Default workspace was clean on lptotxop, with the existing main Vite/native service on 7460/7462. Provider work was qualified separately as tnyoxvlv/7588ab55 above b50c5b02. A separate chat-body workspace remains independent. Final main browser review reproduced one provider-label error: explicitly choosing Anthropic incorrectly renamed the inherited default option to Anthropic.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Landed tnyoxvlv above the newer incremental Markdown context/coordination CLI work. Preserved both additions in the configuration tests, owning CLI/documentation/governance chapters and M25 ledger/history; combined reviewed coverage fingerprints. Main rebuilt the backend and provider helper and staged ignored companion resources beside its existing ACP. Restarted Vite/native service on 7460/7462 without resetting the fixture; repository ID and saved Nyra agent remained. Cold browser confirms JJ Agent label, all three harness icons, eight providers, native missing-auth guidance, and rounded Advanced with inset focus. Old task preview on 7466 was stopped; its workspace/data retained. Corrected only the inherited-default label calculation to use saved application/harness configuration independently of the explicit draft provider. The exact new regression failed once before the fix and passed afterward.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Integration and inherited-default label correction: combined agent-execution server/HTTP tests, Add agent provider and context-rendering stories, web/catalog checks, documentation and ownership/progress validators. Public schemas and helper catalog source match the previously qualified provider change byte-for-byte, so no public-contract regeneration or broader contract lane is repeated. No root aggregate check/test/build/spec:check, unrelated full Rust suite, package publication, installed CLI replacement, reset, model download or live provider prompt.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/agent_execution/configuration/tests.rs
- docs/maintenance/coverage.json
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M25/history/M25/index.md
- catalog/components/store-application/AgentProviderConfiguration.stories.svelte
- docs/site/guides/settings.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- web/src/lib/ui/store-agent-execution/StoreAgentConfiguration.svelte
- spec/src/current/contracts/api/42-compute-and-agent-settings.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/agent_execution/configuration/tests.rs#bundled_cli_path_cannot_be_supplied_by_a_setup
- crates/jj-store-server/src/agent_execution/configuration/tests.rs#provider_changes_do_not_inherit_foreign_models_endpoints_or_effort
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-COMPUTE-008 details 2](details-02.md)
- [E-COMPUTE-008 details 3](details-03.md)
