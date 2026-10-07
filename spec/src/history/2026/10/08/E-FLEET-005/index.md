---
schema: 1
id: E-FLEET-005
---

# E-FLEET-005

<!-- evidence-field:id markdown -->
## Id

E-FLEET-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-036
- JST-UI-015
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Selected ACP avatars, composer reconnect warnings and completed file changes
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yyzxpuru (enclosing Jujutsu change; parent e1c39d71)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/controller/agent-workspace-changes.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChangesPanel.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChangesDiff.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentBindings.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionNotice.svelte
- catalog/fixtures/AgentFleetApplicationFixture.svelte
- catalog/fixtures/agent-fleet-changes.ts
- catalog/components/store-application/AgentChatChanges.stories.svelte
- web/tests/vite/agent-workspace-changes.test.ts
- web/tests/browser/agent-chat-changes.spec.ts
- docs/site/guides/agent-execution.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/contracts/collaboration/index.md
- spec/src/current/progress/M11/history/M11.4/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M13/history/M13.4/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M15/history/M15.5/index.md
- spec/src/current/progress/M15/index.md
- spec/src/current/progress/M18/history/M18.1/index.md
- spec/src/current/progress/M18/index.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/current/progress/M24/history/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/history/index.md
- spec/src/current/progress/M9/history/M9.5/index.md
- spec/src/current/progress/M9/index.md
- spec/src/current/progress/index.md
- spec/src/history/2026/10/08/E-FLEET-005/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-workspace-changes.test.ts#completed panels admit only the exact confirmed snapshot of the latest completed turn
- web/tests/vite/agent-workspace-changes.test.ts#local changes preserve the base and snapshot pin, count changed lines and retain captured patch bytes
- web/tests/vite/agent-workspace-changes.test.ts#binary changes keep counts unknown instead of presenting zero or inventing text
- web/tests/vite/agent-workspace-changes.test.ts#mismatched repository, operation, generations and comparisons are refused
- web/tests/vite/agent-workspace-changes.test.ts#pagination and content reads are bounded, with partial totals explicitly marked
- web/tests/vite/agent-workspace-changes.test.ts#native comparisons use the native read boundary and never invoke a mutation
- web/tests/vite/agent-workspace-changes.test.ts#text retention is bounded independently of exact line counts and aborted reads never start
- web/tests/browser/agent-chat-changes.spec.ts#cold chats show live avatars, composer reconnect warnings and exact saved changes
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-workspace-changes.test.ts
```

**Notes**

Initial run: 5 passed / 1 failed because the assertion matched the SDK's human-readable message rather than its exact error code. The code assertion repair passes 6/6. The final exact bounded-text/abort run passes 7/7. Logs: /tmp/jj-store-agent-changes-unit.log, /tmp/jj-store-agent-changes-unit-final.log and /tmp/jj-store-agent-changes-unit-complete.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte
```

**Notes**

