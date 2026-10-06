---
schema: 1
id: E-UI-090
---

# E-UI-090 details 2

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
pnpm install --offline --ignore-scripts --frozen-lockfile (Design Core)
```

**Notes**

Isolated source-workspace dependencies installed.
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
pnpm install --ignore-scripts --lockfile=false --config.pnpmfile=.local/rich-search-pnpmfile.cjs
```

**Notes**

Ignored temporary source-validation setup; publishable manifest and lockfile retain registry ranges.
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
pnpm install --lockfile-only --ignore-scripts
```

**Notes**

Late diff inspection found the inherited ignored .pnpmfile.cjs substituted a local Design Core checkout into this candidate lockfile. Restored the parent lock, then regenerated with --ignore-pnpmfile before commit.
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
pnpm exec vitest run --project storybook src/shared/filter/search-filter-bar/SearchFilterInput.stories.svelte (Design Core parent before)
```

**Notes**

One regression failed, one passed: custom highlight missing. /tmp/rich-search-dc-baseline.log
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
pnpm exec vitest run --project storybook src/shared/filter/search-filter-bar (Design Core)
```

**Notes**

Three files / ten interaction and accessibility tests. /tmp/rich-search-dc-family.log
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
pnpm exec vitest run --project storybook src/shared/filter/search-filter-bar/SearchFilterInput.stories.svelte (Design Core final)
```

**Notes**

Two tests; custom grammar, reactive completions, controlled draft, selection/undo preservation. /tmp/rich-search-dc-final-focused.log
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
pnpm checks (Design Core first)
```

**Notes**

Spec count/atomicity rejected the new requirement; fixed canonical bullets. /tmp/rich-search-dc-checks.log
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks (Design Core retry)
```

**Notes**

Governance, formatting, types, 111 unit files /715 tests, Turbo and static build passed. Full interaction run completed 206/208 files then stalled; only our runner stopped. /tmp/rich-search-dc-checks2.log
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
pnpm exec vitest run --project storybook src/shared/forms/cycle-picker/CyclePicker.stories.svelte src/shared/shadcn/popover/Popover.stories.svelte (Design Core)
```

**Notes**

Two stalled files rerun cold: four passed. /tmp/rich-search-dc-stalled-stories.log
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
pnpm exec vitest run --project storybook --no-file-parallelism (Design Core)
```

**Notes**

Serial retry hit AppShell timeout under concurrent host load and was stopped; separate cold AppShell rerun passed all15. /tmp/rich-search-dc-storybook-serial.log
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
pnpm exec vitest run --project storybook src/shared/shell/app-shell/AppShell.stories.svelte (Design Core)
```

**Notes**

15 passed. /tmp/rich-search-dc-shell-retry.log
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:workspace:pointer (Design Core development host attempt)
```

**Notes**

Separate dev hosts timed out before unrelated story readiness. Requalified with fresh prebuilt static assets; original logs retained under /tmp/rich-search-dc-*pointer.log and ai-browser.log.
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
pnpm test:shell:pointer (Design Core development host attempt)
```

**Notes**

Separate dev hosts timed out before unrelated story readiness. Requalified with fresh prebuilt static assets; original logs retained under /tmp/rich-search-dc-*pointer.log and ai-browser.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:shadcn:pointer (Design Core development host attempt)
```

**Notes**

Separate dev hosts timed out before unrelated story readiness. Requalified with fresh prebuilt static assets; original logs retained under /tmp/rich-search-dc-*pointer.log and ai-browser.log.
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
pnpm test:ai-chat-browser (Design Core development host attempt)
```

**Notes**

Separate dev hosts timed out before unrelated story readiness. Requalified with fresh prebuilt static assets; original logs retained under /tmp/rich-search-dc-*pointer.log and ai-browser.log.
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
python3 -m http.server 9369 --directory storybook-static --bind 127.0.0.1; pnpm test:workspace:pointer (Design Core static attempt)
```

**Notes**

Two initial tests timed out; diagnostic browser proved asset ERR_CONNECTION_RESET. Stopped only our host and runner; replaced disposable static host with Node HTTP. /tmp/rich-search-dc-workspace-static.log
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
pnpm packages:pack (Design Core)
```

**Notes**

No unpublished version selected; this release-helper no-op did not prove packing. /tmp/rich-search-dc-pack.log
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
pnpm pack --pack-destination .local/packages (Design Core)
```

**Notes**

Packed source-distributed public exports to ignored tarball. /tmp/rich-search-dc-pack2.log
<!-- /run -->
<!-- /evidence-field -->

