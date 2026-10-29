---
schema: 1
id: E-INJECTABLE-STORES-002
---

# E-INJECTABLE-STORES-002 details 4

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
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Attempt 4: Server source anchor caller still expected a fully decoded JJ capsule. Added shared decode_reference with full native identity/closure verification and exact profile/repository/operation binding.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Attempt 5: Extracted service facade size exposed two large-enum-variant lints in CLI. Boxed its inner reusable Git provider.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Attempt 6: Two CLI evidence fixture families still expected validate to decode JJ capsules. Updated them to the same complete qualified decoder.
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
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Attempt 7: Strict all-target Clippy passed across core, Git, JJ, WAL, service, CLI and capture, including direct server/HTTP/RPC consumers. git-clippy-7.log.
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
pnpm dev:cargo -- test -p jj-store-jj --lib --test local_evidence --test local_metadata --test native_codec --test stack_preparation_retention --locked --message-format=short
```

**Notes**

JJ library:95 passed,1 failed,3 ignored. Unchanged sized_recovery_reservations_charge_actual_allowance_and_reject_unbounded_requests failed at cache reopen with Busy. Integration binaries were not run after this failure. Exact rerun and affected integration checks follow separately.
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
pnpm dev:cargo -- test -p jj-store --test local_agent --test local_agent_provenance --test local_agent_provenance_cli --test local_agent_recovery --test local_collab_cli --test local_review_files --test local_work --test native_collab_cli --test review_commands --locked --message-format=short
```

**Notes**

local_agent2 passed; provenance7 passed/1failed. Original structural .validate() no longer rejected a mismatched operation reference. Restored header profile/repository/operation binding in generic admission while retaining required complete injected JJ validation before publication. Later CLI binaries did not run after this failure.
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
pnpm dev:cargo -- test -p jj-store-wal --test collab_commands --test review_commands --test bound_capture --locked --message-format=short
```

**Notes**

27 passed; shared resolver, native commands, bound capture, retained state and original receipt semantics. Log git-wal-final.log; observed total 27.
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
pnpm dev:cargo -- test -p jj-store-capture --test snapshot_publication --test reader_publication --locked --message-format=short
```

**Notes**

31 passed; affected capture readers, snapshot publication, source ordering and receipt recovery. Log git-capture-final.log; observed total 31.
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
pnpm dev:cargo -- test -p jj-store --test local_agent_provenance invalid_capsules_commits_sessions_and_dependencies_never_publish --locked -- --exact
```

**Notes**

Original CLI regression after restoring structural reference binding:1passed. Log git-provenance-exact.log; observed total 1.
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
pnpm dev:cargo -- test -p jj-store-jj --lib query::cache::reservation::tests::sized_recovery_reservations_charge_actual_allowance_and_reject_unbounded_requests --locked -- --exact
```

**Notes**

Exact broader-unit lock failure rerun:1passed without changing production code or assertions. Log git-cache-exact.log; observed total 1.
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
pnpm dev:cargo -- test -p jj-store -p jj-store-jj --test local_agent --test local_agent_provenance --test local_agent_provenance_cli --test local_agent_recovery --test local_collab_cli --test local_review_files --test local_work --test native_collab_cli --test review_commands --test local_evidence --test local_metadata --test native_codec --test stack_preparation_retention --locked --message-format=short
```

**Notes**

65 passed:31 across9affected CLI binaries;34 across4JJ integration binaries. Native codecs, default-feature stack retention, full-reference capsule validation and local capture/provenance/work/review/collaboration verified. Log git-consumers-final-2.log; observed total 65.
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
pnpm dev:cargo -- test -p jj-store-jj --lib --locked --message-format=short -- --test-threads=1
```

**Notes**

Complete JJ library repeat with serial execution:96passed,0failed,3existing ignored measurements. The earlier Busy failure remains recorded and its exact rerun also passed. Log git-jj-lib-final-2.log; observed total 96.
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
pnpm dev:cargo -- test -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

Core/Git final repeat after structural reference fix; full focused suites passed. Log git-core-final-2.log; observed total 14.
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
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Final strict direct-consumer Clippy after restoring structural reference binding:passed. git-clippy-final.log.
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
pnpm check:progress
```

**Notes**

Evidence checker rejected pass with explicit remaining provider-composition gates. Kept E002 pending rather than erasing the incomplete gates. Extraction regression families pass; full provider-composition qualification remains later M32 work.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The first spec-first attempt surfaced the same pending-evidence result inconsistency; corrected E002 to pending and retained all composition gaps.
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
node scripts/contracts.mjs --write
```

**Notes**

Generated both owned OpenAPI and TypeScript artifacts through the managed Cargo wrapper; no HTTP acceptance aggregate invoked.
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
jj --no-pager diff --exit-code -- schemas/openapi.json web/src/lib/api/generated.ts
```

**Notes**

Installed JJ does not support diff --exit-code. Replaced this attempted comparison with an explicit empty changed-path assertion.
<!-- /run -->
<!-- /evidence-field -->

