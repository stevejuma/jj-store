---
schema: 1
id: E-M18-005
---

# E-M18-005 details 3

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
cargo test -p jj-store-http capture_management_contract_is_installation_local_and_actor_free --locked
```

**Notes**

Exact installation-local contract schema/actor/body regression passes.
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
pnpm exec prettier --write <changed TS/Svelte/JS and canonical chapters>; cargo fmt --all
```

**Notes**

Formatting applied throughout implementation and repairs. Keep prior evidence bytes intact; format only this new JSON entry.
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
pnpm exec prettier --check <all changed TS/Svelte/JS/JSON and canonical chapters except historical ledger>; cargo fmt --all -- --check
```

**Notes**

Final formatting checks pass. New evidence entry uses formatted JSON; historic ledger encodings retained verbatim.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress (final)
```

**Notes**

Spec-first31 protected files/8 canonical chapters; validation5 validators/22 chapters/167 requirements; progress136 tasks. M18.2 remains Partial; no full milestone or visual baseline promoted.
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

Narrow editing acceptance: typed current LocalTrusted Admin configuration, saved/effective/default roots, bounded crash-aware persistence, shared-column editor, stale/unknown drafts and real next-start selection. Runtime pause/rescan/import commands and all durable capture state remain independent. Supplemental strict Clippy warnings are inherited diagnostics outside the gate. The live preview still reports capture_management_unavailable for runtime observations; its worker recovery/availability is not qualified by a settings save. Full M18 multi-harness release, native desktop bundle/lifecycle, arbitrary server/credential editing, hot worker replacement and visual baseline approval remain outside this slice. No root aggregate or installed-service reset/publication.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use Settings → Agents → Configuration in preview7470. Save config and restart this selected backend; desktop applies saved config on Quit/reopen with a rebuilt app bundle. Keep the reader limit restored to4. Investigate the retained fixture worker-unavailable diagnostic separately; do not repair journals or infer runtime health from effective config. Continue deferred M18 scope only under its owning gates.
<!-- /evidence-field -->

