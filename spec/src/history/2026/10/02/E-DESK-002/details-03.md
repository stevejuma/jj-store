---
schema: 1
id: E-DESK-002
---

# E-DESK-002 details 3

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
node scripts/desktop.mjs stage --server-dir /Users/stevejuma/code/lapismd/jj-store/target/debug --rustfs-runtime /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs; pnpm desktop:bundle
```

**Notes**

212 assets and explicit paired binaries staged; native debug app rebuilt. /tmp/jj-store-desktop-chrome-app-build.log. Existing installed tools and unrelated demo retained.
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
CUA cold launch rebuilt bundled app; AX and screenshot
```

**Notes**

Existing Projects UI retained same57653 origin. Separate titlebar/title text gone; native close/minimize/fullscreen controls present and header-leading space retained. No visual baseline approval.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CUA sidebar/drag attempts after fresh snapshots and bundle-ID selection
```

**Notes**

Two stale-state guards and one noWindowsAvailable response prevented reliable native gesture verification. Refreshed AX shows active sidebar controls and same Projects route; no native drag movement claim.
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
ps -axo pid=,ppid=,comm= (after native view)
```

**Notes**

Owned app60321/service60346/RustFS60348 remain running for user review. Appearance demo42374 remains separately running. No unrelated daemon restart or installation.
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
pnpm spec:validate
```

**Notes**

5 validators,22 chapters,151 requirements.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (before mapped chapter completion)
```

**Notes**

Default web helper and provenance mappings additionally required api.md and spec-governance.md. Added their owning marker/attribution contracts.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (before Storybook reference correction)
```

**Notes**

Ledger scanner recognizes named executable tests rather than Svelte Story name attributes. Story files retained as test references; exact story names and filters remain in recorded commands/results.
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
cargo fmt --manifest-path desktop/Cargo.toml -- --check
```

**Notes**

Affected native Rust formatting passes.
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
pnpm exec prettier --write catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/fixtures/StoreApplicationFixture.svelte desktop/README.md docs/ui-provenance.json spec/src/api.md spec/src/desktop.md spec/src/milestones.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/desktop-chrome.ts web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-app-shell/header-hover.css web/src/lib/ui/store-application/StoreApplication.svelte
```

**Notes**

Changed supported source/prose/JSON files formatted. Native Rust checked separately.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (Storybook file-only references)
```

**Notes**

Scanner requires fn/def/test declarations for this optional field. Storybook-only acceptance remains named explicitly in actual run commands and results rather than substituting unrelated executable tests.
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
pnpm spec:first (final)
```

**Notes**

12 protected files and5 owning chapters pass after mapped API/provenance chapter completion.
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
pnpm check:progress (final)
```

**Notes**

125 tasks; M17 and M17.3 remain In progress with explicit unqualified native gesture and later release/control gates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Actual native header drag/double-click gestures remain unqualified after CUA
  stale-state/noWindowsAvailable errors. Native chrome rendering and production
  controls have cold smoke evidence.
- Visual baseline approval, full native accessibility, macOS double-click
  preference parity, folder controls, first-render handshake and dirty-edit
  protection remain separate M17 gates.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native window presentation and its production-header consumers only. Exact regression and9 final directly relevant Storybook interactions pass; no diagnostic guard weakened. Cold production WebView/package evidence is distinct from browser harness evidence. Root aggregates, full Rust workspace, storage suites, unrelated UI suites, public signing and DMG qualification are out of scope. No Buzz implementation copied; existing owned Community hover styles only extracted with provenance.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Manually qualify native drag and double-click, then continue the remaining M17.3 native controls/render/dirty-edit and M17.4 distribution gates.
<!-- /evidence-field -->

