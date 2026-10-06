---
schema: 1
id: E-M14-002
---

# E-M14-002 details 2

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
jj rebase -s qmuvzusm -o nqpoyvww; jj squash --into qmuvzusm --use-destination-message
```

**Notes**

Rebased onto completed main c22556aa. Resolved only specification table/ledger conflicts. Preserved all 194 main evidence entries and E-M14-001; original checkout clean and unchanged. Settings change is now ecc3521d.
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
pnpm spec:validate; pnpm check:progress; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts
```

**Notes**

Rebased specification/progress checks passed; all six settings controller tests passed.
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
cargo test --locked -p jj-store-server --test workspace_links
```

**Notes**

Before implementation: workspace_links target absent; no cases executed.
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
cargo test --locked -p jj-store-server --test workspace_links
```

**Notes**

Three persistence, revision, scope, resource-limit, corruption and symlink regressions passed.
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
cargo test --locked -p jj-store-cli --test workspace_management
```

**Notes**

Two bounded native discovery/profile/installation regressions passed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts -- --exact
```

**Notes**

Initial fixture compile exposed imported auth Result alias; qualified std::result::Result. No test cases executed.
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
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts -- --exact
```

**Notes**

One real handler/native authorization fixture passed; 182 unrelated cases filtered.
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
pnpm contracts:generate
```

**Notes**

Generator rejected GET/POST sharing one path. Used explicit POST /local/workspaces/link, preserving the generator invariant.
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
pnpm contracts:generate
```

**Notes**

Generated OpenAPI and TypeScript successfully after route correction.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test --locked -p jj-store-cli --test server_executable local_trusted_executable_bootstraps_registers_and_serves_without_tokens -- --exact --ignored
```

**Notes**

One cold real-RustFS test passed in 11.62 seconds after compilation: isolated native CLI/server startup, verified folder link, unchanged native snapshot and link equality after service restart. Only disposable processes/state were created and cleaned up.
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
pnpm test:contracts
```

**Notes**

Six contract validation tests and generated TypeScript narrowing passed.
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
pnpm web:check
```

**Notes**

Generated contract consumer: zero errors/warnings.
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
pnpm check:boundaries; pnpm check:architecture
```

**Notes**

15 package boundaries and 403 production modules accepted.
<!-- /run -->

<!-- run -->
### Run 14

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

Initial check identified compatibility, mutation and governance chapters mapped by generated contracts and new service/capture code; added the concrete device-state compatibility, retry and qualification contracts.
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
pnpm contracts:check
```

**Notes**

All registered captures ran; 84 actual HTTP/SSE responses conform. Six schema tests plus TypeScript narrowing passed. Includes final POST /local/workspaces/link route.
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
cargo clippy --locked -p jj-store-server -p jj-store-http -p jj-store-cli --lib -- -D warnings
```

**Notes**

Focused package lint/check passed.
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
cargo test --locked -p jj-store-server --test configuration --test local_installation --test workspace_links
```

**Notes**

21 passed: 12 configuration, six installation and three workspace registry tests.
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
cargo test --locked -p jj-store-cli --test workspace_management
```

**Notes**

Final two discovery regressions passed after requiring a regular working-copy directory.
<!-- /run -->
<!-- /evidence-field -->

