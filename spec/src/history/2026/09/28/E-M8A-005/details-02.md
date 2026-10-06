---
schema: 1
id: E-M8A-005
---

# E-M8A-005 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli --tests --locked
```

**Notes**

The first focused compile found a test-only JSON Value versus PathBuf comparison in the new registry-init executable assertion; it was corrected without changing production behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli --tests --locked
```

**Notes**

The affected CLI, RPC, HTTP, server and service code compiles without warnings after the registry and token-journal extractions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test local_installation --locked
```

**Notes**

All six focused installation and registry regressions pass, including private authenticated initialization, alias reservation, atomic initial-token indexing and independent candidate revocation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_created_repositories_attach_with_real_cli_and_survive_host_restart --locked; targeted token flow; token generate; registry init; authenticated repository discovery; repository cursor reset
```

**Notes**

Seven focused invocations pass: immediate initial-admin discovery, existing grant/revoke recovery, both new commands, full-token per-repository reauthorization and generation-bound cursor reset.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreConnection.stories.svelte
```

**Notes**

Five states passed; the Connect interaction exposed missing storage readiness in the story fixture, which hid the token field behind the unavailable alert.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreConnection.stories.svelte
```

**Notes**

All six authenticated and LocalTrusted connection interactions pass with blocking accessibility checks. Visual baselines were not run by user direction.
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
pnpm exec tsx --test web/tests/client.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-bootstrap.test.ts; pnpm web:check; pnpm check:architecture
```

**Notes**

Nine client tests and three controller bootstrap tests pass, including retaining the memory-only token while opening another authorized repository. Svelte diagnostics and the 200-module architecture check pass. Touched Rust modules are 269 lines or fewer and touched web production modules are within the enforced budgets.
<!-- /run -->

<!-- run -->
### Run 8

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

The first contract command inherited the unlicensed full-Xcode developer path; all Rust tests use the available Command Line Tools path instead.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

Generated contracts match; 51 captured HTTP and SSE responses conform, all six contract tests pass and the generated TypeScript compiles.
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
cargo fmt --all -- --check; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-service -p jj-store-rpc -p jj-store-server -p jj-store-http -p jj-store-cli --lib --tests --locked -- -D warnings
```

**Notes**

Focused Clippy found one single-element loop left in changed LocalTrusted path validation after authenticated registry paths were separated.
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
cargo fmt --all -- --check; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-service -p jj-store-rpc -p jj-store-server -p jj-store-http -p jj-store-cli --lib --tests --locked -- -D warnings
```

**Notes**

Formatting and warning-denied Clippy pass for every affected Rust package after simplifying the RPC socket validation.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check AGENTS.md docs/cli/commands.md docs/cli/server-config.md docs/cli/server-operator.md docs/cli/tokens.md spec/src/architecture.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/mutations.md spec/src/spec-governance.md spec/src/api.md spec/src/cli.md spec/src/milestones.md spec/src/security.md spec/src/ui.md spec/src/verification.md
```

**Notes**

All 89 requirements validate; 50 protected files map to ten canonical chapters; all 61 progress tasks are consistent; changed guidance, documentation and specifications are formatted.
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
pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check spec/src/compatibility.md spec/src/milestones.md
```

**Notes**

The first landing check preserved the E-M8A-005 evidence object but exposed that automatic table merging had omitted its M8a.8 task row; the row was restored without changing product behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli --tests --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_integrations --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable token_generate_prints_one_valid_manual_credential --locked -- --exact; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable registry_init_creates_one_private_authenticated_catalog --locked -- --exact; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

After rebasing onto the three newer agent-integration changes, the combined CLI compiles; all three affected agent registry tests, both manual-token registry commands and all three repository bootstrap and switching tests pass.
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
pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check spec/src/compatibility.md spec/src/milestones.md
```

**Notes**

The merged ledger validates all 89 requirements and 61 tasks, retains both agent-ingestion and multi-repository evidence, and formats cleanly.
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

Rust 1.98.1; configuration format 2; repository registry catalog version 2; OpenAPI 3.1; Svelte 5.57.1; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds installation registry catalog version 2 with pending aliases, monotonic generation and token-ID candidate sets. Adds GET /v1/repositories V1 pages and jrr1 HMAC cursors bound to generation and token ID. Native repository identities, manifest CAS, WAL, receipts and bearer-token encoding are unchanged. Browser credentials remain memory-only.
<!-- /evidence-field -->

