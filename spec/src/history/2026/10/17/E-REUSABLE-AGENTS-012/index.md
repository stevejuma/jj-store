---
schema: 1
id: E-REUSABLE-AGENTS-012
---

# E-REUSABLE-AGENTS-012

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-LOCAL-001
- JST-LOCAL-004
- JST-API-001
- JST-API-004
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
- JST-GOV-011
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

- .changeset/typed-history-client.md
- .gitignore
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- package.json
- packages/jj-history/LICENSE.md
- packages/jj-history/NOTICE.md
- packages/jj-history/README.md
- packages/jj-history/package.json
- packages/jj-history/src/generated.ts
- packages/jj-history/src/index.ts
- packages/jj-history/src/parse.ts
- packages/jj-history/src/schema.json
- packages/jj-history/src/tauri.ts
- packages/jj-history/src/validate.ts
- packages/jj-history/src/wire.ts
- packages/jj-history/tests/client.test.mjs
- packages/jj-history/tests/types.ts
- packages/jj-history/tsconfig.json
- packages/jj-history/tsconfig.test.json
- pnpm-lock.yaml
- pnpm-workspace.yaml
- scripts/ci-plan.mjs
- scripts/ci-selection.test.mjs
- scripts/history-client.test.mjs
- scripts/history-contracts.mjs
- scripts/web-architecture.mjs
- scripts/web-architecture.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-012/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-012/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-012/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-012/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- packages/jj-history/tests/client.test.mjs#one vault uses exact commands,
  explicit handle lifetime and lossless native integers
- packages/jj-history/tests/client.test.mjs#timestamps, unsigned ranges, UTF-8
  and escaped strings retain exact values
- packages/jj-history/tests/client.test.mjs#unsafe inputs, extra authority,
  invalid identities and tuples fail before invocation
- packages/jj-history/tests/client.test.mjs#malformed, duplicate, versioned and
  oversized replies never become typed data
- packages/jj-history/tests/client.test.mjs#lost outcomes propagate once, failed
  close remains caller-owned and responses match the query
- packages/jj-history/tests/client.test.mjs#optional Tauri entry uses exact
  namespace and only a text request
- packages/jj-history/tests/client.test.mjs#record keys preserve proto spelling
  and repeated escaped keys reject
- scripts/history-client.test.mjs#history package exports built independent
  entries and matches its native schema
<!-- /evidence-field -->

