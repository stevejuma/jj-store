---
schema: 1
id: E-UI-106
---

# E-UI-106

<!-- evidence-field:id markdown -->
## Id

E-UI-106
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nrystrnv
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
- M18.1
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-015
- JST-COLLAB-005
- JST-COLLAB-023
- JST-API-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main-workspace integration of recorded-agent presentation slices
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main default workspace was clean at xlspzwxw/8601eb72 over qukluvnk/0883061b; session stack-workspace-ui was clean over xpxnolkr/16b217b1. Common ancestor was zontovyv/9e98f99e. Session adds trqyosvm/a31b3167 (contained unavailable-content banner and fresh-fixture evidence) and xpxnolkr/16b217b1 (heading and OpenCode text parts); main adds its separately qualified agent-management and captured-time changes. The merged source had no conflicts; only the application review entry conflicted.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentChatLayout.stories.svelte
- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/components/store-application/AgentExecution.stories.svelte
- catalog/components/store-application/AgentHarnessModels.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/AgentRevisionIndex.stories.svelte
- catalog/components/store-application/AgentSessionSettings.stories.svelte
- catalog/components/store-application/AgentSessions.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/AgentPromptFixture.svelte
- catalog/fixtures/AgentsSkeletonFixture.svelte
- catalog/fixtures/StoreCapturedReactionsFixture.svelte
- catalog/fixtures/agent-command-receipts.ts
- catalog/fixtures/agent-execution-events.ts
- catalog/fixtures/agent-execution-revisions.ts
- catalog/fixtures/agent-execution-session.ts
- crates/jj-store-cli/tests/agent_execution/setups.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/src/local_browser/binding.rs
- crates/jj-store-http/tests/assets.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/avatar.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/browsing.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/security.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser-ui.mjs
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-chat-feedback.mjs
- scripts/dev-ui-config.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/03-requirements.md
- spec/src/current/contracts/collaboration/20-application-presentation.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-bounded-transcript-tail-format.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/contracts/security/04-provisioning-and-token-administration.md
- spec/src/current/contracts/ui/03-requirements.md
- spec/src/current/contracts/ui/12-repository-presentation.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M11/history/M11.4/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M13/history/M13.4/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M18/history/M18.1/index.md
- spec/src/current/progress/M18/index.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/current/progress/M9/history/M9.5/index.md
- spec/src/current/progress/M9/index.md
- spec/src/current/verification/collab-03.md
- spec/src/current/verification/ui-02.md
- spec/src/history/2026/10/06/E-EXEC-003/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-003/details-02.md
- spec/src/history/2026/10/06/E-EXEC-003/details-03.md
- spec/src/history/2026/10/06/E-EXEC-003/details-04.md
- spec/src/history/2026/10/06/E-EXEC-003/index.md
- spec/src/history/2026/10/06/E-EXEC-004/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-004/index.md
- spec/src/history/2026/10/06/E-EXEC-005/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-005/index.md
- spec/src/history/2026/10/06/E-EXEC-006/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-006/index.md
- spec/src/history/2026/10/06/E-EXEC-007/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-007/index.md
- spec/src/history/2026/10/06/E-UI-104/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-UI-104/details-02.md
- spec/src/history/2026/10/06/E-UI-104/details-03.md
- spec/src/history/2026/10/06/E-UI-104/index.md
- spec/src/history/2026/10/06/E-UI-105/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-UI-105/details-02.md
- spec/src/history/2026/10/06/E-UI-105/details-03.md
- spec/src/history/2026/10/06/E-UI-105/details-04.md
- spec/src/history/2026/10/06/E-UI-105/index.md
- spec/src/history/2026/10/06/E-UI-106/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-UI-106/details-02.md
- spec/src/history/2026/10/06/E-UI-106/details-03.md
- spec/src/history/2026/10/06/E-UI-106/index.md
- spec/src/history/2026/10/06/SUMMARY.nav.md
- spec/src/history/2026/10/06/index.md
- spec/src/previous-locations.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/agent-message-content.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/community/agent-editor-harness-tabs.ts
- web/src/lib/community/agent-editor-model-rows.ts
- web/src/lib/community/agent-runtime-contracts.ts
- web/src/lib/controller/agent-activity-session.ts
- web/src/lib/controller/agent-execution-stream.ts
- web/src/lib/controller/agent-execution-types.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-revision-index.ts
- web/src/lib/controller/agent-revisions.ts
- web/src/lib/controller/agents-route-codec.ts
- web/src/lib/controller/production-entity-search.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/ui/community-agent-activity/CommunityAgentActivityFeed.svelte
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptMessage.svelte
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptPlan.svelte
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptTools.svelte
- web/src/lib/ui/community-agent-activity/activity-feed.css
- web/src/lib/ui/community-agent-activity/transcript-plan.css
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorDialog.svelte
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorHarnessIcon.svelte
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorModeField.svelte
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorModelField.svelte
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorModelPicker.svelte
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorThinkingField.svelte
- web/src/lib/ui/community-agent-editor/agent-editor.css
- web/src/lib/ui/community-agent-processing-control/CommunityAgentProcessingControl.svelte
- web/src/lib/ui/community-agent-profile/CommunityAgentProfile.svelte
- web/src/lib/ui/community-agent-profile/CommunityAgentRuntimeFields.svelte
- web/src/lib/ui/community-agent-profile/agent-profile.css
- web/src/lib/ui/community-agents/CommunityAgents.svelte
- web/src/lib/ui/community-agents/agents.css
- web/src/lib/ui/community-agents/model.ts
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/community-avatar-picker/CommunityAvatarPicker.svelte
- web/src/lib/ui/community-avatar-picker/avatar-picker.css
- web/src/lib/ui/community-avatar-picker/image-files.ts
- web/src/lib/ui/community-avatar-picker/legacy-avatar.ts
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/props.ts
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/community-profile-actions/CommunityProfileActions.svelte
- web/src/lib/ui/community-profile-actions/profile-action-tiles.css
- web/src/lib/ui/community-skeleton/CommunitySkeleton.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentChatActions.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentDetailsSidebar.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionNotice.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentLauncher.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentPermission.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentPicker.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRequestRecovery.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRevisionPicker.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRuntimeSettings.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentSetupEditor.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/agent-execution.css
- web/src/lib/ui/store-agent-execution/conversation-error.ts
- web/src/lib/ui/store-agent-execution/harness-catalog.ts
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-agent-execution/runtime-preparation.ts
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreNativeAgentContext.svelte
- web/src/lib/ui/store-work/community-work-collections.css
- web/tests/agent-directory.test.ts
- web/tests/agent-execution.test.ts
- web/tests/agent-harness-catalog.test.ts
- web/tests/agent-message-content.test.ts
- web/tests/agent-revisions.test.ts
- web/tests/agents-route.test.ts
- web/tests/vite/agent-execution-controller.test.ts
- web/tests/vite/agent-revision-index.test.ts
- web/tests/vite/agent-runtime-preparation.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-message-content.test.ts#OpenCode text parts render ordered
  message text with the recorded message identity
- web/tests/agent-message-content.test.ts#mixed, unknown, empty and malformed
  OpenCode parts remain visible
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-106 details 2](details-02.md)
- [E-UI-106 details 3](details-03.md)
