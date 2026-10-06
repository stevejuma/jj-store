---
schema: 1
id: E-M13-029
---

# E-M13-029

<!-- evidence-field:id markdown -->
## Id

E-M13-029
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.5
- M14.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

development-review-startup
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qvtsvzty
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Verified main merge rryrrvzz/9de71790 committed with clean working copy. Existing main Vite runtime still uses the previous backend; repository fixture identity is 1bdb89c409c1b891ac7f180c01a265d5.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; newly rebuilt main target/debug/jj-store; Vite HMR at 7460 and LocalTrusted backend at 7462; existing embedded RustFS runtime and persistent volume.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product, API, storage, manifest, WAL, companion or server-configuration format change. Installation JSON and app-saved ui-server.toml are byte-identical before/after; fixture marker and existing repository/binding identities retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/native-repository-creation.spec.ts#cold native creation
  recovers exact intent across entrypoints and reload
- web/tests/browser/repository-conversion.spec.ts#cold export preflight, pending
  reload, exact retry and verified report
<!-- /evidence-field -->

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
pnpm dev:ui
```

**Notes**

dev-restart: owned main backend/UI replaced after the verified merge commit and completed pnpm dev:backend:build. Search ready, fixture preserved, main workspace and expected loopback ports confirmed. Existing app-saved capture settings retained; no reset or unrelated daemon restart.
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
node --input-type=module (Playwright read-only main review and persisted-state comparison)
```

**Notes**

review-after: UI 200 and zero runtime errors; both ordinary protocol discovery/binding reads and native/local capability reads return 200. Native creation/conversion advertised. Server/UI PIDs changed to 69136/69137. Installation/config hashes, exact fixture marker, native repository IDs and registered folder bindings retained.
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
pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check .local/repository-conversion-main/restart-evidence.json .local/repository-conversion-main/restart-ledger-header.md
```

**Notes**

restart-progress/restart-spec/restart-format-check pass: 137 tasks, 22 chapters/168 requirements and separately formatted new evidence/header. Final evidence append checked with the same document consumers; historical objects preserved. No product tests rerun for this operational documentation-only slice.
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

<!-- evidence-field:scope markdown -->
## Scope

User-authorized persistent main review restart only. Existing runtime ownership checks select the main backend/UI and their storage child; installed commands, independent daemons, source repository migration, fixture reset and visual baselines remain out of scope. Qualified feature/contract/cold evidence remains E-M13-028; this documentation-only slice records actual review readiness.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review server-native creation and both independent conversion controls in main at http://127.0.0.1:7460/. Remaining portable/code profiles and broader M13 gates stay separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


