---
schema: 1
id: E-EXEC-022
---

# E-EXEC-022 details 2

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
cargo test --locked -p jj-store-server --lib agent_execution
```

**Notes**

16 passed, 1 real-install test ignored in this lane. Tooling, presence, journal, delivery and feedback regressions. Linker compact-unwind warning retained.
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
cargo test --locked -p jj-store-http --lib agent_execution
```

**Notes**

5 passed. Local Admin admission, typed fixed inputs, unavailable receipts and existing execution/discussion routing.
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
cargo test --locked -p jj-store-cli --test agent_execution
```

**Notes**

3 passed, 1 authenticated real-agent test ignored. Setup CAS independence, session lifecycle and cold runtime receipt recovery.
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
cargo test --locked -p jj-store-server --lib agent_execution::tooling::install::tests::real_private_codex_install_verifies_pinned_versions_and_cold_selection -- --ignored --exact
```

**Notes**

1 passed. Actual fixed npm download in a disposable private directory, bounded executable/version probes and reopened journal selection. An intermediate failure from macOS /var versus /private/var containment was corrected by canonical staging-root comparison and rerun successfully. No tooling installed into the user fixture.
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
cargo check --locked -p jj-store-service -p jj-store-http -p jj-store-server
```

**Notes**

Affected package checks completed; no full Rust workspace run.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 21
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service -p jj-store-http -p jj-store-server --lib -- -D warnings
```

**Notes**

Strict diagnostic lane blocked by 21 unchanged jj-store-jj warnings. This is not a strict lint pass.
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
cargo clippy --no-deps --locked -p jj-store-service -p jj-store-http -p jj-store-server --lib -- -D warnings
```

**Notes**

Strict diagnostic lane blocked by 2 unchanged jj-store-service warnings (sliced string bytes and type complexity). No lint suppression added.
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
cargo clippy --no-deps --locked -p jj-store-server --lib
```

**Notes**

Three introduced warnings were corrected. Remaining 2 unchanged warnings in agent_execution/feedback.rs and publication_worker.rs recorded as diagnostics; warnings are not strict qualification.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-runtimes.test.ts web/tests/vite/agent-execution*.test.ts
```

**Notes**

12 tests across 2 files, including no install from Ready and original unknown-receipt reservation.
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
pnpm test:storybook catalog/components/store-application/AgentRuntimes.stories.svelte
```

**Notes**

6 interaction/accessibility stories in fresh Vitest/Storybook processes. Before: guide was display:block with a 24-pixel icon; default AlertDialog controls lacked shared button styling. Shared semantic anchor and Dialog+Button composition fix them. An intermediate test had 5 failures because a flex-item anchor correctly computes inline-flex to flex; the final geometric regression checks row layout, centered 12–20-pixel icons and text alignment, preserving the intended contract. Removed unsupported Testing Library exact option. Keyboard Enter opens confirmation; cancel dispatches zero installs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
```

**Notes**

23 stories across 2 directly affected capture/settings files after capture controls were unmounted from Runtimes.
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
node scripts/agent-runtimes-browser.mjs http://127.0.0.1:7460/
```

**Notes**

Fresh Chromium context against the current preserved development fixture. Codex and Cursor Ready, green success chips, no Codex Install, inline Update guide, runtime-only tab, light/dark/narrow axe, no horizontal overflow, zero execution/installation POSTs and zero page errors. Confirmation is accurately reported as not offered for these Ready runtimes; keyboard installation/cancel is covered by the production-component story.
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
pnpm contracts:check
```

**Notes**

Generated contracts match; 300 actual HTTP/SSE responses conform; 7 contract tests and contract TypeScript checks pass.
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
pnpm web:check
```

**Notes**

Zero errors/warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; 278 catalog modules. Initial new-story exact option type error corrected.
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
pnpm check:ui-ownership
```

**Notes**

Existing maintained Community avatars/chips reused; no external source imports or new vendored copies.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries pass.
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
pnpm check:architecture
```

**Notes**

1279 modules. Final UI exceeded the 300-line component budget by 1 counted line; shared confirmation actionLabel removed duplicate expressions and the check passed without changing its threshold.
<!-- /run -->
<!-- /evidence-field -->

