---
schema: 1
id: E-M21-044
---

# E-M21-044 details 4

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
pnpm check:progress
```

**Notes**

progress-picker.log rejects temporary result=incomplete while wider checks were running. Corrected to the supported fail/pass result vocabulary; no historical ledger changes.
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
pnpm exec prettier --write web/src/lib/ui/store-work/StoreProjectsRepositories.svelte spec/src/ui.md docs/site/guides/browsing.md docs/maintenance/coverage.json; pnpm docs:review application "Browsing covers focused Stacks navigation and contextual details, retained canvas and history state, disabled open picker choices during reads, and visible repository-row skeletons preserving loaded content."
```

**Notes**

Formatted the corrected busy predicate and focused requirement/guide explanation; refreshed only the application documentation fingerprint.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/*.stories.svelte
```

**Notes**

projects-loading-consumers.log: 61 pass across all 14 directly affected Projects/WorkStore story files, including the unchanged original loading regression. Closes the one failure from the 182-story direct-picker consumer run; other 181 stories passed in that run.
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
pnpm web:check; pnpm web:build
```

**Notes**

web-check-last-complete.log: zero errors/warnings after directory loading correction. web-build-last-complete.log: fresh final bundle, nine Mermaid diagrams, documentation closure 139 files/13,954,307 bytes, valid links/anchors; unchanged large-chunk warning.
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
pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check changed supported files (excluding spec/src/milestones.md)
```

**Notes**

spec-first-last-complete.log: 45 protected files/five canonical chapters. spec-validate-last-complete.log: five validators, 23 chapters, 173 requirements. format-last-complete.log: all supported changed files formatted; no historical ledger reformat.
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
pnpm check:progress
```

**Notes**

progress-final-check.log: 142 task mappings and evidence bookkeeping pass after using the supported result vocabulary. Historical statuses remain unchanged.
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

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-stacks-lifecycle.test.ts#binding revalidation
  preserves an in-flight canvas selection read
- web/tests/vite/production-stacks-application.test.ts#focused histories read
  the ordinary repository graph and retain the Stacks owner
- web/tests/real/stacks-browser.spec.ts#cold_focused_stacks_navigation_history_reload_and_return
- web/tests/real/stacks-browser.spec.ts#cold_local_megamerges_apply_unapply_and_named_wip_survive_reload
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified frontend slice with Jujutsu; keep live Vite 7476/backend 7478 and Storybook 6057 running for review. Disabled local file/hunk/graph commands, publication and release qualification remain separate work.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Focused repository Stacks navigation, contextual workspace/selection rail and hover-only circular Back qualify with 45 focused source tests, affected interaction/accessibility stories, both combined cold real-folder scenarios, web/catalog checks, UI ownership/architecture and live HMR verification. Open picker choices now pause/resume through reads; the widened Projects check has all 61 direct stories passing after correcting its viewport-read busy indicator. No milestone status promotion.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Rust builds/tests, root aggregates, unrelated catalog suites, package publication, backend/transport changes, screenshot comparisons/baselines. Existing development binaries reused; no fixture reset, installed executable replacement or unrelated service restart.
<!-- /evidence-field -->

