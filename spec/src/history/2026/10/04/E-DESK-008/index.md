---
schema: 1
id: E-DESK-008
---

# E-DESK-008

<!-- evidence-field:id markdown -->
## Id

E-DESK-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

desktop-terminal-main-integration
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nrwwnrqz
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/api.md
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
jj --no-pager st in main and desktop-terminal; jj workspace list; jj log/diff from utlypsqk- to main and terminal
```

**Notes**

Both workspaces clean. Main9faf8cc6 has newer committed Cursor/OpenCode capture settings/runtime work. Only API/architecture/UI/ledger paths overlap; no terminal production-code overlap.
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
jj new @- utlypsqk -m "Merge desktop repository terminal into main"; resolve canonical append conflicts; historical object/raw-prefix assertions
```

**Notes**

Merged terminal0b027a43 with main9faf8cc6 without rewriting either parent. Three canonical files conflicted:API/UI appended chapters retain both features; ledger retains all418 main evidence objects and raw JSON prefix plus unchanged E-DESK-007. Architecture auto-merged. All81 non-spec files from terminal change match feature bytes, including native runtime/tests, UI/tests, dependencies and provenance. M17.5 remains Partial and existing qualification limits are retained.
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
pnpm install --offline --no-frozen-lockfile; restore unchanged registry lock bytes; inspect node_modules/.modules.yaml
```

**Notes**

Installed Tauri API2.11.1 into main using cached packages. Existing local Design Core hook used for local validation; publishable lock remains byte-identical to terminal commit with registry ranges. No dependency points into retired workspace.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/terminal.test.ts
```

**Notes**

merge-units.log:5/5 pass, including uncertain-open identity, renderer reattachment, stale target fencing and obsolete-close callback regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-terminal/Terminal.stories.svelte
```

**Notes**

merge-stories.log:22/22 pass in fresh Storybook/Vite process with interaction and accessibility; direct merged shell consumers only.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:validate; pnpm exec prettier --check spec/src/api.md spec/src/ui.md
```

**Notes**

Zero web/catalog diagnostics;186 catalog modules,941 web modules; ownership and22 chapters/168 requirements pass. Canonical conflict formatting passes.
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
pnpm check:progress before integration evidence entry
```

**Notes**

Merge correctly required a new evidence entry rather than reusing either parent evidence; added this E-DESK-008 record.
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
Hash-verified copy of .local/desktop-terminal-evidence to main; assert clean kpqlktmk workspace; jj workspace forget desktop-terminal; remove exact jj-store-desktop-terminal directory
```

**Notes**

105 retained evidence files verified by SHA-256 before deletion. Rechecked temporary workspace was clean at original empty change. Removed only desktop-terminal registration and directory; other workspaces untouched. Previous native acceptance processes already stopped; no active temporary workspace services found. Main retains the committed terminal parent and all historical acceptance records.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first (default merge comparison)
```

**Notes**

merge-progress-final.log validates137 tasks. spec:first default @- is ambiguous for two-parent merge; rerun with explicit main and terminal bases. Complete historical evidence preserved; no product failure.
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
pnpm spec:first --base 9faf8cc6 --head @; pnpm spec:first --base utlypsqk --head @ (initial explicit-reference attempt)
```

**Notes**

Explicit base/head mode uses Git object references: @ resolved to Git HEAD and gave an uninformative no-change result; utlypsqk was rejected as a bad Git revision. Neither attempt qualifies the merge. Corrected both bases and merged head to actual commit IDs below.
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
pnpm spec:first --base 9faf8cc6452c --head 12e58fe23fc4f6e70099396a2ef906d6698431fe; pnpm spec:first --base 0b027a439054 --head 12e58fe23fc4f6e70099396a2ef906d6698431fe
```

**Notes**

Actual merge compared against both actual parent commits:main81 protected files/7 canonical chapters;terminal77 protected files/10 canonical chapters. Later edit adds only these evidence results. Final progress bookkeeping also passes137 tasks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pnpm10.32.1; Tauri JS API2.11.1. Native engine/toolchain evidence remains E-DESK-007.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No additional product, storage, API, native IPC or persisted-format change during integration.
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

User-authorized main-workspace merge and retirement only. Prior verified terminal files are byte-identical; canonical append conflicts preserve main capture work. Direct UI units, shell interaction/accessibility, type, ownership and architecture checked in main. Native build/lifecycle rerun, root aggregates, unrelated capture suites, visual baselines, package release and service restarts are out of scope because merge introduced no production-code resolution. Original E-DESK-007 native/cold evidence remains valid, with registry-only Design Core and broader desktop distribution qualification separately pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finalize the merge commit in main with clean working copy; retain copied evidence for existing release qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


