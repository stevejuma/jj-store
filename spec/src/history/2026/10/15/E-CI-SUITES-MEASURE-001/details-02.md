---
schema: 1
id: E-CI-SUITES-MEASURE-001
---

# E-CI-SUITES-MEASURE-001 details 2

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
node --test scripts/ci-artifacts.test.mjs
```

**Notes**

Nine focused artifact/report tests passed before document updates.
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
pnpm test:release
```

**Notes**

49 workflow, provenance, reporting and release-caller regressions passed; missing artifacts, wrong SHA/attempt, failed producers, skipped shards and publication blocking are covered.
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
pnpm check:architecture
```

**Notes**

1918 modules passed after reporting changes.
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
actionlint -oneline
```

**Notes**

No workflow lint findings.
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
pnpm exec prettier --check scripts/ci-report.mjs scripts/ci-artifacts.test.mjs docs/releases.md
```

**Notes**

Changed implementation/release prose formatting passed before final measurement documentation.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
python3 /tmp/jj-store-measure.py 38730859426
```

**Notes**

Full coordinator [38730859426](https://github.com/stevejuma/jj-store/actions/runs/38730859426), attempt 1, exact source 2d9cdf2063e5a64c6a8b81df7b8cfdf87032a87e, completed failure in 53m18s; 265.92 total runner-minutes. Standard required and all three producers succeeded. All independent suite jobs finished without cancellations. Expected optional compilation-cache comparison stayed skipped. Complete qualification correctly rejected 11 failed suite/shard jobs.

| Coordinator lane | Result | Job execution |
| --- | --- | ---: |
| Resolve qualification source | success | 0m13s |
| Standard checks / Change selection | success | 0m23s |
| Standard checks / Formatting | success | 1m28s |
| Standard checks / Tooling regression tests | success | 1m18s |
| Standard checks / Specification and release policy | success | 0m46s |
| Standard checks / Preflight gate | success | 0m08s |
| Standard checks / Web type checks and unit tests | success | 2m06s |
| Standard checks / Build standard Rust tests | success | 8m40s |
| Standard checks / Standard Rust tests (shard 2/4) | success | 3m11s |
| Standard checks / Standard Rust tests (shard 1/4) | success | 3m04s |
| Standard checks / Standard Rust tests (shard 4/4) | success | 2m58s |
| Standard checks / Standard Rust tests (shard 3/4) | success | 1m48s |
| Standard checks / required | success | 0m07s |
| Backend compilation / build artifact producer | success | 8m05s |
| Embedded runtime / runtime artifact producer | success | 29m45s |
| API contracts / Generated API contracts | success | 10m16s |
| External storage / Real RustFS storage acceptance | success | 1m20s |
| Native platforms / Native compatibility (aarch64-apple-darwin) | success | 8m23s |
| Native platforms / Native compatibility (aarch64-unknown-linux-gnu) | success | 9m03s |
| Native platforms / Native compatibility (x86_64-apple-darwin) | success | 19m07s |
| Browser assets / assets artifact producer | success | 3m50s |
| Backend Rust tests / Server, CLI and capture tests (shard 5/8) | success | 8m05s |
| Backend Rust tests / Server, CLI and capture tests (shard 4/8) | success | 6m55s |
| Backend Rust tests / Server, CLI and capture tests (shard 1/8) | success | 10m14s |
| Backend Rust tests / Server, CLI and capture tests (shard 7/8) | success | 9m09s |
| Backend Rust tests / Server, CLI and capture tests (shard 8/8) | failure | 7m29s |
| Backend Rust tests / Server, CLI and capture tests (shard 6/8) | failure | 8m52s |
| Backend Rust tests / Server, CLI and capture tests (shard 2/8) | success | 9m39s |
| Backend Rust tests / Server, CLI and capture tests (shard 3/8) | failure | 8m56s |
| UI acceptance / Storybook interactions (shard 1/3) | failure | 7m21s |
| UI acceptance / Application browser (shard 1/2) | failure | 5m41s |
| UI acceptance / Storybook interactions (shard 2/3) | failure | 7m41s |
| UI acceptance / Static catalog browser | failure | 1m51s |
| UI acceptance / Storybook interactions (shard 3/3) | success | 7m54s |
| UI acceptance / Help (Chromium and WebKit) | success | 3m13s |
| UI acceptance / Application browser (shard 2/2) | success | 5m24s |
| Cold application / Cold application browser (shard 4/4) | failure | 7m15s |
| Cold application / Cold application browser (shard 2/4) | failure | 6m57s |
| Cold application / Profile restart acceptance | success | 1m41s |
| Cold application / Cold application browser (shard 3/4) | failure | 6m43s |
| Cold application / Real RustFS Git round-trip | success | 5m11s |
| Embedded storage / Embedded storage lifecycle | success | 6m52s |
| Cold application / Cold application browser (shard 1/4) | failure | 6m42s |
| Complete qualification gate | failure | 0m11s |

Additional observations:

- Embedded runtime: 29m45s job including 19m09s Cargo dev preparation, 8m47s lint preparation, 22s post-cache save.
- Native Intel: 19m07s job; 476s CLI preparation, 228s test-binary preparation, 214.89s completed libtest execution. All native platforms passed 248 tests with 3 ignored on each. Earlier macOS Intel 46m07s included 1431s CLI preparation, 586s test preparation and 257.35s execution; differing warm caches prevent attributing that difference to workflow separation alone.
- Backend: 1242 pass and 3 fail. UI: 1099 pass and 5 fail among 1104 checks. API: 8 pass. Storage: 8 external probes and 4 embedded tests pass. Cold profile restart passes; the Git acceptance parent and its browser child each report a pass and are not two independent Git scenarios.
- All four cold browser shards fail during fixture preparation with agent commands require a JJ workspace; no browser test report is produced. Their five-minute setup failure is unqualified coverage, not a zero-test pass.
- Native jobs waited about 8m30s behind a prior same-source workflow concurrency group. Job created-to-start therefore measures scheduler waiting, not runner-only queueing. Dependency waits and runner-only queueing remain unknown.
- The previous coordinator 38725300550 failed missing protoc; 38728404207 installed protoc but failed a transient crates.io HTTP 500 for rustfs-credentials 1.0.0. A fresh same-source full run verified runtime compilation and lint. Their 58m39s/242.45 and 45m04s/202.58 runner-minute totals describe incomplete qualification attempts, not comparable complete baselines.
- Automatic publication and heavy scheduling remain disabled. Main protection is restored with administrator enforcement, strict required checks and PR/conversation rules intact; immutable tag ruleset remains unchanged.

Observed test failures:

- jj-store::agent_kingfisher::kingfisher_redacts_secrets_and_normalizes_workspace_paths
- jj-store-server::cursor_supervisor::linked_cursor_products_backfill_pause_follow_final_flush_and_restart_without_duplicate_messages
- jj-store-server::capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress
- agent-prompt-markup.spec.ts::cold prompt markup attributes are inert and malformed sections remain visible
- history-query.spec.ts::cold native history search highlights, completes and retains invalid drafts under CSP
- test::cold_catalog_contract
- catalog/components/store-work/ProjectsRepositoryActions.stories.svelte::MobileProjectRepositoryActions
- catalog/components/store-application/StoreFileEditor.stories.svelte::EditAndRename
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
python3 /tmp/jj-store-download-reports.py 38730859426
node /tmp/jj-store-report-summary.mjs /tmp/jj-store-qualification-reports/38730859426/all /tmp/jj-store-qualification-observations/baseline-reports.json
python3 /tmp/jj-store-validate-suite-reports.py /tmp/jj-store-qualification-reports/38730859426/all 2d9cdf2063e5a64c6a8b81df7b8cfdf87032a87e 38730859426
```

**Notes**

31 results artifacts retained; 30 consumer/producer admission records match source/run/attempt. The complete-gate report has no payload. Backend JUnit identities exactly cover all 1245 planned unique tests (1242 pass, 3 fail), unchanged inventory digest 064ed4e2ef0a08ef8736356490aa619fe6168f44d3752c733782842d0b50809e. An initial ad-hoc comparison incorrectly compared tuple-encoded inventory strings with concatenated JUnit names and failed; correcting the analysis to compare binary/test identities produced exact matches for all eight shards. Original downloaded artifacts remain unchanged; the new reporting parser was applied separately to the retained logs.
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
node --input-type=module <<'JS'
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {verifyProducer,verifyReceipt} from './scripts/ci-artifacts.mjs';
const expected={source:'2d9cdf2063e5a64c6a8b81df7b8cfdf87032a87e',repository:'stevejuma/jj-store',run:'38730859426',attempt:'1'};
const {run,jobs}=JSON.parse(readFileSync('/tmp/jj-store-qualification-observations/38730859426-api.json'));
assert.equal(run.status,'completed');
verifyProducer(run,jobs,expected,['build','runtime','assets']);
verifyReceipt(JSON.parse(readFileSync('/tmp/jj-store-producer-source-38730859426/source.json')),expected,'source');
console.log('Verified canonical completed run, exact source receipt, standard gate and all three successful producers. Unrelated product failures do not prevent diagnostic artifact reuse.');
JS
```

**Notes**

Before standalone dispatch, verified canonical coordinator workflow, completed attempt 1, exact SHA, successful source-resolution and standard required gate, and successful build/runtime/assets producers. Unrelated failed test jobs do not establish a test pass and do not invalidate successful producers.
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
python3 /tmp/jj-store-dispatch-suites.py 38730859426 2d9cdf2063e5a64c6a8b81df7b8cfdf87032a87e
```

**Notes**

Dispatched backend 38737681929, contracts 38737685939, storage 38737690387, UI 38737694885, cold 38737698369 and native 38737703387 concurrently against producer 38730859426/1. Completion outcomes are recorded separately; successful dispatch is not a qualification pass.
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
pnpm progress:history
```

**Notes**

Refreshed histories for 155 tasks; retained M30/new evidence and restored 450 unrelated generated paths with Jujutsu after snapshotting the intended slice.
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
pnpm check:progress
```

**Notes**

155 tasks and 817 evidence records passed for the initial measurement record.
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
pnpm spec:first
```

**Notes**

Four protected files and two canonical chapters passed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check README.md docs/releases.md scripts/ci-report.mjs scripts/ci-artifacts.test.mjs
```

**Notes**

New README and release-guide paragraphs needed standard wrapping. Formatted those files and the changed canonical/progress pages; the subsequent complete changed-file formatting check passed.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check README.md docs/releases.md scripts/ci-report.mjs scripts/ci-artifacts.test.mjs spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/governance/03-bounded-specification-and-evidence.md spec/src/current/progress/M30/index.md spec/src/current/progress/M30/history/M30/index.md
```
<!-- /run -->
<!-- /evidence-field -->

