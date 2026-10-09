---
schema: 1
id: E-CHAT-METADATA-MAIN-002
---

# Main metadata handoff and presentation refinements

<!-- evidence-field:id markdown -->
## Id

E-CHAT-METADATA-MAIN-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-012
- JST-API-001
- JST-MUT-001
- JST-PUB-001
- JST-DOC-001
- JST-COMPAT-001
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentInboxTree.stories.svelte
- catalog/components/store-application/ConversationMetadata.stories.svelte
- catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
- catalog/fixtures/AgentInboxTreeFixture.svelte
- crates/jj-store-server/src/agent_execution/metadata_directory.rs
- crates/jj-store-server/src/agent_execution/metadata_sources.rs
- crates/jj-store-server/src/agent_execution/metadata_store.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/collab_pagination/materialization.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- spec/src/current/contracts/collaboration/42-conversation-metadata.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/contracts/ui/38-conversation-metadata.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/metadata-review.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/store-agent-execution/StoreConversationMetadata.svelte
- web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte
- web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/collab_pagination/materialization.rs#metadata_pins_reuse_thread_data_and_select_new_titles_after_publication
- crates/jj-store-service/tests/agent_sessions.rs#session_detail_resolves_current_stable_identity_and_exact_links
- crates/jj-store-cli/tests/agent_execution/metadata.rs#metadata_generates_once_preserves_custom_titles_and_recovers_repository_state
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

Custom JJ 0.45.1, Rust 1.98.1, Node 26.9.0, pnpm 10.32.1; enclosing default change xxnlvury. Metadata changes landed as a12850ec above f1b6ea5a.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing metadata version 1, ThreadStore/AgentStore manifest-selected references and control-version-3 helpers. No HTTP schema, storage format or credential-custody change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rebased metadata is landed in default, the temporary conversation-metadata workspace is retired, and main is restarted with its retained fixture. Titles shrink and ellipsize; Overview and Metadata start closed in one flat Settings group with retained drafts and title-action dividers. Shared read-only rows put navigation before Copy and reserve no missing-action slot. Bounded native metadata pins retain authorization and fresh manifest selection. Exact captured-detail projection omits redundant display enrichment. No reset, agent prompt, rename, global install or package publication.

Independent M24/M25 real-provider, IME, transfer, adapter, distribution and visual-baseline gates stay open. Prior architecture sizes, observed ResizeObserver warnings and linker unwind warnings are retained diagnostics. Root aggregate, full Rust workspace and unrelated storage lanes are out of scope under the targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M24/M25 In progress. Qualify independent provider/platform/IME/transfer gates and visual baselines separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 documents Settings disclosures, flat rows, title ellipsis and
  link-before-copy ordering. JST-COLLAB-034/035/036 document exact captured
  metadata preparation without display-only enrichment. JST-API-001,
  JST-MUT-001 and JST-PUB-001 retain strict HTTP shapes, independent source
  authorization, stable requests/receipts and manifest publication. Cached pins
  reauthorize and select fresh manifests; captured detail keeps immutable
  activity and bounds. JST-DOC-001 retains source-reviewed Help. JST-COMPAT-001
  retains paired helper formats and capability refusals; JST-GOV-008 retains
  owned production sources and provenance. These latter contracts are unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Record the main handoff and requested presentation/read refinements;
  retain M24/M25 In progress and all independent qualification/visual gates.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [Main metadata handoff validation](details-02.md)
- [Main handoff final qualification](details-03.md)
