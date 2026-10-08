---
schema: 1
id: E-COMPUTE-003
---

# E-COMPUTE-003 details 3

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
cargo test --locked -p jj-store-http --lib agent_execution
```

**Notes**

7 passed. New picker case covers Admin-only admission, strict body/query rejection before OS interaction, advertised capability, selected/cancelled result, busy/timeout/invalid/unavailable codes; no import dispatch.
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
cargo test --locked -p jj-store-cli --lib folder_picker
```

**Notes**

4 passed: GGUF/spaces/cancellation/format rejection plus inherited bounded stdout/stderr, timeout, single-flight and cancellation reaping.
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
cargo test --locked -p jj-store-cli --test agent_execution tooling::runtime_receipts_recover_without_execution_or_automatic_install_replay -- --exact
```

**Notes**

1 passed. Runtime inventory includes stable local identity labelled jj-agent. Both companions make Ready; removing coding helper returns Bundle needed. Receipt replay/cold recovery and no automatic install still pass. macOS linker reports compact-unwind size warning.
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
cargo test --locked -p jj-store-server --lib agent_execution
```

**Notes**

26 passed / 1 ignored fixed-npm-download live installer case. All affected supervision/configuration/compute/tooling/journal tests pass; ignored installer qualification is unchanged.
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
/usr/bin/osacompile -o .local/model-picker.scpt .local/model-picker.applescript
```

**Notes**

Compiled the exact fixed model picker AppleScript extracted from the adapter without opening a dialog. Interactive OS selection remains user review, separate from fixture/process/HTTP evidence.
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
pnpm contracts:generate
```

**Notes**

Regenerated registered OpenAPI and TypeScript contract for POST /v1/local/compute/pick, ModelFileSelection and ComputeStatus.model_file_picker.
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
pnpm test:contracts
```

**Notes**

7 contract tests passed and contract TypeScript consumers compile. Registered cargo OpenAPI output was additionally deep-compared against schemas/openapi.json and matched.
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
pnpm docs:generate
```

**Notes**

Regenerated installation endpoint/type reference pages from production; command registry/configuration defaults remain unchanged.
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
pnpm dev:backend:build
```

**Notes**

Link failed with ENOSPC, alongside ring deployment-target and compact-unwind warnings. Cleared only this workspace generated target/debug/incremental cache after compiler tasks completed, freeing 4.9 GiB. No source, fixture or running binaries removed.
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
pnpm docs:review contracts "Regenerated installation API and type references for native GGUF selection and Compute picker availability."
```

**Notes**

First review chain completed application/services/configuration/cli/http reviews, then hit ENOSPC writing the contracts acknowledgement. After cache cleanup reran all affected acknowledgements successfully.
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
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check
```

**Notes**

First documentation check correctly rejected the contracts fingerprint left unacknowledged by ENOSPC. Re-reviewed updated generated API pages.
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
pnpm dev:backend:build
```

**Notes**

Rebuild passed in 21.84s after generated cache cleanup; compact-unwind linker warning remains diagnostic, not distribution qualification.
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
pnpm dev:backend:restart
```

**Notes**

Backend command initially omitted the trusted ACP environment selection: service/compute returned 503. Repeated with explicit bundled desktop/resources/jj-acp and target/debug/jj paths plus preserved fixture/browser/backend settings. Vite PID 9640 remained active; backend and its owned embedded-storage child replaced. No fixture reset, installed executable or unrelated daemon change.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

Final web/catalog: 0 errors / 0 warnings. Catalog boundaries 287 modules; architecture 1327; ownership validator passes with extracted summary provenance. No limits, diagnostics, baselines or Community dependency rules weakened.
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
pnpm docs:review
```

**Notes**

Affected application/services/configuration/cli/http/contracts fingerprints re-reviewed against authored guide and generated endpoint/type pages.
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
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check
```

**Notes**

Final strict docs build: 9 Mermaid diagrams in both themes offline; 141 files / 17,741,491 bytes; all links/anchors valid.
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
pnpm spec:first
```

**Notes**

Expected before evidence completion: skeleton has no fresh changed-file coverage or actual runs. Filled this evidence using enclosing change oyrznskr.
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
cargo fmt --all -- --check
```

**Notes**

All root Rust sources format cleanly; embedded runtime source unchanged.
<!-- /run -->
<!-- /evidence-field -->

