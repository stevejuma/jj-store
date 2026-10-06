---
schema: 1
id: E-DESK-005
---

# E-DESK-005 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CUA main debug app screenshot before; inspect pinned Buzz AppTopChrome, chromeLayout, ThemeProvider, theme.css, window_vibrancy.rs and tauri.conf.json
```

**Notes**

Native traffic lights centered at16px while toggle centered20px; CSS-only glass still painted an opaque gradient.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/desktop-glass.test.ts (before)
```

**Notes**

One failed suite: native glass controller absent; no native handshake existed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/desktop-glass.test.ts web/tests/vite/appearance.test.ts
```

**Notes**

15/15: delayed installation, rapid toggles, failure and explicit retry, disposal and device preference/draft consumers.
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
cargo check --manifest-path desktop/Cargo.toml --offline
```

**Notes**

Initial native vibrancy host compile and generated one-command ACL pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte (first)
```

**Notes**

Two header assertions exposed scoped CSS precedence under20px rem and mobile layout. One AppearanceShell import failed during cold Vite dependency optimization/reload.4 tests passed; failure guards retained.
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
pnpm test:storybook catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte (final)
```

**Notes**

16/16 cold Chromium interactions/accessibility; fixed40px header and23px control center remain at enlarged rem and narrow layouts; browser glass/pickers remain qualified.
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
pnpm test:storybook catalog/components/store-appearance/NativeGlass.stories.svelte
```

**Notes**

2/2 light/dark composition: one translucent gradient wash, opacity30/65, transparent header, no CSS blur and opaque main; normal browser fallback restored.
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
pnpm test:storybook catalog/components/store-application/DesktopChrome.stories.svelte (icon-only sizing refinement)
```

**Notes**

2/2; fixed dimensions restricted to icon-sized buttons, retaining content-sized repository picker buttons.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

0 errors/0 warnings for both type checks;160 catalog modules,838 architecture modules.
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
pnpm check:ui-ownership (new CSS provenance first and second attempts)
```

**Notes**

First missing tracked entry; second Community-copy schema rejected Buzz path. Classified native app-owned CSS under owned with complete separate Buzz source/license attribution; scanner rules unchanged.
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
pnpm check:ui-ownership (final)
```

**Notes**

22 chapters/152 requirements; native source-pinned adaptation and owned presentation boundary accepted.
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
pnpm desktop:test
```

**Notes**

5 host tests pass; explicitly staged real-runtime target remains ignored in this regular command.
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
cargo build -p jj-store-cli --bins --locked; pnpm web:build; node scripts/desktop.mjs stage --server-dir /Users/stevejuma/code/lapismd/jj-store/target/debug --rustfs-runtime /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --web-root /Users/stevejuma/code/lapismd/jj-store/web/dist; pnpm desktop:bundle
```

**Notes**

Current main paired CLI and212 production assets staged; debug app bundled. Web build retains existing chunk-size warning. Required for cold native qualification, no unrelated aggregate.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CUA Quit and cold launch updated main bundle; user screenshot after window use
```

**Notes**

Initial inactive screenshot aligned at23px but user evidence showed active native controls resetting to16px and missing hover glyphs. Native qualification was reopened; not treated as completion.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --manifest-path desktop/Cargo.toml --offline (AppKit bridge first)
```

**Notes**

Default ObjC feature fanout needed uncached optional packages. Restricted to the actual NSWindow/NSView/NSButton and geometry features instead.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --manifest-path desktop/Cargo.toml --offline (AppKit bridge next); pnpm desktop:bundle (next)
```

**Notes**

Compiler rejected a missing window reference and three required documented retained-superview unsafe borrows. Fixed with scoped main-thread safety comments; no unguarded pointer access.
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
cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Final event-driven AppKit geometry/hover tracking host check passes; interop confined to the retained main-thread callback.
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
pnpm desktop:bundle (first AppKit geometry/tracking correction)
```

**Notes**

Debug macOS app built with the original native-button tracking correction; /tmp/jj-desktop-glass-chrome-bundle-final.log.
<!-- /run -->
<!-- /evidence-field -->

