---
schema: 1
id: E-M8A-006
---

# E-M8A-006

<!-- evidence-field:id markdown -->
## Id

E-M8A-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a.1
- M8a.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

persistent-local-development-fixture
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- README.md
- crates/jj-store-server/src/policy.rs
- crates/jj-store-server/tests/configuration.rs
- docs/browser.md
- package.json
- scripts/dev-demo-repository.mjs
- scripts/dev-demo-services.mjs
- scripts/dev-demo-state.mjs
- scripts/dev-demo-state.test.mjs
- scripts/dev-demo.mjs
- scripts/real-browser-acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/milestones.md
- spec/src/security.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-demo-state.test.mjs#development layout defaults to ignored
  checkout state and accepts isolation
- scripts/dev-demo-state.test.mjs#fixture marker distinguishes reusable state
  from partial state
- scripts/dev-demo-state.test.mjs#server configuration refresh is idempotent
- crates/jj-store-server/tests/configuration.rs#explicit_ignored_development_root_allows_only_its_descendants
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
node --test scripts/dev-demo-state.test.mjs
```

**Notes**

The test-first state boundary failed because the persistent development-layout module did not exist.
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
cargo test -p jj-store-server --test configuration explicit_ignored_development_root_allows_only_its_descendants --locked -- --exact
```

**Notes**

The exact policy regression failed to compile because no constrained ignored-checkout development-root boundary existed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The first live reset reached the expected production safety boundary and rejected checkout-local credentials and process state.
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
node --test scripts/dev-demo-state.test.mjs; node --check scripts/dev-demo.mjs; node --check scripts/real-browser-acceptance.mjs
```

**Notes**

All three state/layout/configuration regressions pass and both lifecycle entrypoints parse.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration --locked
```

**Notes**

All ten focused configuration and control-path cases pass, including production checkout rejection, symlink rejection and the exact .local descendant exception.
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
cargo fmt --all -- --check; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

The affected server package is formatted and warning-free across all targets; production policy.rs contains 193 lines and changed scripts contain at most 264 lines.
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
pnpm dev:reset; capture repository and head operation; pnpm dev:restart; pnpm dev:stop; assert fixture/repository/RustFS remain and runtime.json is absent; pnpm dev:restart; compare repository and head operation
```

**Notes**

The explicit reset seeded repository 8365d3c280132ba662bd6af82e2fb763 under .local/dev-demo. Restart and stop/start retained that repository and head operation 855d9848...f8cadd, while stop removed only process metadata. The final stack is running on 127.0.0.1:7460 without credentials.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm check:architecture; pnpm exec prettier --check <changed files>
```

**Notes**

Specification validation passed all 89 requirements; the same-change gate then required the API chapter for browser-harness isolation and architecture for the durable-fixture/process-state split.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm check:architecture; pnpm exec prettier --check <changed files>
```

**Notes**

All 89 requirements, 12 protected files across six canonical chapters, 61 progress tasks and 200 governed web modules validate; every changed prose, script and manifest file is formatted.
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

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; Node.js 26.9.0; development fixture marker format 1; server runtime metadata format 2; configuration format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds an ignored .local/dev-demo fixture marker that separates durable repository, installation and embedded RustFS state from runtime process metadata. An explicit development-only environment root permits only descendants of the checkout's .local subtree; ordinary production and sibling checkout paths remain rejected. Reset may regenerate native JJ IDs, while ordinary restart and stop preserve them. Real browser acceptance uses a separate OS-temporary root.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


