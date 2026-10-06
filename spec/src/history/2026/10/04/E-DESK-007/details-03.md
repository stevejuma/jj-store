---
schema: 1
id: E-DESK-007
---

# E-DESK-007 details 3

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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked --test terminal_sessions --test terminal_targets
```

**Notes**

3 session and 2 target regressions. /tmp/jj-terminal-regressions.log and regressions-final.log.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked --test terminal_sessions
```

**Notes**

3/4 pass; newly added background-job regression detects separate-process-group cleanup gap. /tmp/jj-terminal-jobs.log. Repaired by respecting libproc child PID count; exact regression and final suite below pass.
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
cargo fmt --manifest-path desktop/Cargo.toml --all; pnpm exec prettier --write <changed UI sources and provenance>
```

**Notes**

Pinned Rust formatting and focused UI formatting. /tmp/jj-terminal-format*.log.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked --test terminal_sessions closing_terminal_ends_background_job_in_separate_process_group
```

**Notes**

jobs-fixed.log: exact separate-process-group cleanup regression passes after interpreting proc_listchildpids return as a PID count, with byte-sized input buffer. PTY reads use nonblocking poll so noisy or idle readers cancel promptly.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target cargo test --manifest-path desktop/Cargo.toml --workspace --no-default-features --locked
```

**Notes**

rust-final.log:109 passed,0 failed,2 ignored. Ignored tests are staged managed-runtime qualification and release-mode idle-host latency; neither is described as a pass. This full directly affected desktop/engine lane is required by shared IPC/process lifecycle, not a root Rust workspace run.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked --test terminal_targets
```

**Notes**

targets-last.log:3/3 pass, including newly added native multiple-ready-links/current-revision test after the full109-test run. Local registrations and native links are separately exercised; stale/foreign/malformed targets are refused.
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
cargo clippy --manifest-path desktop/Cargo.toml --workspace --all-targets --locked -- -D warnings (initial)
```

**Notes**

clippy.log:two collapsible-if diagnostics in host shutdown. Collapsed existing nested cleanup condition; no lint suppressions.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target cargo clippy --manifest-path desktop/Cargo.toml --workspace --all-targets --locked -- -D warnings (final)
```

**Notes**

clippy-final.log and native-lint-last.log:strict native desktop/engine all-target checks pass. Only the native.rs module description changed afterward; terminal logic unchanged.
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
pnpm spec:validate (initial)
```

**Notes**

spec.log:unsupported verification status Partial; changed requirement status to the supported vocabulary, with qualification details in this ledger.
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
pnpm dlx @tauri-apps/cli@2.11.4 build --debug --bundles app (initial); cargo build --manifest-path desktop/Cargo.toml --locked (diagnostic attempt)
```

**Notes**

Native packager rejected JS API2.12.1 versus Rust Tauri2.11.5; diagnostic-only generate_context invocation then produced E0283. Removed temporary diagnostic; aligned JS API to2.11.1 with manifest~2.11.1. Bundle/build logs retain these failures.
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
pnpm add -D @tauri-apps/api@2.11.1; final manifest range~2.11.1; pnpm install --ignore-pnpmfile --lockfile-only --frozen-lockfile
```

**Notes**

api-align.log and lock-registry.log pass. Final tracked lock preserves original registry Design Core0.2.0 and adds only Tauri API2.11.1; removed hook-generated absolute/file substitutions and pnpmfile checksum from the publishable lock. Existing local-package hook and node_modules are unchanged; all UI acceptance here uses that documented local Design Core setup.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-terminal/Terminal.stories.svelte (collapse regression)
```

**Notes**

collapse-user.log:one failed/three passed because Enter collapses the preceding content pane, not the terminal. Corrected to ArrowDown keyboard resize/collapse. Native cold opening also exposed generic onCollapse hiding a fresh panel; it now hides only after an actual user drag-end/keyup reports a collapsed pane. No primitive override.
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
pnpm catalog:check (header refinement initial)
```

**Notes**

header-catalog.log:one test typing error from unsupported getByRole exact option; removed that invalid option.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/terminal.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-terminal/Terminal.stories.svelte
```

**Notes**

units-last.log4/4 and header-stories.log4/4; header icon/expanded state, hide focus, folder selection/missing-link settings, retained drafts/navigation, resize/maximize, Cmd+J, IME, bracketed paste and selection/copy boundaries. Normal interaction and axe checks; no visual baselines changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-terminal/Terminal.stories.svelte
```

**Notes**

header-consumers.log:22/22 across4 direct story families after header integration. Main-body extraction and shared header placement justify these direct shell consumers; no full catalog claim.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

header-check.log and catalog-qualified.log:zero errors/warnings,185 catalog modules; header-architecture.log940 modules; header-ownership.log passes. Community header/icon source inspected before Design Core public primitives; source attribution and ownership retained.
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
pnpm web:build; node scripts/desktop.mjs stage --server-dir /Users/stevejuma/code/lapismd/jj-store/target/debug --rustfs-runtime /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --web-root /Users/stevejuma/code/lapismd/jj-store-desktop-terminal/web/dist
```

**Notes**

header-build.log/header-stage.log and close-build.log/close-stage.log:production builds and212 staged assets pass. Existing CSS/global and large-chunk build warnings remain; no Vite development CSP used for native acceptance.
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
TAURI_CONFIG=acceptance-config CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target pnpm dlx @tauri-apps/cli@2.11.4 build --debug --bundles app --config acceptance-config; same environment tauri bundle --debug --bundles app --config acceptance-config after frontend-only updates
```

**Notes**

Native build and header-bundle.log/close-bundle.log pass. acceptance-config is identifier com.lapismd.jj-store.terminal-acceptance and productName jj-store-terminal-acceptance. Unique debug app reuses explicit ignored development binaries and private disposable state; no ordinary app, installed tools or existing deployment replaced.
<!-- /run -->
<!-- /evidence-field -->

