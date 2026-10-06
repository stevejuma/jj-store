---
schema: 1
id: E-DESK-007
---

# E-DESK-007 details 2

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
pnpm install --frozen-lockfile; pnpm add -D @tauri-apps/api@^2
```

**Notes**

Workspace dependencies installed; existing local-package hook substituted sibling Design Core (local validation only). Initial unquoted package range was rejected by zsh; quoted retry succeeded. /tmp/jj-terminal-install.log and api-install.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked --test terminal_sessions
```

**Notes**

Before: terminal module absent. /tmp/jj-terminal-before.log.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --test terminal_sessions
```

**Notes**

Initial retained-session target 1/1. Shared ignored desktop target directory only. /tmp/jj-terminal-runtime.log.
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
cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Fresh workspace had no staged resources directory; created ignored directory before host check. /tmp/jj-terminal-native-check.log initial run.
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
cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Native Tauri bridge compiles. Initial unused transport accessor later removed. /tmp/jj-terminal-native-check.log.
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
cargo test --manifest-path desktop/Cargo.toml --workspace --no-default-features --locked
```

**Notes**

Copied Buzz-only reserved-environment source include absent; replaced cross-app assertion with jj-store secret canaries while retaining real PTY environment tests.
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
cargo test --manifest-path desktop/Cargo.toml --workspace --no-default-features --locked
```

**Notes**

Initial full directly affected native host/engine suites pass. /tmp/jj-terminal-rust-tests.log. Superseded by final lifecycle suite below.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial integration: InputEvent narrowing, public resize handle API and settings section corrected; two initial warnings then one accessibility warning repaired. /tmp/jj-terminal-web-check.log.
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
pnpm web:check
```

**Notes**

0 errors/warnings before later extraction. /tmp/jj-terminal-web-final.log.
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
pnpm catalog:check
```

**Notes**

0 errors/warnings; catalog boundary passes. /tmp/jj-terminal-catalog-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/terminal.test.ts
```

**Notes**

4 controller and keyboard regressions. /tmp/jj-terminal-units.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-terminal/Terminal.stories.svelte
```

**Notes**

2/3 pass; axe rejected close button within tablist. Moved active close action outside tablist without suppressions. /tmp/jj-terminal-stories.log. Interim color extraction also required restoring renderer BOLD constant.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-terminal/Terminal.stories.svelte
```

**Notes**

3/3 interactions and accessibility. /tmp/jj-terminal-stories-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-terminal/Terminal.stories.svelte
```

**Notes**

21/21 direct shell consumers including terminal, with normal axe rules. Wider lane justified by shared main-body composition extraction. /tmp/jj-terminal-consumers.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Renderer and shell source budgets exceeded. Extracted color handling and production main-body composition rather than relaxing limits.
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
pnpm check:architecture
```

**Notes**

936 modules. /tmp/jj-terminal-architecture-final.log.
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
pnpm check:ui-ownership
```

**Notes**

Pinned Buzz provenance, owned Svelte components and public foundational APIs; local Design Core substitution prevents registry-only qualification. /tmp/jj-terminal-ownership.log.
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

Production asset build passes. /tmp/jj-terminal-web-build.log.
<!-- /run -->
<!-- /evidence-field -->

