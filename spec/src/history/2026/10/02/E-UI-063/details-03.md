---
schema: 1
id: E-UI-063
---

# E-UI-063 details 3

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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check (66 changed supported files)
```

**Notes**

Spec-first passes59 protected files/6 chapters; canonical5 validators/21 chapters/147 requirements and changed-file formatting pass. Initial progress gate finds10 references to5 renamed manual-refresh tests and rejects out-of-scope wider qualification notes in remaining for a passing focused slice. Added explicit retired-test entries without rewriting historical evidence; broader gates stay in scope/next, milestone statuses unchanged. /tmp/jj-auto-refresh-{spec-first,spec-validate,progress,format-check}.log.
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
pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/automatic-refresh.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts -g 'new repository data|hash route restores|multiple_heads_require_choice|event_reconnect_resumes' (latest built source)
```

**Notes**

5/5 pass18.6s on final-qualified built source; preview stops automatically. /tmp/jj-auto-refresh-cold-final-qualified.log.
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
pnpm exec prettier --write web/src/lib/ui/store-application/StoreApplication.svelte spec/src/ui.md; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

File/conflict editors now share the existing editing guard as a keyed refresh pause; drafts/target base remain stable until editing finishes. Formatting passes. First file-editor story provider loses browser WebSocket connection before any tests run; no assertion suppression. Fresh exact retry recorded separately. /tmp/jj-auto-refresh-format-file-editor.log and file-editor-stories.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte; repeat exact EditAndRename with settled-dialog assertion and app focus experiment
```

**Notes**

Three fresh runs reproduce one Svelte derived_inert runtime error each despite interaction assertions passing: 2/2 stories then 1/1 selected twice. Bits UI 2.18.1 dismissible-layer deferred focus callback reads disposed derived state. Waiting for Save dialog removal and app focus workaround do not fix it; app workaround fully restored with JJ, no StoreFileEditor production diff retained. Logs file-editor-stories-final, file-editor-settled and file-editor-focus-after. Runtime diagnostics remain enforced.
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
pnpm add -D bits-ui@2.19.4; restore pnpm-lock.yaml with JJ; pnpm install --ignore-pnpmfile --lockfile-only; pnpm install --ignore-pnpmfile --frozen-lockfile
```

**Notes**

Inspected released 2.19.4 destroyed-layer guard before upgrading direct Bits UI 2.18.1 and nested 2.19.3. Initial private local Design Core hook leaked a local resolution into lockfile; restored parent lock then generated and installed clean registry-only lock ignoring the hook. Final tracked lock has six line replacements, no local paths or pnpmfile checksum. No node_modules edits or dependency patches. Logs bits-update and registry-install.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Released Bits UI 2.19.4 fixes exact failure: 2/2 stories pass, zero runtime warnings/errors; preserved settled-dialog assertion. 12.94s. /tmp/jj-auto-refresh-file-editor-released.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (clean registry-only dependency install)
```

**Notes**

Six type errors in five unchanged production consumers reproduce existing Design Core 0.2.0 API gaps: ToolCallStatus unknown, FileListingDirectory/directories/virtualize, AppShell element and OverlaySheet mobilePlacement. Source remains owner; no consumer shims/overrides or source publication. /tmp/jj-auto-refresh-web-released.log. M16 registry-only release qualification remains pending.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit; pnpm web:test; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook (clean registry-only install)
```

**Notes**

Units 238/238 in40 files and web tests70/70 pass. Full released UI stories:187 pass/9 fail in53 files/196 tests,194.65s. Seven failures depend on missing Design Core APIs; two stale Back to app selectors corrected separately. Logs all-units-released, all-web-tests-released and all-stories-released. This is a recorded registry limitation, not a released-only acceptance pass.
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
cp pnpm-lock.yaml /tmp/jj-auto-refresh-registry-lock.yaml; pnpm install --no-frozen-lockfile; cp /tmp/jj-auto-refresh-registry-lock.yaml pnpm-lock.yaml
```

**Notes**

Restored existing private Design Core local-source validation setup without changing tracked registry ranges/lock. Source package0.1.2 at immutable 59f25e0792875f99ba57d60c7da14cf5070166ce, clean owning workspace; source inspected read-only and not modified/published. Released Bits UI2.19.4 retained. /tmp/jj-auto-refresh-validation-install.log. All following UI validation uses this explicitly temporary source setup.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook (restored local-source validation setup)
```

