---
schema: 1
id: E-CI-SUITES-FIX-001
---

# E-CI-SUITES-FIX-001

<!-- evidence-field:id markdown -->
## Id

E-CI-SUITES-FIX-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change plvvnoyr
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair the hosted 300-counted-line module gate by extracting artifact layout declarations without changing verification or coverage. The complete relevant CI tooling tests and exact hosted governance commands are required; local Rust and browser product suites are out of scope for this declaration-only refactor.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-artifact-layout.mjs
- scripts/ci-artifacts.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
- spec/src/current/progress/M30/history/M30/SUMMARY.nav.md
- spec/src/current/progress/M30/history/index.md
- spec/src/current/progress/M30/history/SUMMARY.nav.md
- spec/src/current/progress/M30/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-artifacts.test.mjs#artifact reuse requires successful exact
  producers, not a successful unrelated suite
- scripts/ci-artifacts.test.mjs#receipts reject wrong source, attempts, roles,
  missing payloads and unsafe paths
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
python3 /tmp/jj-store-protected-push.py
```

**Notes**

Pushed initial suite separation to main as d201d91df878cdc785ce47475b188073f1fe08f2. Administrator enforcement was restored immediately; the complete settings matched the saved snapshot.
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
gh workflow run ci-heavy.yml --repo stevejuma/jj-store --ref main -f ref=d201d91df878cdc785ce47475b188073f1fe08f2
```

**Notes**

Run 38724029386 and automatic standard run 38723994218 failed governance before compilation: scripts/ci-artifacts.mjs had 320 counted lines against the 300-line limit. The architecture check was omitted from the initial local verification. This was reproduced locally before extraction.
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

1918 modules checked after extraction.
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
pnpm check:ui-ownership
```
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
pnpm test:release
```

**Notes**

45 tests passed.
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
pnpm progress:history
```

**Notes**

Refreshed M30; restored 448 unrelated generated paths.
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
pnpm check:progress
```

**Notes**

Fresh evidence lacked integer failure counts; corrected record metadata.
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
pnpm check:progress
```

**Notes**

155 tasks and 815 evidence records.
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
pnpm spec:first
```

**Notes**

Unchanged-contract confirmation requires completed validation. Narrowed this record result to the locally qualified declaration extraction; hosted qualification gaps remain explicit in qualification_gaps and M30.
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
pnpm spec:validate --only governance,summary,fileLimits,verification && actionlint -oneline
```

**Notes**

Passed before adding this repair evidence; repeated after final evidence below.
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
pnpm check:progress && pnpm spec:first && pnpm spec:validate --only governance,summary,fileLimits,verification && actionlint -oneline
```

**Notes**

Final checks passed: 815 evidence records, 2 protected files/1 canonical chapter, 3884 chapters/198 requirements; actionlint clean.
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
pnpm exec prettier --check <all changed paths>
```

**Notes**

All changed paths satisfy Prettier.
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

<!-- evidence-field:qualification_gaps list -->
## Qualification gaps

- Hosted full qualification and all six standalone measurements remain pending.
- The pre-existing Kingfisher identity mismatch from run 38716420649 remains
  unresolved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v24.10.0, pnpm 10.32.1, actionlint 1.7.12.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Artifact receipt format 1 and payload definitions are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete local governance validation, push with immediate protection restoration, and dispatch full qualification at the signed pushed SHA.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 artifact admission, complete release qualification and the
  brief administrator exception are unchanged. JST-GOV-003/004/011 evidence and
  milestone duties remain enforced. Only static layout declaration ownership
  moved.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; the hosted module-limit failure, unrun heavy
  suites and existing product regression do not establish qualification.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


