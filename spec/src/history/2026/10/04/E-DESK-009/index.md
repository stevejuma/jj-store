---
schema: 1
id: E-DESK-009
---

# E-DESK-009

<!-- evidence-field:id markdown -->
## Id

E-DESK-009
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

desktop-terminal-header-refinement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tknuwyuq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Five exact terminal-header stories fail: separate close button, 4px separator, maximize leaves details open, and missing tab hover in light/dark. Main simultaneously contains an unrelated active conversion merge, so implementation uses isolated terminal-polish workspace from nrwwnrqz.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-terminal/TerminalHeader.stories.svelte
- catalog/fixtures/TerminalFixture.svelte
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/desktop.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-terminal/StoreTerminalDock.svelte
- web/src/lib/ui/store-terminal/StoreTerminalWorkspace.svelte
- web/src/lib/ui/store-terminal/TerminalViewport.svelte
- web/src/lib/ui/store-terminal/terminal.css
<!-- /evidence-field -->

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
pnpm test:storybook catalog/components/store-terminal/TerminalHeader.stories.svelte (initial dependency setup)
```

**Notes**

A symlink to main node_modules cannot load Storybook setup under the isolated Vite root; no product tests ran. Frozen offline install also rejected the existing ignored local hook checksum. Replaced symlink with an APFS clone of existing installed dependencies; tracked manifests/lock unchanged, original install.log retained.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-terminal/TerminalHeader.stories.svelte (before)
```

**Notes**

before.log:5/5 reproduce requested UI gaps.
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
pnpm test:storybook catalog/components/store-terminal/TerminalHeader.stories.svelte (initial after)
```

**Notes**

after.log:4 pass; keyboard tab navigation exposes pre-existing TerminalViewport autofocus stealing focus. Retain tab focus on session switch rather than weakening assertion.
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
pnpm test:storybook catalog/components/store-terminal/TerminalHeader.stories.svelte (after focus repair)
```

**Notes**

after-focus.log:5/5 pass including native browser pointer CSS hover checks in light and dark themes.
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
pnpm test:storybook catalog/components/store-terminal/Terminal.stories.svelte catalog/components/store-terminal/TerminalHeader.stories.svelte
```

**Notes**

terminal-stories.log:9/9 interaction/accessibility checks in a fresh Vite/Chromium process. Inline close geometry, inactive tab close, arrow/End navigation, directory row removal/tooltip retention, one-pixel separator with keyboard resize, maximize/restore sidebar callback, hover, original sessions/folder/hide/collapse/IME/paste/copy behaviors pass. No axe suppressions or visual baselines.
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
pnpm web:check; pnpm catalog:check (initial and final); pnpm check:ui-ownership; pnpm check:architecture (initial and final); pnpm spec:validate; focused pnpm exec prettier --write/--check
```

**Notes**

web-check-final/catalog-check-final:0 errors/warnings,187 catalog modules. ownership.log passes; architecture-final:942 modules; spec-validate:22 chapters/168 requirements; format-check passes. Community ActivityTabs and Design Core public Button/Resizable contracts inspected first; intentional Button host identity provides terminal-specific controls. Terminal uses default one-pixel Resizable handle and retains its hit area.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first (initial)
```

**Notes**

progress.log passes137 tasks. spec-first.log reports two missing mapped chapters for UI/API and provenance/governance. Added local-only maximize authority and public presentation-contract notes to the required chapters.
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
pnpm spec:first; pnpm spec:validate; focused document Prettier (final)
```

**Notes**

spec-first-final:8 protected files/5 canonical chapters. spec-validate-final:22 chapters/168 requirements. Focused formatting passes; all420 historical ledger entries preserved unchanged.
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
pnpm web:build
```

**Notes**

web-build.log:production frontend build passes. Existing CSS/global and chunk-size warnings retained; no Rust build or host behavior change.
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
jj rebase -r tknuwyuq -o rryrrvzz; jj edit tknuwyuq; jj rebase -s tknuwyuq -o qvtsvzty; preserve both canonical additions and historical ledger objects
```

**Notes**

Main completed its unrelated conversion merge and review-startup record. Rebasing onto final main1175e0df required only additive ledger/governance resolutions; all425 main evidence objects remain unchanged. Production terminal files are byte-identical to original verified880f1e87.
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
pnpm test:storybook catalog/components/store-terminal/Terminal.stories.svelte catalog/components/store-terminal/TerminalHeader.stories.svelte; pnpm web:check; pnpm check:architecture; pnpm check:progress; pnpm spec:first; pnpm web:build (after integration)
```

**Notes**

integrated-stories9/9 in fresh Chromium/Vite, integrated-web-check zero diagnostics, architecture948 modules, progress137 tasks, spec-first8 protected files/5 chapters, integrated-web-build passes. Final parent startup record changes documentation only.
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
node scripts/desktop.mjs stage --server-dir <existing main bundled runtime, then main target/debug> --rustfs-runtime <existing explicit runtime> --web-root /Users/stevejuma/code/lapismd/jj-store-terminal-polish/web/dist; pnpm dlx @tauri-apps/cli@2.11.4 bundle --debug --bundles app
```

**Notes**

desktop-stage-preview/bundle-preview and desktop-stage-integrated/bundle-integrated logs pass. Preview bundle was not launched; final combined212-asset UI uses main rebuilt backend from independently qualified conversion merge. No Rust build/test run for this UI-only change.
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
CUA Quit and reopen main jj-store.app; show terminal; open details; maximize; restore; native accessibility and appearance inspection
```

**Notes**

Cold native WKWebView with existing installation at127.0.0.1:57653. Confirmed old shell idle/no jobs; Quit removed app50857/service50866/runtime50867/shell51586. Reopened same registered repository with updated bundled assets and fresh idle shell. Accessible terminal tablist and separate inline close button retained; duplicate path row absent. Details changed expanded→collapsed on maximize and stayed collapsed after restore. Thin top divider and leading close placement inspected; no visual baseline approval. App remains open with terminal for review.
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
jj new tknuwyuq in main; SHA-256 verified evidence copy; jj workspace forget terminal-polish; remove exact temporary workspace; jj edit tknuwyuq in main
```

**Notes**

Main advanced from1175e0df to terminal slice without a new merge. Temporary workspace was clean after commit;36 evidence files copied byte-identically into main .local/terminal-polish-evidence before deleting only jj-store-terminal-polish. Other workspaces/services untouched. Final record is completed in the same enclosing terminal change.
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
pnpm web:build; SHA-256 comparison of main frontend and running bundle; pnpm spec:first; pnpm spec:validate; pnpm check:progress; focused pnpm exec prettier --check (final main)
```

**Notes**

main-web-build passes. bundle-parity.json confirms all213 files, including manifest, match the updated app bundle exactly. main-spec-first8 protected files/5 chapters; main-spec-validate22 chapters/168 requirements; main-progress137 tasks; main-format passes. All425 prior main ledger objects preserved; only terminal evidence and its task link appended.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pnpm10.32.1; Vitest4.1.11; existing locally substituted Design Core dependency for validation only.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native IPC, API, repository, storage, credential or persistence change.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-DESK-009 details 2](details-02.md)
