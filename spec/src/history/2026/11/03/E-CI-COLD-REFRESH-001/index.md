---
schema: 1
id: E-CI-COLD-REFRESH-001
---

# E-CI-COLD-REFRESH-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-REFRESH-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-ARCH-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/real/native-repository.spec.ts
- spec/src/current/contracts/ui/32-event-driven-freshness-migration.md
- spec/src/current/contracts/api/34-application-freshness-ownership.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/progress/M4/index.md
- spec/src/current/progress/M4/history/M4.9/index.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/native-repository.spec.ts#external publication refreshes
  automatically and preserves selection
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
gh run view 41537333075 --json conclusion,jobs,headSha,url
gh run download 41537333075 --dir /tmp/jj-store-cold-failures.RsC1dN
```

**Notes**

Integrated full qualification at `b751c03` retained the old operation in the
route for the implicit five-second Playwright poll deadline after an independent
CLI publication. The same run exposed four other cold-browser failures recorded
separately; two response-body races were repaired by E-CI-COLD-RESPONSE-001.
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
PATH="/tmp/jj-store-uv.5u5xwl:$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=native-repository.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact cold built-stack file passed all five tests in 30.7 seconds. The
external-publication assertion completed in 6.8 seconds: beyond the former
implicit five-second deadline and within the established 15-second cold
automatic-update bound. Fixture preparation took 211.398 seconds; embedded
storage reported 7.838-second cold and 8.171-second restart measurements.
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
npx --yes pnpm@10.32.1 exec prettier --check $(jj diff --name-only)
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Changed-file formatting, selected specification validation, spec-first
enforcement and the progress-ledger check passed.
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

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; Playwright
1.63.0; uv 0.9.28.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, event, route, snapshot or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The cold browser assertion deadline for independent native publication only.
Freshness producers, monitor cadence, serialized refresh behavior, route
selection and product contracts are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Repair the independent project-preview and reaction-geometry cold failures, then
rerun complete integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The same cold test still requires automatic external-publication refresh,
  the exact new operation and preserved bookmark/commit/change selections. Its
  deadline now matches the existing 15-second cold automatic-update bound.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.9 remains Done and M30 remains In progress. The exact cold gate passes,
  while two independent cold failures and complete qualification remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
