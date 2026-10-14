---
schema: 1
id: E-CI-FIXTURE-001
---

# E-CI-FIXTURE-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 235
hasNote: true
noteEncoding: markdown

```

```shell
GitHub CI 38630131742: ordinary and heavy shards, fast migration test and assets producer
```

**Notes**

All four ordinary shards had 235 missing-file panics from Homebrew-only fixture executables. Examined heavy shards also failed from the same shared fixture. Fast failed because the old unsigned migration commit was local-only; assets rejected a stale generated version-option row. Both Linux builds and Linux ARM native qualification passed.
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
git rev-parse 40382d3599db56b4bdb33f23862e737bc521fe66^{tree} 8ac50d51884d5dcaa6b465e6fe9118b93768c134^{tree}; git merge-base --is-ancestor 8ac50d51884d5dcaa6b465e6fe9118b93768c134 origin/main; gh api repos/stevejuma/jj-store/commits/8ac50d51884d5dcaa6b465e6fe9118b93768c134
```

**Notes**

Both revisions have exact tree d1307d3b219e7900ebf89c0feecf2e9da88b3a26; signed migration is public and retained by main. Original 615 evidence, 142 task and 173 requirement assertions remain.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --test conversion --test local_evidence --test local_stacks --locked -- --test-threads=1
```

**Notes**

41 passed across three repaired stock-JJ consumers.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --test local_metadata --test stack_foreground --test stack_preparation_retention --locked -- --test-threads=1
```

**Notes**

28 passed; one ignored subprocess helper retained under its parent acceptance.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test conversion --test local_agent --test local_agent_binding --test local_agent_pages --test local_collab --test local_review --test local_work --locked -- --test-threads=1
```

**Notes**

68 passed across all seven directly changed service test binaries.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-server --test repository_conversion --test stack_combinations --locked -- --test-threads=1
```

**Notes**

16 passed; one ignored child helper retained.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test stack_cli --locked -- --test-threads=1
```

**Notes**

12 passed.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --lib local_repository::work_cache --locked -- --test-threads=1
```

**Notes**

All eight direct cache unit tests passed.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store --all-targets --locked -- -D warnings
```

**Notes**

Strict lint passed for all four affected packages. Shared fixture source is consumed by multiple packages, justifying these complete package checks and all hosted cohorts.
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
JJ_STORE_DOC_REFERENCE=/Users/stevejuma/code/lapismd/jj-store/.local/build-targets/public-release/debug/examples/docs_reference pnpm docs:generate; pnpm docs:review services "Reviewed unchanged physical permission masks and PATH-resolved JJ in the test-only WorkStore cache fixture. Bounds, physical fingerprints, authorization, immutable source selection and API responses are unchanged; generated CLI reference now also includes its existing version option."; JJ_STORE_DOC_REFERENCE=/Users/stevejuma/code/lapismd/jj-store/.local/build-targets/public-release/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Regenerated one missing --version row; reviewed unchanged physical masks and test-only stock-JJ resolution. Strict manual render: 9 diagrams in both themes, 143 files and 26065680 bytes; links and anchors passed.
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
docker run --rm --platform linux/amd64 node:24.13-bookworm-slim <actual hosted docs_reference>; renderCommand(server) exact comparison
```

**Notes**

Actual hosted Linux renderer matches the regenerated macOS reference byte-for-byte.
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
node --test scripts/docs.test.mjs
```

**Notes**

Eight documentation-tool regressions passed.
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
pnpm test:spec
```

**Notes**

26 passed with the public signed migration pin.
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
pnpm format:rust:check
```

**Notes**

Both root and embedded formatting passed.
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

<!-- evidence-field:scope markdown -->
## Scope

Completed local fixture portability and generated-reference qualification. Root product aggregates are out of scope; full hosted CI and release acceptance remain the next gate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require fresh impact-planned hosted CI; no main push, merge or publication.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-JJ-001, JST-LOCAL-004/005 and JST-MUT-001 native authority, physical
  guards, retention and receipts remain unchanged. Fixtures resolve pinned stock
  JJ/Git on PATH instead of a macOS-only directory, preserving every assertion.
  JST-CLI-002 and JST-DOC-001/002 are aligned with the already implemented
  version option and exact original migration tree; no public command behavior
  changes. JST-GOV-002/004 historical integrity and owner coverage remain
  enforced.

```
<!-- /evidence-field -->

