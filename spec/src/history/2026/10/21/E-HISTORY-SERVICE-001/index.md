---
schema: 1
id: E-HISTORY-SERVICE-001
---

# E-HISTORY-SERVICE-001

<!-- evidence-field:id markdown -->
## Id

E-HISTORY-SERVICE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-21
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-history-service/Cargo.toml
- crates/jj-history-service/README.md
- crates/jj-history-service/src/annotations.rs
- crates/jj-history-service/src/file_history.rs
- crates/jj-history-service/src/frames.rs
- crates/jj-history-service/src/jobs.rs
- crates/jj-history-service/src/lib.rs
- crates/jj-history-service/src/policy.rs
- crates/jj-history-service/src/scopes.rs
- crates/jj-history-service/src/service.rs
- crates/jj-history-service/src/wire.rs
- crates/jj-history-service/tests/admission.rs
- crates/jj-history-service/tests/blame/mod.rs
- crates/jj-history-service/tests/file_history/mod.rs
- crates/jj-history-service/tests/frames.rs
- crates/jj-history-service/tests/operations/mod.rs
- crates/jj-history-service/tests/wire.rs
- crates/tauri-plugin-jj-store/Cargo.toml
- crates/tauri-plugin-jj-store/README.md
- crates/tauri-plugin-jj-store/src/lib.rs
- docs/maintenance/coverage.json
- docs/maintenance/reusable-agent-workflows-progress.md
- docs/site/technical/local.md
- package.json
- packages/jj-history/tests/types.ts
- scripts/check-boundaries.mjs
- scripts/history-service-features.test.mjs
- scripts/local-history-features.test.mjs
- spec/src/current/contracts/architecture/01-architecture.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/21/E-HISTORY-SERVICE-001/SUMMARY.nav.md
- spec/src/history/2026/10/21/E-HISTORY-SERVICE-001/details-02.md
- spec/src/history/2026/10/21/E-HISTORY-SERVICE-001/index.md
- spec/src/history/2026/10/21/SUMMARY.nav.md
- spec/src/history/2026/10/21/index.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/history-service-features.test.mjs#the reusable history service owns no
  Tauri or publication runtime
- scripts/local-history-features.test.mjs#history bridge defaults to a headless
  service and preserves native consumers
- crates/jj-history-service/tests/admission.rs#handles_bind_window_vault_origin_generation_and_exact_operation
- crates/jj-history-service/tests/admission.rs#revoke_waits_for_inflight_reads_and_denies_their_completion
- crates/jj-history-service/tests/frames.rs#renderer_frames_cannot_supply_native_authority
- crates/jj-history-service/tests/wire.rs#envelope_version_shape_and_byte_limits_fail_closed
- crates/tauri-plugin-jj-store/tests/schema.rs#checked_schema_matches_owned_native_history_contract
<!-- /evidence-field -->

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
node --test scripts/history-service-features.test.mjs (before)
```

**Notes**

The independent crate and its adapter dependency were absent.
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
cargo metadata --offline --format-version 1
```

**Notes**

Full all-target resolution requested unavailable rustix-linux-procfs. Host-only no-deps resolution refreshed the lock without dependency versions changing.
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
node --test scripts/local-history-features.test.mjs scripts/history-service-features.test.mjs (first)
```

**Notes**

Four passed; one inherited feature audit incorrectly expected native publication for vault_adoption. The same exact test failed in the original checkout at the parent. Standalone vault tests now require their explicit vault-backup crate gate rather than native publication.
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
node --test scripts/local-history-features.test.mjs scripts/history-service-features.test.mjs (final)
```

**Notes**

Five dependency and feature-boundary regressions passed.
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
pnpm dev:cargo test -p jj-history-service -p tauri-plugin-jj-store --locked
```

**Notes**

14 tests passed: ten admitted real-JJ repository/lifecycle cases, one frame, two wire, one unchanged adapter schema case; no ignored tests.
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
pnpm dev:cargo clippy -p jj-history-service -p tauri-plugin-jj-store --all-targets --locked --features tauri-plugin-jj-store/tauri -- -D warnings
```

**Notes**

Both the neutral service and enabled Tauri transport compile under strict Clippy.
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
pnpm check:boundaries
```

**Notes**

27 package layers pass. The shared history closure has no Tauri or native publication runtime.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (exploratory, unrelated UI lane)
```

**Notes**

Existing UI modules exceed line budgets (401/313/310). No UI code changes are included; this Rust-only extraction uses the repository targeted policy and checks its changed scripts and Rust budgets separately.
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
pnpm history:check
```

**Notes**

Owned schema, generated TypeScript contract and direct client type checks pass.
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
pnpm history:test
```

**Notes**

Nine client tests and negative type contracts pass. The initial negative Blame assertion had two parent-reproducing TypeScript diagnostics; moving its existing ts-expect-error to the rejected root field restores the original security assertion.
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
pnpm spec:validate
```

**Notes**

Seven validators, 4396 chapters and 208 requirements pass. Changed Rust source/test budgets pass separately; the three exploratory UI budget failures reproduce unchanged at the parent and are outside this Rust-only required lane.
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

macOS ARM64; Rust 1.99, JJ 0.45.1, Tauri 2.12.4; source baseline a5dd0ddc; change xqmrnquk.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing version-one bounded JSON history frames and checked schema remain byte-compatible; no repository, operation or cache format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Mechanically move admitted history jobs, handles, scopes, queries and codecs to jj-history-service. Keep existing Tauri imports as explicit adapter reexports of the shared service.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Notes H02: private companion IPC and scoped managed-vault admission, then desktop/hosted consumer qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-001/007 preserve source admission, operation pins, bounds,
  cancellation/drain and publication exclusions. The transport-independent crate
  has the same runtime behavior and public wire schema.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-HISTORY-SERVICE-001 details 2](details-02.md)
