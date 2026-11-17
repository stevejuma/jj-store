---
schema: 1
id: E-SHARED-DEPENDENCY-ADOPTION-001
---

# E-SHARED-DEPENDENCY-ADOPTION-001

<!-- evidence-field:id markdown -->
## Id

E-SHARED-DEPENDENCY-ADOPTION-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-17
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/shared-dependency-adoption.md
- .github/dependency-automation/canonical-intake.json
- .github/versioning-runtime.json
- .github/workflows/ci.yml
- .github/workflows/dependency-completed.yml
- .github/workflows/release.yml
- package.json
- pnpm-lock.yaml
- scripts/ci-workflows.test.mjs
- scripts/dependency-automation/check-workspace-intake.mjs
- scripts/dependency-automation/completion-workflow.test.mjs
- scripts/dependency-automation/version-runtime.test.mjs
- scripts/dependency-automation/workspace-catalog.mjs
- scripts/dependency-automation/workspace-intake.mjs
- scripts/dependency-automation/workspace-intake.test.mjs
- scripts/fixtures/jj-store-registry-intake.json.gz
- scripts/validation/lanes.json
- scripts/version-pr-auth.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/32-version-pr-authentication.md
- spec/src/current/governance/01-specification-governance.md
- spec/src/current/progress/M30/history/M30/index.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/17/E-SHARED-DEPENDENCY-ADOPTION-001/SUMMARY.nav.md
- spec/src/history/2026/11/17/E-SHARED-DEPENDENCY-ADOPTION-001/index.md
- spec/src/history/2026/11/17/SUMMARY.nav.md
- spec/src/history/2026/11/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dependency-automation/workspace-intake.test.mjs#source-owned prose
  formatter may wrap only the exact digest paragraph
- scripts/version-pr-auth.test.mjs#false intent retains existing workflow-token
  orchestration without the App
<!-- /evidence-field -->

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
Normal static-spec4 validators, actual main-range spec-first, progress ledger and changed-file formatter
```

**Notes**

4702 chapters/212requirements validate; source-first18protectedfiles/3canonicalchapters; progress187tasks/1065records; formatterpasses. Historical immutable payloads remain untouched.
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
pnpm exec spec-validator doctor --json; pnpm spec:search 'JST governance release dependency automation'
```

**Notes**

