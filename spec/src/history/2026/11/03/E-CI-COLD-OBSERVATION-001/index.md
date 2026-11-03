---
schema: 1
id: E-CI-COLD-OBSERVATION-001
---

# E-CI-COLD-OBSERVATION-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-OBSERVATION-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-ARCH-005
- JST-GOV-003
- JST-GOV-007
- JST-UI-005
- JST-UI-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/real/event-freshness.spec.ts
- web/tests/real/native-repository.spec.ts
- spec/src/history/2026/11/03/E-CI-COLD-OBSERVATION-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/event-freshness.spec.ts#cold native and filesystem views stay idle through real five-minute renewal
- web/tests/real/native-repository.spec.ts#browser mutation previews and publishes one native request
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
gh workflow run ci-heavy.yml --ref main -f ref=deb40f0c
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/126033416473/logs
```

**Notes**

Run 41669392226 was dispatch-only failure evidence. `actions/checkout` could not
resolve the abbreviated SHA as a branch, tag or full object ID, so every product
lane was skipped. It is not qualification evidence. The workflow was
redispatched with the full merged SHA.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
gh workflow run ci-heavy.yml --ref main -f ref=deb40f0c96b78badac5fcecf1c2e53db7babe50d
gh run view 41671278113 --json headSha,status,conclusion,url,jobs
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/126049803842/logs
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/126049803608/logs
gh run download 41671278113 --name qualification-results-cold-cold-0-1
gh run download 41671278113 --name qualification-results-cold-cold-1-1
```

**Notes**

Complete qualification ran 52 jobs at exact merged SHA
`deb40f0c96b78badac5fcecf1c2e53db7babe50d`. Every root lane except cold
browser shards 1/4 and 2/4 passed; the complete-gate failure was aggregate.
Shard 1 exhausted Playwright's default five-second poll while the native page
still drained initial data reads. Shard 2 retained a preview response handle,
then navigation discarded its body before `response.json()` read it. Traces
were inspected under `/tmp/jj-store-qualification-41671278113.uf2MXG`.
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
npx --yes pnpm@10.32.1 web:check
```

**Notes**

The affected UI package passed Svelte and TypeScript checking with zero errors
and zero warnings. Report:
`.local/validation/runs/jj-store-validation-15a1d1a6-7faf-4f05-af28-0acc2942aba7`.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=native-repository.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

No Playwright case ran. The cold reset reached built UI and documentation setup;
lightningcss emitted existing `:global` warnings, then the documentation build
stopped at `spawnSync uv ENOENT` inside the validation environment. The selected
PR cold shards remain the acceptance proof for both files.
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
npx --yes pnpm@10.32.1 exec prettier --check web/tests/real/event-freshness.spec.ts web/tests/real/native-repository.spec.ts spec/src/history/2026/11/03/E-CI-COLD-OBSERVATION-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

Formatting, four selected document validators, spec-first and progress passed.
Spec-first accepted both protected cold tests through this qualified
unchanged-contract confirmation.
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

Node.js 26.8.2 locally and 22.23.4 remotely; pnpm 10.32.1; Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, wire, generated asset, cache or persisted format
changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cold event-idle and mutation-preview observation only. Initial data reads may
drain for up to the existing 30-second cold readiness bound. Preview JSON is
captured through the established route helper before navigation can discard the
response body. Production behavior and protected assertions are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require both selected PR cold shards, land the test-only repair, then rerun
complete qualification on merged `main` using the full commit SHA.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-API-001 and JST-UI-005/JST-UI-008 retain the same event-driven idle,
  exact preview and committed mutation contracts. The tests only use explicit
  bounded readiness and read response JSON before route navigation.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. All non-cold roots passed complete merged
  qualification; the two repaired PR cold gates and a fresh merged complete run
  remain required. Independent visual approval and deferred release gaps retain
  their existing status.
```
<!-- /evidence-field -->
