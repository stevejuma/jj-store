---
schema: 1
id: E-FLEET-002
---

# E-FLEET-002

<!-- evidence-field:id markdown -->
## Id

E-FLEET-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-012
- JST-API-001
- JST-GOV-008
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

kkzmrymn (enclosing Jujutsu change; parent 73e04276)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/AppearanceComposer.stories.svelte
- catalog/components/store-application/AgentFleet.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/ui-provenance.json
- scripts/agent-fleet-browser.mjs
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/07/E-FLEET-001/index.md
- spec/src/history/2026/10/07/E-FLEET-002/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-FLEET-002/details-02.md
- spec/src/history/2026/10/07/E-FLEET-002/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerAutocompleteLayer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerToolbar.svelte
- web/src/lib/ui/community-chat-composer/composer-agent-audience.svelte.ts
- web/src/lib/ui/community-composer-mention-button/CommunityComposerMentionButton.svelte
- web/src/lib/ui/community-composer-mention-picker/CommunityComposerMentionPicker.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/tests/browser/agent-fleet.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/agent-fleet.spec.ts#cold contextual chat uses exact tags,
  retains pins and reopens independently of details
- web/tests/browser/agent-fleet.spec.ts#cold agent Start warms in place without
  entering a chat
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; current locked pnpm dependencies; Svelte 5; Vitest 4.1.11; existing development fixture and Vite HMR on 7474 preserved.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/schema, journal, native storage, publication or audience-key format changes. Exact saved ACP IDs and opt-in device/actor/home/conversation audience scoping remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Remove New chat only from the right-body chat sidebar header, preserving its title and standard far-right Close. Move the automatic agent mention preference from the composer drawer into the owned Community picker with individual pin buttons. Selecting/pinning an exact saved agent while enabled immediately shows the owned Community @ avatar/dismiss pill; unpinning removes that exact token. Controls preserve blocking; no action sends a message. The @ control opens member suggestions. Shared appearance/thread/work/agent composer consumers justify their complete relevant interaction lane and cold browser acceptance.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified UI alignment; pursue independent M24 qualification separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-034/035/036 and owning UI/governance chapters now specify
  picker-only preference, immediate addressed-avatar feedback and
  title/Close-only sidebar. JST-API-001, JST-DOC-001/002, identity, dispatch,
  pending requests and native publication contracts remain unchanged;
  presentation requires no new API or backend action.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M24 stays In progress for its existing independent adapter, installed
  permission/cancellation and desktop gates. E-FLEET-002 reviews these
  presentation changes independently of E-FLEET-001 installed qualification.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FLEET-002 details 2](details-02.md)
