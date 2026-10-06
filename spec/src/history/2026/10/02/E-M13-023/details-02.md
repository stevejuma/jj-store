---
schema: 1
id: E-M13-023
---

# E-M13-023 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_working_copy
```

**Notes**

Before implementation: missing working_copy facade; one compile diagnostic. /tmp/local-completion-save-before.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --offline -p jj-store-service --test local_working_copy
```

**Notes**

Initial three tests pass. Existing locked rustix1.1.5 reused, adding only the service dependency entry to Cargo.lock. /tmp/local-completion-save-after.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_working_copy (two final expansions)
```

**Notes**

Final four tests cover stale bytes/inodes/workspace/generation/repository, no-follow paths including case variants, binary/limits, hard links, concurrent browser saves, executable mode and temporary cleanup. /tmp/local-completion-save-after{2,3}.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-store-http
```

**Notes**

Affected boundary and dependencies compile. /tmp/local-completion-save-http-check.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate; pnpm contracts:check
```

**Notes**

Two public routes expand inventory to91 paths/24 local/67 unchanged native. Full contract lane widened for shared generated consumers:145 actual HTTP/SSE responses, seven contract tests and TS consumers pass. /tmp/local-completion-save-{generate,contracts}.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

One cold filesystem-only HTTP/CLI test passes; reads unsnapshotted file, saves, rejects stale replay/traversal, proves unchanged operations/bookmarks/companion. Captures actual read/Save responses. /tmp/local-completion-save-cold-http.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (initial generation race)
```

**Notes**

Type check began before concurrent contract generation finished and saw absent new schemas. No fallback types; after generation passes zero errors/warnings. /tmp/local-completion-save-web-check.log.
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
pnpm web:check (final reruns); pnpm catalog:check
```

**Notes**

Web and direct catalog types pass with zero errors/warnings;129 catalog module boundaries. /tmp/local-completion-save-web-check{2,3,4,5,6}.log; catalog{,2}.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts
```

**Notes**

13 local transport cases pass, including exact physical selection, one dispatch after lost response, explicit readback and no displayed code refresh. /tmp/local-completion-save-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Two native story interaction cases pass but one Bits UI/Svelte derived_inert unhandled rejection occurs during teardown. /tmp/local-completion-save-native-stories.log.
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
Exact same StoreFileEditor Storybook target with both changed file panel components restored temporarily to @- in try/finally
```

**Notes**

Same two assertions pass and same inherited Bits UI dismissible-layer focus callback diagnostic reproduces on parent components. Current files restored. /tmp/local-completion-save-native-parent-stories.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Exact same Storybook target with explicit root navigation cleanup
```

**Notes**

Diagnostic persists; added root-preview assertion also fails because renamed NOTES is not a landing document. Entire attempted story edit restored; no weaker assertion or suppression retained. /tmp/local-completion-save-native-stories2.log.
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
pnpm web:build; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo build --locked -p jj-store-cli --bin jj-store (fresh rebuilds)
```

**Notes**

Isolated production bundle and explicit development server rebuilt; installed CLI and live7460 untouched. /tmp/local-completion-save-web-build{,2,3}.log; server-build{,2}.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_FILESYSTEM_PORT=17478 node .scratch/local-save-cold.mjs (three cold exact runs)
```

**Notes**

Original tracked physical-file browser test passes from cold disposable filesystem server/built UI without RustFS/native RPC. Keyboard Working copy, physical unsnapshotted bytes, no rename field, axe, lost-response draft/readback, stale external edit, Cancel/recorded Source, unchanged operations/bookmarks/companion. Final run after lifecycle helper/stat extraction. /tmp/local-completion-save-browser-cold{,2,3}.log.
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
pnpm check:ui-ownership (initial)
```

**Notes**

New original jj-store presentation was placed in the Community-derived directory. Moved it to owned store-working-copy; no invented Community provenance. /tmp/local-completion-save-ownership.log.
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
pnpm check:ui-ownership (final)
```

**Notes**

Original local components and retained maintained imports pass ownership. /tmp/local-completion-save-ownership{2,3}.log.
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
pnpm check:architecture (initial/extraction iterations)
```

**Notes**

File panel exceeded300 counted lines at362/321/303. Extracted working-file lifecycle and existing stat/editability helpers; no threshold change. /tmp/local-completion-save-architecture{2,3,4}.log.
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
pnpm check:architecture (final)
```

**Notes**

729 production modules pass architectural boundaries. /tmp/local-completion-save-architecture5.log.
<!-- /run -->
<!-- /evidence-field -->

