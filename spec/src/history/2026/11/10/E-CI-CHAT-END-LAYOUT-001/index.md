---
schema: 1
id: E-CI-CHAT-END-LAYOUT-001
---

# E-CI-CHAT-END-LAYOUT-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CHAT-END-LAYOUT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte
- catalog/components/store-application/AgentChatLayout.stories.svelte
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M30/index.md
- spec/src/current/verification/ui.md
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
pnpm test:storybook --shard=12/12 --reporter=default --reporter=junit --outputFile=.release/ci-results/storybook-12-12.xml
```

**Notes**

Hosted PR run 42886616289, job 130076773108, source
449de49476e3564ae9620f3f4c75dc0a38b498ac: 80 passed/1 failed in segment
12/12. The initial Jump to End stopped at scrollTop 4069 for a 4828-pixel
scroll range and 736-pixel viewport. The draft bottom was 848.859375 while the
viewport bound was 843, so the existing 1-pixel full-draft assertion failed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook -- catalog/components/store-application/AgentChatLayout.stories.svelte --testNamePattern=MessagesScrollAboveTheBottomComposerWithKeyboardJumpToLatest
```

**Notes**

The exact hosted story passed 1/1 on the local Linux machine before editing,
confirming the hosted failure was timing-sensitive rather than an invariant
layout failure.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook -- catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

The first post-edit cold attempt collected zero tests because the browser could
not fetch the dynamically imported story module. This was a local Vite/browser
startup failure, not a product assertion; it is retained rather than used as
qualification.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook -- catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

The immediate fresh-process rerun passed all 3/3 stories. The strengthened
focus regression adds 24 pixels of layout on the next animation frame and
retains the exact draft-within-viewport assertion.
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
pnpm client:build && for iteration in $(seq 1 10); do pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte --testNamePattern=MessagesScrollAboveTheBottomComposerWithKeyboardJumpToLatest --reporter=dot || exit 1; done
```

**Notes**

The exact original story passed four consecutive fresh Chromium processes.
The fifth process then collected zero tests with the same local dynamic-import
startup failure as Run 3, so the intended ten-run stress command is not claimed
as passing and was not automatically retried.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook -- --shard=12/12
```

**Notes**

The exact hosted segment passed 67 tests including the repaired chat-layout
file, but failed one unrelated Workspace Files readiness assertion, two story
module imports and one iframe connection. The run ended with 12 files passing,
3 files failing, 68 tests collected, 67 passing and 1 failing. These local
harness/readiness failures remain distinct and this run is not qualification.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:check
```
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:test
```

**Notes**

All 193 catalog unit files passed: 1,221 tests passed and one existing test was
skipped.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec spec-validator doctor && node scripts/spec/progress-validation.mjs && pnpm spec:validate && pnpm spec:first --base 449de49476e3564ae9620f3f4c75dc0a38b498ac --head @
```

**Notes**

Doctor passed 10 checks with the existing script-alias warning and progress
validation passed. The default-runtime specification validation then rejected
M30 at 407 lines versus its 400-line limit. Spec-first did not run. The M30
update was folded into its existing chat-readiness paragraph before the native
rerun; the limit was not raised.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first --base 449de49476e3564ae9620f3f4c75dc0a38b498ac --head @
```

**Notes**

All eight validators passed across 4,633 chapters and 211 requirements.
Spec-first then rejected two `#StoryName` test entries because named-test
resolution applies to conventional test sources, not Svelte story exports. The
evidence now names the governed story file while retaining exact story names in
the validation notes.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first --base 449de49476e3564ae9620f3f4c75dc0a38b498ac --head @
```

**Notes**

All validators passed again. Spec-first confirmed that Svelte story files are
not accepted in the structured test-reference field. The field is now empty as
in other Storybook evidence; the story remains in Files and every exact command
and story name remains in Validation.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first --base 449de49476e3564ae9620f3f4c75dc0a38b498ac --head @
```

**Notes**

Validation passed. Spec-first then required the API-side unchanged-contract
review mapped from the protected `web/` source path. The interactive execution
chapter now records explicitly that the next-frame presentation repair adds no
command, receipt, schema or replay authority. Its first form added two lines to
an already 400-line chapter; validation rejected 402 lines, so the review was
folded into the existing paragraph without raising the limit.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:ui-ownership && JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate && JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first --base 449de49476e3564ae9620f3f4c75dc0a38b498ac --head @
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete hosted required checks on the repair head.
- Keep the independent local Storybook startup/readiness failures visible.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.19.0, pnpm 10.34.5, Storybook 10.5.7 and Vitest 4.1.11 on local Linux;
hosted failing runner used Node 22.23.4.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored data, API, message, repository or release format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Jump to End still focuses the draft first and preserves reduced-motion and
smooth navigation. It repeats the same end request once on the next animation
frame so deferred editor measurement cannot leave the newly enlarged scroll
range uncovered. The original 1-pixel assertion, deadlines and process
boundaries are unchanged; there is no test retry.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run specification checks, push the owning PR and require a complete fresh
hosted workflow before merging. Then refresh lapis-server and Desktop to the
merged native lease release without treating this UI repair as native evidence.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001 and JST-UI-015 now explicitly require the end request to
  cover deferred next-frame editor layout while retaining full-draft geometry.
  JST-API-001 remains unchanged because the repair adds no transport or command.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Focused local UI evidence does not replace the
  complete hosted gate, heavy qualification or publication requirements.
```
<!-- /evidence-field -->
