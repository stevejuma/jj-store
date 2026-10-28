---
schema: 1
id: E-PACKAGE-LICENSE-001
---

# E-PACKAGE-LICENSE-001

<!-- evidence-field:id markdown -->
## Id

E-PACKAGE-LICENSE-001
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
- JST-DOC-001
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-history-service/LICENSE.md
- crates/jj-history-service/NOTICE.md
- crates/jj-vault/LICENSE.md
- crates/jj-vault/NOTICE.md
- docs/local-compute.md
- docs/maintenance/coverage.json
- docs/site/technical/capture.md
- docs/site/technical/local.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/28/E-PACKAGE-LICENSE-001/index.md
- spec/src/history/2026/10/28/E-PACKAGE-LICENSE-001/SUMMARY.nav.md
- spec/src/history/2026/10/28/index.md
- spec/src/history/2026/10/28/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 40762147999 --repo stevejuma/jj-store --log-failed
```

**Notes**

Hosted tip 0360cae0: Tooling failed `AssertionError: crates/jj-history-service missing LICENSE.md` from `scripts/package-check.mjs`. Pages build failed separately with `Documentation review required: capture`. Spec-first and offline governance were already green.
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
python3.11 license-file check; pnpm docs:review capture|local-compute|history-client; pnpm docs:review; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Added Apache-2.0 LICENSE.md and matching NOTICE.md for `jj-history-service` and `jj-vault` from the established crate license templates. Refreshed capture, local-compute and history-client documentation reviews for the published Lapis AI / agent-redaction pins and `@lapismd/jj-history` 0.1.0 publishConfig. All Cargo members have license files; docs:review and native spec:first pass.
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

No package or protocol version changed. Agent-redaction pin remains 0.1.1 at eada2b26; Cargo package metadata unchanged aside from adding required license files.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, public API or lockfile change. Capture documentation review fingerprint updated for the pinned redaction source and existing transient snapshot text.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Close the two remaining hosted tip blockers on release/jj-history-trusted-publish: registry package-check required LICENSE.md/NOTICE.md on every workspace Cargo member including the new jj-history-service and jj-vault crates, and Pages required a fresh capture documentation review after the sibling redaction pin and capture source drift.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Confirm hosted PR tooling and Pages build are green on the release tip; no publication gate change beyond existing trusted-publish work.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 qualification packaging, JST-GOV-003 evidence discipline and
  JST-DOC-001 capture manuals retain their contracts. LICENSE/NOTICE files are
  required registry packaging inputs already present on sibling crates; the
  capture review only refreshes the documented redaction pin and fingerprint
  without changing spool, receipt or publication authority.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress at its existing release gates. Adding required
  license files and refreshing the capture docs review promote no publication,
  provider or visual gate.

```
<!-- /evidence-field -->
