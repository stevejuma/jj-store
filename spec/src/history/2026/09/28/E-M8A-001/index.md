---
schema: 1
id: E-M8A-001
---

# E-M8A-001

<!-- evidence-field:id markdown -->
## Id

E-M8A-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a
- M8a.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

embedded-rustfs
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- rust-toolchain.toml
- Cargo.toml
- crates/jj-store-object/src/s3.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/storage.rs
- crates/jj-store-server/tests/embedded_storage.rs
- runtime/embedded-rustfs/Cargo.toml
- runtime/embedded-rustfs/Cargo.lock
- runtime/embedded-rustfs/src/main.rs
- README.md
- package.json
- docs/upstream-lock.md
- docs/embedded-rustfs-qualification.md
- docs/implementation-plan.md
- docs/licenses/NOTICE.md
- scripts/check-progress.mjs
- spec-validator.config.mjs
- spec/src/security.md
- spec/src/object-store.md
- spec/src/adapters.md
- spec/src/compatibility.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/ui.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/embedded_storage.rs#embedded_rustfs_preserves_objects_across_graceful_restart
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test embedded_storage --no-run
```

**Notes**

Test-first boundary: the embedded storage module and RustFS runtime dependency did not exist; the draft also exposed two test API corrections before implementation.
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
cargo test -p jj-store-server --test embedded_storage --no-run
```

**Notes**

The exact released rustfs crate and jj-cli each require a different native sys crate declaring links=mimalloc. Cargo correctly rejected one dependency graph, so the unpatched embedded API moved to a separately locked helper owned by the server.
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
CARGO_BUILD_JOBS=4 cargo build --manifest-path runtime/embedded-rustfs/Cargo.toml --locked
```

**Notes**

The first clean macOS ARM64 helper build completed in 20m12s with RustFS 1.0.0 default features disabled. The separately locked helper avoids the released jj-cli and RustFS mimalloc links conflict without patching either upstream crate.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_RUSTFS_RUNTIME=<absolute helper> cargo test -p jj-store-server --test embedded_storage --locked -- --ignored --exact embedded_rustfs_preserves_objects_across_graceful_restart --nocapture (three test-first attempts)
```

**Notes**

The focused acceptance first found a credential-redaction assertion that reused bucket text, then a package-relative helper path, then a real Tokio worker-stack overflow while RustFS reopened persisted data. The helper now requires absolute owned paths and matches RustFS's 8 MiB debug worker-stack policy.
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-server --test embedded_storage --locked -- --ignored --exact embedded_rustfs_preserves_objects_across_graceful_restart --nocapture
```

**Notes**

Real embedded RustFS created the bucket, accepted an S3Store conditional object write, shut down through the owned channel, reopened the same volume and returned exact bytes. Debug measurements: 899687824-byte binary; first start 11808 ms at 115696 KiB RSS; restart 107 ms at 110880 KiB RSS.
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
pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; pnpm check:architecture; pnpm check:progress; pnpm format:check; pnpm contracts:check; pnpm web:check; pnpm catalog:check
```

**Notes**

Protected mappings, all 75 requirements, 11 package boundaries, the 186-module web budget, 53 milestone tasks, Rust and Prettier formatting, 51 captured HTTP/SSE responses, six contract tests and production/catalog diagnostics pass.
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
pnpm check:embedded-rustfs; pnpm test:embedded-rustfs
```

**Notes**

The exact pinned helper passes Clippy with warnings denied, then the package-owned task rebuilds it and passes the ignored real-runtime restart acceptance through an absolute helper path.
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
pnpm check
```

**Notes**

Specification, package boundaries, architecture budgets, progress, formatting, embedded-helper Clippy, captured contracts, production/catalog diagnostics and the complete locked Rust workspace Clippy lane pass.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete non-visual aggregate passes 23 governance tests, three storage-harness tests, the real embedded restart, six contract tests, 40 production web tests, 34 catalog unit tests, 68 Storybook interaction/accessibility cases, cold catalog browser acceptance and the complete locked Rust workspace. Declared external-service and OS-credential cases remain ignored in the ordinary workspace lane.
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
pnpm build
```

**Notes**

Cold Storybook and production browser bundles, the separately locked embedded RustFS helper and the complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain warnings.
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
VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including cold catalog/browser evidence, interaction/accessibility, locked Rust checks/tests/builds and mdBook.
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

Rust 1.98.1; rustfs crate 1.0.0 with default features disabled, crates.io checksum ecfecedc774cfa9f40fb0b475e2605cbff858282d18e52927a57ccb5105e5dd4 and published source revision d47f54bfb2f39f48bd1adda334bd27e151fe85b8; external RustFS image rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; generic-S3 target cgr.dev/chainguard/minio@sha256:6a1d0b45c8669726bba580ced0bfa4cb9fdeed1ed636dfabd81d1577beb6937b remains unqualified until A3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, WAL, receipt, manifest, API, or configuration format change in A0. The embedded runtime has a separate Cargo lock because released RustFS and jj-cli have incompatible unconditional mimalloc link packages. Credentials pass only through the owned child environment; readiness and shutdown use private stdio. Absolute executable and volume paths remove working-directory ambiguity, and the helper's 8 MiB worker stack matches the upstream debug runtime policy.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


