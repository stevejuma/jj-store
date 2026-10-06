---
schema: 1
id: E-M13-028
---

# E-M13-028 details 2

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
jj --no-pager new nrwwnrqz nmqnqtuz -m "Merge repository creation and conversion into main"; resolve additive chapter conflicts and preserve parent ledger entries
```

**Notes**

Only canonical prose/ledger conflicts; no production-code resolution. All 420 main and 416 conversion parent evidence entries retained exactly, with 423 unique inherited entries. Source M13.5/M14/M14.2 rows retained; M13.5 remains In progress.
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
pnpm exec tsx --test web/tests/native-repository-manager.test.ts web/tests/repository-conversion-manager.test.ts
```

**Notes**

manager-tests: 8/8 intent/unknown/reload/retry/receipt/blocked-preflight units pass in main.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web-check/catalog-check: zero errors/warnings; 187 catalog modules pass. Merged capture/desktop consumers retain independent parent qualification.
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
pnpm exec vitest run --config .local/repository-conversion-main/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryConversion.stories.svelte
```

**Notes**

conversion-stories: 4/4 interaction/accessibility cases pass in fresh main runner; visual-pending retained.
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
cargo test -p jj-store-server --test native_creation --test repository_conversion --locked
```

**Notes**

server-tests: 5 creation and 6 conversion/recovery tests pass in main. Isolated real-RustFS test remains ignored in this focused lane; original explicit E-M13-027 run is distinct and unchanged.
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
pnpm check:progress; pnpm spec:validate; pnpm check:boundaries; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

initial progress/spec/boundaries/ownership/architecture pass: 137 tasks, 22 chapters/168 requirements, 17 packages, 187 catalog and 947 production modules.
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
conversion_merge_base=$(jj --no-pager log -r "heads(::nrwwnrqz & ::nmqnqtuz)" --no-graph -T commit_id); conversion_merge_head=$(jj --no-pager log -r @ --no-graph -T commit_id); pnpm exec spec-validator first --base "$conversion_merge_base" --head "$conversion_merge_head"
```

**Notes**

spec-first: explicit common-ancestor/merge comparison passes 216 protected files and 15 mapped chapters; default single-parent gate is not substituted for merge qualification.
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
node --input-type=module (Playwright read-only existing review discovery with x-jj-store-protocol:1)
```

**Notes**

review-before: existing main Vite service admits both native discovery and local bindings with the ordinary public protocol header. Missing-header probes return access_denied as expected. Saved installation/config hashes and repository identity retained for restart comparison.
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
cargo test -p jj-store-service --test conversion --locked; cargo test -p jj-store-jj --test conversion --test local_evidence --locked
```

**Notes**

service-tests/jj-tests: 5 graph/history, 2 code conversion and 6 retained-evidence tests pass against main capture/WAL consumers.
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
pnpm contracts:generate
```

**Notes**

contracts-generate: combined owning OpenAPI/TS artifacts are identical to the automatic merge; no hand-edited generated contract.
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
pnpm web:build; node scripts/browser-fixtures.mjs
```

**Notes**

web-build/browser-fixtures: current main production bundle builds and actual native HTTP/SSE/mutation fixtures regenerate. Fresh prebuilt host avoids Cargo-lock delays during unchanged browser startup deadline.
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
pnpm dev:backend:build; cargo build -p jj-store-http --example browser_preview --locked
```

**Notes**

backend-build/browser-host-build: main CLI/server binaries and fresh cold test host compile. Existing Darwin linker compact-unwind size warning remains a warning; no lint suppression or binary installation.
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
pnpm exec prettier --check spec/src/cli.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md .local/repository-conversion-main/ledger-header.md; cargo fmt --all -- --check
```

**Notes**

format-check/rust-format-check: conflict-resolution chapters/header and all merged Rust formatting pass; historical ledger JSON bytes are retained.
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
pnpm exec playwright test --config .local/repository-conversion-main/playwright.config.ts web/tests/browser/native-repository-creation.spec.ts web/tests/browser/repository-conversion.spec.ts
```

**Notes**

browser-tests: 6/6 cold main browser cases pass with freshly built main host and production output. Exact pending/reload/retry, unsupported capability/preflight, rejection correction, report download and dismissal lifetime retain the original deadlines.
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
JJ_STORE_TRANSCRIPT_CACHE="$conversion_main_rules" pnpm contracts:check
```

**Notes**

contracts-check: every registered combined server/CLI fixture passes; 240 actual HTTP/SSE responses conform. Seven contract-tool tests and direct generated-client TypeScript check pass. A fresh invocation-owned /tmp cache retains only real compiled Kingfisher rules, shared by the registered fixture processes; no sanitizer bypass, deadline extension or source bodies.
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
python3 (compare every parent evidence object in merged ledger)
```

**Notes**

All 420 main entries and 416 source entries remain semantically exact. Every main raw object is byte-identical; all three source-only records retain their source bytes. Seventeen shared parent records already differed only in formatting; main historical bytes retained.
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
pnpm check:progress; pnpm spec:validate (integration evidence update)
```

**Notes**

progress-final1/spec-final1: 137-task bookkeeping and 22 chapters/168 requirements pass after complete parent preservation and new merge evidence.
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

User-authorized main-workspace integration and persistent dev restart. Public route/schema infrastructure now combines main capture-profile changes with native creation/conversion, so every registered HTTP/SSE fixture and generated client must pass. Exact feature Rust/UI tests and fresh main browser output verify the merged consumer. Parent real-RustFS round-trip/cold production evidence is unchanged; no manual production-code conflict resolution, so repeating isolated real-storage qualification and unrelated capture/desktop/full-workspace/root aggregate lanes is out of scope. No visual baselines, package release, existing repository conversion or fixture reset.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the qualified merge in main, then restart its existing LocalTrusted HMR backend/UI and verify installation, repository, bindings and capture configuration are retained.
<!-- /evidence-field -->

