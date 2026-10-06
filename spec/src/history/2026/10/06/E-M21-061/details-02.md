---
schema: 1
id: E-M21-061
---

# E-M21-061 details 2

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
jj --no-pager st/log/diff; rg and source inspection; read-only jj --ignore-working-copy log/op log and stacks.toml inspection
```

**Notes**

Work began in lmpkknlw above empty 5103af31. A separately authorized Blame integration rebased this same change onto tmotwykn/f3fc8f46 while preserving Stacks edits. No unrelated work included. Evidence under .local/stack-topology-fix/. Final comparison proves all606 prior evidence entries byte-identical. Installed JJ does not support diff --check (usage error); targeted Prettier/rustfmt checks provide formatting verification instead.
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
cargo test --locked -p jj-store-jj --test local_stacks topology::membership_retains_target_merge_identity_and_only_one_live_merge -- --exact
```

**Notes**

native-before.log: target omitted once branches applied. Exact after passed in native-after.log.
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
cargo test --locked -p jj-store-jj --test local_stacks
```

**Notes**

native-all.log:24 passed; inactive-workspace test assumed one merge parent and now explicitly selects the topic edge; concurrent test returned Busy, exact rerun passed (native-reconcile.log). No product admission or assertion contract weakened.
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
cargo test --locked -p jj-store-jj --test local_stacks -- --test-threads=1
```

**Notes**

native-final.log:28 passed. Covers target composition, stable merge/WIP identity, conflicts/dirty work, aliases, 45-deep dependencies, integrated and empty destinations, merges, grouped unapply guards, reconciliation, protected legacy heads and original testy three-column/native-jj-log fixture. Cleanup fixture initially counted hidden IDs through an explicit revset and retained Git refs; corrected to inspect heads(all()) and synchronize legitimate fixture refs (cleanup-debug.log, cleanup.log).
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
cargo test --locked -p jj-store-cli --test local_browser stacks::local_stack_history_is_bounded_pinned_and_read_only -- --exact
```

**Notes**

managed-stack-before.log:ready managed index rejected StackHistory with IndexRebuildRequired. Live browser reproduced the same 409 despite JJ Log being ready.
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
cargo test --locked -p jj-store-cli --test local_browser stacks::local_stack_history_is_bounded_pinned_and_read_only -- --exact
```

**Notes**

managed-stack-after.log:bounded pages match explicit preparation using managed code index; exact operation, source bytes and continuation protections retained.
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
cargo test --locked -p jj-store-cli --test local_browser managed_ -- --test-threads=1
```

**Notes**

managed-affected.log:14 passed/1 failed/1 ignored. Existing source, HTTP, binding/revocation and lifecycle cases pass. New test queried retained disk inventory during an active preparation and received Busy. Replaced that inappropriate inventory probe with bounded polling of actual stack-history readiness, without an explicit start call. Optional external Linux large-repository test remains ignored.
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
cargo test --locked -p jj-store-cli --test local_browser managed_lifecycle:: -- --test-threads=1
```

**Notes**

managed-lifecycle-final.log:5 passed, including automatic preparation from first StackHistory read, revocation, coalescing and teardown. managed-start-final.log also records the exact corrected regression pass. Together with the unchanged managed-code/HTTP passes this closes the 15 applicable managed cases.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/stack-topology-fix/http-contracts.json cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

http-final.log:registered HTTP apply/single-unapply/reapply/grouped-unapply and topology pass,55 real response captures. Initial http.log failed with a403 whose body was discarded; strengthened fixture diagnostics and repeated exact fixture successfully without relaxing admission. Final repeat is recorded separately in http-verified.log.
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
pnpm contracts:generate; responseValidator over captured HTTP responses; pnpm test:contracts
```

**Notes**

contracts.log/captured-contracts.log/contract-tests.log:generated OpenAPI/TypeScript,55 captured responses valid,7 contract tests plus contract tsc pass. contracts-drift.log checks repeat generation for byte-identical output. Full unrelated registered capture suite not run.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit <focused Stacks/navigation tests>
```

**Notes**

ui-focused.log:46 passed/3 failed during implementation; route allowlist and empty-alias expectations corrected. ui-regressions.log:34 passed; history-follow.log:6 passed; ui-affected.log:180 passed before final refinements.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace* web/tests/vite/production-stacks* web/tests/vite/production-stack-adapter.test.ts web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/managed-code-index.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

ui-final-pass.log:21 files/193 tests. Includes accepted mutation to both histories, lazy graph reads, live/pinned refresh/tab/reload/BackForward, branch drafts/diffs/selections, safe ownership loss and transient read retention. Intermediate loading guard broke explicit-refresh supersession (ui-complete.log:158 passed/1 failed); removed it, preserving controller epoch semantics, exact setup6/6 in refresh-regression.log. Mutation test initially intercepted create rather than setup; corrected fixture,7/7 in mutation-history-final.log. An unmatched shell glob started no tests and was replaced by actual discovered paths.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte
```

**Notes**

stories-verified.log:cold Vite/Chromium,5 files/19 interactions pass with blocking runtime/accessibility checks. Earlier topology-story final Cancel lookup was removed because no Cancel exists in that preview; functional grouped-unapply assertions retained. Intermediate card refactor left an undefined branchSelected (stories-final.log/web-final-2.log); repaired. Branch click then switched the browsing column to raw lower owner (stories-complete.log and dependent-story-final.log); repaired by retaining the displayed column for Branches actions while preserving raw member owners in Workspace. Exact production4/4 in dependent-story-verified.log; final full affected family19/19. No screenshots/baselines changed.
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
cargo check --locked -p jj-store-jj -p jj-store-http -p jj-store-service
```

**Notes**

rust-check.log:affected packages pass. Existing native-test unused-assignment and macOS large-unwind linker warnings recorded, no test failure attributed to them.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web-last.log/catalog-last.log:0 errors/warnings;248 catalog modules. Earlier two card diagnostics corrected before final checks.
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

architecture-final-pass.log:1154 modules; ownership.log:1 validator; boundaries.log:17 packages. Intermediate line caps failed in controller/card; related lifecycle wiring and drag/card-selection helpers extracted, preserving ownership. No cap/allowlist changes.
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
pnpm docs:generate; pnpm docs:review local/services/http/application/contracts <topic-specific review>; pnpm docs:check
```

**Notes**

docs-generate.log/docs-verified.log:139-file closure,links/anchors and9 Mermaid diagrams light/dark. Initial docs.log detected stale generated API reference, regenerated. Source review fingerprints updated, including final card/lifecycle changes; prettier normalized coverage.json after review output.
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
pnpm exec prettier --write/check <changed non-ledger files>; rustfmt --edition 2024 --check <changed Rust files>
```

**Notes**

format-verified.log/rust-format-verified.log:targeted formatting passes. Preserve historical ledger entries byte-for-byte; new evidence JSON formatted independently. Intermediate formatter reported only review-generated coverage.json before normalization.
<!-- /run -->
<!-- /evidence-field -->

