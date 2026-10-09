---
schema: 1
id: E-CHAT-LIVE-012
---

# E-CHAT-LIVE-012

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-API-001
- JST-GOV-008
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Sent-context alignment and runtime action placement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vwkkxumy
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Three exact regressions failed: the collapsed context button was transparent, wide XML context started 234px inside its panel, and runtime menus differed by 130.625px because optional actions followed them. An added historical-context assertion separately reproduced a 250px inset.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Sent context uses the public secondary Button variant for themed resting and hover backgrounds. All captured Markdown branches use the existing owned fullWidth option, including legacy captured origin/task/reply content. Nested XML disclosures retain their fixed indentation. Runtime optional actions precede the trailing menu with unchanged keyboard/menu semantics. Community Markdown and Design Core Button/Field sources were inspected; no upstream fix or CSS override was needed. The ownership audit additionally found two previously missing local declarations for app-authored composer selection/footer helpers; those are now recorded, without claiming Community source copying.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live

A fresh in-app browser page inspected preserved session ddcc5f115b95bf017564735592db880f. Sent context measures 24px high with rgb(240, 240, 240) resting background; its 900px panel has a 12px starting inset. Codex, Cursor and jj-agent menu right edges all measure 1238px, including Cursor Update guide. No prompt, installation or configuration save was performed. Warning/error logs are empty, including ResizeObserver in this inspected flow. Screenshots: .local/context-alignment-final.jpg and .local/runtime-menu-alignment-final.jpg. Earlier tab10 could not answer CDP focus requests; a fresh tab in the same browser succeeded, preserving the user tab and drafts.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentCapturedOrigin.svelte
- web/src/lib/ui/store-settings/StoreAgentRuntimes.svelte
- catalog/components/store-application/AgentContextDelivery.stories.svelte
- catalog/components/store-application/AgentRuntimes.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/fixtures/AgentContextDeliveryFixture.svelte
- docs/ui-provenance.json
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/governance/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-012 details 2](details-02.md)
- [E-CHAT-LIVE-012 details 3](details-03.md)
