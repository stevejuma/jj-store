---
schema: 1
id: E-FLEET-002
---

# E-FLEET-002 details 2

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
Browser comment and cua_repl before/after observations on 7474; inspected Community picker, mention button and Design Core shell references
```

**Notes**

Before: New chat in header, persistent checkbox in composer drawer and bare @ while pinned. After: title/Close-only header; preview exposes the addressed Nyra avatar pill with live Offline state. Preserved the user-owned draft, agent setup and preference; no prompt sent. Community source revision 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0, copied into owned UI/provenance; no checkout import or package dependency.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte; pnpm web:check; pnpm check:ui-ownership (initial header-only slice)
```

**Notes**

Initial title/Close-only slice passed existing five stories, web types and ownership before the user expanded this slice to composer audience presentation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte; pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Composer interaction 6/6, web types, 1302-module architecture and ownership pass. Catalog type check caught an unsupported Testing Library exact option in the new pin story; replaced it with an anchored role-name regex. Exact target/type reruns recorded separately.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte; pnpm catalog:check; pnpm check:boundaries
```

**Notes**

Exact rerun 6/6; catalog types 0 errors/warnings and 281 modules; package boundaries pass. Switch placement, immediate avatar pill before Send, picker and toolbar unpin, retained opt-in across replies/reopening, body selectors and Close-only header are exercised.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

Full relevant shared controller/helper consumer lane: 122 files, 757 passed/1 skipped. This is not a repository-wide test pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsFeedback.stories.svelte catalog/components/store-work/StoreWork.stories.svelte; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte -t OpenCodeTypedTextMessageParts
```

**Notes**

First complete direct-consumer interaction lane: 149 passed/1 failed in 29 files. The unmodified OpenCode typed-parts clipboard-menu story observed zero clipboard calls once; its fresh isolated exact run passed (1 passed/8 skipped) without a code/assertion change. Whole-lane rerun is recorded separately; no pre-existing-failure or full-suite claim is inferred.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm progress:history; pnpm check:progress
```

**Notes**

Initial ledger check caught a missing historical source path after deleting the unused legacy audience component, three unsupported Story-title test references and one stale coverage path. Retained the legacy source/provenance with no runtime composer rendering to preserve committed evidence, replaced named tests with executable cold browser test names, and refreshed actual files. No old evidence payload was edited.
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
pnpm docs:review application "Aligned the agent picker switch/pin controls and Community avatar pill beside @; exact local ACP identities, scoped audience persistence, blocked controls and explicit Send remain unchanged. Removed the redundant sidebar New chat control."; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm web:build
```

**Notes**

Production UI and bundled Help passed: 9 diagrams in both themes and 141 files with valid links/anchors. Existing minifier/chunk warnings remain. Restoring the unrendered legacy source afterward requires only a fresh source review/Help build; active bundle behavior remains as qualified by the cold run.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsFeedback.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Complete direct-consumer rerun: 29 files, 150/150 passed in 142.13 seconds. The prior clipboard observation did not recur; no assertion or source was changed to obtain the rerun. No repository-wide test claim.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/agent-fleet.spec.ts
```

**Notes**

Cold fixture browser acceptance: 2/2 passed in 4.5 seconds with fresh built UI and isolated preview server. Includes title/Close-only header, picker preference, addressed-avatar pill, retained exact tags/pins, warm-only Start and Axe. This is fixture UI evidence, not installed-adapter replay.
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
pnpm docs:review application "Verified the picker switch/pin actions and @ avatar pill use exact saved ACP IDs. The unused legacy audience component remains for historical source evidence, while the composer renders no preference drawer; active UI and dispatch contracts are unchanged."; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm web:build; node sourceFingerprint review of docs/maintenance/coverage.json topics
```

**Notes**

Final production UI/Help build after retaining the historical legacy source: 9 diagrams in both themes, 141 Help files, links/anchors valid; all 13 documentation fingerprints current. Existing minifier/chunk warnings remain.
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
pnpm check:architecture && pnpm check:ui-ownership && pnpm check:boundaries
```

**Notes**

Final architecture: 1303 modules; UI ownership passed; 17 package boundaries passed. Historical legacy audience source remains attributed and unrendered.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm docs:check
```

**Notes**

Final docs check passed after production build: source review, 9 diagram pairs and 141-file closure with valid links/anchors. UI HMR fixture/services remain untouched.
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
cua_repl: separate background preview tab on 7474, Open chat, Mention someone, per-agent pin, screenshot and close
```

**Notes**

A fresh preview-only draft initially had automatic mentions off. Pinning Nyra enabled the picker switch and immediately displayed the addressed avatar/dismiss pill, with live Offline state. No Send, session launch, or original-tab navigation; the existing user draft/preferences remain preserved. Screenshot retained locally at .scratch/community-agent-mentions.png; no visual baseline update.
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
pnpm progress:history; restore only unrelated token-equivalent generated Markdown from @-; pnpm exec prettier --write changed files; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check changed files
```

**Notes**

Progress: 145 tasks and 648 evidence records; spec: 7 validators, 3143 chapters and 181 requirements; focused formatting passed. Restored unrelated generator whitespace/table-alignment churn. Preserved committed evidence payloads and meaningful M24/E-FLEET navigation. Enclosing revision is kkzmrymn, not a future self hash.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No root pnpm spec:check/check/test/build aggregate, separate Rust qualification lane, real installed-agent replay (including scripts/agent-fleet-browser.mjs), desktop release qualification, external package publication or visual-baseline update. UI-only presentation changes do not alter backend/API contracts; the cold fixture server reuses the qualified backend. Shared composer consumers justify the complete relevant catalog-unit and direct-consumer interaction lanes, not an unrelated repository-wide suite.
<!-- /evidence-field -->

