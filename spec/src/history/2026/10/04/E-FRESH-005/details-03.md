---
schema: 1
id: E-FRESH-005
---

# E-FRESH-005 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 -
```

**Notes**

stdin script enumerated changed TS/Svelte/Markdown/JSON files from jj diff --name-only and ran pnpm exec prettier --check. Coverage manifest required formatting normalization; corrected before final verification. final-format-all.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/event-freshness.spec.ts --grep-invert "two clients"
```

**Notes**

2 pass/1 fail: full idle renewal and reset/preparation passed. Strengthened historical test selected its unchanged hash while the initial head refresh was still in flight; wait for that refresh to settle before selecting the older operation. cold-idle-local-reset.log. No production change was needed. Passing cases include zero data requests across real five-minute renewal, exactly two streams per native/filesystem page, and no page errors; reset/exact preparation finishes without periodic status reads.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/event-freshness.spec.ts --grep "an external local"
```

**Notes**

1 pass: external CLI change triggers a snapshot read within five seconds; a second publication leaves the explicitly selected older operation intact. cold-historical-pin-final.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Final ledger gate caught an open inherited M15.5 dependency and out-of-scope lanes entered as E-FRESH-005 gaps. Kept M20 In progress and moved qualification boundaries to next; scoped E-FRESH-005 passes without claiming broader milestone completion. progress-complete.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

113 protected files / 14 canonical chapters passed. spec-first-complete.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/event-freshness/check-format.py
```

**Notes**

Changed TS/Svelte/Markdown/JSON Prettier checks pass; final ledger-only update receives its owning formatting repeat. format-complete.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=/tmp/jj-store-event-freshness-cold-20261004 JJ_STORE_DEV_PORT=7570 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:stop
```

**Notes**

Owned isolated server/storage stopped, fixture preserved. Original development services unchanged. cold-stop-final.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

139 tasks pass after correcting final evidence boundaries and preserving the incomplete M15.5 dependency. progress-final-corrected.log.
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

<!-- evidence-field:next markdown -->
## Next

Retain scoped event regressions. M20 remains In progress because existing M15.5 full-flow qualification is incomplete. Root aggregate, unqualified strict/workspace clippy, visual baseline approval, Linux/additional platform qualification and large-fixture/RSS/pressure release lanes are out of scope. One inherited service large-enum lint exemption remains. This mixed LocalTrusted seeded native/real-RustFS and filesystem cold profile does not promote separate bounded-read capacity milestones. E-FRESH-003/004 implementation and cold gaps are closed by this integrated evidence.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/event-freshness.spec.ts#cold native and filesystem views stay
  idle through real five-minute renewal
- web/tests/real/event-freshness.spec.ts#an external local operation triggers a
  committed refresh within five seconds
- web/tests/real/event-freshness.spec.ts#two clients react to Work events,
  preserve drafts, and recover missed replay
- web/tests/real/event-freshness.spec.ts#a stale server cursor resets continuity
  once and exact local preparation retains its pin
- web/tests/real/automatic-updates.spec.ts#open task forms keep their original
  version guards after incoming metadata
- web/tests/vite/automatic-refresh.test.ts#unrelated healthy notices cannot
  postpone a queued event read
- web/tests/vite/capture-event-scope.test.ts#healthy capture reads only changed
  status, sources and configuration interests
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zrmqwtuqxqun
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/application_events.rs
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs
- crates/jj-store-http/src/application_events/authorization.rs
- crates/jj-store-http/src/application_events/monitors.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/application_events/tests.rs
- crates/jj-store-http/src/application_events.rs
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-http/src/search.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/capture_management/settings.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_runtime/freshness.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/local_repositories/watcher.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-server/src/replication/freshness.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-server/src/search_management/mod.rs
- crates/jj-store-server/src/server_settings.rs
- crates/jj-store-server/src/storage_management/mod.rs
- crates/jj-store-server/src/sync_management.rs
- crates/jj-store-server/src/workspaces.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/events.rs
- crates/jj-store-service/src/freshness/native.rs
- crates/jj-store-service/src/freshness.rs
- crates/jj-store-service/src/index_tasks.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_browser.rs
- crates/jj-store-service/src/local_index_tasks.rs
- crates/jj-store-service/src/local_management.rs
- crates/jj-store-service/src/search_management.rs
- crates/jj-store-service/src/server_settings.rs
- crates/jj-store-service/src/storage_management.rs
- crates/jj-store-service/src/sync_management.rs
- crates/jj-store-tantivy/src/paging.rs
- docs/browser.md
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/guides/capture.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/adapters.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/application-events.ts
- web/src/lib/api/generated.ts
- web/src/lib/controller/application-freshness.ts
- web/src/lib/controller/automatic-refresh.ts
- web/src/lib/controller/capture-configuration-observer.ts
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/controller/capture-refresh.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/freshness-journal.ts
- web/src/lib/controller/production-automatic-refresh.ts
- web/src/lib/controller/production-capture-management.ts
- web/src/lib/controller/production-index-preparation.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-read-services.ts
- web/src/lib/controller/production-search-settings.ts
- web/src/lib/controller/production-session-events.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/projects-activity.svelte.ts
- web/src/lib/controller/projects-agent-scope.ts
- web/src/lib/controller/projects-agents.svelte.ts
- web/src/lib/controller/projects-auxiliary-restore.ts
- web/src/lib/controller/projects-command-state.ts
- web/src/lib/controller/projects-dirtiness.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-refresh-scope.ts
- web/src/lib/controller/projects-related-views.ts
- web/src/lib/controller/projects-repositories.svelte.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/settings-refresh.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsRepository.svelte
- web/src/lib/ui/store-threads/thread-navigation-binding.svelte.ts
- web/tests/real/automatic-updates.spec.ts
- web/tests/real/event-freshness.spec.ts
- web/tests/vite/application-freshness.test.ts
- web/tests/vite/automatic-refresh.test.ts
- web/tests/vite/capture-event-scope.test.ts
- web/tests/vite/index-freshness.test.ts
- web/tests/vite/projects-freshness.test.ts
- web/tests/vite/settings-freshness.test.ts
<!-- /evidence-field -->

