---
schema: 1
id: E-M17-052
---

# E-M17-052 details 2

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
cargo test -p jj-store-server --test capture_management rescan::rescan_is_conditional_actor_free_and_preserves_device_policy --locked -- --exact > /tmp/jj-store-agent-capture-m172-rescan-before.log 2>&1
```

**Notes**

Genuine before regression: actual loopback rescan returns 404 instead of 200. No compiler failure.
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
cargo fmt --all
```

**Notes**

Repeated pinned formatting throughout implementation and test additions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management rescan::rescan_is_conditional_actor_free_and_preserves_device_policy --locked -- --exact > /tmp/jj-store-agent-capture-m172-rescan-after.log 2>&1
```

**Notes**

Original regression passes; repeated after strengthening conditional/actor-free/policy-preservation assertions. Final direct management suite and registered capture rerun the strengthened scenario.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::discovery::tests --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-discovery.log 2>&1
```

**Notes**

Two compiler diagnostics from assert_eq requiring Debug for a deliberately opaque fingerprint. Preserve complete source-set equality with assert!; no Debug/privacy or product change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::discovery::tests --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-discovery-after.log 2>&1
```

**Notes**

Both exact private discovery tests pass: busy scan continuity, one coalesced fresh pass, cooldown advancement and cached missing-root recovery. Existing macOS test-binary unwind-size linker warning is non-fatal.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime rescan::explicit_rescan_discovers_before_cooldown_but_preserves_activity_admission --locked -- --exact > /tmp/jj-store-agent-capture-m172-rescan-runtime.log 2>&1
```

**Notes**

Real filesystem/runtime rescan finds new file within two seconds of a 30-second cooldown. Inactive history and growth retain registry/native inventory; fresh start admits sanitized backfill. Pause/stop refuse hints.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-library.log 2>&1
```

**Notes**

All 41 then-registered library entries pass serially in 132.16 seconds, including original cold factory/foreground/policy/loss gates. A later additional observation-only test is run exactly with unchanged production. Non-fatal macOS unwind-size linker warning retained.
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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md > /tmp/jj-store-agent-capture-m172-rescan-format.log 2>&1
```

**Notes**

Direct changed canonical/script formatting.
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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-rescan-docs-format.log 2>&1
```

**Notes**

Formatting repeated after verification chapter update.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-rescan-boundaries.log 2>&1
```

**Notes**

Owned storage/service/adapter boundaries pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-rescan-architecture.log 2>&1
```

**Notes**

Architecture and production module budgets pass.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-rescan-spec-validate.log 2>&1
```

**Notes**

Canonical requirement governance validates before final ledger sealing.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests::observations::source_degradation_is_cached_and_clears_after_successful_recovery --locked -- --exact > /tmp/jj-store-agent-capture-m172-rescan-observations.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out; finished in 7.44s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-runtime-suite.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.44s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_control_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-journal-suite.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-management-suite.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.70s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-supervisor-suite.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 209.29s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-http --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-http-library.log 2>&1
```

**Notes**

One HTTP contract test assumes required-field ordering; actual schema contains exactly version/generation/expected_revision. Retain exact field set, property count, actor-free input, generation pattern and route/security assertions while removing ordering assumption. 13 entries pass; one test fails. No production change.
<!-- /run -->
<!-- /evidence-field -->

