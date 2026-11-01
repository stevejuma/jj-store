---
schema: 1
id: E-DEPENDENCY-AUTOMATION-003
---

# E-DEPENDENCY-AUTOMATION-003

<!-- evidence-field:id markdown -->
## Id

E-DEPENDENCY-AUTOMATION-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-ARCH-005
- JST-UI-001
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

Enclosing Mira-only compatible dependency intake
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- pnpm-workspace.yaml
- patches/bits-ui@2.20.2.patch
- patches/manifest.json
- patches/README.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/ui/01-storybook-presentation-catalog.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/current/verification/ui.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
Core 0.7.4 / Mira 0.1.4 / root Bits UI 2.20.2 candidate: complete Storybook command
```

**Notes**

933 tests passed, 10 failed, one unhandled browser disconnect; 169 of 216 files completed. Eight stale Select-role queries were repaired in the diagnostic candidate, but that candidate is not selected for this PR. Its browser disconnect remains unproven.
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
Core candidate: fresh seven-file focused Storybook cohort
```

**Notes**

25 tests passed, two workspace recovery stories failed. Branch regrouping leaves Bits Command aria-activedescendant pointing to a removed item. The stale-preview case passes alone with diagnostic timing, which is not qualified coverage. No dependency repair or accessibility waiver was added.
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
corepack pnpm@10.32.1 install --frozen-lockfile --ignore-scripts --ignore-pnpmfile
```

**Notes**

Final graph retains Core 0.7.0 and root Bits UI 2.19.5, selects Mira 0.1.4 and applies the exact published patch to its nested Bits UI 2.20.2. All three patched file hashes match afterSha256 in the published manifest. The isolated workspace owns its node_modules.
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
npm exec --yes --package=node@22.23.4 --package=pnpm@10.32.1 -- pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-application/MarkdownReading.stories.svelte catalog/components/store-application/ConversationMarkdown.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte --reporter=verbose
```

**Notes**

11 tests pass across five complete story files. All six workspace recovery stories pass without diagnostic timing or source/assertion edits, including the two failures from the held Core graph. The original interrupted navigation case, Markdown readers and file editor pass in this fresh process. No visual baseline work.
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
npm exec --yes --package=node@22.23.4 --package=pnpm@10.32.1 -- pnpm exec svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Zero errors and zero warnings on the final selected graph.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Fresh complete hosted UI/browser and required CI before merge. Earlier
  clipboard feedback and disconnected-browser failures are not claimed fixed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Mira 0.1.4; Core 0.7.0 retained; root Bits UI 2.19.5 retained; nested Bits UI 2.20.2 with published patch SHA-256 debd3bc7d43917f0967f0cc1e212fc5c7721c751d3b92bf8f31030d1c96289ad; pnpm 10.32.1; targeted Node 22.23.4.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native storage, WAL, RPC or publication format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Qualified focused source intake of the published Mira patch line. Core 0.7.4 remains held after reproducible accessibility failure; the diagnostic candidate is retained locally under diagnostic/core-0.7.4-20261101. No consumer workaround, functional waiver, publication, signing or deployment. Full hosted CI remains the merge boundary.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge the selected intake only after current-head CI passes. Resolve the held Core primitive regression upstream before a later Core intake. Leave Community unmerged.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Record the verified Mira inputs and the identical published Bits UI root
  patch while preserving Core, all original assertions and artifact provenance.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; focused UI/source intake does not qualify
  native/platform delivery or publication.

```
<!-- /evidence-field -->
