---
schema: 1
id: E-UI-016
---

# E-UI-016

<!-- evidence-field:id markdown -->
## Id

E-UI-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.14
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

acceptance
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- README.md
- docs/browser.md
- package.json
- scripts/real-browser-acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/playwright.real.config.ts
- web/tests/real/native-repository.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/native-repository.spec.ts#external publication waits for
  explicit refresh and preserves selection
<!-- /evidence-field -->

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
pnpm exec playwright test --config web/playwright.real.config.ts
```

**Notes**

The first real-stack run connected and opened the SSE stream, then the test expected selecting bookmark main alone to select its commit identity. It now selects the visible main target before recording the stable route. No product change was required.
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
pnpm exec playwright test --config web/playwright.real.config.ts
```

**Notes**

The external CLI publication produced the newer-history control, but the test inspected the route while asynchronous refresh was still restoring it. Completion now waits for the full at_op identity to change rather than an already-visible bookmark.
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
pnpm web:test:real
```

**Notes**

The public task cold-builds and starts the built browser, native server, seeded repository and checkout-owned pinned RustFS; the one real-browser case opens authenticated SSE, selects main, publishes from a separate native CLI process, proves no automatic snapshot change, explicitly refreshes, preserves the commit/change selection, observes the new commit and cleans up the disposable stack.
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
pnpm check:architecture
```

**Notes**

All 148 governed web, catalog and script modules remain within their production, story and test budgets with no fixture or dependency cycles.
<!-- /run -->

<!-- run -->
### Run 5

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

Spec-first required the new external real-browser harness to be recorded in the mapped API, architecture and governance chapters. Those chapters now define the lane, ownership and credential boundary.
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
pnpm format:check
```

**Notes**

Rustfmt passed; Prettier found four authored files plus Playwright's unignored result marker. The authored files were formatted, the disposable real-lane output is now ignored and the generated marker was removed.
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
pnpm spec:first; pnpm check:progress; pnpm web:check; pnpm check:architecture; pnpm format:check
```

**Notes**

Seven protected files map to four canonical chapters; progress bookkeeping passes, production Svelte diagnostics report no errors or warnings, all 148 architecture modules stay within budget, and rustfmt plus Prettier pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including the 148-module architecture guard, 23 browser units, 32 catalog units, 57 Storybook interaction/accessibility cases, cold catalog and built-browser acceptance, the locked Rust workspace and mdBook. Visual baselines remained skipped.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification validation, package boundaries, architecture, progress, rustfmt, 51 HTTP/SSE response contracts, six contract tests, production and catalog diagnostics and locked-workspace clippy all pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The normal aggregate passes 23 governance tests, three storage-harness self-tests, six contract tests, 23 browser units, 32 catalog units, 57 Storybook interaction/accessibility cases, cold catalog acceptance and the complete locked Rust workspace. Declared external-service cases remain ignored in this normal lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The cold Storybook catalog, production browser bundle and complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain warnings rather than failures.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The final bookkeeping check rejected M4.15's stale dependency links to unfinished umbrella tasks and a separately owned remaining-gap entry. The independently accepted search slice now depends on completed M4.9 shared production ownership and M6.1's committed Tantivy reader; broader mutation, restore and performance work remains on its owning milestones.
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
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Final protected-file mapping, milestone bookkeeping, rustfmt and Prettier checks pass after recording the aggregate outcomes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Consume the Design Core lazy listing extension from a separately approved
  registry release and repeat the cold real-browser lane against that
  registry-resolved install.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

RustFS rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; Playwright 1.63.x; built Svelte 5.57.1 browser; target/debug/jj and target/debug/jj-store from the enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The opt-in test passes its URL, repository ID, one-time bearer token and OS-temporary runtime path only through child-process environment variables. The secret is not written to the checkout, route, browser storage, Playwright artifacts or evidence ledger. Runtime metadata remains secret-free and dev:stop removes only the labelled checkout-owned RustFS container and recorded server. No API, storage, native identity or persistent browser format changes. Visual tests remain skipped at the user request.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


