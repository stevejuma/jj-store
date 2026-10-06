---
schema: 1
id: E-UI-091
---

# E-UI-091 details 3

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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

Both directly affected story families pass12 keyboard/axe cases. /tmp/rich-search-main-stories-final.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/history-query.spec.ts
```

**Notes**

Both fresh production-host native/local browser cases pass with unchanged STRICT_CSP. Native checks highlighting, completion, invalid-draft graph retention, frozen time, clear, operation grammar, runtime errors and axe; local proves pinned profile transport. Semantics remain separately proved by actual Rust/HTTP readers. Host is stopped. /tmp/rich-search-main-cold-browser.log
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
pnpm check:types; pnpm test:storybook src/shared/filter/search-filter-bar; pnpm spec:check with JJ-selected --file arguments; pnpm exec prettier --check <integrated files> (Design Core main)
```

**Notes**

Four type/strict consumer checks have zero errors/warnings;3 search family files /10 stories pass; canonical validation, mdBook and spec-first pass. Initial default merge @- selection was ambiguous and replaced by explicit JJ file inventory. Source integration record owns details and original full nonvisual qualification. /tmp/rich-search-main-dc-{types,stories,spec-final,format-check}.log
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
pnpm spec:first --base 50363aec --head @
```

**Notes**

Tool exits successfully but reports no protected files through its Git range path; discarded as acceptance evidence because it does not inspect the active JJ merge.
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
pnpm spec:first with --file arguments from jj --no-pager diff --from 50363aec --to @ --name-only; pnpm spec:validate; pnpm check:progress
```

**Notes**

Actual JJ change inventory qualifies60 protected files /9 owning chapters. Five validators /21 chapters /147 requirements and120 tasks pass. /tmp/rich-search-main-spec-first-jj.log, spec-validate.log and progress-before.log
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
pnpm exec prettier --write <changed production, fixture, story and canonical files>; portable-lock and installed-source assertions
```

**Notes**

Changed files formatted; manifest and lock keep normal registry semver ranges. Active main source installation resolves only ../design-core; ignored temporary dependency hook remains solely in the retirement candidate.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (new integration evidence before versions field)
```

**Notes**

New E-UI-091 omitted required versions metadata. Added observed toolchain and source/registry package versions; no implementation change. /tmp/rich-search-main-progress-final.log
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
pnpm exec prettier --check <all JJ-selected non-Rust integration files>; cargo fmt -p jj-store-cli -p jj-store-http -p jj-store-jj -- --check; pnpm spec:first with explicit JJ-selected --file arguments
```

**Notes**

Every matched integrated frontend/schema/chapter file is formatted; three owning Rust packages pass formatting;60 protected files /9 chapters mapped. /tmp/rich-search-main-format-final.log, rust-format.log and spec-first-final.log
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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte (final diagnostic assertion); pnpm check:progress; pnpm spec:validate
```

**Notes**

Three exact production editor keyboard/axe scenarios pass, explicitly retaining the invalid diagnostic with draft and accepted query after head refresh. Corrected versions metadata passes120-task bookkeeping and147-requirement validation. /tmp/rich-search-main-refresh-final.log, progress-final2.log and spec-validate-final2.log
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
pnpm exec prettier --write/check spec/src/milestones.md and final story; pnpm check:progress; jj commit -m "Integrate rich history search into main"; jj commit -m "Integrate custom search languages into main" (Design Core)
```

**Notes**

Verified integrations committed from both default/main workspaces; both became clean empty working copies. Product implementation parents remain ancestors. Final bookkeeping passes. Retirement-only documentation is folded into these same integration changes.
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
jj --no-pager st (both temporary checkouts); jj --no-pager log -r "lsmrmlpp & ::@-" (app main) and "ookknysq & ::@-" (Design Core main); lsof/ps ownership checks; kill -TERM 88148
```

**Notes**

Both temporary working copies are empty; both implementation parents are retained in main. Found leftover source-qualification Python static host PID88148 with cwd design-core-rich-search and sole listener127.0.0.1:9170. Stopped only that owned host; no remaining process uses either temporary workspace. Main development services and unrelated static hosts retained.
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
jj workspace forget rich-search (app main and Design Core main); trash -v /Users/stevejuma/code/lapismd/jj-store-rich-search; trash -v /Users/stevejuma/code/lapismd/design-core-rich-search
```

**Notes**

Only the two user-authorized rich-search workspace registrations removed. Both folders moved recoverably to macOS Trash after verification and main commits. All eleven remaining app workspace registrations and Design Core release-verify-fix are retained. Main dependency hook, portable manifest/lock and installed graph have no temporary checkout reference.
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
pnpm exec prettier --write/check spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; jj squash --into @- -m "Integrate rich history search into main"; final JJ/paths/dependency assertions
```

**Notes**

Retirement-only evidence formatted and folded into the enclosing integration.120-task bookkeeping and147-requirement validation pass. Both main working copies are clean; implementation parents remain ancestors; both temporary registrations/directories and port9170 are absent. Installed main custom search source remains available with no temporary checkout dependency paths. /tmp/rich-search-main-retirement-{format-check,progress,spec}.log
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

<!-- evidence-field:formats markdown -->
## Formats

Additive QueryInfo, operations_filtered and optional invalid_query diagnostic contracts remain identical to the verified source implementation. Merge retains newer Appearance, agent_session reviews, Threads and automatic refresh; route type reexport preserves existing imports. No storage, native identity, manifest, WAL, receipt, credential or companion format change. Registry-only consumption remains a separately recorded E-UI-090 release gate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Verified source main integration across frontend controllers, custom editor, native/local graph readers and generated contracts. No root jj-store check/test/build/spec:check aggregates, unrelated Rust/runtime/RustFS suites or live-service reset/restart; no package publication. Design Core visual tests and baseline updates excluded by explicit user instruction. Complete prior source qualification remains retained; focused integration is not a full repository pass or milestone completion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Main source integration and authorized workspace retirement are complete. Normal Design Core release and registry-only consumer qualification remain the separately recorded E-UI-090 follow-up; visual baselines remain pending.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ and jj-lib0.45.1; rustc1.98.1; Node24.10.0; pnpm10.32.1; Svelte5.57.1; source Design Core package0.1.2 (registry dependency range^0.2.0); Playwright1.63.0; Vitest4.1.11.
<!-- /evidence-field -->

