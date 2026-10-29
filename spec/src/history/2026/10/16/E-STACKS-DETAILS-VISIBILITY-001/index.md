---
schema: 1
id: E-STACKS-DETAILS-VISIBILITY-001
---

# E-STACKS-DETAILS-VISIBILITY-001

<!-- evidence-field:id markdown -->
## Id

E-STACKS-DETAILS-VISIBILITY-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-16
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21.1
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-UI-014
- JST-UI-020
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

packaged-source-regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change kmuslmrxuktl; owning Design Core source change lxmzsyyk.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Live review: repo to Stacks with Details closed, Repository settings, then Back to repository. The Details landmark retained data-state=closed but lost hidden and occupied 220 CSS pixels with empty content. Both owning-source and consumer regressions reproduced this failure.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
- spec/src/current/progress/M21/history/M21.1/index.md
- spec/src/current/progress/M21/index.md
- spec/src/history/2026/10/16/E-STACKS-DETAILS-VISIBILITY-001/SUMMARY.nav.md
- spec/src/history/2026/10/16/E-STACKS-DETAILS-VISIBILITY-001/details-02.md
- spec/src/history/2026/10/16/E-STACKS-DETAILS-VISIBILITY-001/details-03.md
- spec/src/history/2026/10/16/E-STACKS-DETAILS-VISIBILITY-001/index.md
- spec/src/history/2026/10/16/SUMMARY.nav.md
- spec/src/history/2026/10/16/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; jj 0.45.1; pnpm 10.32.1; Node 24.10.0 in jj-store and 26.9.0 in Design Core; Vitest 4.1.11; Design Core 0.5.1 source with a pending patch Changeset.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No server, API, object, WAL, receipt, operation-pin, persistence or authorization format change. Shared shell reconciliation retains existing durable close/open state after consumer visibility changes. Tracked package.json and pnpm-lock.yaml remain registry-only and unchanged.
<!-- /evidence-field -->

<!-- generated-navigation -->
## Pages

- [E-STACKS-DETAILS-VISIBILITY-001 details 2](details-02.md)
- [E-STACKS-DETAILS-VISIBILITY-001 details 3](details-03.md)