Installed Node-only validator0.2.1; doctor passes with existing custom-alias warning, actual QMD current-spec search returns owning canonical pages. Initial literal -- separator was rejected by the CLI; correct package-command invocation then passed.
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
node --test scripts/dependency-automation/*.test.mjs scripts/version-pr-auth.test.mjs
```

**Notes**

26/26 after exact generated-digest formatting case; initial25/25 focused cases were green, actual source guard then failed1 because proseWrapalways split the digest paragraph. Its exact regression now passes while changed digests remain rejected.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:dependency-intake
```

**Notes**

Normal source guard independently derives all4 external registry cohorts from actual manifests/lock and preserves both public contracts plus the owned Conversations link.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:release
```

**Notes**

94/94 after93/1 identified missing ownership inventory for the new read-only completion workflow. Exact original inventory target then1/1; complete release/CI/auth suite94/94.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:spec; pnpm check:architecture; actionlint
```

**Notes**

Specification infrastructure31/31, production/script architecture including new300-line budgets and pinned workflow syntax pass.
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
Disposable complete unchanged pnpm release:version under official Node22.23.5/pnpm10.32.1 with no Deno/GitHub credentials
```

**Notes**

Actual Changesets2.31.1 plus source metadata helper, lock update and formatter consume exact historical desktop-browser-development note bytes from8ada0ffc38b7 (SHA256487b612e3a8934e080174722dfac9d8a6051725c77ecb6179a25d6d70b70fd69). The note is uncommitted in this disposable fixture, exercising the unchanged default changelog no-commit branch. Root/native packages and helper synchronize0.2.0→0.2.1; public History0.2.0/Conversations0.1.0 remain unchanged. Source facts and5 exact executable/config bindings pass before/after; intenttrue→false. GitHub enrichment branch is not exercised.
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
Disposable no-note whole pnpm release:version plus actual shared intent guard
```

**Notes**

After the prepared fixture consumes the historical note, complete source macro exits0 with zero changed files and actual false intent. This CLI2 source shape needs no generated empty note flag.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Reviewed053 prepareCanonicalFiles on exact controlled released SDK0.2.0→0.2.1 Git fixture
```

**Notes**

Held: stale or manually changed workspace facts. Source proseWrapalways produces an exact2-line digest paragraph while the shared v2 baseline comparator expects1 line. Parent owns the shared bounded-format fix; the actual replay request/source/policy hashes are retained outside tracked source.
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
pnpm audit --json against30d632e3 and source candidate; pnpm audit --prod --json
```

**Notes**

Comparison confirms the same3 pre-existing HIGH picomatch development paths; production0. This is evidence of unchanged findings, not a clean normal audit. Source advisory exceptions are not enabled; owning issue91 tracks repair.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Reviewed shared v2 formatter fix and actual candidate/repeat replay
- Exact-source hosted PR/main checks and successful trusted completion callbacks
- Parent-owned central activation/admission proof; the user's accepted existing
  main publisher failures remain separately reported
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Source private root0.2.0, public History0.2.0, Conversations0.1.0 unchanged. External Core0.13.5, Mira0.2.0, validator0.2.1, VisualDelta0.1.5; new source guard directly declares already-transitive semver7.8.6, deduplicating Changesets transitive semver7.8.5→7.8.6. Complete real recipe qualification uses this new actual graph. Actual Node22.23.5/pnpm10.32.1/CLI2.31.1/Python3.14.8 whole-recipe proof; no Deno.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native JJ/storage/object/WAL/manifest/receipt/public-API or visual baseline changes. V2 source-owned registry policy and managed Markdown facts, exact source recipe qualification and read-only completion caller are new automation metadata.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Four EXTERNAL registry inputs across three explicit root/public manifests; source-owned @lapismd/jj-conversations ^0.1.0→link:packages/jj-conversations is held intact as an owned output, never fabricated as registry intake. All actual source/native/registry/publisher gates and npm ranges remain. Normal quality, canonical aggregate and unconditional CI governance audit source facts outside bot spec exemptions. False intent avoids App/writer acquisition while retaining existing workflow-token release orchestration. The callback pins implementation+policy053ac8be, observes only reviewed workflow IDs/names and PR/push/manual original events, and excludes workflow-call-only completions. Hosted read probe44192021816 verified personal-account owner mapping; only secret/variable NAMES were inspected. Main at30d632e3 has successful PR44143145736/Pages44143145408/orchestration44143145274/nightly44178406761. User explicitly asked to ignore existing main image44149860564 and qualified-release44143383660 failures; no repair or suppression is included. Full product/Rust/browser suites are not run locally under targeted policy: this tooling slice preserves product code and separately exercises the whole actual version macro. Existing hosted gates remain mandatory; no fixture pass claims native/platform/publication acceptance. Global evidence-new navigation sweep was restored to avoid unrelated historical derived churn; only this record, required ancestors and M30 history links are maintained.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 gains bounded shared automation and exact writer-intent
  admission, with source-owned canonical facts before credentials;
  JST-GOV-002/011 retain strict evidence/source governance and all original
  runtime/security/release boundaries.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress, including all pending platform, heavy
  qualification and actual publication gates. Source automation proof and the
  separately user-accepted pre-existing main failures do not promote completion.

```
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge only after actual source gates pass, replay the shared formatter repair against the exact real controlled candidate, then parent completes main/callback/central activation evidence. Track upstream advisory fixes in https://github.com/stevejuma/jj-store/issues/91 without new waivers.
<!-- /evidence-field -->

