---
schema: 1
id: E-VISUAL-NATIVE-PARITY-001
---

# E-VISUAL-NATIVE-PARITY-001 details 8

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
node .local/native-visual-parity/verify.mjs cached
```

**Notes**

All 933 use comparisonSource=cached-actual. Every captureOperationId, actualCapturedAt, actual PNG SHA-256 and baseline SHA-256 remains equal to the fresh run. Visual outcomes are unchanged; no browser recapture occurs.
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
pnpm --silent visual-delta test --affected --dry-run --explain
```

**Notes**

1046-story dry-run selects 131 and excludes exactly 915 unchanged passing stories. The selection is exactly the 18 strict visual mismatches plus 113 out-of-scope standalone blockers. All four changed-within-tolerance outcomes satisfy unchanged policy and are excluded with other passing stories. No blocked story is actually captured by this dry-run.
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
node .local/native-visual-parity/verify.mjs built
```

**Notes**

All 933 persisted PNGs equal the pre-comparison inventory and their static copies. Chip source contains no visualDelta metadata, while its compiled bundle contains the discovered public URL. That URL returns HTTP 200 and exact persisted bytes from isolated server 127.0.0.1:6047.
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
pnpm spec:validate
```

**Notes**

Initial generated evidence validates with 7 validators, 4437 chapters and 208 requirements.
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
JJ_STORE_CHECK_REVISION=kluosrto pnpm check:progress
```

**Notes**

Initial immutable snapshot 6606f07f45c6df47e206c93c3eb7347aca2e5a36 validates 180 tasks and 957 evidence records.
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
pnpm exec prettier --write docs/catalog.md spec/src/current/contracts/ui/01-storybook-presentation-catalog.md spec/src/current/progress/M4/index.md
```

**Notes**

Owned authored documents are formatted; generated historical evidence is intentionally ignored by the normal formatter.
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
node --input-type=module (initial evidence page limit assertion after forced formatting)
```

**Notes**

Forcing formatting of generated history expanded details-07.md to 454 lines, beyond the 400-line contract. The canonical writeEvidence renderer regenerates only this new record; no budget or formatter policy is relaxed and no historical record is changed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base 95d46e440198c4b2eb1fded32f8efee6160eb291 --head 6606f07f45c6df47e206c93c3eb7347aca2e5a36
```

**Notes**

The guard correctly rejected a working-copy mutation made while the initial revision-pinned validation ran. No failure is waived. Final checks hold tracked inputs unchanged through completion.
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
pnpm spec:validate
```

**Notes**

Regenerated canonical history satisfies every 400-line/32768-byte limit; 7 validators, 4437 chapters and 208 requirements pass on frozen snapshot 4a4dd729e86d867df1c1caee7cb46e5fe2fed6f2.
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
JJ_STORE_CHECK_REVISION=kluosrto pnpm check:progress
```

**Notes**

180 tasks and 957 evidence records pass on frozen snapshot 4a4dd729e86d867df1c1caee7cb46e5fe2fed6f2.
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
pnpm spec:first --base 95d46e440198c4b2eb1fded32f8efee6160eb291 --head 4a4dd729e86d867df1c1caee7cb46e5fe2fed6f2
```

**Notes**

One protected file and one canonical chapter pass with all tracked inputs held unchanged. The same checks are repeated after recording these outputs.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Human visual review remains pending; no story review tags, thresholds,
  readiness limits, capture timeouts or assertions are changed.
- 18 strict visual policy failures remain after fresh capture. Existing fixture
  debug output includes random request IDs; previous evidence inspected
  representative AgentChatRuntime and WorkflowRun pairs. This slice does not
  mask IDs, change application/stories, lower thresholds or repeatedly overwrite
  references to conceal differences. The linked comparison page lists current
  non-identical outcomes.
- "113 of 1046 eligible stories remain outside user-selected available-only
  capture: 110 require the Vitest browser host, one exceeds the 15-second
  readiness window, and two repeatedly exceed the 30-second capture window.
  Exact IDs remain in E-VISUAL-BASELINES-MACOS-001/blocked-01.md and
  blocked-02.md. This task does not port them."
- Published npm version remains 0.1.0. PR 49 consolidates the staged baseline
  correction, dependency PRs 42-46 and generated Version Packages PR 48 into a
  0.1.1 candidate. The consumer uses the documented ignored patch archive with
  identical source/dist code. Merge and qualify the candidate through the
  existing release workflow, then refresh the registry dependency and retire the
  temporary override. No registry publication, tag or consumer push is
  performed.
- Inherited full interaction/accessibility lane had 63 failures; this
  artifact-only slice does not qualify it. Two inherited architecture limits
  remain (StoreStackWorkspaceApplication story 401/400, StoreStackWorkspace
  production 310/300). Existing Design Core registry release gap remains.
  Product/Rust/aggregate check, test, build and spec:check lanes are outside
  this scoped artifact/document verification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Visual Delta 0.1.0 plus inspected upstream patch archive; Storybook 10.5.7, Playwright 1.63.0, Chromium 153.0.8010.12, Svelte 5.57.1, native macOS ARM64 Darwin 27, Node 26.9.0, pnpm 10.32.1. Pre-existing packed Design Core 0.6.0 remains; tracked registry ranges remain Visual Delta ^0.1.0 and Design Core ^0.5.1. Upstream candidate 0.1.1 uses Vitest 5.0.5, Svelte 5.57.2, Testing Library React 16.3.3, Storybook Icons 2.2.0 and PostCSS 8.5.31; its source/dist code is identical to the consumer patch archive.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native Darwin ARM64 profile PNG baselines; complete checksum-verified actual/result sidecars, capture-profile schema 2 and result-sidecar schema 4. Runtime/font evidence enters actual and affected identities; Docker image/path contracts remain upstream-owned and unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-authorized upstream parity repair and regeneration of the previously selected 933 available-story baselines. Generic font measurement, worker staging and cache identity fixes belong upstream. This consumer slice changes only approved PNG artifacts, owning documentation and evidence. Temporary operational drivers are ignored and call the public CLI. No production/story implementation, assertion, review tag, shared capture configuration, manifest or lockfile change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the 18 strict differences separately; merge and qualify upstream PR 49 through the normal release workflow, consume published 0.1.1 and retire the temporary override. Story portability and inherited qualification remain independent.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-001 now requires native capture reuse, affected exclusions and
  fresh controls with complete measured host evidence. JST-GOV-002 source
  ownership remains intact: generic behavior is upstream; this consumer owns its
  baselines and integration documentation. No product/domain, Docker namespace,
  visual threshold or human approval contract changes."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.4 retains its status. Native cache parity and regenerated artifacts
  add focused evidence, independently of visual review, excluded standalone
  stories, inherited interaction/accessibility failures and registry
  publication.

```
<!-- /evidence-field -->

