---
schema: 1
id: E-M13-027
---

# E-M13-027 details 3

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
pnpm exec tsx --test web/tests/repository-conversion-manager.test.ts (proxy fix)
```

**Notes**

conversion-manager-proxy-after: four pass with lossless serialization preserving exact proxy intent.
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
pnpm web:build; pnpm exec playwright test --config .local/repository-conversion/playwright.config.ts web/tests/browser/repository-conversion.spec.ts (iterations three/four)
```

**Notes**

conversion-browser3/4: one failure each. Reload requires reopening Settings; same-hash goto retains the current mounted Settings. Added explicit cold reload and completion onClose propagation.
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
pnpm web:build; pnpm exec playwright test --config .local/repository-conversion/playwright.config.ts web/tests/browser/repository-conversion.spec.ts web/tests/browser/native-repository-creation.spec.ts (final)
```

**Notes**

conversion-web-build3/browser5: all five pass with unchanged deadlines; cold exact pending/reload/retry, blocked preflight, downloadable receipt report and creation regressions.
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
cargo test -p jj-store-jj --lib query::language::identity::tests --locked
```

**Notes**

identity-tests: two owning bounded identity/config tests pass after mechanical inherited collapsible_if repair. No suppression or changed behavior.
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
cargo test -p jj-store-server --test repository_conversion --offline --no-run; JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_CONVERSION_BROWSER=1 cargo test -p jj-store-server --test repository_conversion --locked -- --ignored --exact cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart --nocapture (initial)
```

**Notes**

rustfs-conversion-compile1/conversion1: two fixture diagnostics each, ambiguous Work Request and private Reaction state. Use public Reaction query.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_CONVERSION_BROWSER=1 cargo test -p jj-store-server --test repository_conversion --locked -- --ignored --exact cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart --nocapture (iterations two/three)
```

**Notes**

rustfs-conversion2/3: real native import, restart, export and nonempty historical graph/source safety assertions pass before browser phase fails. Correct Axe context ownership, then explicitly select the source committed operation instead of assuming asynchronous head loading has finished.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-wal -p jj-store-server -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

conversion-all-target-check1: affected libraries and direct consumer targets compile; owning workspace target only.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-jj -p jj-store-service -p jj-store-wal -p jj-store-server -p jj-store-http -p jj-store-cli --lib --locked -- -D warnings; cargo clippy -p jj-store-server --test repository_conversion --test native_creation --locked -- -D warnings (iterations one/two/three)
```

**Notes**

conversion-lib-lint1/2 and test-lint1/2/3: inherited service materialization conditional and three storage journal/completion conditionals; new preparation argument count and test needless borrow. Mechanically consolidate conditionals, derive store from destination host and remove needless borrow. No lint suppression. The five command-instance failure counts are 1+1+4+4+1.
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
cargo test -p jj-store-service --test collab_pagination materialization:: --locked
```

**Notes**

materialization-regression: seven exact cache/admission/eviction tests pass; unrelated pager cases out of scope.
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
cargo clippy -p jj-store-jj -p jj-store-service -p jj-store-wal -p jj-store-server -p jj-store-http -p jj-store-cli --lib --locked -- -D warnings (final)
```

**Notes**

conversion-lib-lint3: all owning/direct-consumer libraries pass strict lint.
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
cargo test -p jj-store-service --test conversion --locked; cargo test -p jj-store-server --test repository_conversion --test native_creation --locked; cargo test -p jj-store-jj --test conversion --test local_evidence --locked
```

**Notes**

conversion-service/server/jj-tests-final: 5 service, 6 conversion, 5 creation, 2 code and 6 local evidence tests pass. Isolated RustFS is separately invoked, not counted from ignored default suite.
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
pnpm contracts:check (initial complete capture lane)
```

**Notes**

conversion-contracts-check1: inherited api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free times out. Exact isolated reproduction also fails, disproving concurrent build load as the cause. Preserve original publication deadline.
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
pnpm web:check; pnpm catalog:check (final)
```

**Notes**

conversion-web/catalog-check-final: zero errors/warnings, catalog boundary passes.
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
cargo fmt --all; pnpm exec prettier --write changed TS/Svelte/canonical chapters and separately formatted evidence object
```

**Notes**

conversion-format1 through7, canonical/story/real-browser/manager/browser formatting logs; historical ledger raw bytes preserved.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_CONVERSION_BROWSER=1 cargo test -p jj-store-server --test repository_conversion --locked -- --ignored --exact cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart --nocapture (iteration four)
```

**Notes**

rustfs-conversion4: browser assumed immediate completion. Actual HTTP deadline returns 504 with unknown outcome. Recover original request through status controls, preserving existing deadlines and request identities.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_CONVERSION_BROWSER=1 cargo test -p jj-store-server --test repository_conversion --locked -- --ignored --exact cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart --nocapture (iteration five)
```

**Notes**

rustfs-conversion5: 1/1 passes in 195s. Nonempty Agent/Review/Thread/Reaction graph and historical Work versions survive native import, RustFS shutdown/restart and export. Production UI in both directions passes navigation, downloadable reports, axe and zero runtime errors. Real 504 acknowledgement and 503 installation-lock status recover original requests. Source metadata and dirty files remain intact; stock JJ opens destination.
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
pnpm web:build; pnpm exec playwright test --config .local/repository-conversion/playwright.config.ts web/tests/browser/repository-conversion.spec.ts web/tests/browser/native-repository-creation.spec.ts (dismissal regression)
```

**Notes**

conversion-web-build4/browser6: 6/6 cold browser tests pass, including status retry retaining a dismissed operational error episode until receipt recovery. Existing startup/test deadlines unchanged; already-built isolated host avoids Cargo lock during startup.
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
cargo clippy -p jj-store-server --test repository_conversion --test native_creation --locked -- -D warnings (final)
```

**Notes**

conversion-test-lint4: both feature test targets pass strict lint.
<!-- /run -->
<!-- /evidence-field -->

