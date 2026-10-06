---
schema: 1
id: E-UI-013
---

# E-UI-013 details 3

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
pnpm dev:restart; pnpm dev:restart; curl -fsS http://127.0.0.1:7460/
```

**Notes**

Two consecutive cold runs rebuild the browser and development binaries, replace only the labelled owned RustFS container and recorded native server, seed distinct native repositories and return HTTP 200. Each run prints a fresh repository ID and one-time token; runtime metadata outside the checkout contains no bearer secret.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

The production browser lane exposed two stale acceptance assumptions after intentional persistence and virtualization: localStorage was required to be empty, and graph pagination expected a removed manual More history button.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/reads.spec.ts --grep native_graphs_files_and_source_rendering_are_accessible|graph_and_directory_pages_retain_exact_snapshots_and_query_context
```

**Notes**

The credential/storage assertion passed after permitting only versioned layout keys and rejecting repository IDs/tokens in their values. Graph pagination passed through automatic virtual loading; the same directory test still expected the removed manual More files control.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/reads.spec.ts --grep graph_and_directory_pages_retain_exact_snapshots_and_query_context
```

**Notes**

The exact pagination regression passes 1/1 after waiting for the virtual directory's automatic second page and retaining the original mismatch/context assertions.
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
pnpm spec:check
```

**Notes**

All functional, interaction, browser, contract and boundary lanes passed, then progress parsing rejected an invalid backslash escape in the recorded grep command. The ledger now records the literal alternation without invalid JSON escaping.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Progress validation correctly rejected Storybook story names as named test references because the ledger grammar accepts language-level test definitions. Story outcomes remain recorded by their exact focused commands instead.
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
pnpm web:test:browser
```

**Notes**

All 10 cold production browser interaction/accessibility cases pass, including safe versioned layout storage and automatically paged virtual graph and directory collections.
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
pnpm spec:check
```

**Notes**

All 18 canonical lanes pass: validation, cold catalog build/browser, catalog checks and interactions, production checks/unit/browser, generated contracts, rustfmt, boundaries, progress, full Rust workspace and mdBook/spec-first.
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
pnpm check
```

**Notes**

Specification validation, package and 142-module architecture boundaries, progress, rustfmt, generated contracts, production/catalog Svelte diagnostics and all-target Clippy pass.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The serialized aggregate passes 23 governance tests, 3 storage-harness self-tests, 6 contract tests, 20 browser units, 30 catalog units, 56 Storybook interaction/accessibility tests, one cold catalog browser contract and the full Rust workspace; declared external RustFS/OS-credential cases remain ignored in the normal lane.
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
pnpm build
```

**Notes**

Cold Storybook, production browser and complete Rust workspace builds pass. Existing upstream Design Core :global minifier and bundle-size warnings remain non-failing.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass across the complete authored tree.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue readCommitInspection beyond the first changed-path page and
  readOperationInspection beyond the first operation-change and per-group
  changed-path pages.
- Remove the HttpStoreDataSource.paths recursive=true assumption and wire
  cursor-paged directory reads with lazy tree expansion.
- Validate the released Design Core FileListing extension in lapis-community,
  then consume a separately approved registry release rather than relying on
  local source for final integration.
- Run M4.14 cold built-browser acceptance against an isolated native repository
  and pinned RustFS with external CLI publication, SSE notification and explicit
  refresh.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Registry Design Core ^0.2.0 with guarded local source validation at a9820137 and fddad817; pinned RustFS b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; @tanstack/svelte-virtual 3.17.11; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Versioned jj-store/layout/v1 localStorage keys contain only shell/sidebar, per-page Column Canvas and file-tree layout state. Storybook uses per-instance memory storage. Large catalog fixtures use 600 deterministic records with native identifier widths. The disposable development runtime stores only public process/container/repository metadata under the OS temporary directory; operator policy and creation journals contain verifiers/public token IDs, while each fresh bearer token is printed once and never written to the checkout or URL. package.json and pnpm-lock.yaml remain registry-resolved. No repository identity, selection, cursor, credential, content, native storage, publication or HTTP schema is persisted by the browser slice. Stories remain visual-pending and visual tests were skipped at the user request.
<!-- /evidence-field -->

