---
schema: 1
id: E-M14-005
---

# E-M14-005

<!-- evidence-field:id markdown -->
## Id

E-M14-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.2
- M14.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

integration
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
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
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj rebase -s qmuvzusm -o lvpkolvw; jj squash --into qmuvzusm --use-destination-message; jj squash --into ksppxmms --use-destination-message; jj squash --into sunmproo --use-destination-message
```

**Notes**

Rebased the four settings slices onto current main 36e1f8a0, preserving both newer Community UI commits. Resolved only API/UI prose and milestone table/evidence conflicts. All 196 main evidence entries plus E-M14-001 through E-M14-004 preserved unchanged; no Rust changes relative to the previously verified settings tip.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/http-workspace-links.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

30 tests across five focused files passed after rebase, including settings/transport and the current main route/navigation consumers.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Both checks passed with zero errors/warnings; 100 catalog modules accepted.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm check:progress
```

**Notes**

132 requirements and 111 tasks accepted after conflict resolution.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module (pre-restart snapshot probe)
```

**Notes**

Probe used only the protocol header and received 403 from required LocalTrusted same-origin admission. No mutation occurred. Later probes use the complete documented development headers.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module (import apiHeaders from scripts/dev-demo-search.mjs)
```

**Notes**

Inspection helper is module-private, so the import failed before any request. No pre-restart snapshot comparison is claimed; repository identity and runtime metadata were recorded successfully.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj new tzxzmqxq -m 'Verify landed local settings and restart development demo'
```

**Notes**

Moved the clean main workspace onto the verified rebased stack. Main now includes graph/search separation, workspace links API/UI and independent capture/drain ownership.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:restart
```

**Notes**

Rebuilt and restarted the main-owned LocalTrusted demo at http://127.0.0.1:7460/. Existing fixture reused; repository 1bdb89c409c1b891ac7f180c01a265d5 retained. Old server PID 76759 replaced by 85400. Embedded RustFS startup/readiness and Search readiness passed; restart startup measured 12930 ms, server plus storage 172288 KiB RSS.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module (documented LocalTrusted capability/workspace/search probes)
```

**Notes**

Live GET capabilities, local/workspaces and search/status all return 200. local_workspace_links=true; version 1 links revision 0 with an empty list. Search generation present at repository/durable/indexed position 101, is_stale=false, degraded=false. Saved baseline/post-restart runtime metadata proves the same repository ID with a replacement server process.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cua_repl: createBrowserTab; profile menu > Settings > Repository; Refresh folders; getAXState; dev.logs; getScreenshot; markDeliverable
```

**Notes**

Fresh in-app browser loaded the actual rebuilt native demo, displayed the Local folders form and separate Graph index section, and completed folder refresh with no error/warning console messages. Left the Repository settings page open for review. Read-only live UI/API verification; no folder was linked and no visual baseline approval is claimed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

Final evidence formatted; 132 requirements and 111 tasks validated. Integration evidence changes only the ledger; no new protected implementation edits.
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

Local integration and explicitly requested restart of the existing main development demo. Focused tests cover the combined UI consumers. Previously qualified Rust sources unchanged; the restart command rebuilds browser, native binaries and embedded runtime. Full aggregate tests, release qualification and visual approval are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No format change in this integration. Existing native demo identity, repository and RustFS volume retained by normal dev:restart. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; pnpm 10.32.1; macOS ARM64; existing locked Svelte/Vite and embedded RustFS toolchain.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review Repository &gt; Local folders in the main demo; continue M14.3 agent setup controls and automatic recovery.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Agent enable/setup/remove Settings UI, service-owned recovery worker,
  Search/Storage/Sync management and visual baseline approval remain pending.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/landing-*.log in jj-store-local-settings; .scratch/settings-landing-before.json and .scratch/settings-landing-restart.log in main. Post-restart live probes: .scratch/settings-landing-after.json. In-app browser review tab is retained.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


