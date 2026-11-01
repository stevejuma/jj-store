---
schema: 1
id: E-DEPENDENCY-AUTOMATION-002
---

# E-DEPENDENCY-AUTOMATION-002

<!-- evidence-field:id markdown -->
## Id

E-DEPENDENCY-AUTOMATION-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-ARCH-005
- JST-DOC-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing dependency automation repair change
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/technical/conversations.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/01/E-DEPENDENCY-AUTOMATION-002/SUMMARY.nav.md
- spec/src/history/2026/11/01/E-DEPENDENCY-AUTOMATION-002/index.md
- spec/src/history/2026/11/01/SUMMARY.nav.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/governance/01-specification-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/automation-governance.test.mjs#bot PRs skip only change-specific
  governance
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
node --test scripts/automation-governance.test.mjs
```

**Notes**

Five focused tests pass on the current candidate.
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
node scripts/docs.mjs review conversation-client <review reason>
```

**Notes**

Updated the source fingerprint after explaining the separately retryable post-verification notification in the owning technical page.
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
node scripts/check-progress.mjs
```

**Notes**

Initial refresh correctly rejected three newly edited document/ledger paths until they were added to this fresh evidence record.
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
node /Users/stevejuma/.npm/_npx/88c52e56b926fddf/node_modules/@lapismd/spec-validator/dist/cli.js validate
```

**Notes**

Current candidate passes all 8 static validators across 4561 chapters and 211 requirements. Both changed canonical pages remain at 400 physical lines. The original failed progress probe above is retained.
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
JJ_STORE_CHECK_REVISION=automation/first-party-dependencies-20261101 node scripts/check-progress.mjs
```

**Notes**

186 tasks and 1002 evidence records pass on the amended unmerged candidate. The earlier failed probe remains recorded.
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
spec-validator first --base ba23a872d6541fad431959e2d0f713c81c20aa77 --head automation/first-party-dependencies-20261101
```

**Notes**

Exact PR range passes: 23 protected files and three mapped canonical chapters. The existing main-branch evidence is unchanged.
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
spec-validator build && python3 scripts/spec/book-audit.py
```

**Notes**

Rendered links and anchors pass for 3285 HTML files and 125229 local references. The change-map anchor is retained.
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
node node_modules/prettier/bin/prettier.cjs --check spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/governance/01-specification-governance.md spec/src/history/2026/11/01/E-DEPENDENCY-AUTOMATION-002/index.md
```

**Notes**

All three repaired Markdown files pass the repository formatter.
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

Node 26.9.0 for focused local tools; existing pinned pnpm/Rust unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, WAL, manifest, RPC or npm package-format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused governance/source-documentation repair: preserve the change-map anchor and the 400-line limits, document and review the post-verification notification. Five governance tests and static specification validation pass; hosted merge CI and M30 release qualification remain separate gates. The governance summary is aligned with the trusted-author exemption already defined in public distribution.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge only after normal hosted CI passes. M30 platform/release qualification and App activation remain pending.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: Trusted PR-author exemptions and all functional/publication requirements
  remain defined in public distribution.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; this source check does not promote
  native/platform/publication acceptance.

```
<!-- /evidence-field -->

