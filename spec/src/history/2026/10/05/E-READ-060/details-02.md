---
schema: 1
id: E-READ-060
---

# E-READ-060 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_rpc --locked inventory::rpc_inventory_collects_only_requested_ids_when_other_objects_exceed_the_budget -- --exact --nocapture (initial fixture)
```

**Notes**

Fixture compilation rejected private records::encode and nonexistent Condition::Any. Corrected fixture to serde_json canonical manifest encoding and expected-old-version CAS; production unchanged. rpc-inventory-before-060.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::rpc_inventory_collects_only_requested_ids_when_other_objects_exceed_the_budget -- --exact --nocapture (corrected before fixture)
```

**Notes**

Unchanged production fails ResourceExhausted when requesting the one operation ID after96 verified retained files under32-node server inventory admission. rpc-inventory-before-corrected-060.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::rpc_inventory_collects_only_requested_ids_when_other_objects_exceed_the_budget -- --exact --nocapture (after)
```

**Notes**

The same exact boundary passes through bounded pinned RPC scan pages. rpc-inventory-after-060.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (first expanded fixture)
```

**Notes**

Two pass;publication fixture attempted to physically read the reserved virtual root and gets Store(NotFound). Replaced with the owning roots::operation constructor;production unchanged. rpc-inventory-exact-060.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (corrected fixture)
```

**Notes**

3 pass;actual later code publication,old catalog replay,sparse continued windows,current revocation,tamper/query rejection,expiry/capacity/restart/epoch boundaries. rpc-inventory-exact-corrected-060.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (final fixtures)
```

**Notes**

4 pass;added positive sorted copy IDs,result-limit rejection and exclusion of a staged eighth copy. rpc-inventory-exact-final-060.log.
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
cargo test -p jj-store-wal --lib --locked publication::reader::tests::inventory_pages_seek_deep_anchors_and_keep_old_roots -- --exact --nocapture
```

**Notes**

4096 selected descriptors;deep anchor4000 reads at most80 records for an eight-descriptor window;every127-row page reconstructs exact digest order;old root remains stable after growth;wrong descriptor identity,read budget and byte budget fail. rpc-inventory-wal-exact-060.log.
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
cargo test -p jj-store-rpc --lib --locked inventory::tests -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 44.50s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s; 44.91s; .local/bounded-reads/performance/rpc-inventory-rpc-codecs-060.log
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
cargo test -p jj-store-wal --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 18.57s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.80s; test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.35s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s; test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s; test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s; test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.37s; test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.34s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 39.32s; .local/bounded-reads/performance/rpc-inventory-wal-060.log
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
cargo test -p jj-store-rpc --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 26.08s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.61s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 38.92s; .local/bounded-reads/performance/rpc-inventory-rpc-060.log
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
cargo test -p jj-store-cli --test native_rpc --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 14.75s; test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.60s; 17.81s; .local/bounded-reads/performance/rpc-inventory-native-cli-060.log
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
cargo check -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 36.61s; 36.7s; .local/bounded-reads/performance/rpc-inventory-packages-060.log
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
cargo fmt -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli -- --check
```

**Notes**

; 1.17s; .local/bounded-reads/performance/rpc-inventory-rust-format-060.log
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages); 1.29s; .local/bounded-reads/performance/rpc-inventory-boundaries-060.log
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
pnpm check:architecture
```

**Notes**

web architecture ok (988 modules); 1.38s; .local/bounded-reads/performance/rpc-inventory-architecture-060.log
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
cargo fmt -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli
```

**Notes**

Changed Rust modules formatted before verification.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/mutations.md spec/src/publication.md
```

**Notes**

Changed owning chapters and delivery plan formatted;rpc-inventory-docs-format-initial-060.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

Inspection command unsupported by the installed JJ;no repository mutation. Rust/doc formatting checks provide the applicable whitespace verification.
<!-- /run -->
<!-- /evidence-field -->

