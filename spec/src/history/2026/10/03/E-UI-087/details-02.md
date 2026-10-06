---
schema: 1
id: E-UI-087
---

# E-UI-087 details 2

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
jj --no-pager st; inspect Community navigation, Design Core tracked guidance/owning DC-SHA-050 chapter/history42a125503632; jj rebase -r @ -d4922feea
```

**Notes**

Both source workspaces clean; reuse quoystmy with current main parent so InputGroup and menu repairs remain available. No main source modifications.
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
pnpm spec:search -- tooltip (Design Core)
```

**Notes**

Documented command forwards unsupported -- to installed spec-validator; retry without separator.
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
pnpm spec:search tooltip (Design Core)
```

**Notes**

QMD native modules unavailable; use printed rg fallback and open owning overlays chapter.
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
pnpm test:storybook src/shared/shadcn/tooltip/Tooltip.stories.svelte (source before)
```

**Notes**

Exact restored-default regression fails11px vs12px;1existing story passes. /tmp/dc-tooltip-restore-before.log.
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
pnpm test:storybook src/shared/shadcn/tooltip/Tooltip.stories.svelte (source after)
```

**Notes**

3/3 default28px height, restored padding, hover and retained custom sizing. /tmp/dc-tooltip-restore-after.log.
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
pnpm test:storybook src/shared/shadcn/tooltip/Tooltip.stories.svelte src/shared/shadcn/tooltip/Tooltip.variations.stories.svelte
```

**Notes**

4/4 across2tooltip files, including upstream preview. /tmp/dc-tooltip-restore-story-final.log.
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
pnpm exec prettier --write (foundation source/story/owning chapters); pnpm spec:check; pnpm checks (initial)
```

**Notes**

Canonical new default/token acceptance bullet contains2sentences. Split into atomic bullets; aggregate stops at governance before qualification. Logs format/spec-check/checks.
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
pnpm spec:check (after atomic bullet fix)
```

**Notes**

Validation, mdBook and spec-first3lanes pass. /tmp/dc-tooltip-restore-spec-check-final.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks; pnpm check:types (after atomic bullet fix)
```

**Notes**

New inline consumer source example is parsed as another script by svelte-check, yielding2duplicate Tooltip identifiers. Moved example to colocated Tooltip.sizing.example-sources.ts; production tooltip unchanged. Logs checks-final/types.
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
pnpm pack --pack-destination /tmp/dc-tooltip-restore-pack
```

**Notes**

Source package tarball created without publication. /tmp/dc-tooltip-restore-pack.log.
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
npm_config_pnpmfile=/tmp/jj-appearance-pnpm.cjs pnpm install --no-frozen-lockfile --lockfile=false; cmp foundation/consumer tooltip-content.svelte
```

**Notes**

Explicit documented local source refresh; installed production tooltip bytes match. Consumer registry manifest/lockfile remain unchanged. /tmp/jj-tooltip-restore-install.log.
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
node inline ownership-checked Vite-only restart
```

**Notes**

Verified active owner/workspace and exact PID before stopping68050; start33733 with same runtime root, preserve runtime fields and backendPID42374. No backend restart or fixture reset.
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
pnpm web:check; pnpm catalog:check (consumer)
```

**Notes**

Both0errors0warnings;153catalog modules. /tmp/jj-tooltip-restore-{web,catalog}.log.
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
pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

50/50 across7direct navigation/shell/Projects/repository consumers with a fresh browser after source refresh. /tmp/jj-tooltip-restore-consumers.log.
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
Read-only CUA live cold temporary thread tab, keyboard-focus Projects tooltip, computed style and screenshot
```

**Notes**

Buzz tooltip now30px tall with12px text/18px inherited line and6px/12px padding; existing rotated arrow14.14px. Saved full viewport screenshot and closed temporary tab33. User tab unchanged; no data/preference writes.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (consumer initial)
```

**Notes**

Appearance provenance requires its mapped architecture chapter; added the explicit foundation-source validation boundary. /tmp/jj-tooltip-restore-spec-first.log.
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
pnpm visual-delta test --help (source)
```

**Notes**

No test selection mode specified; canonical Docker dependency bootstrap completed but argument validation refused comparison. Source edited during that bootstrap, so artifact guard also reported the changed story. No baseline update.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm visual-delta test --story-id shadcn-overlays-tooltip--open-tooltip --baseline-path-mode nested-import --external 'vendor/shadcn-svelte-docs/static/**'
```

**Notes**

Canonical container Storybook build passed in5m;1selected story/524total. Capture blocked before comparison by EACCES on /visual-delta/canonical-build-cache/entries temporary tree. Artifact guard also noted the story example edit during the build. No screenshot comparison or baseline approval claimed. /tmp/dc-tooltip-restore-visual.log.
<!-- /run -->
<!-- /evidence-field -->

