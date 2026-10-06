---
schema: 1
id: E-READ-094
---

# E-READ-094 details 2

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
jj --no-pager st/log/diff; jj new qntsvuuv in codex-preview; jj workspace update-stale
```

**Notes**

Inspected main, temporary and preview states; main remained clean. Preview advanced to implementation; stale descendants refreshed after evidence/source updates. Historical ledger entries preserved.
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
cargo check --locked -p jj-store-jj -p jj-store-service -p jj-store-http
```

**Notes**

.local/blame-rust-check-final.log passes; earlier missing ObjectId import, unawaited async has_id and test type ambiguity corrected before verified checks.
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
cargo test --locked -p jj-store-jj --lib query::blame -- --nocapture
```

**Notes**

.local/blame-shared-verified.log:4 pass, original synchronous comparison hard-deadline probe remains ignored historical evidence. Covers both object widths, full-JJ parity, merges, repeated lines, empty/missing final newline, partial windows, retry idempotency, failed final authorization, cache-hit denial, cancellation and expiry/capacity.
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
cargo test --locked -p jj-store-cli --test native_lifecycle path_metadata_crosses_merges_and_retains_the_ancestry_bound -- --nocapture; cargo test --locked -p jj-store-cli --test local_browser progressive_blame_preserves_local_source_and_rejects_stale_generation -- --nocapture
```

**Notes**

blame-native-verified.log and blame-local-verified.log:1 each passes. Native origins Base/Left/Right match merge semantics, cache reuse and foreign operation denial. Local generation/revocation denial and source witness unchanged. Repeated after admission adjustment in blame-native-admission.log/blame-local-admission.log.
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
JJ_STORE_BROWSING_CONTRACT_CAPTURE=$PWD/.local/blame-native-contracts.json cargo test --locked -p jj-store-cli --test native_lifecycle http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured -- --nocapture; JJ_STORE_MANAGED_LOCAL_CAPTURE=$PWD/.local/blame-local-contracts.json cargo test --locked -p jj-store-cli --test local_browser managed_local_http_activates_without_subscribers_resumes_and_captures_tasks -- --nocapture
```

**Notes**

blame-native-http-verified.log and blame-local-http.log:1 each passes, including cold preparation, complete origin response, cached revisit and explicit reset; native unauthenticated cache hit denied. Sixteen registered captures validated using scripts/contracts-validation.mjs against generated schemas.
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
cargo test --locked -p jj-store-cli --test native_lifecycle http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured -- --nocapture
```

**Notes**

First compile of preparation failure guard hit Rust private method path_start_allowed. Changed visibility to its parent module; final registered native/local runs pass.
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
pnpm contracts:generate; pnpm test:contracts
```

**Notes**

blame-contracts.log/blame-contracts-final.log:OpenAPI and TypeScript regenerated. blame-contract-tests.log:7 tests and contract tsc pass. Complete unrelated registered capture suite excluded by focused policy.
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
JJ_STORE_BLAME_LINUX_METADATA=/Users/stevejuma/code/linux/.jj/repo cargo test --locked -p jj-store-jj --lib local_browser::code_index::blame_probe::linux_production_blame_progress_preserves_source -- --ignored --exact --nocapture
```

**Notes**

blame-linux-production.log:one-commit prototype processed only14–64 commits per response; changed to real64-commit windows. blame-linux-windows.log final64-window probe:12 successful responses, source witness unchanged at operation c29523a6 and commit062871f. COPYING3.679/3.971/4.262s,512/1216/1728 cumulative commits; altera4.049/4.646/5.816s,576/1024/1408; inode3.725/5.590/6.601s,384/640/704; CREDITS4.698/4.313/4.075s,448/576/1088. All remained incomplete with0 proven origins over these first3 responses. Other development checks were running; full completion and isolated warm latency not claimed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/http-blame.test.ts web/tests/vite/blame-query.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

blame-ui-verified.log:23 pass. Completed-cache reuse, preparation cancellation, terminal failures, same-selection reset, identity mismatch, concurrent observers, route restoration and late-publication rejection.
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
pnpm exec tsx --test web/tests/store-blame.test.ts
```

**Notes**

blame-model-verified.log:4 pass, including byte/line admission explanations and routes/groups.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreBlame.stories.svelte
```

**Notes**

blame-stories-cold.log:7 pass in fresh Vite/Chromium with blocking runtime/a11y checks. Final size-explanation wiring repeated in blame-stories-last.log. Visual baselines unchanged.
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
pnpm web:check
```

**Notes**

Initial2 generated-number union diagnostics corrected with explicit Number conversion; blame-web-verified.log/blame-web-final.log zero errors/warnings. Final scoped source and size guards checked in blame-web-last.log.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

blame-architecture-verified.log:1155 modules; boundaries17 packages; ownership1 validator. Initial310/301-line source cap failures resolved by owning native Blame factory extraction, without cap/allowlist changes.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Initial missing mapped CLI/compatibility/mutations and subsequently documentation chapters. Updated all owning chapters; blame-spec-first-verified.log39 protected files/9 chapters passes.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

blame-spec-first-verified.log and blame-spec-validate.log pass,5 validators/23 chapters/173 requirements. Final ledger governance recorded in subsequent final logs.
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
pnpm docs:generate; pnpm docs:review local/services/http/application/contracts with topic-specific explanations; pnpm docs:check
```

**Notes**

blame-docs-check.log:139-file documentation closure,links/anchors and9 Mermaid diagrams light/dark. Source fingerprints reviewed again after final UI/admission changes.
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
rustfmt --edition 2024 <changed Rust files>; pnpm exec prettier --write <changed UI/spec/docs files>
```

**Notes**

Targeted formatting only; historical ledger entries retained byte-identically. Final formatting/ledger validation recorded after entry append.
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
pnpm dev:backend:build; JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store pnpm dev:backend:restart
```

**Notes**

blame-preview-restart.log:fixture preserved,only7472 backend replaced;Vite PID302 unchanged. Corrected ignored runtime ownership markers to the observed preview runner before invoking the owned restart. Installed CLI and unrelated main/Stacks services untouched.
<!-- /run -->
<!-- /evidence-field -->

