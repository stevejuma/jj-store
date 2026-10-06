---
schema: 1
id: E-DESK-002
---

# E-DESK-002 details 2

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
CUA bundled app AX/screenshot before; inspect Buzz tauri.conf.json at ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be
```

**Notes**

One visible extra native titlebar/title. Buzz uses Overlay, hiddenTitle and trafficLightPosition16,25; ours retained defaults.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t MacDesktopHeader (symlinked dependency tree)
```

**Notes**

Suite failed to import external addon-vitest setup before running assertions. Replaced only this workspace node_modules symlink with an independent installation; parent dependencies untouched.
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
pnpm install --offline --frozen-lockfile --ignore-scripts
```

**Notes**

Existing ignored local pnpm hook checksum mismatched the canonical lock.
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
pnpm install --offline --no-frozen-lockfile --ignore-scripts; jj restore --from @- pnpm-lock.yaml
```

**Notes**

Independent development tree installed with existing ignored Design Core source hook. Canonical lock restored exactly; no dependency or loader change committed. Not registry-only qualification.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t MacDesktopHeader (unchanged @- shell and application)
```

**Notes**

Exact before regression:1 failed,12 skipped, missing drag-region attribute. Original component bytes temporarily loaded from JJ and edited bytes restored in finally.
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
cargo fmt --manifest-path desktop/Cargo.toml; cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Native Tauri host and two-command dynamic ACL compile. /tmp/jj-store-desktop-chrome-native-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'MacDesktopHeader|Default|Settings' (initial)
```

**Notes**

9 assertion tests passed,5 skipped;1 Bits UI derived_inert teardown diagnostic. Retained this unsuccessful attempt rather than suppressing the diagnostic guard.
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
pnpm web:check
```

**Notes**

0 errors and0 warnings. /tmp/jj-store-desktop-chrome-web-check.log.
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
pnpm catalog:check
```

**Notes**

Final0 errors/0 warnings;130 modules. /tmp/jj-store-desktop-chrome-catalog-check.log.
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
pnpm check:architecture (before module extraction)
```

**Notes**

Story443/400 counted lines; component309/300. Extracted desktop story family and already-global header hover rules without raising budgets.
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
pnpm check:architecture (partial extraction)
```

**Notes**

Component303/300; completed hover extraction rather than compressing source or raising budget.
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
pnpm check:architecture (final)
```

**Notes**

733 modules satisfy the existing architecture budgets.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation/provenance remains valid. Community inspected first, Design Core shell second. Copied hover declarations preserve selectors and their original license/source attribution.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte -t 'MacDesktopHeader|^Default$|^Settings$'
```

**Notes**

Final4 passed/10 skipped, no runtime diagnostics. Earlier intermediate extraction also returned0 but emitted a ResizeObserver console error; retained as incomplete intermediate evidence. Final scoped native CSS and exact40-pixel header check pass. /tmp/jj-store-desktop-chrome-story-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t '^SettingsMobile$' (unchanged @- shell/application)
```

**Notes**

1 passed/11 skipped; isolated before state did not reproduce earlier teardown diagnostic. Edited source restored.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'MacDesktopHeader|Default|Settings' (unchanged @- shell/application)
```

**Notes**

7 passed/5 skipped; broader before state also did not reproduce diagnostic. No claim of a confirmed inherited failure; edited source restored.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte -t 'MacDesktopHeader|Default|Settings' (final)
```

**Notes**

9 passed/5 skipped, no runtime diagnostics. Covers native/narrow header, ordinary browser spacing/clicks and direct settings consumers. /tmp/jj-store-desktop-chrome-story-settings-final.log. Earlier diagnostic not reproduced on final relevant lane.
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
pnpm web:build
```

**Notes**

Fresh production bundle succeeds; inherited Design Core :global minifier and large-chunk warnings retained. /tmp/jj-store-desktop-chrome-web-build.log.
<!-- /run -->
<!-- /evidence-field -->

