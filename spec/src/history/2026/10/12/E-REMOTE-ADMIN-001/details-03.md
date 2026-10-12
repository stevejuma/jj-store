---
schema: 1
id: E-REMOTE-ADMIN-001
---

# E-REMOTE-ADMIN-001 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte
```

**Notes**

Final border-clearance correction: 12 passed. This scoped CSS refinement retains all forms, routes and actions. Cold real-provider acceptance also passed against final Rust/route/controller code and the preceding compact dialog presentation; exact final CSS was checked in a fresh Storybook browser and live HMR app.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
python3 -m py_compile scripts/remote-agent-administration-acceptance.py; node --check scripts/remote-agent-administration-browser.mjs; targeted changed-file Prettier check; cargo fmt --all -- --check
```

**Notes**

Python/JS syntax and Rust formatting passed. The first changed-file Prettier check found three newly generated/reviewed Markdown/JSON files; they were formatted and the same targeted check passed before the later caller-UI refinements.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

Caller-UI steering: New chat removes only the raw first-prompt banner, leaving captured launch context untouched. AI configuration alone adopts an owned Settings card with tinted icon header, edge-to-edge divided rows, right-aligned controls and retry footer. First provider run failed an obsolete nested-border expectation and its following story cascaded due to the unclosed dialog. Updated the assertion to verify the card, no nested border, row divider/control alignment, unchanged Agent name field and footer placement. Exact provider rerun: 3 passed; final three-family run records its actual count separately. Final: 21 passed across all three families. Live main editor inspection verifies the tinted icon header, full-width row dividers, trailing controls, and separate Retry footer. No agent settings were saved or prompts sent during the visual review.
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
pnpm spec:first
```

**Notes**

Final governance initially found the service change missing its mapped mutations chapter. Updated contracts/mutations/24-remote-agent-authority.md and recorded JST-MUT-001; no semantic policy or native publication relaxation.
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
Changed-file pnpm exec prettier --check
```

**Notes**

Final record refresh found only the new M24 history row unformatted. Kept it in the existing evidence table and formatted that file; no unrelated historical rows changed.
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
pnpm check:progress; pnpm spec:first; pnpm exec spec-validator validate --only governance,summary,fileLimits
```

**Notes**

Progress passes for 152 tasks and 776 evidence records; spec-first passes for 62 protected files and 10 canonical chapters after the JST-MUT-001 chapter addition. Three validators pass across 3697 chapters and 188 requirements. Final selected-record outcomes preserve both M24 and M27 In progress.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first
```

**Notes**

Final pass metadata initially retained future platform/provider gates in remaining, which is reserved for gaps in the selected evidence slice. Kept those limits explicitly in scope, qualification_limits and milestone review; remaining is empty because this named slice is complete. Original repaired failures stay in the record.
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
Changed-file pnpm exec prettier --check; pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

All 64 changed Markdown/JSON/JS/TS/Svelte files pass formatting. Four validators pass across 3697 chapters and 188 requirements; authored source/evidence stays within limits and book links are valid. Final web/catalog type checks and production build remain passing; the 21 caller stories, 12 connection stories and real-provider acceptance logs are retained.
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

macOS ARM64; Node 26.9.0; Python 3.14.7; Vitest 4.1.11; Vite 8.3.1; Codex CLI 0.156.1 and installed ACP adapter 1.13.1; bundled control-version-3 jj-acp. Existing paired development JJ/server and real embedded RustFS. No dependency overrides or publication.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Protocol 1/OpenAPI additive endpoints and ManageAgents operator permission. Existing format-2 remote_agents TOML; full-file SHA-256 conditional revision and shared settings lock. Existing native request IDs/snapshots/receipts, manifest/WAL, directory and AgentStore formats unchanged. Credentials remain memory-only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Implement trusted-operator sharing/configuration and same-principal identity/Write onboarding, keep existing execution permissions independent, and qualify the macOS installed-Codex path through a local second instance. Retain original unknown intents; no automatic retry, runtime installation, host restart API, home provisioning, source upload or migration. Compact Add server header and authenticated installation-view reload restoration are included. Focused UI/Rust/public-contract/manual lanes are required by this cross-layer surface. Root aggregate spec:check/check/test/build, full Rust or Storybook suites, Linux, desktop and visual baseline lanes are out of scope. Later user steering removes the New chat context banner and styles only the Add/Edit agent AI configuration section; captured prompts, values, drafts, pending commands, provider discovery and other editor fields remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:diagnostics markdown -->
## Diagnostics

Inherited diagnostics are separate from functional results: missing Svelte-config notices, released dependency CSS :global minification warnings, existing dynamic import/large chunks and macOS linker __eh_frame warnings. Initial storage-budget preflight blocked; guarded cleanup resolved it while preserving development data. No warning thresholds or baseline was weakened. One early controller invocation used the web Vite config; it was replaced by the owning catalog-unit configuration and all named targets passed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify M27 Linux real-storage, desktop delivery and other provider profiles; retain the independent M24 and visual gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-SEC-009/API-011/UI-022 govern independent manage-agents operator
  configuration and native Write onboarding. Canonical
  security/API/CLI/collaboration/compatibility/UI/documentation chapters
  updated. Existing repository authorization, native
  receipts/storage/configuration formats and independent global settings
  contracts remain unchanged. JST-UI-015/016 clarify the banner omission and
  opt-in Settings treatment for AI configuration; runtime/launch values and
  retained context are unchanged. JST-MUT-001 remote authority chapter records
  independent operator configuration and the unchanged native conditional
  Write/receipt publication contract.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M27 remains In progress. macOS cold operator UI and installed Codex
  two-instance real-storage acceptance qualify this slice. Linux, desktop, other
  providers, automatic home provisioning/Git transfer and hostile-user OS
  sandboxing remain separate or deferred; no visual-baseline promotion. M24
  remains In progress; caller-UI presentation refinements retain the existing
  ACP, prompt context, draft and command contracts and do not promote its
  provider/platform gates.

```
<!-- /evidence-field -->

<!-- evidence-field:qualification_limits list -->
## Qualification limits

- Linux real-storage, desktop delivery and other installed-provider profiles
  remain unqualified.
- Hostile-user OS sandboxing, automatic personal-home provisioning and Git
  transfer are separate/deferred.
- Visual baselines remain unapproved; inherited build notices are recorded
  separately.
<!-- /evidence-field -->

