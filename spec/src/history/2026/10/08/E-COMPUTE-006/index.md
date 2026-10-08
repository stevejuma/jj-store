---
schema: 1
id: E-COMPUTE-006
---

# E-COMPUTE-006

<!-- evidence-field:id markdown -->
## Id

E-COMPUTE-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-UI-015
- JST-COLLAB-036
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

Shared automatic mentions and shadcn Settings dropdowns
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xllxrpxv above 16d8ea7a
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Settings edited a defaults draft while the composer persisted an independent conversation flag. Two new browser regressions failed: the Settings switch remained unchecked after composer opt-in, and Thinking rendered a native select. Two new preference-save unit cases failed because a preference-only save did not exist. The Settings dependency audit found six native selects in four owning components.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Both switches now save the authoritative installation preference immediately, preserving other unsaved defaults and secret drafts. Only the app’s own preference-only receipt advances a matching retained draft revision; external edits remain stale until explicitly rebased. Exact pending requests survive unknown responses. Composer recovery retries settings, never Send; ordinary recipient selection stays usable while preference saving is unavailable. Settings refresh updates mounted composers and legacy conversation flags cannot override the shared value. Exact recipient pins remain scoped, retained when disabled, and explicit draft tokens remain intact. Six native dropdowns now compose released Design Core shadcn Select, preserving null choices, disabled sources, keyboard selection and focus return. Final requested-surface verification: 28 browser stories and 6 unit tests passed, both UI type checks passed, ownership and document checks passed. Wider run: 136/137 passed with one intermittent unrelated Copy message submenu test; current isolated and unchanged-parent cases passed. The existing architecture limit violation remains separately owned.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only integration: agent settings controller, Settings controls, shared composer audience/picker, Advanced agent options, direct stories and documentation. Escalated to 25 related story families because the composer and settings are shared by agent chats, discussions and Compute; no Rust, API generation, root aggregates or visual baselines are in scope.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentMentionRecovery.stories.svelte
- catalog/components/store-application/AgentMentionSettings.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/fixtures/AgentMentionPreferenceFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/api/42-compute-and-agent-settings.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- web/src/lib/controller/agent-settings.svelte.ts
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerAutocompleteLayer.svelte
- web/src/lib/ui/community-chat-composer/composer-agent-audience.svelte.ts
- web/src/lib/ui/community-composer-mention-picker/CommunityComposerMentionPicker.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentAdvanced.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentEnvironmentEditor.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- web/src/lib/ui/store-settings/StoreAgentMentionPreference.svelte
- web/src/lib/ui/store-settings/StoreComputePeers.svelte
- web/src/lib/ui/store-settings/StoreSettingsSelect.svelte
- web/tests/vite/agent-mention-settings.test.ts
- web/tests/vite/settings-selects.test.ts
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M25/history/M25/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-mention-settings.test.ts#an older settings observation
  cannot replace a newer saved preference
- web/tests/vite/settings-selects.test.ts#Settings and agent option descendants
  never render native dropdowns
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-COMPUTE-006 details 2](details-02.md)
- [E-COMPUTE-006 details 3](details-03.md)
