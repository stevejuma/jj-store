---
schema: 1
id: E-READ-004
---

# E-READ-004 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/community-projects/RepositoryDocuments.stories.svelte
```

**Notes**

4 tests across 3 affected stories passed from a fresh Vite process, including interaction/accessibility. Visual baselines remain pending.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Initial extraction needed in four controller/transport/loader modules; first extraction left controller 2 counted lines over budget. Extracted read scope as a complete version/cancellation helper.
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
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

928 production modules pass architecture; UI ownership passes. No copied Community component source introduced.
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
pnpm web:build; node scripts/browser-fixtures.mjs; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/reads.spec.ts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

First cold run: 12 passed/2 failed. Found duplicate simultaneous directory continuation requests after files-first publication; added single-flight/status guard while retaining failed cursor. New test incorrectly selected Source as a tab; corrected to actual source-preview region. Final cold rerun pending below.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress
```

**Notes**

Initial bookkeeping missed compatibility/mutation chapters and exact names on two test references. Added mapped requirements and exact regression titles.
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
pnpm web:build; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/reads.spec.ts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

Fresh final built UI and preview process: all 14 browser tests pass, including delayed/failed metadata, pagination mismatch without repeat fetching, file navigation, cancellation and accessibility.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

Full generated response lane failed in unchanged capture_management api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free at its 120-second publication deadline; process finished at 200.16s. Process sample shows cold Kingfisher/Hyperscan sanitizer compilation in the retained OpenCode worker during runtime shutdown. Existing original-workspace binary baseline run is still pending; no timeout or sanitizer rules changed.
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
pnpm test:contracts
```

**Notes**

7 schema/strictness tests plus generated contract TypeScript check pass.
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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

Expanded real local HTTP fixture passes, including commit_summary, row-only paths and scoped file_at.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured -- --exact
```

**Notes**

Expanded actual native HTTP fixture passes, including row-only attribution mode.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/files-local-capture.json target/debug/deps/local_browser-893d8c4057fcb950 --exact filesystem_runtime_registers_and_reads_without_native_storage; JJ_STORE_BROWSING_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/files-native-capture.json target/debug/deps/native_lifecycle-e6652c34f7ed2d93 --exact http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured; node --input-type=module (responseValidator on both captures)
```

**Notes**

Both capture tests pass; all 37 actual affected native/local HTTP/SSE responses conform to generated schemas. This focused pass does not replace the failed full capture lane.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; cargo fmt --all -- --check; pnpm exec prettier --check (changed TS/Svelte/canonical chapters, excluding inherited whole-ledger formatting)
```

**Notes**

Mapped spec-first gate, 136-task evidence ledger, 167 requirements and focused format checks pass. Historical ledger bytes were preserved.
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
jj --no-pager diff --check
```

**Notes**

Installed Jujutsu does not support this option; no diff check ran. Focused Rust and Prettier formatting checks above are the actual formatting evidence.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Full contract lane remains unqualified due to the cold OpenCode sanitizer
  timeout; original-workspace baseline reproduction is running independently. No
  deadline weakened, milestone promoted or release claimed.
- Startup workspace/reference drain, historical membership/index preparation,
  cache quotas, collaboration projections/cards, remaining endpoint audit fixes
  and large-scale/native/real-RustFS qualification remain later slices.
- Existing Design Core registry release qualification and visual baseline
  approval remain pending. Full unrelated aggregate lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement independent workspace paging and exact reference resolution; retain the contract/capture qualification gap and complete the remaining large-repository plan.
<!-- /evidence-field -->

