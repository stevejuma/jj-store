---
schema: 1
id: E-CHAT-CARET-001
---

# E-CHAT-CARET-001 details 2

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
pnpm spec:first --base da752588 --head f5f212ec4bce07459458d0323796b2306deac023
```

**Notes**

The web/ ownership map also requires an API chapter. Added explicit unchanged API confirmation in 26-browser-appearance-state.md, covering retained IDs, source selectors, drafts and existing send contracts. /tmp/caret-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-application/ConversationCaret.stories.svelte web/src/lib/ui/store-agent-chat-body/chat-caret.ts web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte web/src/lib/ui/community-chat-composer/CommunityComposerSurface.svelte spec/src/current/contracts/api/26-browser-appearance-state.md spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/progress/M24/footer-review.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md
```

**Notes**

The final navigation refresh had removed whitespace in 2 generated M24 files. Repaired only those files; no product code changed. /tmp/caret-last-format-check.log.
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
pnpm exec prettier --write spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md
pnpm exec prettier --check spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md
```

**Notes**

Both repaired formatting targets pass; the optional blank between final navigation bullets was removed to retain the authored 400-line bound. /tmp/caret-ledger-format-repair.log and /tmp/caret-ledger-format-recheck.log.
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

Node 26.9.0, pnpm 10.32.1, JJ 0.45.1, Vitest 4.1.11, Storybook 10.5.7, Svelte 5.57.1; macOS ARM64. Released Design Core/Mira packages retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only CodeMirror theme extension. No API, storage, protocol, source offset or draft persistence format changes. Classic composer extension list remains independent.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Jujutsu change mnutkksp on default workspace, parent da752588. Shared 2px, 1.2-height-scale semantic caret theme now drives prototype and live Thread/agent document drafts. Source reading carets remain grey; drafts use primary. Removed duplicate draft caret CSS and negative overhang. Added small internal draft clearance and aligned the avatar. Acceptance covers actual clipping ancestors, empty/first/multiline draft positions and line-end geometry. UI-only: root aggregate lanes, Rust, provider execution and built-bundle lifecycle qualification are out of scope. Each Vitest invocation starts a fresh browser/Vite test process. Existing ResizeObserver diagnostics and unapproved visual baselines remain independent M24 gates. Final coverage is 49 passing direct-consumer scenarios plus a fresh 3-story rerun of the only corrected story family. No additional production edits followed the final web/catalog checks.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue independent M24 execution/platform, ResizeObserver investigation and visual-baseline qualification separately. This presentation slice is verified and committed in the enclosing change mnutkksp.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-015 and JST-UI-018: 37-document-conversations.md specifies
  shared semantic caret geometry, visible editor bounds and aligned internal
  draft clearance. Draft editing, immutable reading, submit commands and Classic
  behavior retain their existing contracts. The mapped API chapter separately
  confirms unchanged native identities, source selectors, drafts, send commands,
  publication and persistence."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This focused presentation slice does not close
  provider, multiple repository, execution, resize or visual-baseline
  qualification. Review is recorded in footer-review.md and the enclosing change
  mnutkksp; no future commit hash is used.

```
<!-- /evidence-field -->

<!-- evidence-field:notes markdown -->
## Notes

Live browser verification after the final inset: scroller start x=652/y=278; first-line caret x=654, top=284.05, bottom=307.45; avatar top=284, bottom=308. Line start and end share exactly the same vertical bounds. Caret has 2px left and 6.05px top clearance. User confirmed it looks okay. Test text was cleared; no prompt sent and no existing user draft changed. Screenshots: .local/document-caret-fixed.png and .local/document-caret-context.png. Current demo fixture/backend preserved; UI HMR served the default workspace, with a separate fresh verification tab.
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CHAT-CARET-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-018
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

- web/src/lib/ui/store-agent-chat-body/chat-caret.ts
- web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerSurface.svelte
- catalog/components/store-application/ConversationCaret.stories.svelte
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/progress/M24/footer-review.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/contracts/api/26-browser-appearance-state.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

