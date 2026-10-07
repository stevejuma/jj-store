---
schema: 1
id: E-FLEET-003
---

# E-FLEET-003

<!-- evidence-field:id markdown -->
## Id

E-FLEET-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-API-001
- JST-GOV-008
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

slztzzxo (enclosing Jujutsu change; parent e668fbfb)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- spec/src/current/contracts/ui/29-editable-agents-settings.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/07/E-FLEET-003/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-FLEET-003/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/store-settings/StoreAgentRuntimeSummary.svelte
- web/src/lib/ui/store-settings/StoreAgentRuntimes.css
- web/src/lib/ui/store-settings/StoreAgentRuntimes.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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

macOS ARM64; Node.js 26.9.0; pnpm 10.32.1; Vitest 4.1.11; locked Svelte 5 and released Design Core dependencies; existing Vite HMR preview on 7474/backend 7476 preserved.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No runtime HTTP/schema, installer, native storage, publication, session or request-identity format changes. Runtime tooling icons do not assert saved-agent presence.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Runtime summaries place the harness avatar in the first column and name, status, description and version text in the second. Both descriptions use the Community secondary text token (12px at the current scale). Loading skeletons retain this alignment. Trailing actions, installation confirmation, readiness and retained recovery remain unchanged. Extracted a private summary component to retain the 300-line architecture budget. This is one settings presentation surface; no shared composer or controller change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep the fleet workspace and preview open for follow-up. Pursue the independent M24 adapter, installed live permission/cancellation and desktop gates separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 owning Agents settings chapter specifies separate avatar/text
  columns and secondary descriptions. JST-API-001, request identities,
  installation, readiness, capture, ACP dispatch and native authority remain
  unchanged. Documentation sources are reviewed under JST-DOC-001/002.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M24 stays In progress with the same independent adapter, live
  permission/cancellation and desktop qualification gates. E-FLEET-003 reviews
  only the requested runtime presentation and local integration scope; prior
  E-FLEET-001/002 payloads and gate selection remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No root aggregate spec:check/check/test/build, Rust qualification lane, new native-agent replay, installed tooling replacement, package publication, service/backend restart or visual-baseline update. No production startup/lifecycle/bundler behavior changed, so UI-only policy does not require a new cold built-bundle lane; prior fleet qualification remains separately recorded in E-FLEET-001/002. No new tests mirror this reversible presentation; the six existing runtime interactions and accessibility checks cover its complete direct consumer surface.
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
Browser comment and cua_repl observation of Settings → Agents → Runtimes on 7474; inspected Community Settings and released Design Core Field presentation
```

**Notes**

Before: avatar/name/status share a heading line and descriptions start under the avatar. User requested separate avatar/text columns and smaller descriptive text. Existing Field primitives and owned Community avatar/chip are retained; no external checkout imports.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search -- JST-UI-012; pnpm spec:search JST-UI-012
```

**Notes**

The documented extra -- was forwarded to spec-validator and rejected. The corrected search succeeded and the returned owning Agents settings chapter was read. No specification tool was changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentRuntimes.stories.svelte
```

**Notes**

Before: 6/6 passed in 7.69s. Initial two-column implementation: 6/6 passed in 7.96s. Final owned-summary extraction: 6/6 passed in 8.79s in a fresh Vitest/browser process. Existing explicit install/update, readiness/Unknown recovery, retained request, prerequisite and loading interactions/accessibility pass. No screenshot baseline update.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm check:ui-ownership; pnpm check:architecture (initial layout)
```

**Notes**

Web types had 0 errors/warnings and UI ownership passed. Architecture caught 310 counted lines in StoreAgentRuntimes.svelte, above 300. Extracted the runtime summary into its own private presentation component; no budget/baseline or assertion weakened.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm check:architecture && pnpm check:ui-ownership (final summary)
```

**Notes**

Final types: 0 errors/warnings; architecture: 1304 modules; UI ownership passed. Source changes remain confined to the runtime settings composition.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review application "Runtime summaries now use an owned presentation component with the harness avatar in the first column and identity, status, smaller descriptions and versions in the second. Loading rows match. Installation, readiness, recovery and trailing actions retain their qualified behavior."; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm docs:check
```

**Notes**

Final source review and Help closure pass: 9 diagrams in both themes, 141 files, valid links/anchors. An initial review/check also passed before extracting the summary and was refreshed afterward. User fixture, runtime, chats and preferences were preserved.
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
cua_repl HMR screenshot and read-only DOM bounds/font observation on the original preview tab
```

**Notes**

Both runtime avatars occupy a 32px first column (left 322px); all descriptive/version text shares the second-column left edge (364.4px) at 12px. Labels, Ready chips, menus and Cursor Update guide remain visible. Screenshot: .scratch/runtime-avatar-columns.png. No navigation, settings change, installation or prompt; original tab retained as a deliverable.
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
jj --no-pager st/log/workspace list in fleet and default jj-store/jj-acp workspaces; jj --no-pager log -r "95335454::slztzzxo"
```

**Notes**

Default app and harness workspaces were empty and still based at the fleet parents (95335454 and f4e97542). App stack contains 73e04276/e668fbfb plus this UI slice; harness stack contains 5bbee2c6. User explicitly requested landing them locally and retaining the workspaces. No new source merge or release qualification is inferred.
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
pnpm progress:history; restore only unrelated token-equivalent generated Markdown; pnpm exec prettier --write changed files; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check changed files
```

**Notes**

Progress: 145 tasks and 649 evidence records; spec: 7 validators, 3145 chapters, 181 requirements. Focused formatting passed. Retained original evidence payloads and meaningful navigation; no milestone status/gate change or broad-suite claim.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


