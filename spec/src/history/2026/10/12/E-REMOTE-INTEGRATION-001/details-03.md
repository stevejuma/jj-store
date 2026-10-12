---
schema: 1
id: E-REMOTE-INTEGRATION-001
---

# E-REMOTE-INTEGRATION-001 details 3

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
node node_modules/@lapismd/spec-validator/dist/cli.js validate
```

**Notes**

7 validators, 3682 chapters, 188 requirements. Regenerate navigation after adding evidence continuation pages; /tmp/jj-land-spec-final2.log. UI-ownership also passes through the owning validator. A mistaken standalone scripts/ui-ownership.mjs invocation was corrected to the registered validator.
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
node --test scripts/spec/evidence-corrections.test.mjs scripts/spec/repaginate-evidence.test.mjs scripts/published-foundations.test.mjs
```

**Notes**

8 pass after final formatting. Original evidence is unchanged; current registry dependency origins and absent pnpm override are rechecked. /tmp/jj-land-ledger-final.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook [101 direct/shared story files]
```

**Notes**

Completed widened consumer lane: 519 pass, 4 fail across 101 files (523 stories), 719.63s. Failures: obsolete Preferences navigation, captured disabled initial workspace toggle, pointer Copy submenu, and a chat-editor geometry assertion seen only in this expanded run. ResizeObserver viewport diagnostic also appears on unchanged main. No full-lane success claimed; /tmp/jj-land-stories-final.log.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte
```

**Notes**

Read-only comparison on unchanged default main: 12 pass. Same viewport ResizeObserver diagnostic recorded there; no main file edits or development-data changes. /tmp/jj-land-main-comparison.log.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

Read-only unchanged main geometry comparison: 2 pass; /tmp/jj-land-main-layout-comparison.log. Subsequent corrected exact subset is recorded separately.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

15 pass across all four expanded-run failing files. Preserve every assertion. Use current Preferences subtab, fresh enabled workspace toggle, native pointer for a pointer-opened copy submenu, and portal-close pointer readiness. Unchanged chat geometry passes without production/style changes. /tmp/jj-land-last-four.log. The earlier full lane remains recorded as 519 pass/4 fail; this is an exact follow-up, not a full-suite pass.
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

macOS ARM64; Node 26.9.0; Rust/Cargo 1.98.1; custom JJ 0.45.1; released Design Core 0.3.1, Mira 0.0.3 and Spec Validator 0.1.4. No package manifest/lockfile substitutions.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

HTTP/OpenAPI inventory merges both parents; public wire/configuration/native formats unchanged from their combined governed changes. Immutable historical payloads remain exact. New ledger correction metadata is governed by E-LEDGER-CLEANUP-001.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-authorized main-workspace rebase, inherited source/story/governance cleanup and verified local landing. Retain main profile/emoji normalization and existing registry packages. UI extractions preserve controller/session ownership, invalidation, polling, original pending request identities, navigation and permissions. Hidden projected panels now use public visibility rather than a source DOM wrapper. Host callbacks capture current projection data across reader remounts. No repository migration, reset, package publication, service installation or visual baseline update. Final metadata gates: node scripts/check-progress.mjs passes (152 tasks, 771 evidence records); node node_modules/@lapismd/spec-validator/dist/cli.js first passes (83 protected files, 5 canonical chapters); validate passes (7 validators, 3682 chapters, 188 requirements). Final catalog type check has zero errors/warnings and 412 modules satisfy boundaries. Final changed-file formatting exposed a 401-line M24 index. Move the complete E-UI-MENU-001 review and related links into its existing agents presentation review, retaining every statement and all task/gate/evidence identities. M24 remains In progress; this is lossless bounded-document cleanup. Its obsolete generated index continuation is declared as removed; the bounded index keeps the same child navigation.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the verified integration and restart main with existing development data; record those execution outcomes separately. M27 installed providers, Linux real storage and desktop qualification remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-SEC-009/API-011/UI-022 preserve remote principal, fixed-origin,
  memory-only credential and durable-request contracts across integration.
  JST-UI-016 preserves exact Settings/draft/selection/Details restoration and
  supported controls; public hiding and retained callback snapshots repair
  existing lifecycle obligations. GOV-004/010/011 metadata repair is separately
  governed by E-LEDGER-CLEANUP-001; original completion/immutability authority
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M27 remains In progress. Fresh deterministic-ACP macOS real-storage
  integration qualifies this slice, not installed provider execution, Linux real
  storage or desktop delivery. Bootstrap bookkeeping validation and source-limit
  cleanup cannot promote product gates. M24 review text is split losslessly with
  no status/gate change.

```
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
<!-- /evidence-field -->

