---
schema: 1
id: E-FORGE-FLOAT-DISCLOSURE-001
---

# E-FORGE-FLOAT-DISCLOSURE-001

<!-- evidence-field:id markdown -->
## Id

E-FORGE-FLOAT-DISCLOSURE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-020
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change okuyunot
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte
- spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md
- spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
- spec/src/current/progress/M31/index.md
- spec/src/history/2026/10/15/E-FORGE-FLOAT-DISCLOSURE-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-FORGE-FLOAT-DISCLOSURE-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
- web/src/lib/ui/store-forge/StoreForgeDetails.svelte
- web/src/lib/ui/store-forge/forge-inline.css
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t PRSummaryStaysAboveChanges
```

**Notes**

Before implementation: 1 failed, 5 intentionally skipped. The floating title trigger remained enabled. The existing interaction story now requires disabled floating disclosure and enabled disclosure after redock. Log .local/forge-float-disclosure-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t PRSummaryStaysAboveChanges
```

**Notes**

Exact after-run: 1 passed, 5 intentionally skipped. Floating disclosure is disabled; redocking re-enables it. Rendered details, file position/spacing, explicit CI inspection and retained PR/file behavior continue passing. Log .local/forge-float-disclosure-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte
```

**Notes**

Final direct-consumer run: 14 stories passed in a fresh browser process. Covers PR header/menu actions, collapsed editor activation while docked, rendered PR details, inline creation/recovery, drafts, float/redock, file switching and failure toasts. No unrelated stories or broader suite repairs. Log .local/forge-disclosure-consumers.log.
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
pnpm catalog:check
```

**Notes**

Zero Svelte errors/warnings and catalog boundaries pass (450 modules). Log .local/forge-disclosure-types.log.
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
node scripts/web-architecture.mjs
```

**Notes**

Maintained module/import boundaries pass (1956 modules).
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
pnpm check:ui-ownership
```

**Notes**

Owned UI and released dependency boundary passes; no new copies, dependencies, source aliases or foundation patches.
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
pnpm evidence:new -- --id E-FORGE-FLOAT-DISCLOSURE-001 --task M31 --requirement JST-UI-020 --requirement JST-API-001
```

**Notes**

Created fresh evidence/day links. Restored 446 unrelated generator-only navigation paths from the parent and reformatted M31. No prior evidence edited.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
curl --max-time 3 http://127.0.0.1:6046/index.json
```

**Notes**

Warm review Storybook stopped responding during HMR. Verified the task-owned listener cwd, command and port before restarting only that process. No shared development/test infrastructure change.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6046 --exact-port --no-open --disable-telemetry
```

**Notes**

Restarted the task-owned review environment after source settled; final index and contextual iframe return HTTP 200.
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
pnpm check:progress
```

**Notes**

Fresh record and explicit unchanged M31 review pass.
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
pnpm spec:first
```

**Notes**

Owning UI chapter and mapped API unchanged-contract confirmation cover this presentation change.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Document limits, governance and navigation pass.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M31/index.md web/src/lib/ui/store-forge/StoreForgeDetails.svelte web/src/lib/ui/store-forge/forge-inline.css
```

**Notes**

All changed authored files pass; generated evidence uses its canonical renderer.
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

Node v26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11/Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API/schema, forge action, editor/draft, pending request/receipt, credential, operation-pinned file/hunk, persistence, storage, manifest or WAL change. Only floating disclosure input and local presentation styles change; the same editor/content and detachable host remain mounted.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The floating PR title hides its disclosure chevron and uses a transparent button background with full title opacity. CSS pointer-events:none prevents mouse activation; the public shadcn Collapsible disabled state also prevents keyboard activation while floating. Docked PRs retain normal collapse/expand and header-pencil behavior. Existing maintained PR title composition and released Collapsible RootProps were inspected; no new Community source or foundation overrides. The existing focused interaction story adds floating-disabled and redocked-enabled assertions, preserving its file placement, rendered details and CI checks. Low-impact CSS values get no new mirroring tests. Acceptance is limited to the changed UI, relevant type/ownership checks and 14 direct Forge stories. Root aggregates, standalone inspection/creation-window geometry stories, unrelated Stack/controller/Rust suites, infrastructure repairs and visual baselines are out of scope under user direction and targeted policy. No screenshot comparison or baseline update; visual approval remains separate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the floating PR in http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 live-provider, native/platform, packaged and visual gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 specifies static floating PR disclosure, hidden chevron,
  transparent styling and CSS pointer suppression. JST-API-001 is explicitly
  unchanged: same forge actions, editor/drafts, exact pending intents and
  read/publication authority."

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FORGE-FLOAT-DISCLOSURE-001 details 2](details-02.md)
