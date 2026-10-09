---
schema: 1
id: E-CHAT-WORKSPACE-002
---

# E-CHAT-WORKSPACE-002 details 2

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
pnpm progress:history; pnpm check:progress; pnpm spec:first
```

**Notes**

History regeneration touched unrelated Bootstrap/M25 files and their coverage checks refused them. Restored only those generated files, then reran the focused document checks.
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
pnpm contracts:check
```

**Notes**

The first final run overlapped an in-progress next-slice edit and stopped at compilation. That edit was set aside; no fixture failure was observed.
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
pnpm contracts:check
```

**Notes**

Stable provider slice: generated contracts match, 300 actual HTTP/SSE responses conform; seven contract tests and TypeScript contract consumer pass.
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
pnpm check:progress
```

**Notes**

Final progress records valid; M13.3 and M24 remain In progress.
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
pnpm spec:first
```

**Notes**

Protected-file coverage and owning chapters validate.
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
pnpm spec:validate
```

**Notes**

All seven specification validators pass.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

New portable targets exposed two unsafe native-ID property assumptions in native-review-context.ts. Added explicit portable-reader unavailability guard.
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
pnpm web:check
```

**Notes**

Final web type check: zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Catalog type check: zero errors/warnings; 349 catalog modules satisfy ownership boundaries.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity.test.ts
```

**Notes**

17 tests pass, including no native read for portable Git IDs. Existing Svelte-config and Node module.register deprecation diagnostics remain.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI boundary validator passes.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress && pnpm spec:first
```

**Notes**

Evidence 001 incorrectly set overall pass while retaining later-slice gaps; restored pending. Regenerated task histories and added their changed-file coverage.
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
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Portable target extension, selected review inventory and retained-source reader compile.
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
cargo test --locked -p jj-review --test portable
```

**Notes**

Four original portable anchor tests pass before adding the shared-envelope compatibility test.
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
cargo test --locked -p jj-review
```

**Notes**

28 model tests pass, including the new legacy-byte and portable-diff dependency regression. Shared public model justifies the complete owning model package.
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
cargo test --locked -p jj-store-cli --test local_review_files
```

**Notes**

Final two isolated flows pass: Unicode, whole files, both diff sides, missing paths, mismatched blobs, unknown-outcome replay, old selection refusal, edit/rename/delete, operation-store loss, Git GC and cold retained reads. Earlier one-test run also passed. macOS linker warns about large __eh_frame.
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
cargo test --locked -p jj-store-service --test local_review
```

**Notes**

Eight local review tests pass.
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
cargo test --locked -p jj-store-wal --test review_commands --test review_projector --test review_catalog
```

**Notes**

16 tests pass: shared projection/catalog/direct native consumer scope, including explicit native refusal of the unqualified portable carrier.
<!-- /run -->
<!-- /evidence-field -->

