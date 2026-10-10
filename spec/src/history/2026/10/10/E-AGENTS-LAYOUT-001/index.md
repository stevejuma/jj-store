---
schema: 1
id: E-AGENTS-LAYOUT-001
---

# E-AGENTS-LAYOUT-001

<!-- evidence-field:id markdown -->
## Id

E-AGENTS-LAYOUT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-012
- JST-COLLAB-034
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-agents/CommunityAgents.svelte
- web/src/lib/ui/community-agents/agents.css
- web/src/lib/ui/community-agent-profile/CommunityAgentProfile.svelte
- web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte
- web/src/lib/ui/store-settings/StoreAgentChatGptAccounts.svelte
- spec/src/current/contracts/ui/13-shared-application.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/collaboration/41-chatgpt-plan-provider.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/agents-presentation-review.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/history/2026/10/10/E-AGENTS-LAYOUT-001/index.md
- spec/src/history/2026/10/10/E-AGENTS-LAYOUT-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
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
jj --no-pager st
CUA separate live Agents tab; read-only DOM geometry
```

**Notes**

Default workspace clean at parent ca656a6d; enclosing change qxortzqu. Before: gallery limited to 76rem and centered; compact rail48 px, tooltip container32 px, button40 px, avatar bounds10–46 px, four-pixel center error. New chat lacks its icon. View sessions is a header action. Manage usage is an unstyled external link; account actions use the vertical side column.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentSessions.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatGpt.stories.svelte
```

**Notes**

14 agent execution, 11 session/profile, and 2 ChatGPT account interaction/accessibility stories pass. These direct families cover retained drafts, lifecycle actions and external link protection. Initial -t filters for Standalone agent management and compact-profile-lifecycle-actions matched zero cases (14/11 skipped); those runs are not acceptance and each complete affected file was rerun. Storybook starts fresh for each command; Vite default-config diagnostics remain. Visual baselines stay unapproved.
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
pnpm web:check
pnpm check:ui-ownership
```

**Notes**

Web check: zero errors/warnings after all presentation edits. Ownership validation passes. No new runtime, catalog dependency, public type or upstream package edit.
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
CUA live gallery/details/session navigation; read-only DOM geometry; screenshots
```

**Notes**

After: full-width page with16 px header inset. Compact avatar bounds6–42px in48 px rail: equal6 px clearances; trigger4–44px. View sessions is first208 px tile before Add agent, with the same transparent/white rest and shared hover treatment; only centered icon has muted background. New chat renders one24 px Lucide icon, matching profile actions. View sessions navigates to the existing inbox. A transient HMR update error reading fn blanked the review tab; explicit reload restored the fresh component. A subsequent direct card name click opened details. Original tab was retained independently.
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
CUA Settings → Agents → ChatGPT accounts; read-only DOM geometry; screenshots
```

**Notes**

Manage usage retains its HTTPS destination, _blank and noopener noreferrer, using the existing Design Core outlined link convention. Continue, Check accounts and Manage usage share one row below the description: all36 px high, same909.17 px top; description bottom893.17 px leaves16 px gap. Check/Manage are160 px wide; Continue accommodates its longer label. Per-account and pending actions use existing full-width/wrapping Settings modifiers. No authentication or sign-out action was exercised live. Screenshots inspected locally; no baseline approval.
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
pnpm evidence:new -- --id E-AGENTS-LAYOUT-001 --task M24 --requirement JST-UI-015 --requirement JST-UI-012
pnpm progress:history
pnpm check:progress
pnpm spec:validate
```

**Notes**

Owning requirements and both milestone reviews updated; unrelated navigation-only generator changes restored. The first progress check rejected unsupported named Story references in the Tests field and one omitted generated navigation path. Removed those references (Story acceptance remains named in Run 2) and included that path before revalidation. Progress passes with 146 tasks and 725 evidence records; spec validates seven validators, 3452 chapters and 181 requirements. Changed-file Prettier check passes. The documented spec:search command with an extra -- was rejected; owning requirement was located and read directly with rg. No root aggregate, Rust, backend build/restart, package publication or visual baseline lane; this is an owned UI presentation slice.
<!-- /run -->

<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved; this is focused presentation acceptance.
- M24/M25 provider, permission/cancellation, desktop/platform and prior ResizeObserver gates retain their independent status.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Default Jujutsu change `qxortzqu`, parent ca656a6d; Jujutsu 0.45.1, Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Vitest 4.1.11, released Design Core 0.2.0 and Lucide 1.21.0. No backend rebuild/restart.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only Svelte/CSS changes and canonical Markdown. No stored formats, generated contracts, credentials or provider protocol changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

JST-UI-015 gallery header width, body sessions navigation and profile action icon; JST-UI-012 collapsed footer geometry; JST-COLLAB-034 ChatGPT account action presentation. Community source and Design Core Button inspected first. Existing owned styling conventions and public primitives retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain unapproved visual baselines and independent M24/M25 qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning UI chapters specify full-width Agents heading, body sessions tile, profile icon and centered collapsed footer; ChatGPT provider chapter specifies below-description action rows and outlined usage link. Navigation, execution and authentication semantics remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M25 remain In progress. This slice qualifies focused presentation only; visual baselines and independent runtime/platform gates remain open.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


