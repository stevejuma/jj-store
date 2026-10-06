---
schema: 1
id: E-M21-062
---

# E-M21-062 details 2

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
jj --no-pager st/log/diff; source inspection of Changeyard, jjbutler and GitButler; read-only native sample inspection
```

**Notes**

Implementation in enclosing sktnmmmv above lmpkknlw/fe3cb82c. Changeyard 4b90e180, jjbutler a9763eeb and GitButler 9a1aa402 inform composition only; native JJ parents remain authority. Evidence is under .local/stack-discovery-fix/. No sample graph commands, configuration edits or unrelated files included.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-segments.test.ts
```

**Notes**

segments-before.log: missing drawing edge into the excluded target erased dependent cards; 3 passed/1 failed. Initial exact after:4 passed in segments-after.log. Final file has7 passing cases including merge/page boundaries and empty integrated history.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

native-full.log:31 passed. Initial inventory expectation omitted the already integrated main bookmark (29 passed/1 failed in native-discovery.log); expectation now includes the complete inventory. Stable native target/merge/WIP, conflicts, existing mutations and reconciliation remain covered.
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
cargo test -p jj-store-jj --test local_stacks discovery:: --locked
```

**Notes**

native-discovery-verified.log:4 passed/28 filtered, adding the4096-candidate limit after the31-test full binary.32 unique native cases qualified overall; read-only pre-setup/historical operation, aliases/shared bases, actual fork behind advanced target, upper-only membership/lower insertion, exclusive grouped unapply, native merges and45-deep boundaries covered.
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
cargo test -p jj-store-http --lib local_browser::stacks::tests --locked
```

**Notes**

http.log:3 admission and registered-schema tests pass. Guarded read-only topology POST is versioned, operation/target/binding pinned and subject to existing local admission.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/stack-discovery-fix/http-contracts.json cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

http-contract-capture.log:existing refresh request returned403 before topology query, matching the previously recorded intermittent fixture failure. No admission relaxation.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/stack-discovery-fix/http-contracts.json cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

http-contract-recheck.log:exact rerun passed;56 real responses, including topology, validate in captured-contracts.log. The fixture retains exact-operation reads and validates actual HTTP response contracts.
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
pnpm contracts:generate; responseValidator over .local/stack-discovery-fix/http-contracts.json; pnpm test:contracts
```

**Notes**

contracts-generate.log/captured-contracts.log/contract-tests.log:OpenAPI and TypeScript generated,56 responses valid,7 contract tests plus contract tsc pass. Initial generation found the missing await after segment became async; caller repaired without changing native policy.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit <focused Stacks controller, adapter and production application files>
```

**Notes**

core-unit.log:27 passed; controller.log:137 passed/1 failed from a legacy graphless mock boundary; controller-final.log:158 passed/2 failed because the production mock lacked the new topology route. Fixed compatibility fallback and realistic route fixture, retaining missing-edge shapes. application-recheck.log:7 passed. controller-verified.log:19 files/161 passed. No lifecycle, draft or command assertion weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-segments.test.ts web/tests/vite/stack-workspace-topology.test.ts web/tests/vite/stack-workspace-stack-history.test.ts web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

final-regressions.log:4 files/36 passed after final native-order refinement. This rechecks range ordering, graph edge exclusions, beyond-page boundaries, raw owner retention and lazy adapter behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
```

**Notes**

stories-verified.log:cold Vite/Chromium6 files/23 interactions pass with blocking axe. Earlier topology1 pass/1 failure used incorrect combobox name; use the actual Bookmarks label. Earlier affected18 pass/4 failures lacked the new production route; exact production4 passed after fixture correction. Shared case2 pass/1 axe failure exposed duplicate form names; stack-qualified composer/region labels fix accessibility while preserving raw state ownership. Existing lower files, commit insertion, drag/drop, inline controls and retained selections remain covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte
```

**Notes**

topology-cold-final.log:fresh3-story interaction/accessibility pass. Confirms dependent search, two cards with native commit ownership, aliases, partial application and shared lower-card grouped unapply.
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
cargo check -p jj-store-jj -p jj-store-http --locked
```

**Notes**

rust-final.log:affected Rust packages pass. Existing unused-assignment/linker warnings remain nonfatal. No storage or manifest publication changes.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web-verified.log/catalog-verified.log:0 errors/warnings and248 catalog modules. Initial missing generated types and bigint/testing-library option diagnostics resolved before these final checks.
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
pnpm check:ui-ownership; pnpm check:boundaries; pnpm check:architecture
```

**Notes**

ownership.log/boundaries.log/architecture-verified.log:ownership passes,17 package boundaries,1156 modules. Intermediate local-session310/302 and lane301 line-cap failures corrected by extracting topology-query cache and removing redundant Boolean wrapping, without cap or allowlist changes.
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
pnpm docs:generate; pnpm docs:review local/http/application/contracts <topic-specific explanation>; pnpm docs:check
```

**Notes**

docs-generate.log/docs-review.log/docs-final.log:generated references refreshed;139-file closure, links/anchors and9 Mermaid diagrams in light/dark pass. Final application fingerprint refreshed again in docs-review-complete.log after commit ordering correction.
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
pnpm exec prettier --write <changed non-ledger files>; cargo fmt --all
```

**Notes**

format*.log:targeted non-ledger TS/Svelte/Markdown/JSON formatting applied; Rust formatted. Historical evidence objects remain untouched. Final formatting checks are recorded separately below.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

spec-first-verified.log/spec-validate.log:46 protected files/7 canonical chapters and5 validators/23 chapters/173 requirements pass before this ledger append. No milestone status promotion.
<!-- /run -->
<!-- /evidence-field -->

