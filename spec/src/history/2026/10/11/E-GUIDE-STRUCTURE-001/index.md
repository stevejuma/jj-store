---
schema: 1
id: E-GUIDE-STRUCTURE-001
---

# E-GUIDE-STRUCTURE-001

<!-- evidence-field:id markdown -->

## Id

E-GUIDE-STRUCTURE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->

## Tasks

- M19

<!-- /evidence-field -->

<!-- evidence-field:requirements list -->

## Requirements

- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
- JST-DOC-004

<!-- /evidence-field -->

<!-- evidence-field:date markdown -->

## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->

## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->

## Revision

rwvtmooq
<!-- /evidence-field -->

<!-- evidence-field:files list -->

## Files

- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/browsing.md
- docs/site/guides/capture.md
- docs/site/guides/editing.md
- docs/site/guides/reviews.md
- docs/site/guides/settings.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/progress/M19/index.md

<!-- /evidence-field -->

<!-- evidence-field:tests list -->

## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->

## Scope

Documentation-only main-workspace review of all 12 authored Guides pages. Six
dense guides now separate setup, routine actions, inspection and recovery
through descriptive H2/H3 headings, section introductions and shorter
paragraphs. Agent setup precedes usage. Browsing agent content and UI selection
instructions no longer sit under the local CLI section. Existing headings,
source prose, commands, qualifications and anchor URLs are retained. The
stored-session review entry point is documented from the preceding verified UI
slice. No application, renderer, runtime or command code changes.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->

## Before state

Interactive agent sessions: 452 source lines, six subsections, a 164-line
section and a 35-line paragraph mixing different workflows. Browsing: ten
subsections with an 88-line local-workspace section; unrelated agent UI content
appeared under Use Stacks from the local CLI. Editing: four subsections with a
47-line Stacks section. Capture, Settings and Reviews also combined multiple
reader tasks. Six shorter/coherent guides were reviewed and needed no structural
edit. Baseline source captured at /tmp/jj-guides-before.json.
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
pnpm spec:search JST-DOC-001
```

**Notes**

Read documentation requirements, manual ownership, delivery gates and M19. Used
the writing-guidelines skill for reader-shaped headings, task order,
introductions and paragraph separation. /tmp/jj-guide-structure-requirement.log.
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
pnpm docs:check
```

**Notes**

Before edits, source-derived references pass but application documentation
review is stale after the preceding committed UI slices.
/tmp/jj-guide-structure-before.log.
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
pnpm docs:review application "Reviewed owned Empty fallbacks, continuous project rows, exact repository-bound agent avatars and expandable stored-session reviews with unchanged authorization, anchors, drafts and receipt recovery. Guide sections now distinguish these interactions and their capability limits."
```

**Notes**

Reviewed the previously verified main UI changes and updated only the
application topic reason/fingerprint. No source contract generation or unrelated
review was changed.
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
pnpm docs:check
```

**Notes**

After reorganization: strict MkDocs generation, source/reference review, nine
light/dark Mermaid diagrams and local asset/link/anchor closure pass.
/tmp/jj-guide-structure-after.log. Reruns for section introductions and final
paragraph separation also pass; final closure 141 files, 22560363 bytes.
/tmp/jj-guide-structure-final-build.log. Builds ran sequentially.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-GUIDE-STRUCTURE-001 details 2](details-02.md)
