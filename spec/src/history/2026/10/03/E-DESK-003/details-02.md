---
schema: 1
id: E-DESK-003
---

# E-DESK-003 details 2

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
jj rebase -s ykmzsnwm -o suvkzynx; resolve foundation/chrome conflicts with jj new and jj squash --use-destination-message
```

**Notes**

Desktop foundation and chrome now descend from main67040334. Preserved main navigation, automatic refresh, history language and Appearance. All298 main evidence entries compare equal, and both desktop entries remain. No conflicted ancestors.
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
cp -cR target .local/desktop-integration-target (main workspace)
```

**Notes**

APFS-cloned isolated Cargo target preserves the active main development executable and service; no unrelated service restart.
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
pnpm exec prettier --write resolved spec chapters, ledger and CommunityAppShell.svelte/header-hover.css (initial)
```

**Notes**

Conflict resolution temporarily omitted the closing style tag; formatter caught1 syntax failure. Restored tag, formatted and verified before further qualification.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial)
```

**Notes**

Each command reported3 errors: missing direct autocomplete/highlight dependencies and stale local Design Core custom-language props. Refreshed the ignored dependency tree using the existing source hook.
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
pnpm web:build (initial)
```

**Notes**

1 unresolved @codemirror/autocomplete import from the stale dependency tree; fixed by dependency refresh.
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
pnpm test:storybook DesktopChrome.stories.svelte StoreApplication.stories.svelte (initial selection)
```

**Notes**

2 native stories pass,1 application suite fails to import after Vite dependency optimizer reload. Refreshed dependencies and used fresh runners for final qualification.
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
pnpm install --offline --no-frozen-lockfile; jj restore --from @- -- pnpm-lock.yaml
```

**Notes**

Installed main direct editor dependencies, current Bits UI and current Design Core through the existing ignored hook. Restored the tracked registry lockfile; no dependency or loader configuration change committed.
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
pnpm check:architecture (before prop extraction)
```

**Notes**

1 failure: combined StoreApplication has322 counted lines against existing320 limit. Extracted its typed composition props without raising the budget.
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
pnpm test:governance
```

**Notes**

44/44 pass. Full relevant governance lane required because desktop extends protected-file and ledger scanners; /tmp/jj-desktop-integration-governance.log.
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
pnpm desktop:check
```

**Notes**

Locked native Tauri host check passes; /tmp/jj-desktop-integration-native-check.log.
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
pnpm desktop:test
```

**Notes**

5 regular host/owned-process tests pass;1 real-runtime test ignored by default, explicitly run separately; /tmp/jj-desktop-integration-host-tests.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/.local/desktop-integration-target cargo test --locked -p jj-store-cli --test desktop_supervision --test server_executable
```

**Notes**

Supervision1pass/1ignored; foreground executable14pass/3ignored. Full regular foreground target covers the extracted shared startup/readiness/signal boundary; /tmp/jj-desktop-integration-cli-tests.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/.local/desktop-integration-target JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test --locked -p jj-store-cli --test desktop_supervision -- --ignored --exact desktop_owner_eof_stops_real_service_and_retains_installation
```

**Notes**

1/1 real RustFS EOF shutdown/restart and retained format2 configuration proof; /tmp/jj-desktop-integration-real-supervision.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/.local/desktop-integration-target cargo check --locked -p jj-store-cli; cargo build --locked -p jj-store-cli --bins (same isolated target)
```

**Notes**

Focused CLI check and paired development executable build pass; /tmp/jj-desktop-integration-cli-{check,build}.log. Main development service remains running from its original binary.
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
pnpm test:storybook DesktopChrome, StoreApplication, StoreWorkspaceSettings, StoreSettingsIdentifier (fresh runner)
```

**Notes**

22/22 interactions pass; /tmp/jj-desktop-integration-stories-final.log.
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
pnpm test:storybook desktop/application/settings, ProjectsHub and StoreAppearance direct consumers
```

**Notes**

36pass/1fail across6 families. UnsupportedReviewsEmptyState fixture assumed local profile implied absent reviews, which main no longer does; /tmp/jj-desktop-integration-direct-consumers.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsHub.stories.svelte -t UnsupportedReviewsEmptyState (unchanged main)
```

**Notes**

Reproduced the exact1failure against main67040334. Fixture now explicitly omits its optional reader for the unsupported scenario; production behavior and assertions unchanged; /tmp/jj-desktop-integration-main-baseline.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsHub.stories.svelte -t UnsupportedReviewsEmptyState (repaired)
```

**Notes**

Exact original regression1pass/10skipped; /tmp/jj-desktop-integration-fixture-regression.log.
<!-- /run -->
<!-- /evidence-field -->

