---
schema: 1
id: E-CONVERSATIONS-NPM-005
---

# E-CONVERSATIONS-NPM-005

<!-- evidence-field:id markdown -->
## Id

E-CONVERSATIONS-NPM-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-005
- JST-UI-020
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-31
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change pzwuqopq
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte
- spec/src/current/contracts/ui/42-application-story-parity.md
- spec/src/current/progress/M30/index.md
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
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Protected PR checks run 41166372248, Storybook shard 3/3
```

**Notes**

Head 48d9e813: 332 of 333 shard tests pass; CanvasStartsWithoutTopPadding fails to find the Stack feature/api region after 1244 ms. The other 18 source jobs pass, including all four ordinary Rust shards, both application-browser shards, Help and cold static catalog. The required gate correctly fails. Retained artifact qualification-results-ui-storybook-2-1 contains exact source/run/attempt and JUnit counts.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t CanvasStartsWithoutTopPadding
```

**Notes**

Before the repair, an independent cold Docker run passes the exact case, with nine deliberately filtered cases: jj-store-validation-01d2f827-cb72-479f-89de-c07eb76e63ce. This is intermittent cold loader readiness, not a claimed deterministic local padding regression.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t CanvasStartsWithoutTopPadding
```

**Notes**

After adding the same production views.stacks() beforeEach used by existing application story families, exact fresh-process Docker run jj-store-validation-c22c13d6-9536-449c-a349-06379ad51e13 passes one case; nine are deliberately filtered. No query timeout, geometry assertion or diagnostic guard is changed.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte
```

**Notes**

Fresh-process Docker run jj-store-validation-3368e856-9549-4f4e-8370-4a21ebe6691c passes all ten affected light/dark refinement, geometry, hover and content cases, none filtered. The actual production application root and lazy Stacks view remain the rendered path.
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
pnpm catalog:check
```

**Notes**

Docker run jj-store-validation-62fd4f8a-296d-481e-8030-92ce351c65cd passes: zero Svelte errors/warnings, all 489 catalog module boundaries and application composition parity. No assertion, timeout, diagnostic or visual-baseline relaxation.
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
pnpm spec:validate; pnpm check:progress; pnpm spec:first --base 95f54da1e151858c3e0305bad8a87ed8c3b590fe
```

**Notes**

Completed separately and sequentially in Docker: all eight validators (4,551 chapters/211 requirements), 186-task/999-record progress bookkeeping (jj-store-validation-4ee7a950-a42e-4160-88dc-59b5a9198578), and spec-first coverage of 11 protected files across six owning chapters (jj-store-validation-622b6de3-b1fc-42d5-ab9c-18154c258fa5). QMD conceptual search and doctor pass on native Node24; the expected Docker-wrapper script-alias warning is retained. Broader root Rust/spec aggregates are outside this private story setup slice; fresh full protected and release CI remain blocking.
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

Locked source dependencies and Storybook 10.5.7/Vitest 4.1.11. Independent public jj-conversations remains 0.1.0; no package, application or Cargo version change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public API, wire/storage/receipt format, runtime behavior or presentation style changes. Existing story identities and visual-pending tags are retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed focused acceptance of the ten-case Stacks refinement story family. The missing setup now awaits the actual production lazy view loader instead of letting cold module work race an implicit role-query deadline. Full protected source CI and all subsequent complete-main/artifact/npm/GitHub release gates are separate pending M30 work.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Pass fresh complete protected PR #27 checks, merge the exact current-base source, then complete main heavy qualification, immutable artifact/provenance, actual npm and GitHub release verification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-001, JST-UI-005 and JST-UI-020 retain the same production
  application composition, stack interactions and exact geometry/hover
  assertions. Only this story family adopts the already established
  production-loader setup. Completed exact and all-ten-case cold Docker runs
  support this bounded source acceptance; cold production application and static
  catalog gates remain independent.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. The original protected source failure remains
  recorded. A focused story repair does not establish protected merge,
  complete-main heavy qualification, a publishable npm artifact or public
  registry availability.

```
<!-- /evidence-field -->

