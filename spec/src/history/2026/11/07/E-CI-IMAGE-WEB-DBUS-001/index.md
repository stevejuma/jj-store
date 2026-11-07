---
schema: 1
id: E-CI-IMAGE-WEB-DBUS-001
---

# E-CI-IMAGE-WEB-DBUS-001

<!-- evidence-field:id markdown -->
## Id

E-CI-IMAGE-WEB-DBUS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/publish-main-image.yml
- scripts/ci-workflows.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/07/E-CI-IMAGE-WEB-DBUS-001/index.md
- spec/src/history/2026/11/07/SUMMARY.nav.md
- spec/src/history/2026/11/07/index.md
- spec/src/history/2026/11/SUMMARY.nav.md
- spec/src/history/2026/11/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- .changeset/admitted-history-bridge.md
- .changeset/ci-cold-browser-freshness.md
- .changeset/core-ui-acceptance.md
- .changeset/crate-overviews.md
- .changeset/desktop-browser-development.md
- .changeset/headless-capture.md
- .changeset/history-wire-integers.md
- .changeset/injected-prompt-coordination.md
- .changeset/injectable-collaboration-stores.md
- .changeset/linux-vault-directory-sync.md
- .changeset/local-history-profile.md
- .changeset/main-local-qualification.md
- .changeset/native-agent-privacy.md
- .changeset/remote-test-audit-repairs.md
- .changeset/repository-removal.md
- .changeset/reusable-session-acquisition.md
- .changeset/reusable-workspace-discovery.md
- .changeset/search-save-notice.md
- .changeset/shared-conversation-stream.md
- .changeset/shared-read-admission.md
- .changeset/standalone-companion-provider.md
- .changeset/typed-conversation-client.md
- .changeset/typed-history-client.md
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
GitHub Actions run 42310110333 job 128209237872 on a630e97f23a78572b3cc11619e1540f5fabf2ad8
```

**Notes**

The main-image web job ran `pnpm web:build` with the Node setup and without
credential-store headers. Help staging compiled `docs_reference`, and
`libdbus-sys` failed because `dbus-1.pc` was absent.
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
node --test scripts/ci-workflows.test.mjs && node scripts/check-progress.mjs && spec-validator validate --only governance,summary,fileLimits,verification --json && spec-validator first --base main --head @ --json
```

**Notes**

The workflow regression requires `libdbus-1-dev` before `pnpm web:build`.
Progress and spec-first passed for this change.
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

Node 22.23.4 on the hosted runner; Rust toolchain 1.99.0; libdbus-sys 0.2.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No package, Cargo, archive or image-tag format changes. The web job installs
the existing credential-store build packages before staging Help.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The main container web bundle can compile the documentation reference that
`web:build` stages into Help. Architecture and index publication stay gated on
that bundle. The removed changeset files were consumed by Version Packages;
the progress gate still required them because that bot pull request skips it.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge the workflow repair and rerun the main image publication for the same
source class. GHCR publication remains unqualified until that run succeeds.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 still publishes the browser and Help in the main image.
  The web job now installs pkg-config and libdbus-1-dev before web:build
  because Help staging compiles docs_reference.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This restores the web bundle input for the
  main image. Cargo, native bundles, Pages and GHCR publication stay open.
```
<!-- /evidence-field -->
