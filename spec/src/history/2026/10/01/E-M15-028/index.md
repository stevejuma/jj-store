---
schema: 1
id: E-M15-028
---

# E-M15-028

<!-- evidence-field:id markdown -->
## Id

E-M15-028
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

aligned-thread-pane-headers
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change loorksls
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact Community alignment story: 1 failed / 2 passed; standard settings header is 56px versus main 39px (17px mismatch). Inspected Community settings and Design Core Shell/Guidance. Header block padding is consumer-owned; structural shell tokens already set the authoritative height.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; inherited Design Core 0.1.2 snapshot.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Remove duplicate consumer header height/padding; standard Thread pane and toolbar retain aligned separators. Outer rail remains independently controlled with its own treatment. No persistence/API formats change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.css
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/store-threads/StoreThreadHeader.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- web/tests/browser/threads-alignment.spec.ts
- spec/src/ui.md
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte (before); catalog/components/store-threads (after)
```

**Notes**

17px gap before; exact geometry regression and complete 8/8 direct stories pass after. /tmp/jj-thread-headers-before.log and after.log.
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
pnpm web:check; pnpm catalog:check; pnpm web:build
```

**Notes**

Both type checks zero errors/warnings; 121 catalog modules. Build passes with inherited size advisory.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs threads-alignment.spec.ts
```

**Notes**

Cold 1/1, settings/reply header bounds within 1px of main; separators present; independent rail and pointer/reply/reaction regressions retained.
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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-headers node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Main PID29250/storage29251. Served build matches; all snapshots/thread/messages/fixture/installation unchanged; search ready and HTTP200.
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

Main UI-only header slice. Only the standard Thread pane uses changed right header rule; independent outer rails excluded. Complete direct Thread stories/cold bundle and governance; Rust/root aggregates out of scope. Visual-pending remains separate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this rebuilt header slice, then scope reaction unknown/error feedback to message metadata in main and replies.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


