---
schema: 1
id: E-INBOX-FRESHNESS-001
---

# E-INBOX-FRESHNESS-001 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base 2cd2903b59a7a12c6066ea8ff558bef757adc9a4
```

**Notes**

Earlier attempt before recording removed_tests failed on three historical polling-test references. Removed tests are now explicitly recorded with event-only replacements. /tmp/jj-inbox-spec-first-initial.log.
<!-- /run -->

<!-- run -->
### Run 2

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

146 tasks and 718 evidence records passed after named-test and removed-test bookkeeping. /tmp/jj-inbox-progress-current.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Post-boundary check caught use of ES2023 findLast in the ES2022 surface. Replaced with reverse/find; target remains unchanged. /tmp/jj-inbox-web-delivery-final.log.
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
pnpm catalog:check
```

**Notes**

Same ES2022 compatibility issue; fixed without changing tsconfig or platform requirements. /tmp/jj-inbox-catalog-delivery-final.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Additional evidence split page details-04 lacked generated navigation. Updated only this evidence directory through owned navigation helpers. /tmp/jj-inbox-spec-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/inbox-request-lifecycle.test.ts web/tests/vite/agent-freshness.test.ts
```

**Notes**

Final ES2022-compatible boundary/cache/outage regression suite: 17 passed. Prior 896-unit/87-story runs cover the complete shared impact surface; the final edit preserves behavior by using an older supported array idiom. /tmp/jj-inbox-boundary-compat-final.log.
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
pnpm web:check
```

**Notes**

Final source after compatibility repair: zero errors/warnings. /tmp/jj-inbox-web-boundary-final.log.
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
pnpm catalog:check
```

**Notes**

Final source after compatibility repair: zero errors/warnings and 357 boundaries. /tmp/jj-inbox-catalog-boundary-final.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```

**Notes**

Final owned boundary: one validator passed. /tmp/jj-inbox-ownership-final.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Final audit still reports only the six inherited byte-identical oversizes already recorded. No new overages; no limits changed. /tmp/jj-inbox-architecture-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base 2cd2903b59a7a12c6066ea8ff558bef757adc9a4 --head c0f24f2688b0097b1fb75daff15fabc27832e3d2
```

**Notes**

Explicit full commit range passed: 64 protected paths and three owning chapters. /tmp/jj-inbox-spec-first-final.log.
<!-- /run -->

<!-- run -->
### Run 12

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

Final 146 tasks and 718 evidence records pass. /tmp/jj-inbox-progress-delivery.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Final seven validators pass after the evidence navigation repair. /tmp/jj-inbox-spec-delivery.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/fixtures/AgentExecutionFixture.svelte catalog/fixtures/AgentFleetApplicationFixture.svelte catalog/fixtures/StoreThreadsFixture.svelte catalog/fixtures/agent-execution-lifecycle.svelte.ts catalog/fixtures/agent-fleet-changes.ts spec/src/current/contracts/api/35-scoped-application-freshness.md spec/src/current/contracts/ui/22-automatic-application-updates.md spec/src/current/contracts/ui/32-event-driven-freshness-migration.md spec/src/current/progress/M24/SUMMARY.nav.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/inbox-freshness-review.md spec/src/current/progress/M24/index.md spec/src/current/progress/M25/history/M25/index.md spec/src/current/progress/M25/index.md spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/SUMMARY.nav.md spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/details-02.md spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/details-03.md spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/details-04.md spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/index.md spec/src/history/2026/10/10/SUMMARY.nav.md spec/src/history/2026/10/10/index.md web/src/lib/api/local-client.ts web/src/lib/controller/agent-changes-cache.ts web/src/lib/controller/agent-chat-navigation.svelte.ts web/src/lib/controller/agent-chat.svelte.ts web/src/lib/controller/agent-compute.svelte.ts web/src/lib/controller/agent-discussion-reader.ts web/src/lib/controller/agent-discussion-refresh.ts web/src/lib/controller/agent-discussions.svelte.ts web/src/lib/controller/agent-execution-launch-context.ts web/src/lib/controller/agent-execution-refresh.ts web/src/lib/controller/agent-execution-stream.ts web/src/lib/controller/agent-execution.svelte.ts web/src/lib/controller/agent-freshness.ts web/src/lib/controller/agent-history-cache.ts web/src/lib/controller/agent-resource-disposal.ts web/src/lib/controller/agent-runtimes.svelte.ts web/src/lib/controller/agent-selection-history.ts web/src/lib/controller/application-freshness.ts web/src/lib/controller/automatic-refresh.ts web/src/lib/controller/capture-refresh.ts web/src/lib/controller/conversation-metadata.svelte.ts web/src/lib/controller/http-threads-data-source.ts web/src/lib/controller/local-inbox-threads.ts web/src/lib/controller/production-automatic-refresh.ts web/src/lib/controller/production-projects-hub.svelte.ts web/src/lib/controller/production-projects-integration.ts web/src/lib/controller/production-store-connect.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/projects-command-state.ts web/src/lib/controller/projects-navigation.svelte.ts web/src/lib/controller/projects-refresh-scope.ts web/src/lib/controller/repository-thread-navigation.svelte.ts web/src/lib/controller/retained-reads.ts web/src/lib/controller/settings-refresh.ts web/src/lib/ui/community-inbox/CommunityInbox.svelte web/src/lib/ui/community-navigation/CommunityNavigation.svelte web/src/lib/ui/store-agent-chat/StoreAgentBindings.svelte web/src/lib/ui/store-agent-chat/StoreAgentChangesPanel.svelte web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte web/src/lib/ui/store-agent-execution/StoreConversationMetadata.svelte web/src/lib/ui/store-application/StoreApplicationNavigation.svelte web/src/lib/ui/store-threads/thread-navigation-binding.svelte.ts web/src/lib/ui/store-threads/thread-navigation.ts web/tests/vite/agent-changes-cache.test.ts web/tests/vite/agent-discussions.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-freshness.test.ts web/tests/vite/application-freshness.test.ts web/tests/vite/automatic-refresh.test.ts web/tests/vite/capture-freshness.test.ts web/tests/vite/inbox-request-lifecycle.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/sidebar-read-lifecycle.test.ts
```

