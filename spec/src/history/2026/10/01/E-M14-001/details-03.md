---
schema: 1
id: E-M14-001
---

# E-M14-001 details 3

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
jj --no-pager log -r '541edf40..default@' -n 12; jj --no-pager diff --from 541edf40 --to 'default@' --summary; jj -R /Users/stevejuma/code/lapismd/jj-store --no-pager st
```

**Notes**

Final read-only review found concurrent native review/activity UI work in the original checkout (change nqpoyvww). This isolated slice remains based on 541edf40; no rebase, integration or modification of that active work was attempted.
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

One UI behavior and its direct consumers. Graph preparation uses the existing native task API; no Rust, HTTP schema or native format change. Root aggregates, Rust builds/tests, fresh Rust fixture capture and unrelated browser/Storybook suites are out of scope under the targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire, native, WAL, receipt, agent-spool, local companion or stored route format changes. Existing Settings section and graph action are corrected. M14 delivery scope reuses M13 profile-aware capture and jj agent drain; filesystem-only browser support remains gated on M13.4. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; locked Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11 and Playwright 1.63.0. Native Rust/format dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement M14.2 typed local management and verified device links, retaining the M13.4 serving gate and reconciling canonical documents with concurrent UI work before integration.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- M14.2 through M14.7 remain planned; no agent management API, automatic drain
  worker, editable Search, storage or sync UI is claimed.
- M10.8 pressure/fault/topology qualification and backup/restore remain deferred.
- Visual baseline approval remains separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Local settings workspace .scratch contains unit, web/catalog, Storybook, build, specification and cold-browser logs/configuration; web/test-results/settings contains the browser results. Disposable Vite preview ended with Playwright.
<!-- /evidence-field -->

