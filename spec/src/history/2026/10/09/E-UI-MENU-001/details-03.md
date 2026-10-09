---
schema: 1
id: E-UI-MENU-001
---

# E-UI-MENU-001 details 3

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
pnpm spec:first
```

**Notes**

Final gate passes for 6 protected files and 3 canonical chapters.
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
pnpm check:progress
```

**Notes**

Overall pass is disallowed while any wider gaps are recorded. Evidence stays pending, matching M24/M19 status; individual focused runs retain their passing results.
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

Final bookkeeping passes with overall evidence pending for explicitly retained gaps: 146 tasks, 708 evidence records.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

Final source document validation passes: 4 validators, 3382 chapters, 181 requirements.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Existing ResizeObserver diagnostics remain open. M24 stays In progress; M19
  stays Partial; visual baselines remain unapproved.
- Root aggregate, Rust, generated API contract, built-bundle and native/manual
  content qualification are out of scope for this UI-only presentation slice.
- Generated HTML book link audit unavailable because spec/book has no HTML
  pages. Browser bridge timed out during optional additional picker inspection
  and temporary-tab cleanup; automated picker interactions passed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; Svelte 5.57.1; macOS ARM64. The enclosing Jujutsu change is this evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, API, source-anchor, authorization, publication or device preference format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Right-aligned fixed-width landing Help via public Card.Footer. Shared app menu/popover heading typography, including portals; remove bold overrides in workspace, branch and commit pickers and profile identity. Menu actions, selection and focus contracts stay unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the focused presentation acceptance; continue the separately governed M24/M19 qualification and existing resize diagnostics.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-018 specifies normal-weight menu typography matching items;
  JST-DOC-003 specifies fixed trailing Help geometry. Existing routing,
  authentication, authorization and documentation lifecycle contracts are
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress and M19 remains Partial. This presentation slice
  does not qualify wider workspace publication/recovery, native IME, packaged
  WKWebView or visual baselines.

```
<!-- /evidence-field -->

