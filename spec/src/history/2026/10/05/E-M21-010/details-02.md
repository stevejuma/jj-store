---
schema: 1
id: E-M21-010
---

# E-M21-010 details 2

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
cargo test --locked -p jj-store-http --lib working_tree_observation_contract_is_registered_pinned_and_path_free
```

**Notes**

working-transport-before.log: exact contract regression proves the observation route was absent.
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
cargo test --locked -p jj-store-http --lib working_tree
```

**Notes**

working-transport-first.log: missing Access trait import for the concrete registered binding. Import repaired without changing admission.
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
cargo test --locked -p jj-store-http --lib working_tree
```

**Notes**

working-transport-second.log: two pass, one test used the wrong ErrorBody field. Corrected assertion to the existing code field; contract retained.
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
cargo test --locked -p jj-store-http --lib working_tree
```

**Notes**

working-transport-third.log: three focused tests pass; one explicit real CLI fixture remains ignored in the ordinary lane.
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
cargo test --locked -p jj-store-http --lib working_tree_observation_contract_is_registered_pinned_and_path_free
```

**Notes**

working-transport-exact-final.log: the identical before target passes.
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
cargo test --locked -p jj-store-http --lib
```

**Notes**

working-transport-http-full.log: 20 pass, one expected route-inventory count needs updating for the new endpoint (115 vs 114).
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
cargo test --locked -p jj-store-http --lib
```

**Notes**

working-transport-http-repaired.log: complete HTTP library passes 21 tests. Route inventory now explicitly includes working-tree/observe, 115 total routes and 41 local routes. Explicit real-checkout fixture is separately run, not waived.
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
cargo test --locked -p jj-store-service --lib transport_summary_rejects_oversized_complete_change_sets
```

**Notes**

working-transport-summary.log: complete summaries above 4096 changes return Limit; private ignore bytes do not serialize.
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
cargo test --locked -p jj-store-service --test working_tree_projection --test working_tree_scan --test local_working_copy --test stack_workspace_contracts
```

**Notes**

working-transport-service-regressions.log: 23 directly affected scan, sparse projection, physical Save and owned-command contract regressions pass.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/.local/stacks-backend-evidence/working-transport-cold-captures.json cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

working-transport-cold-first.log: existing cold real filesystem-only server/registration fixture passes with actual dirty observation and stale generation responses. Code heads, companion selection and physical/metadata bytes remain unchanged. Retained macOS linker unwind-table size warning, no installed-service changes.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-http --lib registered_observation_route_reads_real_checkout_without_publication_or_content_leaks -- --ignored
```

**Notes**

working-transport-real-router.log: explicit disposable real colocated checkout passes administrative loopback router, content/path privacy, exact heads and stale binding assertions.
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
pnpm contracts:generate
```

**Notes**

working-transport-contract-generate.log: OpenAPI and portable generated client types include the registered owned observation endpoint.
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
node --input-type=module (working-transport-focused-capture-validation)
```

**Notes**

working-transport-focused-capture-validation.log: two actual cold registered-server responses validate; input requires exact generation/operation and rejects arbitrary paths and malformed identities. Full inventory lane is recorded separately when complete.
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
cargo clippy --locked -p jj-store-http --no-deps --all-targets -- -D warnings
```

**Notes**

working-transport-http-lint.log: HTTP package and targets pass strict lint. This does not qualify unchanged JJ dependency failures.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service --no-deps --lib -- -D warnings
```

**Notes**

working-transport-service-lint.log: same reads.rs large enum and stack_workspace/selection.rs manual multiple diagnostics, both byte-identical to e687b039. All six known JJ/service owning files were rechecked unchanged; no waiver or suppression.
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
cargo check --locked -p jj-store-http -p jj-store-service --all-targets
```

**Notes**

working-transport-package-check.log: directly affected packages and targets compile; source-derived fixture commands additionally compile the CLI/server consumers.
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
pnpm web:check
```

**Notes**

working-transport-web-types.log: generated client consumer has zero errors and warnings. No UI/runtime behavior changes.
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
pnpm check:boundaries
```

**Notes**

working-transport-boundaries.log: 17 boundaries pass.
<!-- /run -->
<!-- /evidence-field -->

