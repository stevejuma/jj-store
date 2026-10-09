---
schema: 1
id: E-CHAT-FOOTER-002
---

# E-CHAT-FOOTER-002 details 3

<!-- evidence-field:remaining list -->
## Remaining

- M24 remains In progress. Independent workspace publication/recovery and
  visual-baseline gates remain open. Existing wider ResizeObserver diagnostics
  are not claimed resolved; no warning appeared in this focused live inspection.
- Rust, generated transport, root aggregates, full catalog, built-bundle and
  platform qualification are out of scope for this UI-only button/layout slice.
  No startup or lifecycle behavior changed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; Svelte 5.57.1; macOS ARM64. Revision is the enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, storage, preference, source-anchor, publication or request identity format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Community reference inspected first, then public Design Core button variants. The owned workspace button uses secondary rest/hover styling, a shrinking flex basis and trailing auto margin. Existing full accessible label, Git folder icon, menu/copy actions and expanded-formatting row remain intact. Current short runtime labels fit the live 548px footer; longer fixture labels wrap without clipping at that width.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain responsive and hover regression coverage; continue independently governed M24 workspace qualification and resize investigation.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/015 now specify the grey rest/hover surface, repository label
  shrinking and wrapped-start alignment. JST-API-001 remains unchanged as
  explicitly recorded in its owning chapter; inspection, copy, selection and
  Send retain existing request contracts.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Focused interaction/accessibility and live
  geometry pass without expanding native publication/recovery, platform or
  visual qualification.

```
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
pnpm exec prettier --write spec/src/current/progress/M24/footer-review.md
```

**Notes**

Formatted the new topic page after navigation generation.
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
pnpm exec prettier --check spec/src/current/progress/M24/footer-review.md
```

**Notes**

Final topic formatting passes.
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
pnpm check:progress
```

**Notes**

Final coverage includes M24 navigation: 146 tasks and 710 evidence records.
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
pnpm spec:first
```

**Notes**

Final spec-first gate passes: 3 protected files, 2 canonical chapters.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

Final 4 document validators pass: 3390 chapters, 181 requirements.
<!-- /run -->
<!-- /evidence-field -->
