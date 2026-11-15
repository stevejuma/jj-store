---
schema: 1
id: E-VAULT-HISTORY-FRONTIER-002
---

# E-VAULT-HISTORY-FRONTIER-002

<!-- evidence-field:id markdown -->
## Id

E-VAULT-HISTORY-FRONTIER-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/history/2026/11/15/SUMMARY.nav.md
- spec/src/history/2026/11/15/index.md
- spec/src/current/progress/M34/index.md
- spec/src/current/verification/local-03.md
- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/history/operation_frontier.rs
- crates/jj-store-jj/src/vault_workspace/history/publication_tests.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/vault_workspace/history/publication_tests.rs#immutable_reader_accepts_ancestral_heads_without_changing_the_published_frontier
- crates/jj-store-jj/tests/vault_publication.rs#initialized_transfer_does_not_take_the_capture_lease
- crates/jj-store-jj/tests/vault_publication.rs#retained_publication_does_not_take_the_capture_lease
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --lib vault_workspace::history::publication_tests::immutable_reader_accepts_ancestral_heads_without_changing_the_published_frontier --locked -- --exact
```

**Notes**

1/1, unchanged native regression; report jj-store-validation-1284e46a-ca6c-4e1f-ab94-009656841da8.
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --lib vault_workspace --locked
```

**Notes**

4/4 native unit cases; report jj-store-validation-7262039a-6624-490a-a029-189399c5d7a4.
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
node scripts/validation.mjs run validation:rust clippy -p jj-store-jj -p jj-vault -p jj-history-service --all-targets --features jj-store-jj/vault-backup --locked -- -D warnings
```

**Notes**

Affected three-crate strict Clippy passes; report jj-store-validation-55374edf-2f11-4e5e-83f8-42ee4845c69d.
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
node scripts/validation.mjs run check:progress
```

**Notes**

Fresh receipt initially had no recorded runs; stopped with missing actual runs, report jj-store-validation-599d5f34-17d5-49cb-9395-ce54cfda622a. Actual results now recorded before rerun.
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --test vault_publication --locked
```

**Notes**

Correct integration target passes 4/4, including capture-lease independence; report jj-store-validation-df50ad9a-29b1-451a-a28e-63b841bd3fd5.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run test:storybook catalog/components/store-application/ApplicationMenu.stories.svelte -t NativeChangeDraftSurvivesMobileToDesktopRelocation
```

**Notes**

Actual serialized execution fails 0/1 with the same toHaveFocus regression on stock published Core 0.13.3; six unselected family cases are excluded by the exact test filter. Report jj-store-validation-ffd67554-bb8c-45fe-b498-8af69e1a00f6. Source repair is merged in Design Core #83; version PR #84 has all 11 checks green but publication remains separately unapproved. No assertion or CI gate is waived.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run check:progress
```

**Notes**

Receipt rerun found three missing test paths, three invalid bare test-name references and unsupported pending run states; no product test failed. Report jj-store-validation-2332210a-5472-4e56-8e35-281a2cdb168a. Named references are now explicit source-file#function; attempts collecting no tests are retained separately with no qualification.
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
node scripts/validation.mjs run check:progress
```

**Notes**

187 tasks and 1060 evidence records validate after schema correction; report jj-store-validation-9f44815e-7b0c-4052-8b60-cea0c367c2b0.
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
node scripts/validation.mjs run spec:validate
```

**Notes**

8 validators, 4677 chapters, 212 requirements; report jj-store-validation-55a46469-c07b-4c3a-bb2a-60998bc2e948.
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
node scripts/validation.mjs run docs:check
```

**Notes**

144 documentation files, valid links and anchors; report jj-store-validation-4875cbcd-4225-4bca-a9bc-51c4115ee0b8.
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
node scripts/validation.mjs run spec:first
```

**Notes**

Before: one failed gate, retained below. The appended receipt described an unchanged contract despite belonging to the original protected behavior-change commit, and its pending UI gate prevented using it as a completed unchanged-contract confirmation. Report jj-store-validation-3ca3038b-4f19-48f4-85ec-9c7a401e931e. The review now explicitly describes the enclosing PR canonical expansion relative to main; source/gate requirements remain unchanged. After correction: spec-first passed for 5 protected files and 3 canonical chapters; report jj-store-validation-25b5ea85-81cd-406b-8f7c-b5832652ea70.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Publish the checked Design Core fix only with the pending explicit approval,
  consume its normal npm version, then pass required UI/CI before merge.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Source qualified previously at 87be3ad68530653c6c6fd1e5a2463a217cf3f044; rebase destination main 9a804c393487cf8ae6c5535ae8c4bec54b38d5e5. Rust 1.99.0, Node 24.15.0, pnpm 10.32.1; managed Linux Docker runner.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ object, operation and history formats unchanged. No source API or runtime behavior change from the qualified fix.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

PR #86 conflicted after main merged forge credential replacement and the published Core 0.13.3 intake. Only two date navigation lists conflicted; both now retain the new main receipt and original immutable-history receipt. All Rust fix bytes are retained. A separate qualified-source bookmark keeps 87be3ad6 reachable for explicitly recorded private consumers while the PR head changes. No prior receipt is edited.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish affected verification, push the rebased feature head, link it and continue monitoring; merge only after all required checks pass.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "The enclosing PR retains the original JST-LOCAL-007/JST-JJ-005
  canonical expansion recorded in E-VAULT-HISTORY-FRONTIER-001: unique maximal
  ancestral-frontier reads are admitted without mutation. Those chapters are
  changed relative to current main. This rebase adds no further runtime or
  contract change from qualified 87be3ad6; the full PR remains pending its
  separate UI dependency gate."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.6 remains Partial; this source rebase does not qualify Lapis Sync,
  OS custody, native applications or release boundaries.

```
<!-- /evidence-field -->

<!-- evidence-field:non_qualifying_attempts yaml -->
## Non qualifying attempts

```yaml
- command: node scripts/validation.mjs run validation:rust test -p jj-store-jj
    --features vault-backup --lib vault_publication --locked
  note: "Exit 0 but zero tests collected: incorrect unit filter for an integration
    target, not qualification. Report
    jj-store-validation-b07c5dbd-2282-4e6d-af21-d3c16c90cba0; corrected target
    follows separately."
  qualification: none
- command: node scripts/validation.mjs run test:storybook
    catalog/components/store-application/ApplicationMenu.stories.svelte -t
    NativeChangeDraftSurvivesMobileToDesktopRelocation
  note: First invocation refused before execution while the workspace validation
    lease was active. No browser test ran; the subsequent serialized invocation
    is retained separately.
  qualification: none

```
<!-- /evidence-field -->

