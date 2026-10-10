---
schema: 1
id: E-AGENT-TRANSCRIPT-VIEW-001
---

# E-AGENT-TRANSCRIPT-VIEW-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Expanded acceptance: 55 passed, 1 failed, 94.70s. Document arbitrary prompt tags were split as ordinary Markdown and lost the established disclosure renderer. Preserved the full sanitized text in an inert context block when prompt tag syntax is present; no writable source is fabricated. /tmp/transcript-acceptance.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-projection.test.ts
```

**Notes**

11 projection tests passed in 4.44s, including recorded tools retaining command/output without writable targets, source-map invariants, literal capture bytes and Unicode output. /tmp/transcript-projection.log.
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
```

**Notes**

Initial check: zero errors and warnings. /tmp/transcript-web.log. Also /tmp/transcript-web-final.log passed after the first source pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Removed unsupported Testing Library role option exact and used anchored names. /tmp/transcript-catalog-final.log.
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
pnpm catalog:check
```

**Notes**

Typed the legacy fixture state getter to retain the text media-type literal. /tmp/transcript-catalog-verified.log.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Intermediate layout propagation incorrectly placed a snippet shorthand in the body rather than its attribute, producing unsupported children. Fixed before final acceptance. /tmp/transcript-web-acceptance.log and /tmp/transcript-catalog-acceptance.log.
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

Final production source: zero errors and warnings. /tmp/transcript-web-final-pass.log.
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

Final catalog source: zero errors/warnings, boundaries OK for 367 modules. /tmp/transcript-catalog-pass.log.
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

Owned UI boundary passes; no Community/catalog production imports. Initial 3471 and final 3473-chapter checks both pass. /tmp/transcript-ownership-pass.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search JST-UI-018
```

**Notes**

QMD index refresh blocked by native Node ABI 147 mismatch. Used the reported rg fallback and read the owning current contract. This is a search-tool environment failure, not the product acceptance result.
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
rg -n -i --glob '*.md' 'JST-UI-018' spec/src/current/contracts/ui
```

**Notes**

Found 03-requirements.md, 08-device-appearance.md and 37-document-conversations.md; current source governs the change.
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
pnpm evidence:new -- --id E-AGENT-TRANSCRIPT-VIEW-001 --task M24 --requirement JST-UI-015 --requirement JST-UI-018
```

**Notes**

Created the fresh bounded evidence family. Restored the generator-only whitespace changes in unrelated existing contracts, milestones and evidence pages; retained this day index and the targeted M24 review.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

55 passed, one Classic Copy message keyboard submenu failed to become visible; 94.98s. /tmp/transcript-stories-pass.log. Exact cold recheck below passes without source or assertion changes; final cold group is repeated for this transient menu-transition concern.
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
pnpm spec:first --base 2a1d5e6a --head 83029db222cc2ba70a5379570b039ee773290a27
```

**Notes**

Spec-first passes for 18 protected files and two owning canonical chapters. /tmp/transcript-spec-first.log.
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

macOS ARM64; jj 0.45.1; Node v26.9.0; pnpm 10.32.1; Vitest 4.1.11; Storybook 10.5.7; released Design Core and Mira dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Device appearance schema, conversationView document/classic, retained AgentStore captures, ThreadStore/ReviewStore authority, native/local exact selectors and request/receipt identities are unchanged. The existing preference now selects the renderer at all active recorded hosts.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only presentation and direct adapters/fixtures. Production hosts audited: Projects landing and selected project agent responses, repository sessions and code/log context via StoreAgentContext; review/activity embedded captures via StoreNativeAgentContext; live StoreAgentConversation and StoreAgentChatSidebar already use StoreLiveConversation and the saved preference. The old store-agent-execution/StoreAgentTranscript helper has no production caller and is left unchanged. Existing bounded periodic refresh in StoreRecordedConversation remains outside this presentation slice. No Rust, root aggregate, built-bundle release, installed provider, restart publication or visual-baseline lanes were run: they are out of scope for these app-owned UI changes. Cold Vitest Chromium processes and a fresh live Vite tab/reload exercise startup. The current native fixture/backend is preserved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Verified and committed in enclosing default-workspace change umouvlrv. Continue independent M24 execution/platform, ResizeObserver and visual-baseline qualification separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-015/JST-UI-018: 37-document-conversations.md specifies parity
  across recorded hosts, read-only captures without a prompt/footer, explicit
  legacy ReviewStore forms in the document scroll area and a bounded visible
  Send footer. API 26-browser-appearance-state.md confirms unchanged identities,
  pagination, sanitization, draft ownership, authorization and
  request/receipt/publication contracts."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. footer-review.md and the M24 evidence/history
  rows review this focused presentation slice in enclosing change umouvlrv. No
  future commit hash is used. Independent provider, native restart execution,
  resize investigation and unapproved visual baselines retain their gates.

```
<!-- /evidence-field -->

<!-- evidence-field:notes markdown -->
## Notes

Live cold Projects Agents proof: repository c235f6f48a44e496f4f22a150a7a0356, recorded session 18141050b137d2a58c35fef89c22f8cf. DocumentBodies=1, ClassicComposers=0, one inline next-message draft, no Classic session footer. Initial own Send footer bottom=766.40 fell below panel bottom=712; bounded document wrapper plus in-scroll reviews fixes footer top=662/bottom=711 inside the panel. Recorded patch tool details opened. Screenshot .local/transcript-view-proof/projects-agent-document.png is ignored local proof. No prompt or review was sent, no user draft was changed. A transient intermediate HMR render was reloaded cold; final browser warning/error logs were empty. No ResizeObserver warning reproduced in the observed tab; this does not resolve earlier reports or approve visual baselines. Final source audit replaces the Classic-positioned message-action wrapper with the existing owned StoreRecordMenu inside the Document hover bar. Exact message, turn and session copy menu items are visible; structured content-part item IDs map back to their captured parent IDs without quote matching. Explicit Add session review comment opens and closes inside the live document; no content was submitted.
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-AGENT-TRANSCRIPT-VIEW-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

