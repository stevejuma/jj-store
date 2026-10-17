---
schema: 1
id: E-REUSABLE-AGENTS-006
---

# E-REUSABLE-AGENTS-006

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-004
- JST-ARCH-005
- JST-ARCH-007
- JST-API-001
- JST-API-006
- JST-COMPAT-001
- JST-DOC-001
- JST-GOV-002
- JST-GOV-011
- JST-UI-001
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/typed-conversation-client.md
- .gitignore
- .prettierignore
- docs/maintenance/coverage.json
- docs/site/technical/conversations.md
- docs/ui-provenance.json
- package.json
- packages/jj-conversations/LICENSE.md
- packages/jj-conversations/NOTICE.md
- packages/jj-conversations/README.md
- packages/jj-conversations/package.json
- packages/jj-conversations/src/agent-mentions.ts
- packages/jj-conversations/src/client.ts
- packages/jj-conversations/src/delivery.ts
- packages/jj-conversations/src/index.ts
- packages/jj-conversations/tests/client.test.mjs
- packages/jj-conversations/tests/projections.test.mjs
- packages/jj-conversations/tests/types.ts
- packages/jj-conversations/tsconfig.json
- packages/jj-conversations/tsconfig.test.json
- pnpm-lock.yaml
- pnpm-workspace.yaml
- scripts/ci-plan.mjs
- scripts/ci-selection.test.mjs
- scripts/conversation-client.test.mjs
- scripts/docs/review.mjs
- scripts/web-architecture.mjs
- scripts/web-architecture.test.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/architecture/06-browser-presentation-ownership.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/ui/02-repository-owned-community-presentation.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-006/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-006/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-006/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-006/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
- spec-validator.config.mjs
- web/src/lib/community/agent-mentions.ts
- web/src/lib/conversations/prompt-delivery.ts
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- packages/jj-conversations/tests/client.test.mjs#concurrent callers share one
  exact submission and input/receipt mutations stay isolated
- packages/jj-conversations/tests/client.test.mjs#response loss keeps the
  original request unknown and refresh never submits
- packages/jj-conversations/tests/client.test.mjs#restart restores only a scoped
  identity and cannot manufacture a new command body
- packages/jj-conversations/tests/client.test.mjs#foreign receipts never replace
  retained identity or permit another submission
- packages/jj-conversations/tests/projections.test.mjs#delivery views retain
  full-width sequences and late queue receipts cannot revive completion
- scripts/web-architecture.test.mjs#headless package sources share the
  production size boundary
- web/tests/agent-mentions.test.ts#agent tags preserve exact setup IDs and never
  become member keys
- web/tests/vite/queued-agent-messages.test.ts#unknown admission never invents a
  queued message or resubmits it
- scripts/conversation-client.test.mjs#conversation package exports only its
  built independent client boundary
<!-- /evidence-field -->

