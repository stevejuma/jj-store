---
schema: 1
id: E-M8A-002
---

# E-M8A-002

<!-- evidence-field:id markdown -->
## Id

E-M8A-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a
- M8a.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

managed-local-storage
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-object/src/s3.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config/format-2-example.toml
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/storage.rs
- crates/jj-store-server/src/storage/embedded.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-server/tests/local_installation.rs
- crates/jj-store-server/tests/embedded_storage.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/server_executable.rs
- docs/cli/server-config.md
- docs/cli/server-commands.md
- package.json
- spec/src/object-store.md
- spec/src/cli.md
- spec/src/security.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/configuration.rs#older_configuration_formats_are_explicitly_rejected
- crates/jj-store-server/tests/configuration.rs#format_two_backup_targets_are_tagged_bounded_and_strict
- crates/jj-store-server/tests/local_installation.rs#local_initialization_is_atomic_private_and_restart_stable
- crates/jj-store-server/tests/embedded_storage.rs#managed_local_storage_locks_verifies_and_fails_closed
- crates/jj-store-cli/tests/server_executable.rs#local_init_creates_strict_format_two_configuration
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test configuration --test local_installation --locked
```

**Notes**

Test-first boundary: format-2 access/storage types, migration and the local installation API did not exist.
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
cargo test -p jj-store-server --test configuration --test local_installation --locked
```

**Notes**

Nine strict configuration cases and two atomic local-installation cases pass, covering format-1 preservation, separate migration, tagged backup targets, private installation state and checkout rejection.
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
cargo test -p jj-store-cli --test server_executable local_init_and_configuration_migration_are_explicit_and_non_destructive --locked
```

**Notes**

The real executable creates a secret-free local format-2 installation and migrates a format-1 source without changing it.
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
pnpm test:managed-rustfs
```

**Notes**

Pinned embedded RustFS passes lifetime-lock, exact sentinel, conditional write, graceful/abrupt restart, prepared-server shutdown, missing volume/identity/bucket, private credential and occupied-port acceptance.
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
pnpm test:rustfs
```

**Notes**

The isolated pinned external RustFS lane passes all eight create/CAS, stale-token, range/checksum, interrupted-upload, unknown-outcome, restart and conditional-delete probes.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server -p jj-store-cli --locked
```

**Notes**

The widened lane exposed three TLS fixtures whose exact commented template anchor was omitted from the new example; restoring that format-2 example guidance fixed the compatibility regression.
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
cargo test -p jj-store-cli --test server_executable --locked
```

**Notes**

All 12 runnable cold executable cases pass after the compatibility repair; the declared external-container case remains ignored in this ordinary lane.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The first aggregate exposed the A0 low-level embedded test's obsolete implicit bucket-provisioning assumption after startup and provisioning became separate owned phases; the test now provisions explicitly before its transport write.
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
pnpm check
```

**Notes**

All specification, boundary, 186-module architecture, progress, formatting, embedded-helper Clippy, contract, browser/catalog diagnostic and locked-workspace Clippy checks pass.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The non-visual aggregate passes 23 governance tests, three harness tests, both real embedded RustFS acceptances, six contracts, 40 browser units, 34 catalog units, 68 Storybook interaction/accessibility cases, cold catalog acceptance and the complete locked Rust workspace.
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
pnpm build
```

**Notes**

Cold Storybook and production browser bundles, the separately locked embedded RustFS helper and the complete workspace build pass. Existing Design Core :global and bundle-size diagnostics remain non-failing warnings.
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
VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including spec-first mapping, functional Storybook interaction/accessibility, cold catalog/browser acceptance and the locked Rust suite. Visual baselines were skipped by user direction.
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

Rust 1.98.1; rustfs crate 1.0.0 with default features disabled and source revision d47f54bfb2f39f48bd1adda334bd27e151fe85b8. External provider qualification is deferred.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds strict TOML configuration format 2 with tagged access, authoritative storage and optional repeated wal-copy targets. The later LocalTrusted scope reduction removes the temporary pre-release format-1 reader and migration command. Local initialization state uses bounded JSON version 1 identity, empty catalog, internal credentials and ownership records, plus an immutable S3 sentinel at _jj_store/v1/installation-sentinel. Native object, WAL, receipt, manifest and public API formats are unchanged. Backups remain disabled and deferred.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