First run: existing fleet family 6/6 passed; new family 0/2 passed. The panel lookup expired before the retained-status refresh, and the warning assertion named the wrong public composer identity. An explicit ten-second lookup allowance retains the exact panel/count assertions; the selector now uses ai-chat-composer. Exact new-family rerun passes 2/2, including expansion, exact captured diff, no dispatch, warning recovery/draft retention and accessibility. Logs: /tmp/jj-store-agent-changes-stories.log and /tmp/jj-store-agent-changes-stories-rerun.log. Stories remain visual-pending. The final post-extraction family run passes 8/8 in 221.54s: the two new stories, Selected live avatar and completed file panel and Reconnect warning stays inside the composer and clears after refresh, plus all six existing AgentFleet stories, including Contextual sidebar and persistent agent tags. Final log: /tmp/jj-store-agent-changes-stories-split.log.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Web checking passes with 0 errors/warnings. Initial catalog checking refused the test's Playwright-only exact option; the Testing Library role selector uses an anchored name. Final catalog checking passes with 0 errors/warnings and its 285-module boundary audit. Logs: /tmp/jj-store-agent-changes-web-check.log and /tmp/jj-store-agent-changes-catalog-check-final.log. The final post-extraction web and catalog checks both pass with 0 errors/warnings; the catalog boundary audit covers 286 modules. Final logs: /tmp/jj-store-agent-changes-web-check-split.log and /tmp/jj-store-agent-changes-catalog-check-split.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vite build --config web/vite.config.ts
node scripts/web-manifest.mjs
pnpm exec playwright test --config .scratch/agent-fleet-playwright.config.ts web/tests/browser/agent-chat-changes.spec.ts
```

**Notes**

Fresh production assets and manifest pass. The first cold browser attempt failed 0/1 because the fixture's Commit metadata unavailable toast intercepted Send. Its ordinary dismiss control removes that fixture condition; no forced click or weakened assertion is used. Exact fresh-host rerun passes 1/1 in 21.8s, with zero Axe violations on the page and diff dialog, one mocked Send and zero controls. It checks selected authoritative presence, twenty-file totals, expansion, exact base/operation diff, retained draft/reconnect warning recovery, one summary load across refresh and shared inbox presentation. The final post-extraction production build/manifest and fresh-host browser rerun pass 1/1 (28.0s test, 42.0s total), preserving zero page/dialog Axe violations and all exact assertions. Final logs: /tmp/jj-store-agent-changes-production-split.log and /tmp/jj-store-agent-changes-cold-split.log. Existing source-owned CSS/chunk-size warnings remain distinct from build success. Test-data screenshot: /tmp/jj-store-agent-changes-cold-sidebar.png. Logs: /tmp/jj-store-agent-changes-production.log, /tmp/jj-store-agent-changes-manifest.log and /tmp/jj-store-agent-changes-cold-browser-final.log.
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
pnpm exec tsx /tmp/jj-store-agent-changes-live-read.ts
pnpm exec tsx /tmp/jj-store-agent-changes-live-selection.ts
```

**Notes**

The final production comparison reader admits the real saved local session and returns one complete file, LICENSE, +21/-0. It compares starting commit 215a1f6e0dcfae7518abe13ec08c4ce4ce85d6a1 with saved commit 1c49137344f74c77cc705c91a408378f55780d8a at the retained operation for repository 99c578be80984b0f13e03dc0d1168b2e. The private diagnostic uses ordinary SDK protocol, Origin and Fetch Metadata headers; an earlier incomplete-header diagnostic returned 403, without admission changes or bypasses. The second diagnostic reads all 282 retained events through readAgentEventHistory, then uses completedAgentChanges exactly as the UI does; its confirmed selection returns the same complete LICENSE comparison. Logs: /tmp/jj-store-agent-changes-live-read-final.log and /tmp/jj-store-agent-changes-live-selection.log. No real Send, resume, control or Undo was issued.

The in-app preview separately shows Nyra's exact avatar, authoritative Error presence, original retained messages, grouped tools and timed lifecycle rows. Its repository/history loading was intermittent, and its saved-file Retry continues to show unavailable even while the same comparison reader succeeds through the service. Browser network observation returned no usable request events, so its cause is not established. This is not a live visual pass for the panel; the cold fixture proves rendered panel/diff behavior. The original user tab/draft was retained. Dev UI/backend PIDs 48413/54464 remain running on 7474/7476 without restart/reset.
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
pnpm check:architecture
pnpm check:ui-ownership
pnpm check:progress
pnpm spec:validate
pnpm docs:review application 'Reviewed selected and retained ACP header identities, composer reconnect recovery, bounded saved file comparisons and the extracted read-only diff dialog; API admission and execution authority remain unchanged.'
pnpm docs:review
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/history/2026/10/08/E-FLEET-005/index.md
```

**Notes**

Architecture initially failed on two files (372/321 counted lines against the 300 limit). Extracting the fixture data and saved diff dialog passes the unchanged checker across 1321 modules. UI ownership passes (one validator; 3153 chapters/181 requirements). Initial progress checking refused three unsupported Storybook named-test references and seventeen navigation/history files without fresh evidence coverage. Exact story names remain recorded under Run 2; direct named tests stay in Tests, and generated navigation files are listed with an unchanged-history confirmation. Rerun passes 145 tasks/651 evidence records. Specification validation passes all seven validators. Documentation application fingerprints were refreshed after the extraction and review validation passes. Initial focused formatting found one generated coverage.json warning; its formatter repair and the nineteen-file focused check pass. Logs: /tmp/jj-store-agent-changes-architecture-final.log, /tmp/jj-store-agent-changes-architecture-rerun.log, /tmp/jj-store-agent-changes-ownership-split.log, /tmp/jj-store-agent-changes-progress-final.log, /tmp/jj-store-agent-changes-progress-rerun.log, /tmp/jj-store-agent-changes-spec-final.log, /tmp/jj-store-agent-changes-docs-review-split.log, /tmp/jj-store-agent-changes-docs-review-verified.log and /tmp/jj-store-agent-changes-format-rerun.log. Final bookkeeping is qualified by Run 7.
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
pnpm docs:build
node scripts/docs.mjs stage
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target node scripts/docs.mjs stage
pnpm progress:history
pnpm check:progress
pnpm spec:validate
pnpm exec prettier --check docs/site/guides/agent-execution.md docs/maintenance/coverage.json spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/contracts/documentation/08-interactive-agent-execution.md spec/src/current/progress/M24/index.md spec/src/history/2026/10/08/E-FLEET-005/index.md
```

