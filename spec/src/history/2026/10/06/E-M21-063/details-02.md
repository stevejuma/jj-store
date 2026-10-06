---
schema: 1
id: E-M21-063
---

# E-M21-063 details 2

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
jj --no-pager st/diff; owned Community and Design Core dialog source inspection; shadcn-svelte Dialog documentation
```

**Notes**

Started clean otvxrvnt above sktnmmmv/555e25ad. Generic UI dispatch only prepared membership and required a second Apply command click. No backend or native graph changes required. All evidence under .local/stack-membership-immediate/.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
```

**Notes**

before-exact.log:2 passed/1 failed because a single Apply click never produced Unapply. Earlier title filter matched no generated story names (before.log:3 skipped), so the complete exact file was run.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
```

**Notes**

after-exact.log:3 passed with immediate native-style membership execution and explicit absence of a dialog after each action.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-membership.test.ts
```

**Notes**

unit-initial.log:4 passed/1 failed from a test calling a nonexistent draft message setter; use the actual workspace/stack-keyed draft title. Product behavior was not changed to satisfy that fixture.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-membership.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace-topology.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts
```

**Notes**

controller.log:7 files/97 tests passed. New cases cover one-click exact requests without preview calls, retained WIP/drafts, duplicate click blocking, teardown, unknown-result retained identity/no local retry, stale rejection/refresh and capability/non-membership exclusion. Existing grouped-unapply mapping and preview-based other commands remain covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte
```

**Notes**

stories.log:fresh Vite/Chromium26 passed/1 failed. Membership, grouped unapply and direct unknown-outcome recovery pass with blocking axe. Existing create-stack assertion still expected the empty text replaced in E-M21-062; updated it to the current target-relative empty state.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
```

**Notes**

story-recheck.log:fresh15/15 passed, closing the previous failure; all27 unique affected stories qualified. Other command previews and in-app conflict resolution remain exercised. No screenshot comparisons or baseline updates.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web.log/catalog.log:0 errors/warnings;248 catalog modules.
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate
```

**Notes**

architecture-final.log:1157 modules;ownership.log:1 validator;spec-first-final.log:14 protected files/3 canonical chapters;spec-validate.log:5 validators/23 chapters/173 requirements. Initial architecture301-line failure resolved by an ordinary early return in UI dispatch. Initial spec-first required api.md for local-session presentation wording; chapter now records unchanged wire/guard semantics. No caps or allowlists changed.
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
pnpm docs:review application <single-click membership and outcome-recovery explanation>; pnpm exec prettier --write <changed non-ledger files>; pnpm web:build
```

**Notes**

docs-review.log/format*.log/web-build.log:review fingerprint updated,source formatted,built UI and staged Help pass. Existing large-bundle warning remains; no dependencies or build configuration changed. Build is required for the exact cold native browser acceptance, not full release qualification.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

docs.log:an overlapping docs check collided with web:build Help staging while cleaning docs/build. Reran sequentially after the build; no source change or assertion relaxation.
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
pnpm docs:check
```

**Notes**

docs-final.log:139-file closure,links/anchors and9 Mermaid diagrams in light/dark pass after sequential rerun.
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
node scripts/stacks-browser-acceptance.mjs --grep '^cold_local_megamerges_apply_unapply_and_named_wip_survive_reload$'
```

**Notes**

native-browser.log:no tests matched because Playwright grep includes the full file/title; disposable service/folder cleaned up. Corrected to the unique unanchored test name.
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
node scripts/stacks-browser-acceptance.mjs --grep cold_local_megamerges_apply_unapply_and_named_wip_survive_reload
```

**Notes**

native-browser-exact.log:1 passed in13.9s from fresh built UI,server and disposable native folder. Actual Apply/Unapply update checkout/TOML without dialog; independent named WIP,diffs,settings and reload remain valid. Existing explicit target/debug executables used; no Rust build or installed CLI replacement. Fixture/service cleaned up.
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
CUA read current live tab on http://127.0.0.1:7476/
```

**Notes**

Current user view was focused Commits at operation5a41b5c5 after independent sample activity. Left its navigation and repository untouched; all mutation verification used disposable fixtures. Existing Vite HMR server remains running.
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
pnpm exec prettier --check <changed non-ledger files>; pnpm spec:first; pnpm check:progress
```

**Notes**

format-verified.log: all changed non-ledger files use Prettier style; spec-first-verified.log: 14 protected files/4 canonical chapters; progress.log: 142 tasks. Historical ledger entries preserved verbatim.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the current HMR preview; ordinary Apply/Unapply is immediate, while uncertain local outcomes require state inspection.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Exact membership browser regression,focused controller/direct consumers,cold affected interaction/axe and real native-folder browser acceptance,web/catalog types,source ownership/architecture and canonical/documentation/progress gates.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- Rust source/tests,full root aggregates,contract
  regeneration,cross-provider/release gates,package publication,screenshot
  baselines and sample repository repair are outside this frontend slice.
- No user sample commands or reset and no unrelated service restart. Milestone
  status is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

