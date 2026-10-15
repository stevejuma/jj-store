---
schema: 1
id: E-CI-HEAVY-001
---

# E-CI-HEAVY-001 details 2

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
gh pr view 1 --repo stevejuma/jj-store --json state,headRefOid,mergeStateStatus,mergeable,statusCheckRollup,url
```

**Notes**

Verified exact b71f2c4c head, CLEAN/MERGEABLE, successful required and all standard jobs. Hosted run 38712281519 also confirmed exact Rust cache restore, preserving 1050 inventory identities / 1031 runnable tests.
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
gh run cancel 38712300595 --repo stevejuma/jj-store
```

**Notes**

Cancellation requested and subsequently confirmed completed/cancelled. No sccache adoption or completed benchmark claim.
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
gh pr merge 1 --repo stevejuma/jj-store --merge --match-head-commit b71f2c4c7d852728c4828b457e9a2d0a703d51c3
```

**Notes**

Merged normally, no admin override. Confirmed merge commit 615822b93de172e7bae13c3c77262018769ad48a. Required protected context remains required with strict base and Actions app 15368.
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
gh workflow run ci-heavy.yml --repo stevejuma/jj-store --ref main -f ref=615822b93de172e7bae13c3c77262018769ad48a
```

**Notes**

Dispatch accepted: https://github.com/stevejuma/jj-store/actions/runs/38714895280 . All standard checks precede all heavy lanes. Dispatch acceptance does not prove qualification.
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
gh api repos/stevejuma/jj-store/actions/jobs/116318450537/logs --allow-escape-sequences
```

**Notes**

Prior run 38700726811: original_local_agent_descriptor_remains_readable_before_v2_publication fails at local_agent_provenance.rs:193 with Corrupt. Shard ran 19/173 cases: 18 passed, 1 failed; 154 not run. Other canceled/skipped lanes remain unqualified.
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
pnpm dev:disk
```

**Notes**

143.2 GiB free, 68.3 GiB managed outputs. Reused already registered public-release target. No active target, development fixture or service cleaned or restarted.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store --test local_agent_provenance original_local_agent_descriptor_remains_readable_before_v2_publication --locked --target-dir .local/build-targets/public-release -- --exact --nocapture
```

**Notes**

Exact hosted failure reproduced locally: Result::unwrap received Corrupt at line 193. One failed, seven filtered; exit 101. Existing macOS linker emits large __eh_frame warning.
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
pnpm dev:cargo -- test -p jj-store --test local_agent_provenance legacy_local_agent_descriptor_is_rejected_without_rewrite --locked --target-dir .local/build-targets/public-release -- --exact --nocapture
```

**Notes**

One corrected regression passed: rejects unsupported reads and new writes; retains exact metadata commit and descriptor bytes, preserves supported pinned session reads, and repeats after reopening. No production behavior changed.
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
pnpm dev:cargo -- fmt -p jj-store -- --check
```

**Notes**

Affected Rust formatting passed.
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
pnpm check:progress
```

**Notes**

Initial evidence validation rejected pending as a per-command result. Corrected the dispatch command result to successful dispatch only; overall qualification remains pending.
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
pnpm dev:cargo -- test -p jj-store --test local_agent_provenance --locked --target-dir .local/build-targets/public-release -- --test-threads=1
```

**Notes**

Complete affected binary: 8 passed, no ignored or filtered cases; 27.29 seconds. Existing macOS linker warning retained.
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
pnpm dev:cargo -- test -p jj-store-service --lib agent_descriptor::tests --locked --target-dir .local/build-targets/public-release
```

**Notes**

Both decoder regressions passed; 59 unrelated cases filtered.
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
pnpm dev:cargo -- clippy -p jj-store --test local_agent_provenance --locked --target-dir .local/build-targets/public-release -- -D warnings
```

**Notes**

Affected test and dependency compilation passed strict Clippy.
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
pnpm check:progress
```

**Notes**

155 tasks and 813 evidence records passed after correcting the dispatch result metadata.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

4 validators, 3872 chapters and 198 requirements passed.
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

An unfinished evidence record cannot confirm an unchanged contract. Finalized only the now-complete focused regression scope; full hosted qualification remains explicitly outside that pass.
<!-- /run -->

<!-- run -->
### Run 17

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

The unchanged-contract evidence omitted the CLI source mapping. Added explicit JST-CLI-004 confirmation for the CLI-owned regression and managed executable custody; no command behavior change.
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
pnpm check:progress
```

**Notes**

Final focused evidence and milestone review passed: 155 tasks, 813 evidence records.
<!-- /run -->
<!-- /evidence-field -->

