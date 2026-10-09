---
schema: 1
id: E-CHAT-LIVE-013
---

# E-CHAT-LIVE-013 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Initial documentation review observed three unrelated generated progress pages before restoration and rejected an overall pass result with retained failures/open gates. Record corrected to pending; targeted successful checks remain explicitly recorded (.local/inbox-tree-spec-first.log).
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial ledger review reported the same transient generated-history coverage and overall-result issues; targeted implementation results remain passing (.local/inbox-tree-progress-check.log).
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
pnpm spec:validate --only governance,summary,fileLimits,book
```

**Notes**

Four document validators pass across 3335 chapters (.local/inbox-tree-spec-check.log).
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
pnpm spec:first
```

**Notes**

Final gate passes for eleven protected files and three canonical chapters (.local/inbox-tree-spec-first-final.log).
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
pnpm check:progress
```

**Notes**

Final ledger passes for 146 tasks and 696 evidence records; M24 remains In progress (.local/inbox-tree-progress-check-final.log).
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
pnpm exec prettier --write spec/src/current/progress/M24/index.md spec/src/current/progress/M24/history/M24/index.md
```

**Notes**

Milestone prose formatted; the generated history is refreshed into its original generator format below.
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
kill -TERM 11727
```

**Notes**

Stopped only the supplemental Storybook dispatcher whose verified command served 6048. Existing live Vite/backend were preserved.
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
pnpm exec prettier --check web/src/lib/ui/community-inbox/CommunityInbox.svelte web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte web/src/lib/ui/community-inbox/CommunityInboxItem.svelte web/src/lib/ui/community-inbox/inbox.css web/src/lib/ui/community-inbox/model.ts web/src/lib/ui/store-agent-execution/session-presentation.ts web/tests/managed-agent.test.ts web/tests/agent-inbox-groups.test.ts catalog/components/store-application/AgentInboxTree.stories.svelte catalog/fixtures/AgentInboxTreeFixture.svelte docs/ui-provenance.json spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M24/index.md
```

**Notes**

All matched changed authored files pass formatting (.local/inbox-tree-format-check.log).
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 independent provider/adapter, permission/cancellation, native IME,
  desktop/platform and visual gates remain open.
- Prior broad legacy-story failures and the development acceptance-script size
  gap remain recorded in E-CHAT-LIVE-011; these unrelated lanes were not
  repeated.
- Existing temporary foundational dependency validation setup is retained; no
  manifest or lockfile changes.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, request, receipt, execution/binding, captured source or persisted preference format changes. SessionInboxItem adds exact repository ID/name solely to its owned presentation model.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only sidebar presentation and direct adapter/story consumers. Rust, generated public contracts, root aggregate lanes, broad unrelated stories and visual baseline updates are out of scope under targeted verification. Fresh Storybook/Vite browser test processes and a fresh live browser page/reload were used; no built-bundle/lifecycle changes require a cold backend restart.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M24 In progress and continue its independent qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 specifies repository disclosure groups, exact identity
  grouping, compact children and responsive ellipsis. JST-API-001 and
  JST-GOV-002/008 authorization, publication and UI ownership boundaries remain
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress; focused repository-sidebar acceptance does not
  close independent platform, provider, native IME or visual approval gates.

```
<!-- /evidence-field -->
