---
schema: 1
id: E-REUSABLE-AGENTS-006
---

# E-REUSABLE-AGENTS-006 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/Conversation*.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-work/Projects*Agent*.stories.svelte --reporter=verbose
```

**Notes**

Cold affected family:41files/216interaction-accessibility tests pass in185.81seconds, exit0. Browser ResizeObserver-loop diagnostic during runtime settings remains explicit; no Svelte/axe assertion weakened.
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
pnpm spec:first
```

**Notes**

Early audit correctly refused missing fresh evidence/ledger coverage before this new E006record was added. Final audit follows below.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Initial final-format inventory found7unformatted authored files; targeted Prettier repair preserves source/contracts. Final exact inventory passes.
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
pnpm check:progress
```

**Notes**

171tasks/863evidence records pass; M33/M33.3stayInprogress and E006retains pending wider qualification.
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
pnpm spec:first
```

**Notes**

After fresh evidence, provenance change also required its owning UI chapter. Added the exact unchanged presentation/ownership note; no gate removed.
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
pnpm spec:validate
```

**Notes**

7validators/4178chapters/207requirements pass.
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
pnpm spec:first
```

**Notes**

Final mapped gate:31protectedfiles/9canonicalchapters pass with new SDK contract, retained UI ownership, fresh pending evidence and explicit milestone review.
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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Final exact changed authored-file inventory passes after formatting the newly required UI chapter. No spec line limit changed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Aggregate cold catalog qualification is incomplete: parent-reproduced
  ClosedDetailsStaysAbsentAfterRepositorySettings fails and the full run stalled
  after that file. No release or complete UI qualification is claimed."
- The affected 41file/216story interaction/accessibility run passes with one
  ResizeObserver-loop browser diagnostic on
  RuntimeSettingsSubmitExactNegotiatedPreferencesWithoutSendingTheChatDraft;
  retain this diagnostic for wider UI qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node24.10.0; pnpm10.32.1; TypeScript5.9.3; @lapismd/jj-conversations0.1.0; installed Design Core0.5.1. Final archive SHA25647775f3e59fb09ed9db8759ca0461478f12e272e6c149231162ee69e68f26d46.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Original mention UTF8 hex token bytes, legacy UI pubkey property order, generated execution DTOs and native AgentStore/ThreadStore/request receipts unchanged. SDK projections retain generic full-width sequences; original UI alone converts to Number. Scoped client state is session-only and adds no durable encoding.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Independent headless ESM/type client and existing mention/delivery consumer adapters. Injected encoding/transport, immutable request binding, one concurrent submit, status-only unknown recovery, identity-only restart restoration, isolated receipt clones and explicit host finality. Normal workspace install builds public exports; SDK metadata has no workspace/source/runtime/UI dependencies. No Notes/Tauri, provider or durable backend acceptance claimed. Changes to shared node workspace/build inputs justify full web/catalog checks and the cold broader UI attempt. Focused success is not a full-suite pass; inherited architecture and catalog failures remain separately recorded.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Adopt shared typed coordination through source-backed host adapters, then focused local-history/Tauri and Notes first usable milestone.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 adds a public headless typed client contract and built
  workspace integration. Owning architecture/distribution/governance/docs
  chapters describe the new package. Existing JST-API-001/006 generated
  execution DTOs, transport routes and JST-COMPAT-001 receipt/token identities
  remain unchanged. Community ownership/credit stays with the app adapter;
  unknown SDK outcomes cannot replay prompts.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M33 and M33.3 remain In progress. Shared client artifact and legacy
  helper consumers are qualified, while source-backed store/workspace/context
  adapters, shared UI adoption and downstream integration remain required.
  M33.4-.8 remain Planned.

```
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mtyrvlvm
<!-- /evidence-field -->

