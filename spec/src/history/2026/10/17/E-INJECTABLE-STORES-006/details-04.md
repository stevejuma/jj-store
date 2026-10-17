---
schema: 1
id: E-INJECTABLE-STORES-006
---

# E-INJECTABLE-STORES-006 details 4

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
node --test scripts/governance.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

39passed with tracked files frozen throughout revision-pinned metadata validation; original historical evidence remains intact.
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
pnpm check:progress
```

**Notes**

162tasks/856evidence records, with all five M32 tasks reviewed as Done within the agreed source-library scope.
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
pnpm spec:first
```

**Notes**

23protected files/4owning canonical chapters passed.
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

7validators/4124chapters/206requirements passed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <all changed authored Markdown/JavaScript/JSON paths>
```

**Notes**

Final format audit detected5task-history tables rewritten by the draft evidence recorder after initial formatting; no product test failed. All other authored paths and immutable history guards passed.
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
pnpm exec prettier --write spec/src/current/progress/M32/history/M32/index.md spec/src/current/progress/M32/history/M32.1/index.md spec/src/current/progress/M32/history/M32.2/index.md spec/src/current/progress/M32/history/M32.3/index.md spec/src/current/progress/M32/history/M32.4/index.md
```

**Notes**

Formatted the5rewritten task-history tables. Finalization formats all changed authored paths after recording evidence so the recorder cannot leave table alignment drift.
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

macOS ARM64; rustc1.99.0(b940084d7,2026-09-28), JJ0.45.1, Git2.55.0. Workspace libraries0.1.0; stock jj-lib exactly0.45.1. Independent workspace has its own locked371-package resolution; main Cargo.lock is unchanged in this slice.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authoritative formats, repository type markers, API schemas, CLI commands or receipt identities changed. Generated manual references catch up with already-committed repository deletion routes/outcomes in the unchanged OpenAPI schema. Existing Git ref/canonical records, native preparation, WAL/manifest and original receipt formats remain.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Independent private source consumer demonstrates all five factory-backed stores, one-store-only substitution, stable original receipts, Git GC/reopen and preserved stock-JJ source operation heads. Optional native example uses MemoryStore, exact serialized preparation and checkpoint/reopen; real cold embedded RustFS proof remains separately recorded in E005. Source SDK guide stays under docs; concise READMEs cover every maintained crate root. Shared build-storage inventory adds one independently managed Cargo root, requiring its complete16-test tooling suite. Documentation tooling/review changes require both direct14-test suites and the complete strict manual closure. Root aggregate, unrelated UI/catalog, release/platform qualification and publication are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified source-only embedding qualification. Retained Vite development stack is running; WorkflowStore injection and publication remain separate work.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-006/JST-LOCAL-001/003/JST-PUB-004/005/JST-JJ-001/002/003/004/005
  retain their qualified store construction, publication authority, canonical
  identities and receipt behavior. Owning
  architecture/JJ/documentation/governance chapters describe independent
  executable acceptance and the managed Cargo root. JST-DOC-001/002/003 source
  coverage now includes core/Git/native; existing manuals and stale references
  are reviewed without changing application behavior.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M32 and M32.1-.4 are Done for the agreed source-library scope. E001-E004
  qualify shared engines, five independent factories, both providers and
  original receipt/fault semantics; E005 qualifies migrated app consumers and
  real cold embedded RustFS; E006 qualifies the independent
  stock-JJ/plain-Git/native/combined host, one-store substitution, graphs, docs
  and all31README roots. Retained Vite fixture restored. WorkflowStore, new
  platforms/topologies and package/release publication remain outside scope.

```
<!-- /evidence-field -->