**Notes**

All changed tracked files formatted. /tmp/jj-inbox-prettier-delivery.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base 2cd2903b59a7a12c6066ea8ff558bef757adc9a4 --head 1cd166e7f243f6cc9f418cb5873703c41adb8c3f
```

**Notes**

Final explicit source/evidence range qualifies 64 protected files and three canonical chapters. /tmp/jj-inbox-spec-first-delivery.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live browser first-load connection queueing remains; precise owner
  unconfirmed. No missing index or stream leak proven.
- Explicit preparation status observations in agent-revision-index, http-blame,
  index/search preparation and FreshnessJournal fallback require follow-up event
  coverage.
- Six inherited architecture oversizes remain unchanged. M24/M25
  distribution/adapter gates and ResizeObserver diagnostics remain open. Visual
  baselines unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0; pnpm 10.32.1; custom JJ 0.45.1; Vitest 4.1.11; Storybook 10.5.7; Svelte 5.57.1; enclosing change klrylvzw.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

HTTP/schema, native/local publication, receipt identities and comment-reply protocol unchanged. Caches and drafts remain session-only. No browser persistence or backend dispatch added.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI read-lifecycle change across inbox and direct shared readers. Production owns retained scopes, hidden dirtiness, transcript and immutable preview caches. Supported inbox uses one application feed; legacy activity stream retains guarded header/heartbeat deadlines. Audit covers runtime/compute Settings, conversation metadata, native/local Threads, Projects/sidebar, capture and automatic refresh. Full catalog-unit lane and direct story family are required because lifecycle helpers and fixtures have multiple consumers. Root aggregates, Rust and release/platform lanes are out of scope. Stable live browser navigation across a969/ddcc produced no API reads, retained 7/11 article nodes and no loading indicator. A cold-start cache race was then fixed by retaining the exact committed-event boundary. No messages were sent or drafts edited. The available ResizeObserver log query returned no entries; this limited observation does not close prior diagnostics. CUA CDP was used because Chrome DevTools MCP is not connected; no trace-based Web Vitals claims are made. After the cold-boundary repair and HMR recapture, returning from ddcc to the startup a969 session again produced zero API reads, seven message articles and no loading indicator.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Resolve live connection admission/queue ownership and qualify explicit preparation task events; keep independent milestone gates open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-005/011/012/015/017 and JST-API-001 freshness chapters now
  require retained matching scopes, no idle/outage view-data polling,
  request-count regressions, hidden sidebar ownership and supported shared-feed
  inbox activity. JST-COLLAB-034, publication/authorization and public API
  shapes are unchanged: exact source identities, frozen prompts, native/local
  authority and receipts retain their existing contracts."

```
<!-- /evidence-field -->

