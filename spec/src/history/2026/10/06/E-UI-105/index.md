---
schema: 1
id: E-UI-105
---

# E-UI-105

<!-- evidence-field:id markdown -->
## Id

E-UI-105
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-005
- JST-UI-005
- JST-UI-015
- JST-API-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xpxnolkr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared collection typography and OpenCode typed text-part presentation
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean separate session workspace at xpxnolkr, parent a31b3167. The collection trigger’s later font: inherit reset its secondary text size. OpenCode’s native normalizer deliberately preserves output.parts as JSON; the UI treated the known typed-text array as ordinary JSON in activity and transcript fallback. The exact unit and heading regressions each failed once before implementation.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/collaboration/agent-message-content.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreNativeAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-work/community-work-collections.css
- web/tests/agent-message-content.test.ts
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/fixtures/AgentPromptFixture.svelte
- catalog/fixtures/StoreCapturedReactionsFixture.svelte
- docs/maintenance/coverage.json
- spec/src/current/contracts/ui/12-repository-presentation.md
- spec/src/current/contracts/collaboration/20-application-presentation.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/progress/M9/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M9/history/M9.5/index.md
- spec/src/current/progress/M11/history/M11.4/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-message-content.test.ts#OpenCode text parts render ordered
  message text with the recorded message identity
- web/tests/agent-message-content.test.ts#ordinary JSON and other integrations
  retain their visible JSON representation
- web/tests/agent-message-content.test.ts#mixed, unknown, empty and malformed
  OpenCode parts remain visible
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-105 details 2](details-02.md)
- [E-UI-105 details 3](details-03.md)
- [E-UI-105 details 4](details-04.md)
