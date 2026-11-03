---
schema: 1
id: E-CI-EVIDENCE-001
---

# E-CI-EVIDENCE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-EVIDENCE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-packages.mjs
- scripts/ci-tests.mjs
- scripts/ci-evidence.mjs
- scripts/ci-evidence.test.mjs
- .prettierignore
- docs/reference/ci-validation-plan-status-2026-11-02.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-evidence.test.mjs#targeted backend gate binds both shards to the
  selected heavy packages
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
node --test --test-name-pattern='targeted backend gate binds both shards' scripts/ci-evidence.test.mjs
```

**Notes**

The exact targeted evidence regression passed, including rejection of a stale
ordinary-profile JUnit path and admission of the exact heavy-profile report.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown
```

```shell
node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

82 of 87 tests passed. Five metadata fixtures could not spawn `cargo` because
the machine did not yet have the repository-pinned Rust toolchain; no assertion
failed.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

All 87 CI, validation, release-policy and automation regressions passed locally
with the repository-pinned Rust 1.99.0 toolchain.
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
npx --yes pnpm@10.32.1 exec prettier --check docs/reference/ci-validation-plan-2026-11-02.md scripts/ci-packages.mjs scripts/ci-tests.mjs scripts/ci-evidence.mjs scripts/ci-evidence.test.mjs docs/reference/ci-validation-plan-status-2026-11-02.md
```

**Notes**

This initial formatting repair passed, but inspection showed that it damaged
the imported plan's Markdown structure. That formatting result was discarded;
Run 8 verifies the final byte-preserving exclusion instead.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
```

**Notes**

Governance, summary, file-limit and verification validators passed across 4582
chapters and 211 requirements.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
```

**Notes**

Spec-first passed: six protected files mapped to the two owning canonical
chapters.
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
node scripts/check-progress.mjs
```

**Notes**

Progress bookkeeping passed for 186 tasks and 1008 evidence records. M30
remains In progress.
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
npx --yes pnpm@10.32.1 exec prettier --check .
```

**Notes**

The exact full formatting lane passed. The imported CI plan remains verbatim
under the same reference-plan exclusion policy as the earlier native plan.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The hosted required gate and this repair's integrated PR attempt.
- Fresh canonical-main scheduled qualification.
- Existing UI lifecycle failures owned by later plan slices.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

CI selection schema 1 and evidence schema 1 are unchanged. Nextest profile
paths remain `target/nextest/ci/junit.xml` and
`target/nextest/heavy/junit-heavy.xml`.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused CI evidence-path repair. Test selection, shard partitions, package
closures and complete scheduled/release qualification are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run the hosted required gate and retain the remaining product/harness failures
for their owning slices.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 and JST-GOV-003/007 now require profile-exact JUnit
  evidence and removal of the current report before execution. No public API,
  persisted format, package selection or test identity changed.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This repair restores the targeted evidence
  gate but does not complete hosted, scheduled, UI lifecycle or release
  qualification.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
