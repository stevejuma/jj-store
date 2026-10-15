---
schema: 1
id: E-CI-SUITES-MEASURE-001
---

# E-CI-SUITES-MEASURE-001 details 3

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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Four validators, 3889 chapters and 198 requirements passed before final standalone outcomes were appended.
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
pnpm check:ui-ownership
```

**Notes**

One validator, 3889 chapters and 198 requirements passed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
python3 /tmp/jj-store-measure.py 38737681929 38737685939 38737690387 38737694885 38737698369 38737703387
```

**Notes**

All six independent workflows completed against producer 38730859426/1. No suite/shard was canceled or skipped. These are diagnostic reruns, not a replacement full qualification. Elapsed includes scheduler waiting; runner-minutes sum parallel job execution, including post-cache steps, and are not billing costs. Backend/UI/cold reuse shared build artifacts. API/native/embedded-storage still compile local targets. Different cache states and overlap prevent treating these samples as a controlled speedup benchmark.

| Standalone suite | Run | Result | Elapsed | Runner-minutes | Maximum scheduler wait |
| --- | --- | --- | ---: | ---: | ---: |
| backend | [38737681929](https://github.com/stevejuma/jj-store/actions/runs/38737681929) | failure | 10m38s | 66.73 | 0m04s |
| contracts | [38737685939](https://github.com/stevejuma/jj-store/actions/runs/38737685939) | success | 6m40s | 6.58 | 0m02s |
| storage | [38737690387](https://github.com/stevejuma/jj-store/actions/runs/38737690387) | success | 6m10s | 7.37 | 0m03s |
| ui | [38737694885](https://github.com/stevejuma/jj-store/actions/runs/38737694885) | failure | 9m13s | 39.87 | 0m03s |
| cold | [38737698369](https://github.com/stevejuma/jj-store/actions/runs/38737698369) | failure | 10m24s | 34.07 | 5m30s |
| native | [38737703387](https://github.com/stevejuma/jj-store/actions/runs/38737703387) | success | 28m43s | 41.93 | 6m08s |

backend: reported counts {"passed": 1243, "failed": 2, "skipped": 0}. Missing test counts: none. Failed cases: jj-store::agent_kingfisher::kingfisher_redacts_secrets_and_normalizes_workspace_paths; jj-store-server::cursor_supervisor::linked_cursor_products_backfill_pause_follow_final_flush_and_restart_without_duplicate_messages

contracts: reported counts {"passed": 8, "failed": 0, "skipped": 0}. Missing test counts: none. No failing structured cases reported; job outcomes still govern acceptance.

storage: reported counts {"passed": 12, "failed": 0, "skipped": 0}. Missing test counts: none. No failing structured cases reported; job outcomes still govern acceptance.

ui: reported counts {"passed": 1098, "failed": 6, "skipped": 0}. Missing test counts: none. Failed cases: agent-prompt-markup.spec.ts::cold prompt markup attributes are inert and malformed sections remain visible; history-query.spec.ts::cold native history search highlights, completes and retains invalid drafts under CSP; local-search-settings.spec.ts::Search tabs preserve drafts and separate Save from Apply; test::cold_catalog_contract; catalog/components/store-work/ProjectsRepositoryActions.stories.svelte::MobileProjectRepositoryActions; catalog/components/store-application/StoreFileEditor.stories.svelte::EditAndRename

cold: reported counts {"passed": 3, "failed": 0, "skipped": 0}. Missing test counts: cold-cold-0, cold-cold-1, cold-cold-2, cold-cold-3. No failing structured cases reported; job outcomes still govern acceptance.

native: reported counts {"passed": 744, "failed": 0, "skipped": 9}. Missing test counts: none. No failing structured cases reported; job outcomes still govern acceptance.

native-aarch64-apple-darwin: Cargo preparation [{"path": "build.log", "profile": "dev", "seconds": 217}, {"path": "tests.log", "profile": "test", "seconds": 122}]; completed libtest execution [{"path": "tests.log", "execution_seconds": 159.9, "counts": {"passed": 248, "failed": 0, "skipped": 3}}].

native-aarch64-unknown-linux-gnu: Cargo preparation [{"path": "build.log", "profile": "dev", "seconds": 154}, {"path": "tests.log", "profile": "test", "seconds": 152}]; completed libtest execution [{"path": "tests.log", "execution_seconds": 140.27, "counts": {"passed": 248, "failed": 0, "skipped": 3}}].

native-x86_64-apple-darwin: Cargo preparation [{"path": "build.log", "profile": "dev", "seconds": 573}, {"path": "tests.log", "profile": "test", "seconds": 249}]; completed libtest execution [{"path": "tests.log", "execution_seconds": 267.16, "counts": {"passed": 248, "failed": 0, "skipped": 3}}].

The Git parent and browser child are reported separately, not two independent Git scenarios. Four cold fixture failures have unavailable browser test counts, not zero passing tests. The scheduled-import timeout passed on this rerun but remains an unresolved baseline failure; the Kingfisher identity and cursor supervisor failures recurred. UI failure variation is retained, not waived.
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
python3 /tmp/jj-store-download-reports.py 38737681929 38737685939 38737690387 38737694885 38737698369 38737703387
```

**Notes**

All 27 standalone job reports were downloaded and checked with ci-report.mjs and the ad-hoc suite-report verifier. Every artifact admission record identifies the exact requested source and producer 38730859426/1. Backend JUnit identities match every planned shard and the original 1245-test digest exactly. Raw original reports remain unchanged; additive TAP and native phase parsing was applied separately.
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
pnpm progress:history
```

**Notes**

Final measurement data regenerated M30 history; 452 unrelated generated paths restored from the saved parent without touching the requested slice.
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
pnpm check:progress
```

**Notes**

Final completed-run record: 155 tasks and 817 evidence records passed.
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
pnpm spec:first
```

**Notes**

Four protected files and two canonical chapters passed after full measurement data.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Four validators, 3890 chapters and 198 requirements passed after all standalone results.
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
pnpm check:architecture
```

**Notes**

1918 modules passed.
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
actionlint -oneline
```

**Notes**

No findings.
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
pnpm exec prettier --check README.md docs/releases.md scripts/ci-report.mjs scripts/ci-artifacts.test.mjs spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/governance/03-bounded-specification-and-evidence.md spec/src/current/progress/M30/index.md spec/src/current/progress/M30/history/M30/index.md
```

**Notes**

All changed non-archived files passed formatting.
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
jj git fetch --remote origin
```

**Notes**

Main remains at measured 2d9cdf20. Only an unrelated Dependabot remote bookmark changed; no unrelated source work was modified.
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
gh api repos/stevejuma/jj-store/rulesets/25110878
```

**Notes**

Immutable version tags ruleset remains active with deletion/update restrictions. Earlier main protection inspection confirmed administrator enforcement, strict required checks, PR and conversation rules; final push uses the same snapshot/try/finally restoration helper as earlier verified rollout pushes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Repair the cold browser fixture JJ-workspace setup failure and execute all
  four browser shards; these tests did not start.
- Repair Kingfisher identity, cursor-supervisor, baseline capture timeout and UI
  failures in separately governed changes; repeated passes do not erase failed
  samples.
- Full qualification is failing. M30 and other release gates remain incomplete;
  publication and automatic heavy scheduling remain disabled.
- The separately approved sccache comparison, linker changes and test
  reclassification remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v24.10.0, pnpm 10.32.1, actionlint 1.7.12; hosted Rust 1.99.0. Exact suite source 2d9cdf2063e5a64c6a8b81df7b8cfdf87032a87e. Embedded producer installed protobuf-compiler 3.12.4-1ubuntu7.22.04.6 (libprotoc 3.12.4).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Artifact/timing formats remain version 1; additive TAP counts, Cargo target preparation, libtest execution durations and explicit unknown runner_queue_seconds. Existing queue_seconds is scheduler creation-to-start, which can include concurrency waiting. Completed-run API job timing includes post-cache operations and remains distinct from in-job snapshots.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Fix the cold fixture and retained product failures, then require a fresh fully passing coordinator before any publication. Preserve these measurements as the first complete job-level baseline; cold browser coverage remains unqualified.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 and JST-GOV-003/004/011 now require measured counts and
  precise timing semantics, distinguish source-preserving standalone retries
  from new producer attempts, and retain failed qualification evidence. No
  product API, test inventory, release gate, automatic schedule or test-pass
  caching changed.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Six standalone histories, successful artifact
  reuse and local regression checks do not qualify failed or unstarted product
  tests. The next action is to repair qualification failures and rerun the
  complete release gate.

```
<!-- /evidence-field -->

