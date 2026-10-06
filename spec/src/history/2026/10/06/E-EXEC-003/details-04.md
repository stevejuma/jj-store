# E-EXEC-003 final repairs and governance

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatRuntime.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte
pnpm exec prettier --write web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte web/src/lib/ui/store-agent-execution/StoreAgentExecutionNotice.svelte web/src/lib/ui/store-agent-execution/runtime-preparation.ts web/src/lib/ui/store-agent-execution/conversation-error.ts web/src/lib/controller/agent-execution.svelte.ts web/src/lib/ui/store-application/StoreApplication.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/fixtures/AgentExecutionFixture.svelte catalog/fixtures/agent-command-receipts.ts catalog/fixtures/agent-execution-events.ts
pnpm docs:review
pnpm docs:review application "Agent chats use the owned Community composer row and shared latest-scroll control; exact configuration receipts gate prompting and editor completion, and resume errors stay in the composer with explicit retained-context recovery."
pnpm dev:ui
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 pnpm dev:backend:restart
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH python3 /tmp/agent-resume-inspect.py
```

**Notes**

The first resume/runtime target failed 4/6 before the repair: no composer error,
and real dispatching receipts blocked Send/navigation. The layout target failed
on message/composer overlap. After repair, six runtime stories and the exact
long-history layout story passed, then the final 44-story direct-consumer target
in Run 7 passed. Intermediate assertions needed a real bounded receipt wait and
the fixture shell needed the same bounded viewport as the application. The
scroll button assertion now waits for its reactive visibility, preserving all
geometry, scrolling and keyboard assertions. Type checks exposed the wrong
controller route property and optional fixture preference shape; corrected
without widening contracts. Counted source limits exposed 301/303-line files;
receipt/history helper extraction and concise navigation callbacks restored
the existing 300-line budgets. Some runs were interrupted by incoming steering;
they are not counted as passes. Final type, source and ownership results are
recorded in Runs 8/11.

Bare docs:review correctly failed for stale application coverage; the explicit
review after updating both agent guides passed. Additional changed-file
Prettier runs covered the layout story/feed, fixture receipt/history helpers,
runtime receipt unit, shared editor/settings story and both guides. No ignored
diagnostic is represented as an actual passing run.

The direct installed-runner load-only diagnostic returned session_not_found
(RPC -32603, provider details field). Its first attempt lacked runner PATH and
was repaired. The bounded diagnostic terminated its owned process group and
sent no prompt/tools. .local/agent-resume-diagnostic.log retains only its safe
classification, code and field names. Raw provider errors and credentials were
not published through ACP or saved in this evidence. jj-acp was read-only.

Live CUA inspection in a temporary background tab confirmed separate message
viewport/composer rows (both boundary coordinates 522px), a keyboard-triggered
shared latest-scroll control and the entire newest message above the composer.
Screenshots .local/agent-composer-row-final.png and
.local/agent-composer-latest-final.png are observation evidence only. The
temporary proof tab was closed; the user-visible fixture is preserved.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm progress:history
pnpm check:progress
pnpm spec:first
pnpm spec:validate
pnpm docs:check
```

**Notes**

Initial check:progress and spec:first rejected a temporary pending run result
(runs admit only pass/fail) and missing file coverage for the newly generated
M13.4 history. This record now preserves those actual failures, and E-EXEC-005
names its generated milestone history. Spec validation and docs:check passed;
format checking identified nine authored/generated files needing formatting.
They were formatted before repeating governance checks. Final same-change
evidence, milestone histories and protected chapter review follow below.
All root aggregates, unrelated Rust/UI packages, release qualification and
visual baselines remain outside this targeted slice.
<!-- /run -->
<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
pnpm spec:first
pnpm spec:validate
pnpm docs:review application "Final formatted agent guides and owning chapters describe Community inbox composition, shared runtime editing, exact configure receipts, safe composer resume errors and explicit recovery; source and publication boundaries are unchanged."
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/current/contracts/collaboration/36-interactive-agent-execution.md spec/src/current/progress/M13/history/M13.4/index.md spec/src/current/progress/M13/index.md spec/src/current/progress/M23/history/M23/index.md spec/src/current/progress/M23/index.md spec/src/current/verification/collab-03.md spec/src/current/verification/ui-02.md spec/src/previous-locations.md
pnpm docs:check
```

**Notes**

Progress passed for 144 tasks and 625 evidence records; spec-first passed for
127 protected files and seven owning canonical chapters. All seven validators,
3059 chapters and 178 requirements passed. The final manual closure is 141
files, with valid links/anchors. A changed-file Prettier check passed after
formatting the nine files; Rustfmt, package boundaries (17 packages), source
architecture (1243 modules), copied UI ownership, 44 production stories,
40 controller/direct-consumer tests and cold native browser/restart evidence
also passed. No visual baseline updates or root aggregate lanes.
<!-- /run -->
<!-- /evidence-field -->
