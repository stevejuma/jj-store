---
schema: 1
id: E-CONFIRMATION-CHAPTERS-001
---

# E-CONFIRMATION-CHAPTERS-001

<!-- evidence-field:id markdown -->
## Id

E-CONFIRMATION-CHAPTERS-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-22
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/spec/change-confirmations.mjs
- scripts/spec/ci-revision-regression.test.mjs
- spec/src/current/governance/24-unchanged-contract-chapters.md
- spec/src/current/governance/index.md
- spec/src/current/governance/SUMMARY.nav.md
- spec/src/current/progress/Bootstrap/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/spec/ci-revision-regression.test.mjs#unchanged confirmations cover
  mapped canonical requirement references only
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
pnpm exec spec-validator first --base @-- --head @-
```

**Notes**

Before the provider repair, isolated history shutdown change E-HISTORY-CLOSE-001 failed its mapped architecture/26 chapter gate. Its qualified unchanged confirmation named JST-ARCH-007, but lookup returned only architecture/03-requirements.md. No mapped topic was changed to work around this defect.
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
node --test --test-name-pattern='unchanged confirmations cover mapped canonical requirement references only' scripts/spec/ci-revision-regression.test.mjs
```

**Notes**

Before source repair, the canonical detail chapter was absent from confirmation ownership.
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
node --test --test-name-pattern='unchanged confirmations cover mapped canonical requirement references only' scripts/spec/ci-revision-regression.test.mjs
```

**Notes**

After repair, exact canonical references satisfy their configured mapping. Navigation, background documents, different IDs and partial ID matches cannot satisfy it. Both JJ and Git revision readers are covered.
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
pnpm test:spec
```

**Notes**

All 29 specification-tooling tests pass, including all-parent/range freshness, immutable evidence and archives, revision pinning and completion gates. This full relevant suite is required because the confirmation provider is shared governance infrastructure.
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
pnpm exec spec-validator first --base @-- --head @-
```

**Notes**

With the repaired provider, the isolated history shutdown change passes its original mapped ownership gate. No owning-topic mapping or overlapping user documentation was modified.
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
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

The isolated history shutdown change passes 180 tasks / 951 evidence records. Unrelated working-copy missing evidence errors remain in the four original user documents.
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
pnpm spec:validate --only fileLimits,summary
```

**Notes**

Both document validators pass; bounded authored chapters and complete navigation.
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
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

Isolated committed confirmation-provider slice: 180 tasks and 952 evidence records.
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
pnpm exec spec-validator first --base @-- --head @-
```

**Notes**

Isolated committed provider slice: two protected files, one owning canonical chapter.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check scripts/spec/change-confirmations.mjs scripts/spec/ci-revision-regression.test.mjs spec/src/current/governance/24-unchanged-contract-chapters.md spec/src/current/governance/index.md spec/src/current/governance/SUMMARY.nav.md spec/src/current/progress/Bootstrap/index.md
```
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

Node.js 24.10.0; pnpm 10.32.1; @lapismd/spec-validator registry dependency unchanged; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No evidence schema or product formats changed. Existing requirement IDs resolve to additional canonical pages with exact references.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared repository-owned confirmation provider only. It retains known definitions for ledger validation and collects canonical chapter references from the same revision-pinned source. Excludes index/SUMMARY pages and noncanonical material. Original production-to-topic mappings, fresh complete coverage, passing evidence and immutable history remain enforced. Revision is the enclosing Jujutsu change. Product Rust, UI, release and provider lanes are out of scope; source lifecycle and cold app evidence remain E-HISTORY-CLOSE-001 and the JJ Desktop ledger.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep original product qualification gates; use mapped canonical confirmation lookup for unchanged contracts.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Canonical governance/24 specifies exact referenced-chapter resolution
  under JST-GOV-002/011, preserving all acceptance, freshness and ownership
  requirements. This fixes general split-definition/detail-topic handling
  without weakening or editing mappings.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Bootstrap remains Done. Its checkpoint records this tooling repair; M33
  and JJ Desktop provider/distribution qualification stay separate.

```
<!-- /evidence-field -->

