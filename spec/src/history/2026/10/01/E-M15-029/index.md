---
schema: 1
id: E-M15-029
---

# E-M15-029

<!-- evidence-field:id markdown -->
## Id

E-M15-029
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

targeted-reaction-message-metadata
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change krylzqny
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Reaction errors reach a page Alert; reply messages lack supplements and show a panel error. Corrected exact unknown-main/reply regressions: 2 failures plus 1 unhandled rejection. Initial display-name filter skipped 5 tests; fixture helpers first needed hover settling and a main-log text scope. /tmp/jj-thread-metadata-before.log through before3.log.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Svelte 5.57.1; inherited Design Core 0.1.2 snapshot, registry qualification separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Target-keyed errors retain exact revision/snapshot identities and discard stale sessions. Error kind scopes page feedback while pending receipts keep original requests across refresh, source reset, status not_known and retry. Replies receive owned message supplements. Existing MessageMetadata footer contains confirmation/conflict/error and recovery controls; Markdown bodies remain untouched. ApiError moved unchanged to a transport-free module, preserving client re-export/class identity for deterministic catalog refusals. No wire/native/persistence formats changed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/api/api-error.ts
- web/src/lib/api/client.ts
- web/src/lib/controller/collab-actions.svelte.ts
- web/src/lib/controller/message-reactions.svelte.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/ui/community-thread-panel/CommunityThreadMessage.svelte
- web/src/lib/ui/community-thread-panel/props.ts
- web/src/lib/ui/store-threads/StoreMessageReactions.svelte
- web/src/lib/ui/store-threads/StoreThreadReplies.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/tests/browser/reaction-metadata.spec.ts
- web/tests/vite/reaction-feedback.test.ts
- spec/src/milestones.md
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t 'Unknown.*[Rr]eaction'
```

**Notes**

Exact 2/2 after; no unhandled rejection; /tmp/jj-thread-metadata-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

25/25 shared direct Thread/Agent/Review/WorkStore stories. Added definitive rejection and reran complete Thread family: 11/11; combined direct qualification 26 checks. /tmp/jj-thread-metadata-stories.log and thread-final2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts web/tests/vite/reaction-feedback.test.ts; same project web/tests/vite/production-store-collaboration.test.ts; pnpm exec tsx --test web/tests/client.test.ts
```

**Notes**

25 Thread/reaction tests, 16 direct collaboration controller tests, 10 HTTP client tests. Retry request retained across refresh, reset and not_known status; definitive rejection stays targeted; Thread failures keep page feedback. Initial new test forgot explicit reaction reload after controller refresh: 1 failure, repaired test setup and reran 25/25.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm web:build
```

**Notes**

Type checks zero errors/warnings, 121 catalog modules, architecture 626 modules, production build passes with inherited size advisory. Initial story getByRole exact option caused 3 TS errors, removed unsupported option. Architecture initially 303/300 counted lines; reuse existing page-error getter instead of duplicate. Catalog rejected direct client import for fixture ApiError; split transport-free class and preserve client export. Final focused gates pass.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs threads-alignment.spec.ts reaction-metadata.spec.ts
```

**Notes**

Final cold built bundle 3/3 main/reply unknown metadata, stable message body and original status request, plus alignment/edit/reply/reaction-hover flow. /tmp/jj-thread-metadata-final-browser.log.
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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-metadata-final node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Owned main service restarted after rebuilt UI. Served index matches built assets; repository snapshots, directory, selected thread/messages, installation and fixture unchanged; search ready/HTTP200. /tmp/jj-thread-metadata-final-restart.log.
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
pnpm spec:first; scoped pnpm exec prettier --write/check; pnpm check:progress; pnpm spec:validate
```

**Notes**

Canonical UI/API chapters and same-change evidence. No thresholds/baselines loosened.
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
CUA live main verification: direct close/open settings, read header geometry, native pointer hover and screenshot
```

**Notes**

Main/settings header top49/bottom88 with 1px separators, no intermediary menu; shadcn tooltip shows 1 reaction with emoji and authorized actor. /tmp/jj-thread-final-controls.png; no user messages or pending actions changed.
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

<!-- evidence-field:scope markdown -->
## Scope

User-authorized main UI-only slice and rebuild/restart after each slice. Complete direct presentation/controller/HTTP boundaries plus cold bundle; Rust/root aggregates and independent real-RustFS/runner/registry/visual-baseline gates out of scope. M15 remains In progress; visual-pending unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified main slice; use rebuilt application at port7460. Keep independent milestone qualification and pre-existing Projects-filter story failures separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


