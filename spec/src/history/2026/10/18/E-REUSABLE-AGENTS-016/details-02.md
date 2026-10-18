---
schema: 1
id: E-REUSABLE-AGENTS-016
---

# E-REUSABLE-AGENTS-016 details 2

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
node --test scripts/conversations-features.test.mjs
```

**Notes**

Before: 3 pass and 1 actual boundary failure for the optional owned domain/core edges. Retained in .local/reusable-conversations/store-adapter-boundary-before.log.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-conversations --features store-capture --locked
```

**Notes**

Before: Cargo rejects the missing feature. This is capability-absence evidence, not a runtime regression. Retained in store-adapter-feature-before.log.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-conversations --features store-capture --no-default-features
```

**Notes**

Initial optional feature compiles; 20 existing coordination/capture/stream cases pass. Cargo resolves only the new owned optional and provider-test dependency edges.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-git --test store_capture --locked
```

**Notes**

Initial new test compilation uses gix::ObjectId for a MetadataCommit. Corrected the fixture to the existing public MetadataCommit type.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-conversations --features store-capture --locked --test store_capture
```

**Notes**

Initial new codec test calls a nonexistent CollabObjectRef::new. Corrected the fixture to its public typed fields.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-conversations --features store-capture --locked --test store_capture
```

**Notes**

All 3 private codec cases pass: full-width ordinals/native accepted time/versions and exact byte vectors, malformed/tampered preparation and receipt rejection, and pre-IO aggregate/field bounds.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-git --test store_capture --locked
```

**Notes**

Initial 7, then 8, then final 9 provider tests pass after incremental additions. Final run includes stale Thread after preparation and provider commit without original journal preparation. Production NativeCapture uses production StoreCapture and real Git CAS with a fsynced single-owner test-host journal. No output is released on unknown/rejected persistence, and recovery never executes or prepares.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-native --test factories store_capture::native_sink_retains_exact_preparation_and_recovers_unavailable_original_receipts_read_only -- --exact
```

**Notes**

1 exact case passes across ordinary commit, lost manifest ACK and missing selected receipt profiles. Actual native typed preparation and publication survive journal encode/decode, later Thread writes, checkpoint, recovery and fresh provider/journal construction; replay stages zero objects and reauthorizes. Object storage is memory-backed, not real RustFS.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-agent -p jj-store-core -p jj-store-conversations -p jj-store-git -p jj-store-native --features jj-store-conversations/store-capture,jj-store-conversations/native-privacy --locked
```

**Notes**

All 113 then-present cases pass: Agent 50, core 7, conversations 28, Git 21 and native 7. Expanded from the exact regression because the shared optional boundary has two concrete provider consumers. The subsequently added stale Thread case passes in the final 9-case Git run, giving 114 distinct qualified cases across these suites. Includes all 5 real-scanner privacy cases.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-conversations --no-default-features --locked
```

**Notes**

All 20 default-profile cases pass. Default feature graph remains free of owned store/domain interfaces, scanner, application services and runtimes.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- clippy -p jj-agent -p jj-store-core -p jj-store-conversations -p jj-store-git -p jj-store-native --all-targets --features jj-store-conversations/store-capture,jj-store-conversations/native-privacy --locked -- -D warnings
```

**Notes**

Strict all-target Clippy passes for all 5 affected crates.
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
node --test scripts/governance.test.mjs scripts/conversations-features.test.mjs
```

**Notes**

All 35 cases pass, including the 5 conversation feature/dependency cases. Exact standalone feature tests also passed repeatedly during implementation.
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
pnpm check:boundaries
node scripts/package-check.mjs
```

**Notes**

All 25 workspace package boundaries pass; all 26 Cargo packages have contained compilation inputs, registry-compatible dependencies and crate-root README/LICENSE/NOTICE files.
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
pnpm spec:search 'JST-ARCH-007'
```

**Notes**

The existing QMD native module does not match Node ABI 137. Used the advertised scoped rg fallback and read canonical architecture/03-requirements.md. No QMD repair or index/tool qualification is inferred.
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
pnpm exec spec-validator validate --only governance,fileLimits,verification
```

**Notes**

First draft added prose to the already-full change-map chapter: 407 physical lines. A shorter draft still had 402 lines. Both runs failed one file-limit check; moved the new topic to reusable-capture-boundaries.md and restored the old chapter unchanged.
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
pnpm exec spec-validator validate --only governance,fileLimits,verification
```

**Notes**

All 3 selected validators pass across 4,250 chapters and 207 requirements after the topic split. No aggregate product/spec-check lane is substituted.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm dev:backend:build
```

**Notes**

Both jj-store and jj-store-jj development binaries build; preflight rebuilds/stages the verified four-helper Lapis AI bundle. macOS ld emits its existing compact-unwind-size warning; no Rust error or failed build. No installed binary is replaced.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/desktop/resources/jj-store-rustfs pnpm dev:backend:restart
```

**Notes**

Backend-only restart succeeds with Vite retained. UI 7460, backend 7462, preserved .local/dev-demo, search ready. No reset, migration, unrelated service restart or fixture cleanup.
<!-- /run -->
<!-- /evidence-field -->

