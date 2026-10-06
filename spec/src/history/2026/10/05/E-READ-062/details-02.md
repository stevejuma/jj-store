---
schema: 1
id: E-READ-062
---

# E-READ-062 details 2

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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 21.42s; test result: ok. 61 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.09s; 27.02s; .local/bounded-reads/performance/landing-jj-lib-062.log
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
cargo test -p jj-store-wal --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 37.07s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.22s; test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.29s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.53s; test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s; test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.45s; test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.72s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.38s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 70.2s; .local/bounded-reads/performance/landing-wal-062.log
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-rpc --locked
```

**Notes**

RPC tests did not start;seven test binaries fail to link on disk exhaustion (errno28). Environment issue retained;landing-rpc-062.log.
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
jj rebase -r @ -o nksqwmtt (main)
```

**Notes**

Clean main lqvkpvpm advanced onto verified nksqwmtt11042048;14 added/35 modified files;no conflicts or parallel main work.
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
python3 copy and byte-compare E-READ-057 through061 local evidence
```

**Notes**

139 evidence log/result files copied into main ignored .local/bounded-reads/performance;temporary originals retained.
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
python3 remove main target/debug/incremental compiler cache
```

**Notes**

Only disposable generated incremental cache removed after linker disk exhaustion;executables,deps,test evidence,services,fixtures and repositories retained.Disk available9.4GiB afterward.
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
cargo test -p jj-store-rpc --locked (first retry)
```

**Notes**

18 RPC tests passed;replication_control could not execute truncated3472-byte artifact left by disk-full build.Permission denied(os13);deleted only that generated executable for relinking.
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
python3 remove truncated replication_control-637c8b5923dbd0d9 generated executable
```

**Notes**

3472-byte nonexecutable artifact removed;normal Cargo relinking required.No source or service binary changes.
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
cargo test -p jj-store-rpc --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 4.44s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.44s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.60s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 10.88s; .local/bounded-reads/performance/landing-rpc-retry-062.log
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
cargo test -p jj-store-cli --test native_rpc --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 1m 33s; test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.62s; 96.94s; .local/bounded-reads/performance/landing-native-rpc-retry-062.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 15.42s; test result: ok. 228 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.98s; 79.37s; .local/bounded-reads/performance/landing-native-lifecycle-retry-062.log
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 10.73s; test result: ok. 37 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 46.23s; 59.72s; .local/bounded-reads/performance/landing-local-browser-retry-062.log
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
cargo test -p jj-store-service --lib --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 36.59s; test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.76s; 38.84s; .local/bounded-reads/performance/landing-service-lib-retry-062.log
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
cargo check -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 43.32s; 43.42s; .local/bounded-reads/performance/landing-packages-retry-062.log
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
cargo fmt -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli -- --check
```

**Notes**

; 1.51s; .local/bounded-reads/performance/landing-rust-format-retry-062.log
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages); 0.31s; .local/bounded-reads/performance/landing-boundaries-retry-062.log
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
pnpm check:architecture
```

**Notes**

web architecture ok (988 modules); 0.46s; .local/bounded-reads/performance/landing-architecture-retry-062.log
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md spec/src/milestones.md
```

**Notes**

docs/implementation-plan.md 124ms (unchanged)
spec/src/architecture.md 94ms (unchanged)
spec/src/milestones.md 1445ms; .local/bounded-reads/performance/landing-format-062.log
<!-- /run -->
<!-- /evidence-field -->