**Notes**

Full run completes only32 of53 files:143 tests pass/2 fail, plus1 browser-provider WebSocket closure at RepositorySearch. Two failures are stale Back to app return selectors inherited from earlier settings header change. Assertions/diagnostics not disabled; changed selectors to existing Back to jj-store. Fresh sequential sharding used to complete all files without provider pressure. /tmp/jj-auto-refresh-all-stories-validation.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte -t 'AppearanceFocusPreservesChannelAndReplyDrafts|FocusAndSplitPreserveToolDiscussionDrafts'
```

**Notes**

Exact two repaired settings return/draft cases pass with6 unrelated skipped,14.34s. Both draft assertions preserved. /tmp/jj-auto-refresh-settings-return-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit; pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm web:build (restored validation setup)
```

**Notes**

Full catalog units238/238 in40 files pass; web/catalog report0 errors/warnings and129 catalog modules; architecture735 modules; fresh UI build12.93s with existing chunk-size advisory. Logs all-units-validation, web-validation, catalog-validation, architecture-validation and build-validation. Root aggregate/Rust commands not run.
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
pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/automatic-refresh.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts -g 'new repository data|hash route restores|multiple_heads_require_choice|event_reconnect_resumes'
```

**Notes**

Latest built UI/local-source validation5/5 pass27.2s: automatic current-head/historical behavior, hash/back/forward/reload, multiple-head choice and reconnect/reset. Fresh isolated preview7458 stops automatically. /tmp/jj-auto-refresh-cold-validation.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 JJ_STORE_REAL_TASK=c8a8573c0204be10644dcd77d16df2b8 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/automatic-updates.spec.ts
```

**Notes**

Latest live native/HMR2/2 pass27s, actual own/external comments update and observer draft retained; WCAG axe0 violations/runtime0 errors. Test comments tombstoned in finally. Stale metadata case intercepts reads and refuses captured writes, no real task/project metadata change. /tmp/jj-auto-refresh-live-validation.log.
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
node /tmp/jj-auto-refresh-restart-ui.mjs (two UI-only dependency refreshes)
```

**Notes**

UI PID36218 to50285 to52247 after dependency installs; backend PID36217 unchanged, fixture retained. Explicit process/root checks and bootstrap readiness before atomic runtime metadata update. Logs ui-restart and ui-restart-validation. No Rust rebuild/backend restart/reset.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 1/3; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 2/3
```

**Notes**

Sequential fresh-browser runs pass62/62 in18 files49.80s and69/69 in18 files73.04s. No browser/build jobs run concurrently. /tmp/jj-auto-refresh-stories-shard-{1,2}.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 3/3
```

**Notes**

17 files complete:64 pass/1 fail,79.38s. Appearance tool-discussion helper synchronously looks for Appearance before profile-menu onOpenChangeComplete finishes its close animation. Exact same story alone passes1/1 in9.63s, showing a timing race. No product/runtime diagnostic failure. /tmp/jj-auto-refresh-stories-shard-3.log and appearance-open-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte -t 'AppearanceFocusPreservesChannelAndReplyDrafts|FocusAndSplitPreserveToolDiscussionDrafts'
```

**Notes**

After awaiting the accessible Appearance control in both directly affected stories,2/2 pass with6 unrelated skipped12.98s. Existing Focus/Split draft assertions remain unchanged. /tmp/jj-auto-refresh-settings-await-after.log.
<!-- /run -->
<!-- /evidence-field -->

