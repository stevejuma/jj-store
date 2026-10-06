---
schema: 1
id: E-M15-019
---

# E-M15-019 details 2

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
pnpm test:governance
```

**Notes**

40 tests pass, including all 7 UI ownership regressions; /tmp/jj-alignment-governance-final.log. Earlier null/mapping provenance failures were corrected without an allowlist.
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
pnpm check:ui-ownership
```

**Notes**

Focused plugin passes; rejects runtime/type/reexport/lazy/require/CSS imports, package/config/lock resolutions, external production symlinks and missing/invalid attribution.
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
jj file list -r 741b8285e2081f1c9af58afe5a9b72f538fad781; jj file list -r d859f33a484a9238bb11f974f4ead8e9be729ec6 (in sibling lapis-community); Python source-path audit
```

**Notes**

All 177 original source references exist at the recorded revisions; /tmp/jj-provenance-invalid.json is empty. Read-only reference inspection only.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Finalization exposed multiline JST-GOV-008 requirement table formatting; merged into one valid canonical row.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Added actual search/transport qualification text to mutations.md, cli.md and compatibility.md; protected provenance now maps to governance and UI chapters.
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
pnpm spec:validate; pnpm spec:first
```

**Notes**

5 validators, 21 chapters and 145 requirements pass; 207 protected files and 9 changed canonical chapters mapped before final ledger additions. /tmp/jj-alignment-spec-validate-final.log and /tmp/jj-alignment-spec-first-final.log.
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
pnpm check:architecture; pnpm check:boundaries
```

**Notes**

605 production modules and 16 packages pass. Oversize modules were extracted; counted-line budgets were not raised.
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
pnpm exec prettier --write <changed supported files>; cargo fmt --all; pnpm format:rust:check
```

**Notes**

Changed supported files formatted; Rust format checks pass. Local loader/editor configuration remains untracked and unchanged.
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

<!-- evidence-field:scope markdown -->
## Scope

User ownership decision explicitly supersedes parent shared-package guidance only for Community. Design Core/Mira remain foundational dependencies. The inherited Design Core development substitution is retained and reported, not presented as registry qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the owned closure and check:ui-ownership on future UI changes. No Community package publication, main landing or main restart.
<!-- /evidence-field -->

