---
schema: 1
id: E-CHAT-WORKSPACE-007
---

# E-CHAT-WORKSPACE-007 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts
```

**Notes**

Final 5 tests pass: four original immutable-capture/gutter tests plus direct workspace grammar/highlighter range coverage for TypeScript, JavaScript, Python, Java and JSON with Unicode.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

7 passed, 1 failed: the header-toggle story queried a virtualized row immediately after reopening. Reacquired the live panel and awaited the restored row before holding the next read; exact skeleton and retained-file assertions remain.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

8 stories pass with icon controls, a long branch label and keyboard min/max rail sizing. Subsequent user feedback requested later wrapping; changed the base selector flex basis from9rem to5rem and retained responsive assertions. ResizeObserver warning remains.
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
pnpm web:check
```

**Notes**

Two runs pass with zero errors/warnings for the affected UI.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story passed unsupported exact:true options to Testing Library; removed those options since string names already match exactly.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings and 351 catalog modules after correcting story types.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI boundaries pass; no source copies or dependency changes.
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
cua_repl live inbox: source reader and comparison controls
```

**Notes**

Live TypeScript shows a line-number gutter and colored export/string tokens on a transparent surface. Both headers retain1px bottom borders. Separate Comment on file row and repeated missing-default description are absent. Small selectors have Lucide comparison/branch icons and accessible full titles. Inspection reads only; no execution, publication, fixture reset or service restart.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- ResizeObserver loop completed with undelivered notifications remains
  reproducible during viewport changes. No suppression or resolved claim.
- "Previous feature gates remain incomplete: full repository reader presentation
  reuse, historical source projection/navigation and cold native/local
  multi-agent publication/restart/GC acceptance. Earlier unrelated repository
  inset failures remain recorded, not waived."
- Visual baselines unapproved. Rust, generated API contracts and root aggregate
  lanes are out of scope for this UI-only slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ0.45.1; pnpm10.32.1; Vitest4.1.11; Svelte5.57.1. The enclosing Jujutsu change is this evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, API, publication, comment-anchor or persisted preference format changes. File reader presentation reuses the exact same block text and source identities.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Follow-up inbox file reader and comparison control presentation. File reader uses the existing Mira/CodeMirror comment widgets with public numbered code presentation, existing repository language resolution and installed grammar modes. Captured chat grammar behavior remains unchanged. Files/Changes and inspector title borders are restored per the user correction to E-CHAT-WORKSPACE-006; full-width listing dividers and constrained square joins remain.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete remaining workspace source/history acceptance and investigate the separately recorded ResizeObserver diagnostics. No backend restart or fixture reset needed.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/015/018 specify transparent highlighted file sources with
  separate gutters, existing hover comments, retained header dividers, compact
  icon selectors and truncation before wrapping. JST-API-001 is explicitly
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. The scoped reader/control acceptance passes;
  wider workspace, runtime-warning and visual gates remain open. M13.3
  qualification is unchanged.

```
<!-- /evidence-field -->

