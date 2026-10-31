---
schema: 1
id: E-CONVERSATIONS-NPM-002
---

# E-CONVERSATIONS-NPM-002

<!-- evidence-field:id markdown -->
## Id

E-CONVERSATIONS-NPM-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-31
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-conversations-producer.mjs
- scripts/ci-ui-producer.mjs
- scripts/ci-artifacts.mjs
- scripts/ci-ui.test.mjs
- scripts/ci-artifacts.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-ui.test.mjs#Conversations qualification admits canonical main or
  scoped-tag UI producers
- scripts/ci-artifacts.test.mjs#Conversations qualification admits canonical
  main or scoped-tag heavy producers
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
gh workflow run npm-publish-jj-conversations.yml --repo stevejuma/jj-store --ref main -f mode=attest
```

**Notes**

PR #25 passed protected standard run 41151767892 and Pages 41151767696, then merged as b1c1cb5fc01bc0b71727f512c52f5214559ecbe0. Its first complete attest qualification, run 41154212641, rejects all seven UI jobs before browser execution with Unapproved UI producer workflow. The new top-level npm workflow was missing from the existing canonical producer admission. No publishable package or provenance bundle is produced.
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
node --test --test-name-pattern 'Conversations qualification admits' scripts/ci-ui.test.mjs scripts/ci-artifacts.test.mjs
```

**Notes**

Before the production repair, the exact new UI and heavy producer cases both reject the legitimate canonical npm qualification context.
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
node --test --test-name-pattern 'Conversations qualification admits' scripts/ci-ui.test.mjs scripts/ci-artifacts.test.mjs
```

**Notes**

Both cases pass after narrow canonical-main/manual or package-scoped-tag admission. Existing source/repository/run/attempt, same-run UI, successful standard gate, producer uniqueness/success and payload checksum checks remain intact. Tests additionally reject foreign workflows, feature dispatches, unrelated tags, wrong attempts/runs and failed gates.
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
pnpm release:check
```

**Notes**

All 69 release/CI tests pass in Docker, including both new canonical producer regressions and all existing admission/receipt/security cases. All 28 package metadata checks pass; coordinated Cargo versions stay 0.1.0.
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
pnpm spec:validate
pnpm spec:first --base b1c1cb5fc01bc0b71727f512c52f5214559ecbe0
pnpm check:progress
pnpm spec:build
```

**Notes**

Eight static validators pass over 4,544 chapters and 211 requirements. Spec-first maps five protected source files to both updated canonical chapters. The progress ledger passes 186 tasks and 996 evidence records, and mdBook builds. The governance page remains exactly 400 physical lines. All source formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Merge the focused repair after current-base protected checks.
- Rerun complete immutable-source qualification, then actual npm
  bytes/provenance/consumer verification and the scoped GitHub release.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

@lapismd/jj-conversations 0.1.0; Spec Validator 0.2.0; producer receipt format 1; Node 24 local and Node 22 Docker; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Producer and payload receipt encodings, package bytes, wire/native storage formats and Cargo versions are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Change xwwpvspq admits the existing complete qualification graph when called by the newly merged Conversations npm workflow. This repairs its top-level workflow context; it does not admit unknown workflows, bypass any gate, reuse a UI result as heavy qualification, or publish any Rust/application package.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge the focused repair after current-base protected checks, rerun the entire main attest qualification and complete actual npm publication.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 and the governance map now explicitly bind the new
  canonical npm qualification producer context while preserving complete
  qualification, artifact identity and every existing admission check.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. The first nested qualification failure is
  retained, and neither source PR success nor the focused producer tests qualify
  a public release.

```
<!-- /evidence-field -->

