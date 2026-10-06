---
schema: 1
id: E-M14-006
---

# E-M14-006 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

Before: six existing cases passed; two new picker regressions failed at the missing action. .scratch/folder-picker-before.log
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
cargo test --locked -p jj-store-cli --lib folder_picker::tests
```

**Notes**

First compilation failed because a test moved a picker while its pinned future still borrowed it; fixed the fixture to share an Arc.
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
cargo test --locked -p jj-store-cli --lib folder_picker::tests
```

**Notes**

Three pass: exact path bytes/cancellation/missing helper/invalid output; single-flight/timeout/request cancellation with kill and reap; bounded stdout/stderr before process exit. .scratch/folder-picker-rust.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-workspaces.test.ts web/tests/vite/http-workspace-links.test.ts
```

**Notes**

Intermediate nine tests passed before the added transport POST check.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-workspaces.test.ts web/tests/vite/http-workspace-links.test.ts web/tests/vite/production-store-settings.test.ts
```

**Notes**

Final 16 pass; draft-only selection, cancel/failure preservation, capability gating, single flight, late-response exclusion, protected empty POST and existing settings consumers. .scratch/folder-picker-unit.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts -- --exact
```

**Notes**

One passes: local admin only, no GET invocation, selection/cancellation/error replies, bearer/read/foreign-origin/unknown-body rejection, no workspace verification or repository publication from picking. Final handler samples also checked by contracts:check. .scratch/folder-picker-http.log
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
pnpm contracts:generate
```

**Notes**

Regenerated OpenAPI and TypeScript from registered Rust handlers.
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
pnpm contracts:check
```

**Notes**

Generated artifacts match; all handler capture fixtures and six schema/strictness tests plus TypeScript contracts pass. .scratch/folder-picker-contract-check.log
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Both type checks pass with zero errors/warnings; catalog boundaries accept 100 modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

Fresh Vitest/Vite process: six interaction/accessibility stories pass, including selection, cancellation, failure/manual fallback and unsupported connection. .scratch/folder-picker-stories.log
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
cargo clippy --locked -p jj-store-service -p jj-store-http -p jj-store-server --lib -p jj-store-cli --lib --test native_lifecycle -- -D warnings
```

**Notes**

Invocation rejected duplicate --lib; no checks ran.
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
cargo clippy --locked -p jj-store-service -p jj-store-http -p jj-store-server -p jj-store-cli --lib --test native_lifecycle -- -D warnings
```

**Notes**

Affected libraries and native lifecycle test compile/lint with no warnings. .scratch/folder-picker-clippy.log
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
pnpm web:build
```

**Notes**

Fresh production build passes; existing bundle-size warning retained. .scratch/folder-picker-build.log
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

One cold browser test passes against a fresh preview process: selection only fills draft, cancellation retains it, save/revision recovery and route retention work, mobile overflow and axe pass. HTTP picker is stubbed in this lane. .scratch/folder-picker-browser.log
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
pnpm check:architecture; pnpm check:boundaries
```

**Notes**

422 web modules and 15 package boundaries accepted.
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
cargo fmt -p jj-store-service -p jj-store-http -p jj-store-server -p jj-store-cli -- --check
```

**Notes**

One final HTTP error tuple needed rustfmt after status adjustment; formatted again.
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
cargo fmt -p jj-store-service -p jj-store-http -p jj-store-server -p jj-store-cli; pnpm exec prettier --write <changed TS/Svelte/CSS/Markdown files>
```

**Notes**

Formatted changed files; generated contracts are formatted by their generator.
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
pnpm dev:restart
```

**Notes**

Main-owned demo rebuilt and restarted at http://127.0.0.1:7460/; repository 1bdb89c409c1b891ac7f180c01a265d5 and persisted data reused, Search ready, server PID 97083. .scratch/folder-picker-dev-restart.log
<!-- /run -->
<!-- /evidence-field -->

