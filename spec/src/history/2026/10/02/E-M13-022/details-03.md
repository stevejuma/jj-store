---
schema: 1
id: E-M13-022
---

# E-M13-022 details 3

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_FILESYSTEM_PORT=17478 node scripts/local-browser-acceptance.mjs
```

**Notes**

Fixture hardcodes workspace target/debug/jj in agent seed, not CARGO_TARGET_DIR. Copied owned binaries into ignored workspace target/debug; no change to shared seed infrastructure.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_review local_review_browser_pages_and_detail_pin_committed_metadata -- --exact
```

**Notes**

Intermediate compile: shared ReviewReader lacks lookup_thread; switched to owned snapshot lookup. No weakened catalog checks.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_review
```

**Notes**

Intermediate regression fixture dropped second TempDir before use. Retained fixture owner; final six tests pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-store-http -p jj-store-cli
```

**Notes**

HTTP has no direct WAL dependency. Inferred owned catalog key from service interface, keeping existing crate boundary.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-store-service -p jj-store-http -p jj-store-cli
```

**Notes**

All affected packages check with no diagnostics.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Executable inventory generates new local review schemas and removes unused local sync capability.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

143 actual HTTP/SSE responses conform, seven contract tests and TypeScript contract consumers pass. Includes native captures and new local review responses.
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
JJ_STORE_FILESYSTEM_PORT=17478 node scripts/local-browser-acceptance.mjs
```

**Notes**

Fresh filesystem service/built UI: three unchanged browsing, Projects unknown outcome, folder recovery cases pass. New review case reveals premature status lookup while write pending; request not_known before publication. Added pending/lookup disabled state, retaining draft and request.
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
JJ_STORE_FILESYSTEM_PORT=17478 node .scratch/local-review-cold.mjs
```

**Notes**

Exact disposable cold review fixture passes creation/recovery/parity/reply but blank-editor assertion counted Mira placeholder text. Assert explicit empty-content placeholder, retaining actual saved-reply checks.
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
JJ_STORE_FILESYSTEM_PORT=17478 node .scratch/local-review-cold.mjs
```

**Notes**

Exact original review test cold with minimal real filesystem fixture, omitting unrelated expensive agent seed. Keyboard dialog+axe, lost-response draft, original receipt recovery with no blind retry, CLI detail equality, committed historical pin, unchanged code heads, replies and reload pass. Earlier three unchanged full-fixture cases retained; no claim of a new four-case full-suite pass.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Protected mapping required compatibility, mutation and governance chapters in addition to local/API/CLI/UI; owning semantics added.
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
pnpm spec:validate; pnpm check:progress
```

**Notes**

Five validators,21 chapters,146 requirements;120-task ledger bookkeeping pass before final annotation.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm web:check
```

**Notes**

Final owning gates:43 protected files/eight chapters, five validators/21 chapters/146 requirements,120-task ledger, zero Svelte errors/warnings.
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
cargo fmt --all; pnpm exec prettier --write changed UI/test/canonical files
```

**Notes**

Targeted format passes, no dependency manifest/lock changes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository review reads/creation/reply/edit/workflow transport/CLI stage only; shared native reconstruction and existing views retained. Complete contract lane widened because four public routes/generated schemas affect consumers. Native/local provider and direct UI/Storybook consumers qualified; root aggregates/unrelated RustFS/full Storybook out of scope. Portable code/agent anchors, additional moderation UI, local code editing/search/conversion and M13 full completion remain separate gates. Default data and live7460 retained. Visual approval is not part of this stage.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Repository review stage qualified; commit this slice, then implement portable review anchors/native carrier, physical Save, local search, explicit Snapshot/recoverable commands and complete collaboration conversion; local sync is removed from active scope.
<!-- /evidence-field -->

