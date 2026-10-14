---
schema: 1
id: E-APPLICATION-MENU-001
---

# E-APPLICATION-MENU-001 details 3

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
Live CUA: final cold Workflows document after structural HMR; refresh current user Workflows tab after confirming no dialog/draft; responsive screenshot and temporary tab cleanup
```

**Notes**

A warm structural HMR retained an old teleported Main body. Cold document confirms one Main, one Application menu and scrollWidth390 at width390. Current user tab was refreshed without saving data. Screenshot /tmp/jj-store-workflow-mobile-menu.png uses a 390x844 viewport clip. Browser overrides reset; no native service restart.
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
pnpm spec:first
pnpm check:progress
pnpm spec:validate
pnpm exec prettier --check $(jj diff --name-only)
pnpm check:ui-ownership
```

**Notes**

Final scope gate: 15 protected files / 6 owning chapters; progress 153 tasks / 790 evidence records; 7 validators / 3771 chapters / 196 requirements. Formatting and UI ownership pass. Final cold current user tab: one Main, one Application menu, no top Application navigation at width390. Temporary verification tab closed and browser/device overrides reset. No live data or config mutations, service restart, unrelated generated navigation churn, visual-baseline update or wider aggregate lane. Logs spec-complete.log, progress-complete.log, spec-validation-complete.log, format-complete.log and ownership-final.log under /tmp/jj-store-application-menu-*.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
- Existing ResizeObserver loop-delivery diagnostics appear during resize checks;
  assertions and blocking Axe pass. This diagnostic is previously retained
  separately in M24 agents-presentation review and is not qualified by this
  slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0, pnpm 10.32.1, jj 0.45.1; Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11, Design Core ^0.3.1. Native binaries and released dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Svelte snippets/props and owned UI documentation/provenance only. Native APIs, command/request receipts, stored workflow definitions, configuration files and URLs retain their formats.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One shared horizontal Application menu, desktop top-header or narrow body-toolbar placement, Workflows adopting Projects mobile controls, persistent command owners and focus restoration. Top-level headerMenu override removed. UI acceptance passes; overall evidence stays pending only for explicitly retained independent gates and diagnostics. No full Rust, root aggregate, API-generation or full catalog/browser/build lane was run under the targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain separate native provider/Forgejo/platform and visual-baseline gates; previously recorded ResizeObserver qualification remains independent.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-016 and JST-WF-008 placement guidance now requires the shared
  mobile toolbar and one menu. JST-GOV-002/008 ownership unchanged; native
  authority, reload authorization and JST-UI-019 toasts preserved.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M27 and M28 remain In progress. Focused menu/navigation acceptance
  cannot qualify native execution, live providers, remote Forgejo protocol,
  other platforms or approved visual baselines.

```
<!-- /evidence-field -->
