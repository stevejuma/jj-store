---
schema: 1
id: E-READ-040
---

# E-READ-040 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts -t 'code card' (before)
```

**Notes**

Native/local cards request log/graph instead of finite head metadata. code-summary-ui-before-040.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts -t 'code card' (after)
```

**Notes**

2pass/1.95s. Exact operation and finite50 heads; no graph call. code-summary-ui-after-040.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked (initial)
```

**Notes**

Local summary initially used native statistics type; corrected to LocalGraphStats. code-summary-check-initial-040.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --test local_browser --locked code_summary -- --nocapture (initial)
```

**Notes**

New local tests used unwrap_err on a non-Debug Response. Use err().unwrap() to preserve exact error assertions. code-summary-rust-initial-040.log.
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
cargo test -p jj-store-cli --test native_lifecycle --test local_browser --locked code_summary -- --nocapture (corrected)
```

**Notes**

One local timestamp-order fixture assumed separate Git seconds; made source timestamps explicit. code-summary-rust-corrected-040.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CODE_SUMMARY_NATIVE_CAPTURE=$PWD/.local/bounded-reads/performance/code-summary-native-capture-040.json cargo test -p jj-store-cli --test native_lifecycle --test local_browser --locked code_summary -- --nocapture (first final)
```

**Notes**

2local pass;5native fixtures reject default single-thread test runtime. Use the existing factory-required multithread runtime. code-summary-rust-final-040.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CODE_SUMMARY_NATIVE_CAPTURE=$PWD/.local/bounded-reads/performance/code-summary-native-capture-040.json cargo test -p jj-store-cli --test native_lifecycle --test local_browser --locked code_summary -- --nocapture (runtime corrected)
```

**Notes**

2local/2native pass. Synthetic commits incorrectly retained virtual-root change identity (2denials); node assertions incorrectly used JSON-node stats as object counts (1limit). Set ordinary change identity and assert exact object_reads instead. code-summary-rust-runtime-corrected-040.log.
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
JJ_STORE_CODE_SUMMARY_NATIVE_CAPTURE=$PWD/.local/bounded-reads/performance/code-summary-native-capture-040.json cargo test -p jj-store-cli --test native_lifecycle --test local_browser --locked code_summary -- --nocapture (fixture corrected)
```

**Notes**

7pass (2local/5native). Native2metadata+requested commits, no trees/ancestors, lexical windows/time ties/empty root/truncation/ancestor uncertainty, limits, historical pin/current revocation/no-publication. Local damaged indexes/missing extras remain byte-identical, current read needs at most3metadata records; old operation proof remains bounded. Native HTTP strict limits/unknown fields and zero graph cache entries. code-summary-rust-fixture-corrected-040.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/local-browser.test.ts (initial)
```

**Notes**

47pass/7.24s before the extra cancellation case. code-summary-ui-consumers-initial-040.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts (initial)
```

**Notes**

19pass/1fail initially: already-aborted summary dispatched a request. Added before/after cancellation checks to the new summary method; exact same file then20pass/6.15s. code-summary-local-ui-040.log/code-summary-local-ui-corrected-040.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/local-browser.test.ts (final)
```

**Notes**

48pass/5.69s. Both card profiles avoid history, foreign repository/epoch/operation/profile/generation rejected, cancellation preserved, multiple operation heads remain incomplete without a summary request. code-summary-ui-consumers-final-040.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
```

**Notes**

20pass/134.25s directly affected interaction/accessibility stories in a fresh test process. Full cold production-browser qualification and visual baselines deferred. code-summary-stories-040.log.
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
node scripts/contracts.mjs --write; node --input-type=module with scripts/contracts-validation.mjs responseValidator over only code-summary-native/local-capture-040.json
```

**Notes**

OpenAPI/TypeScript generated and3actual native/local summary responses validated. Inventory118/42local; complete combined route captures explicitly deferred. code-summary-generate-040.log/code-summary-responses-040.log.
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
JJ_STORE_CODE_SUMMARY_LOCAL_CAPTURE=$PWD/.local/bounded-reads/performance/code-summary-local-capture-040.json cargo test -p jj-store-cli --test local_browser --locked local_point_http_uses_ready_membership_and_only_falls_back_when_cold -- --nocapture
```

**Notes**

1pass/7.81s. Actual local cold summary preserves operation pin, finite count uncertainty and sampled metadata; no continuation, limit0/257 return413 and unknown fields400. Existing prepared membership/old source seal/fallback proof retained. code-summary-local-http-040.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked content_read -- --nocapture; cargo test -p jj-store-cli --test native_lifecycle --locked query_workers -- --nocapture; cargo test -p jj-store-cli --test local_browser --locked bounded_reads:: -- --nocapture
```

**Notes**

16pass:7content/3shared-worker/6local direct consumers. Corruption/ranges/stream deadlines/authorization, shared cancellation capacity and scoped metadata/history/file reads retain behavior. One registered large-repository smoke remains ignored by its existing environment gate; comprehensive scale verification stays deferred. code-summary-content-consumers-040.log/code-summary-worker-consumers-040.log/code-summary-local-consumers-040.log.
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
cargo test -p jj-store-http --lib --locked code_summary_contract -- --nocapture
```

**Notes**

1pass/0.59s. Finite default/no-cursor response schemas retain native64-byte/Git20-byte IDs and compact metadata. code-summary-contract-unit-040.log.
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
pnpm web:check (initial and final); pnpm catalog:check; pnpm check:architecture; pnpm check:boundaries; pnpm spec:validate; pnpm check:ui-ownership; targeted Prettier check; cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli -- --check (initial and final)
```

**Notes**

web/catalog0errors/0warnings;190catalog modules,973architecture modules,17package boundaries,5validators/23chapters/172requirements,UI ownership and formatting pass. Logs code-summary-web-check-040.log/code-summary-web-final-040.log/code-summary-catalog-check-040.log/code-summary-architecture-040.log/code-summary-boundaries-040.log/code-summary-spec-040.log/code-summary-ui-ownership-040.log/code-summary-format-check-040.log/code-summary-rust-format-040.log/code-summary-rust-format-final-040.log. Root aggregate, full contracts, full Rust/UI, real-RustFS, cold production/scale and release lanes are out of scope under the authorized performance-delivery split.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts (corrected)
```

**Notes**

20pass/6.15s exact cancellation target rerun. code-summary-local-ui-corrected-040.log.
<!-- /run -->
<!-- /evidence-field -->

