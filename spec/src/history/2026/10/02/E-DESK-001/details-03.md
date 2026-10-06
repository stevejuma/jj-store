---
schema: 1
id: E-DESK-001
---

# E-DESK-001 details 3

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
pnpm web:build
```

**Notes**

Production bundle succeeds; inherited Design Core :global minifier and chunk-size warnings retained.
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
node scripts/desktop.mjs stage --server-dir /Users/stevejuma/code/lapismd/jj-store/target/debug --rustfs-runtime /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs
```

**Notes**

Paired binaries and 212 allowlisted production assets staged; generated resources ignored.
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
JJ_STORE_DESKTOP_TEST_RESOURCES=/Users/stevejuma/code/lapismd/jj-store-desktop/desktop/resources cargo test --manifest-path desktop/Cargo.toml --no-default-features --test managed_runtime -- --ignored --exact real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
```

**Notes**

Real host launch/protected bootstrap, origin and identity/config retention, occupied-port refusal and restart pass.
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
node scripts/check-boundaries.mjs (workspace and unchanged main before)
```

**Notes**

Existing service jj-review dependency from parent36c92787 missing in boundary checker; reproduced on unchanged main.
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
node --test --test-name-pattern service_owned_review_reader_is_within_layer_boundary scripts/governance.test.mjs (before)
```

**Notes**

Exact regression confirms checker drift; source allowlist corrected to current owned reader dependency.
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
node --test scripts/governance.test.mjs (after)
```

**Notes**

25 governance tests pass, including focused owned reader boundary regression.
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
node scripts/check-boundaries.mjs (after)
```

**Notes**

16 packages pass; desktop independent manifest has no repository, upstream JJ or storage dependencies.
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
node scripts/web-architecture.mjs
```

**Notes**

730 production/script modules within budgets; desktop Rust modules separately below300 counted lines.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write desktop/Cargo.toml [changed prose/JSON/JS list]
```

**Notes**

Prettier has no TOML parser; remaining supported files formatted. TOML validated by Cargo.
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
cargo fmt -p jj-store-cli
```

**Notes**

Affected CLI formatted.
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
cargo fmt --manifest-path desktop/Cargo.toml
```

**Notes**

Desktop Rust formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate (before verification row correction)
```

**Notes**

New verification rows initially appended outside existing table; inserted into canonical table.
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
pnpm spec:validate (after)
```

**Notes**

5 validators,22 chapters,151 requirements pass.
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
pnpm check:ui-ownership
```

**Notes**

Owned production UI provenance checks pass; no Community runtime dependency added.
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
pnpm spec:first
```

**Notes**

29 protected files and5 owning canonical chapters pass before ledger finalization.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (before evidence entry)
```

**Notes**

Three missing task evidence links plus absent same-change evidence; this entry supplies them.
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
JJ_STORE_DESKTOP_DATA_DIR=/private/tmp/jj-store-desktop-native.tc7xYp desktop/target/debug/jj-store-desktop
```

**Notes**

Fresh isolated native process reports managed page loaded. CUA cannot attach to an unbundled executable; UI interaction not yet qualified.
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
/bin/kill -TERM 29371
```

**Notes**

Abruptly terminated only the owned test host. Verified host29371/server29437/RustFS29438 all gone; temp installation retained. Demonstrates OS-close owner-pipe cleanup, distinct from explicit Quit.
<!-- /run -->
<!-- /evidence-field -->

