---
schema: 1
id: E-HISTORY-CLOSE-001
---

# E-HISTORY-CLOSE-001

<!-- evidence-field:id markdown -->
## Id

E-HISTORY-CLOSE-001
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

2026-10-22
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-history-service/src/jobs.rs
- crates/jj-history-service/tests/admission.rs
- spec/src/current/contracts/architecture/28-completed-history-shutdown.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/progress/M33/checkpoints.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-history-service/tests/admission.rs#completed_shutdown_allows_later_window_revocation
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-history-service --test admission completed_shutdown_allows_later_window_revocation --locked
```
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
pnpm dev:cargo -- test -p jj-history-service --test admission completed_shutdown_allows_later_window_revocation --locked
```

**Notes**

Exact previously failing Busy regression now passes; repeated shutdown and later window revocation succeed while admission remains denied.
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
pnpm dev:cargo -- test -p jj-history-service --locked
```

**Notes**

14 passed: 11 admission/history, one frame and two wire cases.
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
pnpm dev:cargo -- clippy -p jj-history-service --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- fmt -p jj-history-service
```
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
pnpm spec:validate --only fileLimits
```

**Notes**

One validator, 4399 chapters before final evidence update.
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
pnpm desktop:bundle --debug (in ../jj-desktop)
```

**Notes**

Cold normal development bundle rebuilt with this source fix, without the acceptance feature. 31 packaged helper/native hashes verified.
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
'src-tauri/target/debug/bundle/macos/JJ Desktop.app/Contents/MacOS/jj-desktop' (in ../jj-desktop), followed by native UI smoke
```

**Notes**

Opened isolated fixture, read pinned revision and hello.txt, published synthetic conversation, loaded bundled agent settings, then closed native window. Exit 0; no Busy or history cleanup warning. No live provider prompt.
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

macOS ARM64; Rust 1.99.0; JJ 0.45.1; unchanged dependency lockfiles.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No repository, wire or cache format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Successful history shutdown cancels and joins jobs and drains reader scopes, then retires scopes so later Tauri destruction callbacks succeed. Serial lifecycle exclusion and closed admission remain enforced. Revision is the enclosing Jujutsu change. Full workspace, unrelated UI, provider and release lanes are out of scope for this cleanup correction. The initial consumer acceptance succeeded but reported Busy during later destruction; a new source regression reproduced it. Source attempts through the consumer manifest could not resolve dependency dev-dependencies, so all source tests were run from their owning workspace. Working-copy progress validation has pre-existing M33/M33.4 missing E-REUSABLE-PROGRESS-050 and fresh-evidence errors in four user-edited documents, reproduced at baseline 61bb29ae; the isolated committed source slice is checked separately. An evidence-new navigation regeneration was restored to that baseline for all unrelated pages, preserving the four user changes exactly.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain separate Notes and JJ Desktop provider/distribution gates; no broader milestone is promoted.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-007 already requires canceled and joined jobs, supervised drain
  and host-owned lifetime. This internal cleanup repair retires scopes only
  after successful serialized shutdown. Admission remains closed; operation
  pins, authorization, publication, wire and cache formats are unchanged.
  Architecture/28 documents the existing contract consequence without modifying
  the overlapping user-edited chapter. Exact before/after regression, all owning
  crate tests and cold native consumer teardown qualify the repair.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33/M33.4 remain In progress; Notes managed-conversation, restart,
  live-provider and distribution gates remain separate. The checkpoint records
  this narrow native lifecycle correction.

```
<!-- /evidence-field -->

