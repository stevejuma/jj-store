---
schema: 1
id: E-M9-035
---

# E-M9-035 details 3

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
pnpm test:storybook
```

**Notes**

One full Storybook rerun lost its Chromium connection before executing stories. No product assertion failed; the direct clean rerun is recorded next.
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
pnpm test:storybook
```

**Notes**

The clean full Chromium rerun passes all 84 Storybook interaction and automatic accessibility tests. Changed stories remain visual-pending.
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
pnpm catalog:test:browser
```

**Notes**

The cold static Storybook browser lane passes with the primary Diff assertion scoped independently from inline agent tool diffs.
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
pnpm check
```

**Notes**

Specification validation, boundaries, architecture, progress, formatting, embedded RustFS, generated contracts, Svelte checks, catalog boundaries and workspace Clippy all pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

The complete governance, storage, embedded and managed RustFS, LocalTrusted, contract, 46-test web, 52-test catalog, 84-story Storybook, cold catalog-browser and locked Rust workspace suites pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Storybook, the production browser bundle, pinned embedded RustFS and the complete locked Rust workspace build successfully. The build retains the existing Svelte global-selector and large-bundle warnings.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The first owned-demo reset found runtime.json naming server PID 25365 after that process and its RustFS child had already exited. The reset stopped before removing or replacing repository state.
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
rm .local/dev-demo/runtime.json && DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

After verifying both recorded owned PIDs were absent and port 7460 was free, the stale ignored runtime record was removed. A cold reset seeded repository fcc2d52fc6bf485bb81cc7d976ad6c04, reached current search state and left the LocalTrusted service running.
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
production-browser inspection on http://127.0.0.1:7460
```

**Notes**

The cold live application shows avatar/name/state cards for Codex, Cursor and OpenCode with captured starts and 38s, 46s and 39s durations; Community-style transcript tools and copy controls; seven review threads spanning agent, path, operation, repository and revision anchors plus open/resolved histories; the bottom composer; inferred revision/path Review columns; close-route cleanup; and Files author avatar, message, revision and relative time metadata.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The final mapped spec-first audit found the broad CLI, fixture, service, WAL, script and catalog slice still needed narrow canonical notes in cli, compatibility, mutations, publication, spec-governance and ui. Each mapped chapter now records its owned behavior and limits.
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
pnpm spec:first
```

**Notes**

The mapped spec-first gate passes for 59 protected files across nine canonical chapters after the owned collaboration contracts and limits were recorded.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Specification validation and the cold catalog-browser lane passed before the known captured production-browser fixture stopped the aggregate gate. All 20 legacy scenarios still wait for removed Repository ID or Repository token controls; the current authenticated application uses one Access token and discovery, and LocalTrusted bootstrap also needs its current repository-catalog response. This pre-existing browser-fixture migration is outside the collaboration implementation and remains explicit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The live demo capture has no exact code association publication path, so
  revision selection can open its anchored Review discussion but cannot infer an
  Agent context or inline changed file from that seeded session; the catalog
  proves both behaviors with an explicit valid association.
- The pre-existing captured browser fixtures still target removed Repository ID
  and Repository token controls instead of current manual-token discovery and
  LocalTrusted catalog bootstrap, so pnpm spec:check stops with 20
  browser-interaction failures after the cold catalog lane passes.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; Node.js 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core ^0.2.0; Community ^0.1.0; Codex fixture 0.156.1; Cursor fixture 3.22.7; OpenCode fixture 1.18.30.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, AgentStore or ReviewStore encoding changes. HTTP/OpenAPI version-1 presentation DTOs add bounded resolved AgentActivity, nullable first/last captured activity timestamps, exact AgentCodeAssociation values on detail/relation reads and validated anchors on review summaries. Installed hooks populate existing observation timestamps from the local system clock; missing, unmatched and beyond-bound evidence omits derived timing. Log and Branches hashes may retain stable agent session and review thread IDs. Browser context traversal is capped at 4,096 inspected rows and rejects cursor cycles; visual baselines remain unapproved.
<!-- /evidence-field -->

