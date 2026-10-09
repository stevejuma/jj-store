---
schema: 1
id: E-CHAT-METADATA-002
---

# E-CHAT-METADATA-002 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
pnpm catalog:check
```

**Notes**

Each checker found one recorded-search rune error: a local variable named state shadowed $state. Renamed the derived view. A later interrupted-job fixture lacked repository:null; corrected the typed fixture without weakening schemas.
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
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

Earlier seven metadata/settings stories passed after repairing dt/dd ownership, proxy cloning, binding and duplicate Overview headings. Initial wider family had outdated labels and assertions; the final family is recorded below.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

Seventeen passed; persistent agent recipients disappeared after document Send. Shared host submissions bypassed the classic composer reset. Added the scoped draft-reset hook to successful sends and receipt recovery; retained user-removal behavior.
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
pnpm test:storybook catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

Exact seven-story fleet regression passed, including persistent recipient pins after document Send and reopening.
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
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Final cold Storybook process passed 77 stories across ten directly affected files with interaction, accessibility and runtime-diagnostic guards. Expanded scope covers shared composer reset, settings, recorded/live navigation, inline comments and body geometry. Log: /tmp/metadata-ui-family-release.log.
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
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

Ten final sidebar stories passed after aligning Close conversation details with its heading and menu.
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
pnpm test:storybook catalog/components/store-application/ConversationMetadata.stories.svelte
```

**Notes**

Four final detail stories passed, including Stop waiting after an interrupted inference, with one cancel and no refresh/inference POST. The fixture uses deliberate initial snapshots and a complete typed query. Log: /tmp/metadata-details-delivery.log.
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
pnpm catalog:test catalog/controller/conversation-metadata.test.ts web/tests/vite/agent-settings.test.ts web/tests/vite/agent-mention-settings.test.ts web/tests/vite/conversation-session.test.ts
```

**Notes**

Sixteen tests passed: duplicate prevention, later rename drafts, exact unknown requests, failed-request release, independent settings and conversation session retention. Log: /tmp/metadata-unit-final.log.
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
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
```

**Notes**

Final web/catalog checks report zero errors/warnings; catalog boundary audit covers 352 modules. Production remains independent of catalog and Community checkout code. Logs: /tmp/metadata-web-delivery.log, /tmp/metadata-catalog-clean.log, /tmp/metadata-ownership-final.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/governance.test.mjs
pnpm check:boundaries
```

**Notes**

Twenty-nine governance tests and 17-package dependency audit passed. The parent already depended on pure jj-collab from the server; the audit omitted that approved composition edge. Added positive and reverse-edge/upstream refusal coverage with an owning architecture clarification.
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
pnpm contracts:generate
pnpm docs:generate
```

**Notes**

Regenerated additive API types and maintained CLI/Help references. Added CLI argument help rather than suppressing undocumented-argument validation.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

The wider public-contract capture lane found a real historical local-session regression: discovery called the current-head inbox guard. Fixed enrichment to read the requested committed companion snapshot. Log: /tmp/metadata-contract-check.log.
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
cargo test -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage --locked -- --exact
```

**Notes**

Exact regression passed in 25.58 seconds, including an old pinned directory remaining empty after newer capture. Log: /tmp/metadata-historical-regression.log.
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
PATH="/Users/stevejuma/.local/bin:$PATH" pnpm web:build
```

**Notes**

Final built bundle and strict Help build passed: 141 files with valid links/anchors. Earlier failures were missing CLI help, stale documentation review fingerprints and uv absent from the shell PATH; corrected source documentation/reviews and used the existing uv executable. Existing Lightning CSS/global pseudo-class, mixed Tauri import and large-chunk warnings remain. Log: /tmp/metadata-final-built.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/conversation-metadata.acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --web web/dist --helper runtime/local-agent/target/debug/jj-store-local-agent
```

**Notes**

First retained acceptance run reached native API publication and failed on an outdated close-button locator. After correcting the label and searchbox role, the next run reached browser assertions but used browser.newPage with Axe; replaced it with an explicit BrowserContext.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/conversation-metadata.acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --web web/dist --helper runtime/local-agent/target/debug/jj-store-local-agent
```

**Notes**

Final isolated real-RustFS/native and local companion flows passed: profile write refusal, selected metadata, custom-title preservation, duplicate receipts, native search, loaded directory discovery, draft remount, restart and no read/browser inference. Four built-browser visits passed Axe excluding color contrast (visual gate separate), zero metadata POSTs from inspection, no page errors and no ResizeObserver warnings. Helper used a controlled HTTP-compatible provider; this is not authenticated real-model compliance. Log: /tmp/metadata-cold-delivery.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Inherited counted-line violations reproduced against 61df439c: scripts/dev-ui.acceptance.mjs 311, community-inbox/inbox.css 303, StoreAgentChatBody.svelte 309, each capped at 300. This change retains those counts; new/changed metadata files fit. The new composer hook initially exceeded its cap and was extracted into existing owned helpers; composer now counts 299. No cap or baseline was weakened.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

Final generated contracts match and 300 actual HTTP/SSE responses conform. All seven contract tests and contract TypeScript checks pass after fixing historical local-snapshot discovery. Log: /tmp/metadata-contract-delivery.log.
<!-- /run -->
<!-- /evidence-field -->

