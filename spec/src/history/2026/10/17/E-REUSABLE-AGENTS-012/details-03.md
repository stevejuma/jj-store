---
schema: 1
id: E-REUSABLE-AGENTS-012
---

# E-REUSABLE-AGENTS-012 details 3

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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Initial coverage.json formatter mismatch after source review; corrected with exact Prettier write.
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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Final changed authored-source formatting passes.
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
pnpm dev:clean -- --dry-run
```

**Notes**

Read-only eligible-output preview: free 23.0 GiB, below native reserve; would clean idle first-party outputs in root and desktop targets. No cleanup or service stop/restart performed in this slice.
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
pnpm check:progress
```

**Notes**

171 tasks / 869 immutable evidence records; M33/M33.4 remain In progress.
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
pnpm spec:first
```

**Notes**

29 protected files / five canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

Seven validators / 4,202 chapters / 207 requirements pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "The aggregate architecture audit retains three unchanged UI size failures:
  StoreStackWorkspaceApplication.stories.svelte (401/400),
  StoreApplicationMain.svelte (313/300), StoreStackWorkspace.svelte (310/300).
  The exact parent checker reports the same three on byte-identical UI sources;
  its only additional failure is the newly generated history type file. No
  waiver or release qualification claimed."
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0; pnpm 10.32.1; TypeScript 5.9.3; lossless-json 4.3.1 for encoding; native schema producer committed at 300adee5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version-1 bounded UTF-8 JSON text from owned Rust schema. Safe integers decode as numbers; larger native i64/u64/usize decode as bigint. Integer-only decoder rejects duplicate keys, exponent/fraction tokens, excess nesting and malformed input; record spelling including __proto__ is preserved. Existing native/persistent formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Independent headless @lapismd/jj-history package, optional injected Tauri entry, generated types/runtime schema, strict exact integer frames, immutable vault scope and caller-owned handle lifetime. Normal governance includes complete runtime/type checks and cold source/archive consumers. CI source selection and exact generated-type size exception retain authored limits. No app UI, native Rust, provider, RustFS or live service change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Integrate Notes native repository admission and history; retain aggregate size failures until resolved or explicitly accepted. First usable Notes milestone remains incomplete.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 adds a host-injected standalone client derived from owned
  Rust transport contracts, with independent public entries and exact integer
  types. JST-API-001/004 preserves opaque admission and bounded frames; no
  filesystem, cache, native caller, execution, retry or publication authority.
  JST-GOV-002/011 keeps source drift and archive acceptance in normal
  governance; only the exact generated contract is excluded from authored size
  counts.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33/M33.4 stay In progress. Client and archive acceptance do not
  complete Notes physical-vault/window integration, native WebView history or
  durable chat. Existing E010 CLI Busy and E006 wider UI gaps retain their
  original results. This record retains the three aggregate size failures; no
  milestone Done or waiver.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uqwtvwws
<!-- /evidence-field -->

