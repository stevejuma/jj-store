---
schema: 1
id: E-READ-095
---

# E-READ-095

<!-- evidence-field:id markdown -->
## Id

E-READ-095
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
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared Source and Blame language highlighting
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qvvzswvy
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Blame used only the Design Core built-in tokenizer, which did not support C/C++ or Rust. Source used a separate Rust regex extension. TypeScript already rendered colors; extensionless Linux README intentionally selected plain text.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse existing public CodeMirror C/C++ and Rust grammars through CodeBlock custom-tokenizer and flatTokensToLines APIs; preserve source text, multiline context, pending attribution and shared styles. Supersedes E-READ-092 C presentation deferral via the existing public extension point, without changing Design Core.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64;pnpm10.32.1;Vitest4.1.11;existing provisioned Design Core local validation package reports0.1.2(manifest ^0.2.0 unchanged);@codemirror/lang-cpp6.0.3;@codemirror/lang-rust6.0.2;@lezer/highlight1.2.5. No dependency refresh or publication.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No backend/API/schema/native identity/source-state format change. Existing file contents and public syntax token offsets only.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreBlame.stories.svelte
- catalog/fixtures/StoreBlameFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte
- web/src/lib/ui/community-projects/StoreFileEditor.svelte
- web/src/lib/ui/community-projects/repository-source-tokenizer.ts
- web/src/lib/ui/store-blame/StoreBlameView.svelte
- web/tests/repository-source-tokenizer.test.ts
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreBlame.stories.svelte
```

**Notes**

blame-highlight-before.log: initial3 failures/7 passes, two caused by fixture directory mismatching its sample path. Corrected fixture route. blame-highlight-baseline.log:2 failures/8 passes reproduce missing Rust/C keyword tokens; TypeScript computed colors and extensionless README pass.
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
pnpm exec tsx --test web/tests/repository-source-tokenizer.test.ts
```

**Notes**

blame-highlight-tokens.log:4/4, C/C++ directives/keywords, Rust lifetime/raw strings, multiline comments and bounded ordered line offsets, unknown/default-language fallback.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

blame-highlight-web.log and blame-highlight-catalog.log:0 errors/0 warnings,248 catalog modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreBlame.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

blame-highlight-stories.log:18/18, fresh Vitest/Vite Chromium process. Actual computed colors, exact source reconstruction, pending origins, Source/Blame switching, existing source/edit and route/cache interactions. No visual baseline update.
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
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

blame-highlight-architecture.log:1155 modules. blame-highlight-ownership.log:ownership validator passes. Existing editor grammar dependencies and public Design Core extension only; no upstream patch, copied tokenizer or added dependency.
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
pnpm exec prettier --write <changed UI/spec/test/docs files>; pnpm docs:review application "Source and Blame share the existing C/C++ and Rust editor grammars through the public tokenizer extension; extensionless README stays plain text."; pnpm docs:check
```

**Notes**

Targeted formatting, application fingerprint and browsing guide updated. blame-highlight-docs-check.log:139-file closure; links/anchors and9 Mermaid diagrams in light/dark pass.
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
pnpm spec:first
```

**Notes**

blame-highlight-spec-first.log:mapped API chapter missing for protected UI/tests. Added source-response/line-offset clarification; no transport changes.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

blame-highlight-spec-first-final.log:9 protected files/3 canonical chapters. blame-highlight-spec-validate.log:5 validators/23 chapters/173 requirements. Repeated after ledger append below.
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
pnpm exec tsx -e <read-only Linux file tokenizer timing probe>
```

**Notes**

blame-highlight-linux-tokens.log:altera_edac.c62285 bytes/5496 tokens/249ms;fs/inode.c85316 bytes/7579 tokens/101ms. One development sample each; these are source highlighting timings, not blame attribution benchmarks.
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
jj --no-pager st; jj rebase -r @ -d qvvzswvy (preview workspace); CUA fresh Linux Blame tab
```

**Notes**

Clean preview advanced to the fix. README remains plain text. Nested-file URLs without directory context hit existing unavailable-file handling; using the original include/crypto directory context loaded acompress.h.145 mounted token spans,keyword #ifndef rgb(215,58,73),comment rgb(106,115,125),source rgb(36,41,46),3 pending origin cells. Saved .local/blame-highlight-linux.png; temporary tab closed. No backend restart.
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
jj --no-pager st (main)
```

**Notes**

Main Stacks discovery became committed555e25ad,then acquired active Apply and unapply stacks without confirmation work in otvxrvnt. spec/src/ui.md still overlaps; main untouched under AGENTS. Preview and retained workspace provide the verified change.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

blame-highlight-progress.log: the ledger checker does not recognize Svelte Story names as named tests. Kept executable node test references and recorded browser stories under their actual Vitest run above.
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
pnpm exec prettier --check <changed files>; pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

blame-highlight-progress-final.log:142 tasks. Governance-final:9 protected files/4 chapters,5 validators pass. Initial formatting check reported three pre-existing table spacing lines in the ledger; normalized those spaces only, retaining every historical JSON entry. Final formatting/progress/spec commands repeated before commit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/repository-source-tokenizer.test.ts#C and C++ source use genuine
  syntax tokens including preprocessor directives
- web/tests/repository-source-tokenizer.test.ts#multiline comments retain their
  type without crossing source line boundaries
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

UI-only focused tokenizer tests, affected Source/editor/Blame stories with cold browser interaction and color checks, types, architecture/ownership and documentation/spec governance. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Main landing remains blocked by active overlapping Stacks UI changes in
  otvxrvnt. Preview contains the fix.
<!-- /evidence-field -->


<!-- evidence-relations -->
## Resolution and supersession

The original result remains **pending**. Milestone gates select their own evidence.

- [E-READ-096](../E-READ-096/index.md) resolves this entry. Main integration completed the previously recorded landing blockers.
<!-- /evidence-relations -->
<!-- generated-navigation -->
## Pages

- [E-READ-095 details 2](details-02.md)
