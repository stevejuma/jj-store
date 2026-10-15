---
schema: 1
id: E-CI-GOVERNANCE-001
---

# E-CI-GOVERNANCE-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 491
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CHECK_BASE=a7458b22d0c9e4913a46132a7d4fc83684b44511 JJ_STORE_CHECK_REVISION=0d4ddb10280e4c93e31bb9ce8d1ee1873b55f8ff pnpm check:progress
```

**Notes**

Before repair: exported Git-only selected-tree view with read-only pointer to the existing object database, Node 24.10.0. Exactly reproduced hosted run 38692022436: whole-PR changed paths were compared against direct-parent evidence, falsely requiring earlier records to be fresh.
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
pnpm --package=node@22 dlx node --test scripts/spec/ci-revision-regression.test.mjs
```

**Notes**

Before repair: multi-commit Git/JJ comparison failed because Git included earlier.mjs and its inherited evidence. Merge-parent regression passed.
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
pnpm --package=node@22 dlx node --test scripts/spec/ci-revision-regression.test.mjs
```

**Notes**

After repair: both regressions pass, including unchanged parent evidence, unqualified latest and earlier PR changes, pinned reads, all merge parents and immutable records.
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
node --test scripts/spec/*.test.mjs scripts/governance.test.mjs scripts/ci.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

Node 22.23.3: 75 tests passed, zero skipped. Covers all specification tooling and direct progress/CI/automation consumers.
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
JJ_STORE_CHECK_BASE=a7458b22d0c9e4913a46132a7d4fc83684b44511 JJ_STORE_CHECK_REVISION=0d4ddb10280e4c93e31bb9ce8d1ee1873b55f8ff pnpm check:progress
```

**Notes**

After applying the repaired reader to the original Git-only reproduction: 155 tasks and 807 evidence records passed on Node 22.23.3.
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
pnpm check:architecture; pnpm check:ui-ownership (initial exported-tree validation harness)
```

**Notes**

Architecture passed. Ownership could not load a new regression file because the initial view shared live repository metadata while exporting an older tree. Replaced that harness with a pinned tree, private HEAD/index/refs copied at a temporary JJ checkpoint, and read-only shared objects. Verified its index matched the selected commit before rerunning; the checkpoint was abandoned. No production fix was made for this harness error.
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
pnpm check:architecture
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 0.5s. Log /tmp/jj-store-governance-lane-1.log.
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
pnpm check:ui-ownership
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 1.8s. Log /tmp/jj-store-governance-lane-2.log.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 0.9s. Log /tmp/jj-store-governance-lane-3.log.
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
pnpm check:progress
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 86.5s. Log /tmp/jj-store-governance-lane-4.log.
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
pnpm spec:first --base a7458b22d0c9e4913a46132a7d4fc83684b44511 --head 605e87d20cfd60a12da39d9293e5e231739aa41b
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 89.9s. Log /tmp/jj-store-governance-lane-5.log.
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
node scripts/changeset-check.mjs
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 0.1s. Log /tmp/jj-store-governance-lane-6.log.
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
actionlint
```

**Notes**

Pinned Git-only local CI view 605e87d20cfd60a12da39d9293e5e231739aa41b; Node 22.23.3. Duration 0.3s. Log /tmp/jj-store-governance-lane-7.log.
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
CI_STAGE=early CI_PLAN='{}' CI_JOBS='{"format":{"result":"success"},"tooling":{"result":"success"},"governance":{"result":"success"}}' node scripts/ci-required.mjs
```

**Notes**

Run after all seven local governance steps passed. Success admitted; separately exercised failure, cancelled, skipped and missing for each of the three prerequisites: all 12 correctly rejected. Hosted fast job 116281127765 reported only required CI lane governance: failure. No aggregator or workflow change is needed.
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
pnpm exec prettier --check scripts/spec/ci-revision-regression.test.mjs scripts/spec/change-confirmations.mjs scripts/spec/revision-reader.mjs spec/src/current/governance/06-verification-scope.md spec/src/current/progress/M30/index.md
```

**Notes**

All modified non-history source and canonical pages match repository formatting.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Final evidence/navigation check passed with four validators and 198 requirements; final bounded record has 3856 chapters in the specification tree.
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
pnpm check:progress
```

**Notes**

Final native JJ selected-change check: 155 tasks, 808 evidence records; same-change bookkeeping passed.
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
pnpm spec:first
```

**Notes**

Final native JJ owning-topic gate: three protected files, one canonical chapter. This closes the evidence-only update after the complete Git-only CI-context lane.
<!-- /run -->
<!-- /evidence-field -->
