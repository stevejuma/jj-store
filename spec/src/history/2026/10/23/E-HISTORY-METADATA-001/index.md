---
schema: 1
id: E-HISTORY-METADATA-001
---

# E-HISTORY-METADATA-001

<!-- evidence-field:id markdown -->
## Id

E-HISTORY-METADATA-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-23
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-history-service/src/service.rs
- crates/jj-history-service/tests/admission.rs
- crates/jj-history-service/tests/metadata/mod.rs
- spec/src/current/contracts/architecture/29-pinned-history-metadata.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/progress/M33/checkpoints.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-history-service/tests/metadata/mod.rs#ready_handles_read_metadata_without_rebuilding_or_advancing_the_source
<!-- /evidence-field -->

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
pnpm dev:cargo -- test -p jj-history-service --test admission ready_handles_read_metadata_without_rebuilding_or_advancing_the_source --locked (before)
```

**Notes**

Ready handles rejected QueryInfo with IndexRebuildRequired. Cold JJ Desktop acceptance independently failed on Overview with the same error.
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
pnpm dev:cargo -- test -p jj-history-service --test admission ready_handles_read_metadata_without_rebuilding_or_advancing_the_source --locked (after)
```

**Notes**

1 regression passed: query info, workspace/bookmark metadata, selected and ancestor operation details, rejected future operation and byte-identical source.
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
pnpm dev:cargo -- test -p jj-history-service -p tauri-plugin-jj-store --locked
```

**Notes**

16 tests passed across service admission, frames, wire and transport schema; doc tests passed.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-history-service -p tauri-plugin-jj-store --all-targets --locked -- -D warnings; pnpm dev:cargo -- fmt -p jj-history-service
```
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
pnpm test:native-app (../jj-desktop)
```

**Notes**

Cold built WebView read actual content in all seven complete screens, then exercised native IPC, publication/reopen, token revocation, retained drafts, guarded physical save, code snapshot and teardown.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CHECK_REVISION=@- pnpm check:progress (baseline 471c5c77)
```

**Notes**

Two missing E-REUSABLE-PROGRESS-050 references, two fresh-ledger/evidence failures, and four existing documentation coverage failures; unrelated parent changes preserved.
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
JJ_STORE_CHECK_REVISION=@- pnpm check:progress (isolated source commit); pnpm spec:validate --only fileLimits
```

**Notes**

Progress passed for the source slice with the four unrelated documents excluded
(180 tasks, 953 records). File limits passed (4404 chapters, 208 requirements).
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

macOS ARM64; Rust 1.99.0; JJ 0.45.1; no dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire, repository or cache format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Ready history handles route point metadata to the existing bounded source reader under their admitted reader lease and original operation pin. Ancestor operation details require an explicit bounded relation proof. The same source service is consumed by the rebuilt JJ Desktop WebView; fixtures never use a user repository.
Broader workspace/UI/provider/release qualification is out of scope. Existing
M33 progress-document errors remain separately identified in the baseline run;
the four unrelated documents are excluded from this source commit.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain original M33 and Notes gates. Review complete screens in JJ Desktop separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 now explicitly covers prepared-handle point metadata
  dispatch in architecture/29. Source admission, immutable pins, bounded
  ancestry, lifecycle leases and read-only behavior remain enforced.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33/M33.4 remain In progress. Native consumer coverage does not promote
  Notes managed conversation, live providers or distribution.

```
<!-- /evidence-field -->

