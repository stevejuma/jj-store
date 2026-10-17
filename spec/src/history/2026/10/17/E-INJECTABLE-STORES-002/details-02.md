---
schema: 1
id: E-INJECTABLE-STORES-002
---

# E-INJECTABLE-STORES-002 details 2

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
pnpm dev:disk
```

**Notes**

Initial free 30.6 GiB; default dev server and embedded RustFS were already running and remain untouched.
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
pnpm spec:search -- "JST-LOCAL"
```

**Notes**

The current spec-validator rejects the forwarded -- argument; source was opened directly. No specification lookup was guessed.
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
pnpm dev:cargo -- test -p jj-store-service --test local_collab local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc --locked -- --exact
```

**Notes**

Before extraction: 1 passed. git-baseline.log.
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
pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features companion --offline --message-format=short
```

**Notes**

Initial manifest replacement accidentally marked dev tonic-prost optional. Corrected the declaration.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features companion --offline --message-format=short
```

**Notes**

Focused feature exposed eight missing timestamp/signature conversions; moved the existing pure conversions beside owned DTOs.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-git -p jj-store-jj --no-default-features --features jj-store-jj/companion --offline --message-format=short
```

**Notes**

Pure target resolver needed the resource-limit payload in the new core error type.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-git -p jj-store-jj --no-default-features --features jj-store-jj/companion --offline --message-format=short
```

**Notes**

Extracted page helpers needed base64 and the Git file reader needed its local safe-path import.
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
pnpm dev:cargo -- check -p jj-store-git -p jj-store-jj --no-default-features --features jj-store-jj/companion --offline --message-format=short
```

**Notes**

Both standalone provider and focused JJ feature compiled at that point. An unused helper warning was subsequently removed by exposing its readonly API.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-service --offline --message-format=short
```

**Notes**

JJ stack wrapper needed explicit gix ref-edit imports and readonly Git ID access.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-service --offline --message-format=short
```

**Notes**

One remaining private GitOid field access corrected to as_git().
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-service --offline --message-format=short
```

**Notes**

Service boundary needed neutral error mapping and readonly graph accessors. Original browser/cache/conversion policy remains in service.
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
pnpm dev:cargo -- check -p jj-store-service --offline --message-format=short
```

**Notes**

Service compiled after persistence delegation. Eight reexport/import warnings subsequently corrected.
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
pnpm dev:cargo -- check -p jj-store-git -p jj-store-service --offline --message-format=short
```

**Notes**

All five Git factory implementations compiled with service. This preceded the latest selected-membership and preparation additions.
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
pnpm dev:cargo -- check -p jj-store-git --offline --message-format=short
```

**Notes**

Standalone Git provider compiled with minimal gix features; no native object/WAL/service dependency.
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
pnpm dev:cargo -- test -p jj-store-core -p jj-store-git --offline --message-format=short
```

**Notes**

New factory test referenced a nonexistent project field; corrected to the owned actor timestamp.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

Core six tests and Git two descriptor tests passed. Factory tests: one passed, two failed because newly authored expectations incorrectly treated independent domain versions as publication sequence numbers. Expectations now check independent versions and a single atomic metadata history step; corrections are NOT rerun yet. Postflight reached the 30 GiB free reserve.
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
pnpm dev:clean -- --dry-run
```

**Notes**

No eligible cleanup: live jj-store PID33970 and embedded RustFS PID34018 protect their targets. Free space remained 30.0 GiB rounded; actual available space is below the configured reserve. No cleanup or service stop performed.
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
node scripts/package-check.mjs
```

**Notes**

New crate initially lacked its required LICENSE.md; added standard project license and notice.
<!-- /run -->
<!-- /evidence-field -->

