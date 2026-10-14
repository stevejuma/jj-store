# GOV verification

E-AUTOMATION-GOVERNANCE-001 verifies the JST-ARCH-005 generated PR exception to
JST-GOV-002/003/004/011 with workflow conditions, unchanged planned test lanes,
direct release-intent checks and Changesets versioning. Evidence is in
[the automation record](../../history/2026/10/14/E-AUTOMATION-GOVERNANCE-001/index.md).

| ID          | Status      | Evidence                                                                                                                                                             |
| ----------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-GOV-001 | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](../progress/index.md)                                 |
| JST-GOV-002 | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs` with bounded large-ledger reads; `spec-validator.config.mjs`; [bootstrap evidence](../progress/index.md) |
| JST-GOV-003 | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](../progress/index.md)                                 |
| JST-GOV-004 | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](../progress/index.md)                                 |
| JST-GOV-005 | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](../progress/index.md)                                 |
| JST-GOV-006 | Implemented | `rustfmt.toml`; `package.json`; `spec-validator.config.mjs`; [formatting evidence](../progress/index.md)                                                             |
| JST-GOV-007 | Implemented | `AGENTS.md`; [verification scope](../governance/index.md); [targeted verification evidence](../progress/index.md)                                                    |
| JST-GOV-008 | Implemented | `scripts/ui-ownership.test.mjs`; `pnpm check:ui-ownership`; [owned presentation evidence](../progress/index.md)                                                      |
| JST-GOV-009 | Implemented | `scripts/dev-ui.test.mjs`; `scripts/dev-demo-processes.test.mjs`; `scripts/dev-ui.acceptance.mjs`; [E-UI-057](../../history/2026/10/02/E-UI-057/index.md)            |
| JST-GOV-010 | Implemented | [Migration evidence](../../history/2026/10/06/E-GOV-003/index.md)                                                                                                    |
| JST-GOV-011 | Implemented | [Migration evidence](../../history/2026/10/06/E-GOV-003/index.md)                                                                                                    |
