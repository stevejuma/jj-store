---
schema: 1
id: E-CI-OFFLINE-GOVERNANCE-001
---

# E-CI-OFFLINE-GOVERNANCE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-OFFLINE-GOVERNANCE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/actions/setup/action.yml
- scripts/ci-provision-offline-governance.mjs
- scripts/ci.test.mjs
- scripts/conversation-client.test.mjs
- scripts/history-client.test.mjs
- scripts/validation/prepare.mjs
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/28/E-CI-OFFLINE-GOVERNANCE-001/index.md
- spec/src/history/2026/10/28/E-CI-OFFLINE-GOVERNANCE-001/SUMMARY.nav.md
- spec/src/history/2026/10/28/index.md
- spec/src/history/2026/10/28/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci.test.mjs#CI pins the shared local-source package and rejects mutable revisions
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:governance
```

**Notes**

Hosted PR checks run 40715450525 on tip be3d30a3: Tooling regression tests failed 117 passed / 3 failed of 120. The same three offline clean-install regressions as E-CI-SUITES-FIX-003 (conversation-client fresh source workspace; history-client independent archive and fresh source workspace) failed with `pnpm install --offline` on ubuntu-22.04. Spec-first failed separately on E-REUSABLE-PROGRESS-050's unqualified unchanged-contract confirmation. Native macOS governance remains green 120/120.
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
node scripts/ci-provision-offline-governance.mjs; node --test scripts/conversation-client.test.mjs scripts/history-client.test.mjs scripts/ci.test.mjs
```

**Notes**

Shared provisioner stores `typescript@5.9.3` and `lossless-json@4.3.3`. Twenty focused tests pass, including all three offline install regressions and the setup-action pin that now requires `ci-provision-offline-governance.mjs` plus `cargo fetch --locked`.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Spec-first passes on the tip that inherits E-REUSABLE-PROGRESS-050 authored with contract review status changed (ledger repair, not a completed unchanged confirmation) and covers the hosted CI offline-store scripts with this record.
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

<!-- evidence-field:versions markdown -->
## Versions

Native host macOS ARM64; pnpm 10.32.1; Node 22+; hosted CI ubuntu-22.04 with the same packageManager pin. No package or protocol version changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, public API or lockfile change. Hosted setup now runs the same offline registry-metadata provision Docker prepare already used for governance lanes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Hosted GitHub Actions setup did not provision `pnpm store add` metadata for the jj-conversations and jj-history offline clean-install regressions, while Docker prepare.mjs already did for `test:governance` and aggregate `test`. Extract scripts/ci-provision-offline-governance.mjs, call it from .github/actions/setup and from prepare.mjs, surface offline install stderr in the client tests, and assert the setup action pins the provisioner. Companion to E-CI-SUITES-FIX-003 for the hosted path.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Confirm hosted PR tooling and Spec-first lanes are green on the release tip; no publication or release gate change.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 hosted qualification selection and JST-GOV-003 independent
  suite evidence remain unchanged. This mirrors the existing Docker offline-store
  provision onto hosted CI setup so the same governance regressions can resolve
  without network; no product contract or publication authority changes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress at its existing release gates. Hosted offline
  governance provisioning promotes no publication, provider or visual gate.

```
<!-- /evidence-field -->
