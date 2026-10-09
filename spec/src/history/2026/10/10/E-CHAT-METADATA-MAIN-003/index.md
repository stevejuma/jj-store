---
schema: 1
id: E-CHAT-METADATA-MAIN-003
---

# Unchanged authority confirmation for main refinements

<!-- evidence-field:id markdown -->
## Id

E-CHAT-METADATA-MAIN-003
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
- JST-API-001
- JST-CLI-004
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

- crates/jj-store-server/src/agent_execution/metadata_directory.rs
- crates/jj-store-server/src/agent_execution/metadata_sources.rs
- crates/jj-store-server/src/agent_execution/metadata_store.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/collab_pagination/materialization.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/store-agent-execution/StoreConversationMetadata.svelte
- web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte
- web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/agent_sessions.rs#session_detail_resolves_current_stable_identity_and_exact_links
- crates/jj-store-service/tests/collab_pagination/materialization.rs#metadata_pins_reuse_thread_data_and_select_new_titles_after_publication
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
cargo test -p jj-store-service --test agent_sessions --locked
cargo check -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --locked
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
```

**Notes**

Nine source tests, all-target checks, zero web/catalog diagnostics and ownership pass. E-CHAT-METADATA-MAIN-002 records exact commands/logs, the four default-stack publication/recovery tests, cache authorization/immutability regressions, actual HTTP validation and cold live inspection. This confirmation concerns unchanged authority boundaries only; the requested UI defaults/layout refinements are governed separately there.
<!-- /run -->
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

Rust 1.98.1 and custom JJ 0.45.1; enclosing default change xxnlvury.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/schema, source selector, storage, helper control or publication format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Qualified internal read/projection refactor and shared presentation controls. Required backend/API/mutation/publication/compatibility/documentation/ownership contracts remain unchanged. UI presentation changes belong to E-CHAT-METADATA-MAIN-002. No broader provider, platform or visual qualification is claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep the independent M24/M25 qualification gates open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-COLLAB-034/035/036 retain exact committed sources, source and
  participation authorization, frozen metadata context and bounded tool-free
  inference. Cached materializations and captured-detail projection change
  internal preparation cost, not authored context or publication semantics.
  JST-API-001 retains strict existing HTTP/CLI shapes.
  JST-CLI-004 retains explicit loopback selection, bounded metadata commands
  and responses, original request recovery and no silent retry. Server read
  preparation changes no CLI command, route, dispatch or response contract.
  JST-MUT-001 and JST-PUB-001 retain independent permissions, stable requests/receipts and
  manifest publication. JST-COMPAT-001 keeps paired helper/control formats and
  refusals. JST-DOC-001 keeps source-reviewed bundled Help. JST-GOV-008 keeps
  owned sources and provenance. Requested UI presentation refinements are
  separately documented in E-CHAT-METADATA-MAIN-002.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Add explicit passing unchanged-boundary evidence beside the changed
  presentation contract; keep M24/M25 In progress and independent gates open.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


