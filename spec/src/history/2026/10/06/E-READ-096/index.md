---
schema: 1
id: E-READ-096
---

# E-READ-096

<!-- evidence-field:id markdown -->
## Id

E-READ-096
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-GOV-003
- JST-GOV-005
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Integrate progressive Blame and highlighting into main with Stacks preserved
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu merge change wzwzmqwq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main contained committed Stacks discovery 555e25ad with active otvxrvnt working changes; verified Blame 0c1e775d and highlighting f554f5b1 were on the retained side branch and preview only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Integrate both committed feature histories, resolving shared ledger/documentation conflicts. Main became clean after Stacks was committed; retain its exact source/test bytes and move its empty working change above the verified merge. No feature redesign, service restart or publication.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Same pinned libraries and local validation dependencies as E-READ-094/095 and the Stacks parent. pnpm 10.32.1; Vitest 4.1.11; Rust 1.98.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Union of already verified additive Blame and native Stacks contracts. No new transport or native storage format.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/spec-governance.md
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
jj --no-pager st (main and retained workspace); jj new qvvzswvy sktnmmmv -m "Integrate progressive Blame with native Stacks"
```

**Notes**

User explicitly requested main integration after the active-overlap warning. Prepared a merge above both committed parents in retained bounded-reads. Production files and generated contracts merged without conflict; resolved only ledger and documentation fingerprint conflicts.
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
python3 structural ledger merge; pnpm docs:review local/services/http/application/contracts with integration explanation; pnpm docs:generate; pnpm docs:check
```

**Notes**

Preserved every ledger entry from both parents with no common-ID disagreement; retained Stacks task table plus E-READ-094/095. Combined source fingerprints reviewed. blame-main-doc-check.log: 139-file closure, valid links/anchors and 9 diagrams in light/dark.
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
cargo check --locked -p jj-store-http
```

**Notes**

blame-main-rust.log: production JJ/service/HTTP boundary compiles; no handwritten Rust conflict resolution was required.
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
pnpm web:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

blame-main-web.log: 0 errors/0 warnings. Architecture: 1157 modules. Ownership validator passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreBlame.stories.svelte
```

**Notes**

blame-main-stories.log: cold Chromium 10/10, including source colors, text preservation, route switching, pending origins and reuse.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/http-blame.test.ts web/tests/vite/blame-query.test.ts web/tests/vite/stack-workspace-topology-query.test.ts
```

**Notes**

blame-main-query-tests.log: 10/10 across Blame and Stacks direct consumers.
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
pnpm exec tsx --test web/tests/repository-source-tokenizer.test.ts web/tests/store-blame.test.ts
```

**Notes**

blame-main-model-tests.log: 8/8 for token offsets, source languages, line ranges, links, routes and admission explanation.
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
pnpm contracts:check
```

**Notes**

Intentionally stopped with SIGTERM/exit143 during the unrelated api_snapshot capture fixture after generated-schema comparison and affected native/local production captures had succeeded. The standard command also pulls in capture/import/conversion/settings fixtures outside this merge; no full contract-capture pass is claimed. Stopped only the owned command and its seven child processes.
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
pnpm contracts:generate; pnpm test:contracts
```

**Notes**

Final generated schema/TypeScript remain identical to the automatically merged contracts. blame-main-contract-unit.log:7/7 contract tests and contract TypeScript check pass.
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
jj rebase -r @ -d qvvzswvy -d otvxrvnt (temporary workspace); structural ledger merge; pnpm docs:review application; pnpm docs:check
```

**Notes**

During verification main Stacks work became committed43f6e70b and main became clean. Updated merge to include that latest parent; only ledger/fingerprint conflicts needed resolution. All609 entries from each parent retained without alteration. Final documentation closure passes.
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
pnpm web:check; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-membership.test.ts web/tests/vite/blame-query.test.ts
```

**Notes**

After incorporating committed direct Stacks actions:0 type errors/warnings and9/9 query/membership tests. No Blame production logic or Rust changed during this second merge.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base <committed-main-parent> --head <integration>
```

**Notes**

The temporary JJ workspace has no colocated .git. Explicit range mode invoked Git without its object database. Corrected by setting GIT_DIR from jj git root; no source defect.
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
GIT_DIR=$(jj git root) pnpm spec:first --base <committed-main-parent> --head <integration>; pnpm spec:validate; pnpm check:progress; targeted Prettier
```

**Notes**

Explicit-range spec-first passes for48 protected files/10 canonical chapters. Validation:5 validators,23 chapters,173 requirements. Progress:142 tasks,bookkeeping only. Final repeat after this evidence append.
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
Snapshot main and hash Stacks source/test files; jj rebase -s klusvmur -d wzwzmqwq (main); jj --no-pager st; ancestry and SHA-256 preservation checks
```

**Notes**

Main clean working change moved above the combined merge. Both Blame feature commits and the latest Stacks commit are ancestors of main. All12 Stacks production/test files match their recorded pre-merge bytes. Main is clean; bounded-reads and codex-preview retained. .local/blame-main-landing-before.json records the checkpoint. No services restarted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/repository-source-tokenizer.test.ts#multiline comments retain their
  type without crossing source line boundaries
- web/tests/store-blame.test.ts#blame links keep operation and path but clear
  incompatible context
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Focused combined Rust HTTP compile, native/local production browse captures, generated-contract parity and contract unit/type checks, UI types, Blame and Stacks query/model tests, cold Blame stories and governance/documentation checks. Broader unrelated contract capture execution was stopped and is not claimed as a pass.
<!-- /evidence-field -->


<!-- evidence-relations -->
## Resolution and supersession

The original result remains **pass**. Milestone gates select their own evidence.

- This entry resolves [E-READ-094](../E-READ-094/index.md). Main integration completed the previously recorded landing blockers.
- This entry resolves [E-READ-095](../E-READ-095/index.md). Main integration completed the previously recorded landing blockers.
<!-- /evidence-relations -->
<!-- generated-navigation -->
## Pages

- [E-READ-096 details 2](details-02.md)
