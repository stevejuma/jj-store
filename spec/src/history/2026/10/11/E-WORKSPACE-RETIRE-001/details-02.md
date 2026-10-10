---
schema: 1
id: E-WORKSPACE-RETIRE-001
---

# E-WORKSPACE-RETIRE-001 details 2

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node v26.9.0; pnpm 10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native, API and ACP formats unchanged. Markdown evidence schema 1. Retirement uses registered JJ workspace identities and recoverable macOS Trash.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue UI work in default. M24 remains In progress; existing provider, live permission/cancellation, desktop/platform and visual-baseline gates are independent. Root aggregate, Rust, UI and cold acceptance lanes were not rerun because this slice changes only bookkeeping.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
requirements:
  - JST-GOV-002
  - JST-GOV-011
  - JST-UI-015
reason: Fresh workspace-retirement evidence and milestone review preserve all
  implementation, API, ownership, persistence and UI contracts. No protected
  production source or dependency changed in this slice.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
tasks:
  - M24
reason: All identified session work is in default and the temporary checkouts
  are retired. M24 remains In progress; this cleanup does not qualify real
  adapters, platform distribution or visual baselines.

```
<!-- /evidence-field -->

