---
schema: 1
id: E-READ-089
---

# E-READ-089 details 2

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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_reads_bound_registration_inspections -- --nocapture
```

**Notes**

.local/binding-cache/regression-before.log: 238 inspections versus at most 20.
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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_reads_bound_registration_inspections -- --nocapture
```

**Notes**

regression-after.log: trait import missing in extracted binding; corrected. Initial cargo fmt also reported not-yet-created tests module; created and formatted.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked local_browser::admission
```

**Notes**

cache-unit.log: test constructor used from_bytes instead of owned RepositoryId::new; corrected.
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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_reads_bound_registration_inspections -- --nocapture
```

**Notes**

regression-after-2.log: test diagnostic required Debug on Response; removed diagnostic formatting, preserved assertion.
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
pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md
```

**Notes**

spec-format.log: four edited chapters need formatting; corrected by same files with --write.
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
cargo test -p jj-store-cli --test local_browser --locked admission_cache:: -- --nocapture
```

**Notes**

admission-tests.log; 35.15s. Exact admission tests pass: 10 physical inspections versus 238 before; fresh boundary and synchronous invalidation tests pass.
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
cargo test -p jj-store-service --lib --locked local_browser::admission
```

**Notes**

cache-unit-final.log; 23.98s. Exact admission tests pass: 10 physical inspections versus 238 before; fresh boundary and synchronous invalidation tests pass.
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
cargo test -p jj-store-server --lib --locked local_repositories::watcher::tests
```

**Notes**

.local/binding-cache/watcher.log; 196.42s.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

.local/binding-cache/local-browser.log; 165.72s.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-server -p jj-store-cli --all-targets --locked
```

**Notes**

.local/binding-cache/packages.log; 136.09s.
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
pnpm dev:backend:build
```

**Notes**

.local/binding-cache/backend-build.log; 39.63s.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store pnpm dev:backend:restart
```

**Notes**

Backend replaced; fixture and Vite preserved.
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
python3 -u .local/binding-cache/benchmark.py after
```

**Notes**

18 admitted HTTP requests, three rounds, same unoptimized profile, OS caches retained. / names: before 1.474s, after median 1.159s (1.140–1.195s). / metadata: before 12.475s, after median 1.310s (1.161–1.448s). arch names: before 1.241s, after median 1.212s (1.119–1.326s). arch metadata: before 9.336s, after median 1.171s (1.155–1.355s). block names: before 1.373s, after median 1.185s (1.092–1.210s). block metadata: before 22.591s, after median 1.280s (1.198–1.540s). Attribution unchanged: root 19/22 path_change/snapshot, arch 5/18, block 0/50. Warm quick metadata meets five-second target. Not a browser or OS-cold benchmark.
<!-- /run -->

<!-- run -->
### Run 14

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

Ledger rejected pass with explicit deferred gaps, and then unsupported partial status; retained pending for overall responsiveness work, with slice verification recorded separately.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- No release, optimized-build, OS-cold, RustFS or visual qualification claimed.
  Broader unrelated suites out of scope. Names-only setup still about 1.2s;
  directory response reuse and listing setup are the next measured slice. Reader
  reuse, completed server path-result cache and soft attribution allowance
  remain conditional on measurements.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Admission-cache slice verified: commit and land it; retain temporary workspace. Continue directory UI response reuse and listing setup investigation. Remaining items describe separate qualification and conditional optimizations, not failures in this slice.
<!-- /evidence-field -->