**Notes**

The initial documentation build passes, rendering nine diagrams in both themes and validating 141 files/17,421,240 bytes. The first staging invocation was stopped with exit 143 after detecting an uncached Cargo parser-reference build; it is not a successful stage. The final canonical staging command uses the existing target cache and passes, including the same diagram/link/anchor closure, then copies Help into web/dist/help. No Rust source changed in this UI follow-up. Log: /tmp/jj-store-agent-changes-docs-stage-shared.log. Generated M24 history is refreshed with this final result; the final specification/progress/format checks qualify that bookkeeping. The initial final-bookkeeping progress check refused pass with remaining gaps, so the overall evidence result is pending while all focused functional runs pass. The document formatter found regenerated M24 index spacing; that table is formatted before the exact rerun. Final logs: /tmp/jj-store-agent-changes-history-final.log, /tmp/jj-store-agent-changes-progress-complete.log (initial refusal), /tmp/jj-store-agent-changes-progress-qualified.log (rerun), /tmp/jj-store-agent-changes-spec-complete.log, /tmp/jj-store-agent-changes-document-format-complete.log (initial spacing warning) and /tmp/jj-store-agent-changes-document-format-qualified.log (rerun). No root aggregate, Rust product tests, new source-package checks, installed-adapter execution, visual baselines or release lanes were substituted for focused UI acceptance.
<!-- /run -->

<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live in-app repository loading/saved-file Retry remains unavailable; the exact service reader and cold fixture pass separately.
- Session-specific Undo is unavailable; generic repository undo is not invoked.
- E-FLEET-004's source qualification decision remains pending. No main integration or source full-check acceptance is inferred from this follow-up.
- Visual baseline approval, installed adapter and desktop/platform gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; JJ 0.45.1; Vitest 4.1.11; existing registry-resolved Design Core with its previously recorded local validation setup. No new source package changes or publication.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing AgentExecutionSession repository bindings, retained events, native Window4/FileDiffResponse and local CodeQueryResponse formats are unchanged. No schema, identity, mutation, receipt, WAL, publication or credential contract changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Before: the contextual header showed Chat without its selected agent avatar, reconnect failures were a paragraph outside the composer, and completed snapshots showed only Snapshot saved/Review changes. Now exact selected recipients reuse the owned ACP avatar/live-state component; unaddressed drafts retain Chat. Update warnings live in the composer with command-error precedence. Sidebar and inbox share a read-only file panel comparing the captured base and saved commit at its exact operation. It admits at most two 50-path pages, four concurrent file comparisons and 2 MiB of retained patch text. Binary, partial and unavailable counts are explicit. The first three paths expand, and selecting a path opens its captured diff. Undo is explicitly unavailable until a guarded per-session inverse is qualified. No real prompt, control, checkout or undo was issued.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the fleet workspace and preview for follow-up. Investigate live in-app repository/read availability independently of the passing exact comparison and cold UI qualification. Local landing remains subject to the pending source qualification decision recorded in E-FLEET-004. Keep M24's independent gates and per-session Undo open. Generated navigation/history formatting retains all historical evidence, requirement IDs, statuses and qualification gates unchanged.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-034/JST-COLLAB-036 and JST-UI-015 now govern selected header presence, composer warnings and exact completed-file panels. JST-DOC-001/JST-DOC-002 document the delivered UI and unsupported Undo; reference formats, admission, source ownership and execution authority remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This presentation/read qualification does not close source full checks, additional adapters, live permission/cancellation, desktop/platform qualification or a session-specific inverse mutation.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


