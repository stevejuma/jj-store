---
schema: 1
id: E-M17-081
---

# E-M17-081 details 3

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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm exec prettier --check docs/implementation-plan.md spec/src/collaboration.md spec/src/ui.md spec/src/cli.md spec/src/architecture.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All changed documentation formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Plan snapshot 3 October 2026; JJ 0.45.1; installed Codex CLI 0.156.1. Existing configured profiles remain version-qualified; Desktop is an independent trial gate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Documentation and milestone scope only. No public or private persistence/API/runtime format change. All 335 prior evidence objects and their raw JSON prefix retained. Main/capture historical task mapping is planned, not applied.
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

User requested a functional feature plan with UI prioritized, extras Deferred, and rebasing against main. Existing Codex startup/readers/history/recovery and UI are reused. M17.2 is Partial for the larger reader scope; M17.3-M17.7 Deferred. M17.8 is the independent usable-preview gate. Rebase, automatic freshness, isolated setup and actual Codex/service/browser/restart qualification remain pending implementation. This entry accepts planning only; product tests, UI/Rust builds, root aggregate suites, deployment and restarts are out of scope. The verified prefix boundary remains in the preview ancestry; unfinished OpenCode integration is parked. Main active state and existing services preserved. Documentation-only replan acceptance complete. The next implementation step is rebase against the newly rechecked committed main followed by UI work. No rebase or preview startup performed in this planning change.
<!-- /evidence-field -->

