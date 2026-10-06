---
schema: 1
id: E-M17-016
---

# E-M17-016 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-activity-before.log 2>&1
```

**Notes**

Three missing-interface compile errors establish absent activity probing, read-only lookup and activity-backed acquisition.
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
cargo fmt -p jj-store-capture
```

**Notes**

Initial activity/catalog formatting; repeated during the slice.
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
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-activity-after.log 2>&1
```

**Notes**

One missing Clone implementation in the device-only candidate. Added Clone without Debug/serialization or new authority.
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
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-activity-corrected.log 2>&1
```

**Notes**

The two initial exact historical/growth and partial-context regressions pass.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Repeated formatting after direct server admission and expanded negative tests.
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
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-activity-expanded.log 2>&1
```

**Notes**

Eight activity test cases pass, including the parent-invoked fresh-process offline-tail helper.
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
cargo test -p jj-store-server --test capture_activity --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-activity-server.log 2>&1
```

**Notes**

Three admission tests pass; fixture has one unused import, removed before final strict lint.
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
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-activity-final.log 2>&1
```

**Notes**

Ten expanded activity test cases pass; later tightened initial arming against an unverified changing baseline.
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
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-activity-arming.log 2>&1
```

**Notes**

Ten final activity cases pass after conservative arming; starts missed before baseline verification remain historical. Includes byte/record bounds, CLI/Desktop distinction, partial old/new starts, unavailable context, prefix rewrite, root rebinding and native session replacement.
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
cargo test -p jj-store-server --test capture_activity --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-activity-final-server.log 2>&1
```

**Notes**

Three admission tests pass after arming changes; final unused fixture import removed without suppression.
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
cargo test -p jj-store-capture --test codex_activity --test codex_context --test codex_transcript --test codex_bound_source --test codex_discovery --test source_registry --test source_registry_policy --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-activity-readers.log 2>&1
```

**Notes**

56 complete direct activity/context/discovery/reader/lease/journal test cases pass. Fresh-process activity-tail and lease helpers are invoked by their parent acceptance tests.
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
cargo test -p jj-store-capture --lib source_registry:: --locked > /tmp/jj-store-agent-capture-m172-activity-unit.log 2>&1
```

**Notes**

Three focused registry unit tests pass, including 4096 admission digest exhaustion, duplicate rejection, retained alias budget and inherited lease release.
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
cargo test -p jj-store-server --test capture_activity --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-activity-clean-server.log 2>&1
```

**Notes**

All three direct server admission tests pass without warnings. Unbound/currently removed repositories select no new source registration; current activity directory differs from header cwd.
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
cargo test -p jj-store-cli --test workspace_management server_capture_admits_only_published_registry_bindings_and_revalidates_them --locked -- --exact > /tmp/jj-store-agent-capture-m172-activity-workspace.log 2>&1
```

**Notes**

Exact actual native directory/socket/catalog verifier acceptance passes; five unrelated workspace-management cases remain filtered.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-activity-check.log 2>&1
```

**Notes**

Owning runtime, direct server adapter and CLI consumer all-target checks pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-activity-boundaries.log 2>&1
```

**Notes**

17-package owned dependency boundaries pass; no dependency added.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-activity-architecture.log 2>&1
```

**Notes**

Existing web static architecture checker passes for 719 modules. No web product change or UI-suite claim.
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
rustc --version
```

**Notes**

Verified Rust 1.98.1 (48a229cea 2026-09-01).
<!-- /run -->
<!-- /evidence-field -->

