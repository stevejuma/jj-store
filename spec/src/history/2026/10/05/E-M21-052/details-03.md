---
schema: 1
id: E-M21-052
---

# E-M21-052 details 3

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
pnpm check:ui-ownership
```

**Notes**

completion-ownership.log: repository ownership boundary passes.
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
pnpm web:build (initial)
```

**Notes**

browser-build-current.log: Vite passed; generated installation API reference stale after delivered DTOs.
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
pnpm docs:generate
```

**Notes**

docs-generate.log: generated installation and type references refreshed.
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
pnpm web:build (index fix)
```

**Notes**

browser-build-index.log: Vite passed; documentation source review required for delivered mutations.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review (initial)
```

**Notes**

docs-review-before.log: local source review stale; checked all fingerprints before reviewing four affected topics.
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
pnpm docs:review local; pnpm docs:review http; pnpm docs:review contracts; pnpm docs:review application (with topic-specific reasons)
```

**Notes**

docs-review-local/http/contracts/application.log and docs-review-application-final.log: manuals reviewed and updated for native mutations and guarded readers.
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
pnpm web:build (completion)
```

**Notes**

completion-web-build.log: Vite and documentation closure pass; existing large-chunk advisory retained.
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
pnpm exec vite build --config web/vite.config.ts
```

**Notes**

browser-vite-recovery.log: recovery source built; direct Vite alone does not stage admission manifests/help.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (first)
```

**Notes**

browser-mutations-first.log: new operation required explicit graph preparation. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (second)
```

**Notes**

browser-mutations-second.log: file drag locator targeted a child rather than draggable button. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (third)
```

**Notes**

browser-mutations-third.log: CLI fileset command needed fixture cwd. No assertion weakening or unknown-command replay.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (fourth)
```

**Notes**

browser-mutations-fourth.log: commit row has locator needed page scope. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (fifth)
```

**Notes**

browser-mutations-fifth.log: assignment form needed Preview before Apply. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (sixth)
```

**Notes**

browser-mutations-sixth.log: native hunk apply succeeded; retained WIP diff patch was canceled during history update. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (seventh)
```

**Notes**

browser-mutations-seventh.log: unresolved preparation notice disappeared after navigation. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (eighth)
```

**Notes**

browser-mutations-eighth.log: view condition projected but application recovery observer still resolved it. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (ninth)
```

**Notes**

browser-mutations-ninth.log: direct Vite build had not staged browser manifest/help; no browser test started. No assertion weakening or unknown-command replay.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (tenth)
```

**Notes**

browser-mutations-tenth.log: ancestor visible in UI but absent from native command projection; file commit/identity checks passed. No assertion weakening or unknown-command replay.
<!-- /run -->
<!-- /evidence-field -->

