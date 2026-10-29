---
schema: 1
id: E-REUSABLE-PROGRESS-005
---

# E-REUSABLE-PROGRESS-005 details 2

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
cd ../design-core-history && pnpm checks
```

**Notes**

Fresh complete source qualification after the syntax/disclosure/metadata fixes: 821 unit tests, 834 stories in 242 files, all four strict type profiles, specification, formatting, static catalog and 16+11+3+142=172 browser cases pass; one existing platform-specific skip. Earlier superseded/interrupted and live-reload attempts remain recorded in source records and E-REUSABLE-PROGRESS-004.
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
cd ../design-core-history && node scripts/with-storybook-env.mjs pnpm exec vitest run --project storybook src/shared/ai/message-metadata/MessageMetadata.contrast.stories.svelte
```

**Notes**

Two source regressions fail before the repair (3.45:1 on conversation, 3.13:1 on muted), then pass. Existing semantic colors are mixed within the owning dark component style; no new public token/prop, consumer override or accessibility suppression.
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
cd ../design-core-history && node scripts/with-storybook-env.mjs pnpm exec vitest run --project storybook src/shared/ai src/shared/chat
```

**Notes**

All 188 directly affected cases in 39 files pass.
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
cd ../design-core-history && pnpm package:local
```

**Notes**

Prepared public archive SHA256 3f7fefb5ca546ea3ce4abfcb87ef1099c2b181fb4b506ad29cd286884b726525, source snapshot 2c6f0b9e. CSS audit: 253 files, three compiled inputs, no remaining utility references. Local migration validation only.
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
cd ../lapis-desktop-history && pnpm local:install
```

**Notes**

14 public archive routes verified; tracked dependency files preserved. Final current API archive also installed with SHA256 e8072f731ca2d3db7dfb140a85cfc9fd9c8fef63c1b151d079de89bdc77f0887, producer snapshot 17e72beb.
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
cd ../lapis-desktop-history && LAPIS_CATALOG_THEME=dark LAPIS_CATALOG_VIEWPORT=desktop pnpm exec vitest run --config vitest.catalog.config.ts catalog/plugins/ai/CommandsAndResults.stories.ts catalog/plugins/ai/Jsonl.stories.ts catalog/plugins/ai/TraceAndScrolling.stories.ts -t 'Skills And Slash|Malformed Record|Failed Message And Retry'
```

**Notes**

Three formerly failing consumer contrast cases pass, ten unrelated cases skipped. The eight affected family retry subsequently passes 25 cases in each theme/viewport combination, 100 total, on its recorded pre-rebase app baseline.
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
cd ../lapis-desktop-history && jj rebase -r @ --onto 447f8244a9447a6eea47c1d82af1d7aedd656c69
```

**Notes**

Current desktop review source retained. Two coverage-document conflicts resolve by preserving the latest parent content and appending the history mapping. No production-code conflict. The pre-rebase consumer results retain their original scope.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
cd ../lapis-desktop-history && pnpm check:types
```

**Notes**

Current app initially reports one missing resolveMerged method in the older API artifact and eight unsupported exact options in role queries. The API fix is rebased onto existing qualified producer 8a27bc36; removing unsupported options preserves ByRole full string-name matching. A separate cold import attempt executes no tests after six new static icons trigger optimization reload; those icons are now pre-bundled.
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
cd ../lapis-notes-history && pnpm --filter @lapis-notes/api exec vitest run src/lib/__tests__/workspace.test.ts -t "attaches a controller-opened view"; pnpm --filter @lapis-notes/api test; pnpm --filter @lapis-notes/api build
```

**Notes**

On current producer base 8a27bc36: exact two pass/129 skipped, full API 1160 cases in 127 files pass, public build/publint/declaration contracts pass. The existing merged-conflict API is preserved; no shim or consumer implementation is added.
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
cd ../lapis-desktop-history && pnpm check:types
```

**Notes**

Current app plus refreshed producer: zero errors and warnings.
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
cd ../lapis-desktop-history && LAPIS_CATALOG_THEME=light LAPIS_CATALOG_VIEWPORT=desktop pnpm exec vitest run --config vitest.catalog.config.ts catalog/plugins/repository/History.stories.ts -t 'Restores Historical Operation'
```

**Notes**

Before: one failure/16 skipped, restored cd operation replaced by current ab head. After: one pass/16 skipped. Initial undefined controller state no longer overwrites the saved explicit pin. The assertion also requires the first owned reader to open only the historical operation.
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
cd ../lapis-desktop-history && pnpm test:storybook catalog/plugins/repository/History.stories.ts
```

**Notes**

All 17 production history cases pass in all four configurations: 68 total. This is current-app fixture acceptance, not native WebView, native persistence or visual baseline approval.
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
pnpm dev:disk; pnpm dev:clean -- --dry-run; pnpm dev:clean -- --apply
```

**Notes**

Guarded inventory and preview identify idle jj-store target artifacts. Apply recovers free space from 23.9 to 32.9 GiB. No service stop, fixture reset, protected resource deletion or unrelated active desktop target cleanup. No Rust builds run concurrently with cleanup.
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
cd ../lapis-notes-history && pnpm check && pnpm test && pnpm build && pnpm test:storybook && pnpm spec:check
```

**Notes**

Current producer 8a27bc36 plus attachment fix: check 18/18 tasks, test 18/18, build 10/10, all 261 Storybook interaction/accessibility cases across 59 files, and specification checks pass. Session 3290 exits zero; earlier-base and failed attempts remain in source evidence.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cd ../lapis-desktop-history && pnpm test:storybook
```

**Notes**

Current desktop-light catalog:169 pass,4 fail/173 cases,44 files; other configurations do not run. Reading Note/Reading Note Dark expose stale Markdown artifact; Settings Source assumes an expanded sidebar; Leaving Workspace queries an intentionally inert shell. Source/current-app counts remain distinct.
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
cd ../lapis-desktop-history && pnpm local:install
```

**Notes**

Refresh isolated overlay to primary app existing Markdown reading archive: source8c345d30, SHA25694961f48973de70cc8a610a88c2064fc7e06051113915dd03d481a4689a91401. No tracked manifest/lockfile substitution.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cd ../lapis-desktop-history && pnpm exec vitest run --config vitest.catalog.config.ts catalog/plugins/Workspace.stories.ts catalog/plugins/desktop/FileConflicts.stories.ts catalog/plugins/terminal/Terminal.stories.ts -t 'Reading Note|Leaving Workspace|Settings Source'
```

**Notes**

After archive refresh but before harness fixes:2 pass,2 fail,28 skipped. After real sidebar navigation and hidden role queries limited to the intentionally inert case:4 pass,28 skipped. Retains terminal persistence and actual disabled/inert assertions. Full affected/current-app acceptance is still running.
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
pnpm check:progress
```

**Notes**

171 tasks,874 evidence records. No milestone completion inferred from source gates.
<!-- /run -->
<!-- /evidence-field -->

