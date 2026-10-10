---
schema: 1
id: E-CHAT-QUEUE-001
---

# E-CHAT-QUEUE-001 details 5

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/AgentSessionState.stories.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte
```

**Notes**

13 pass: all 10 queue interaction/axe stories and three session-state stories, including enabled independent Stop while exact Queue confirmation is pending. One older footer story still fails initial controls before formatting assertions. /tmp/jj-queue-pending-stop-final.log. No full direct-family pass claim.
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
pnpm catalog:check
```

**Notes**

Final independent Stop rendering and preview/alignment helper: 0 errors and 0 warnings; 380 modules pass boundaries. /tmp/jj-queue-catalog-stop-final.log.
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
pnpm check:progress
```

**Notes**

146 tasks, 742 evidence records; pending evidence accurately retains the broader headless startup gap. /tmp/jj-queue-progress-complete.log.
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
pnpm spec:validate
```

**Notes**

7 validators, 3530 chapters, 181 requirements pass. /tmp/jj-queue-spec-complete.log.
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
pnpm spec:first
```

**Notes**

38 protected files, 3 canonical chapters; unchanged wire/source-ownership confirmation and changed UI owner accepted. /tmp/jj-queue-first-complete.log.
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

1 validator, 3530 chapters, 181 requirements; native ownership declarations and copied attribution pass. /tmp/jj-queue-ownership-complete.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Broader headless AgentFleet/ConversationFooterLayout startup remains
  unqualified: parent comparison reproduces three AgentFleet initial failures,
  changed tree has seven and an intermittent footer initial failure. Focused
  queue/typography/alignment interaction and accessibility checks pass; do not
  claim a full direct-family pass."
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; Storybook 10.5.7; Svelte 5.57.1. Validation uses the pre-existing local Design Core package hook; portable manifests and lockfile unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing AgentExecution/AgentConversation API version 1, ACP control version 3, native storage format 2 and review publication contracts unchanged. ChatPromptSubmission.author and publication=queued are internal UI state only; no generated contract, browser storage or credentials added.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-018 now requires content-sensitive Send/Stop selection and
  captured pending-prompt cards shared by both views. JST-COLLAB-034 enqueueing,
  cancellation, exact identities and publication authority remain unchanged.
  JST-GOV-002/008 unchanged-contract confirmation: application-owned queue and
  extracted legacy projection preserve source ownership and copied attribution;
  no external Community dependency. JST-API-001/006 unchanged-contract
  confirmation in the owning API chapter: no endpoint, generated DTO,
  authorization, journal or control format changes; existing semantic
  acknowledgements and exact cancellation/status reads are used. Queued text
  typography, centred short-text alignment, symmetric card padding and bounded
  disclosure are presentation-only; original captured submissions and dispatch
  identities remain intact."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This evidence qualifies the focused UI
  acceptance, not real adapter/provider, live permission/cancellation,
  desktop/WKWebView or platform gates. See queued-messages-review.md; enclosing
  revision tszmluon.

```
<!-- /evidence-field -->

<!-- evidence-field:diagnostics markdown -->
## Diagnostics

Inherited architecture oversizes remain assigned to their existing source owners and are outside the narrow queue acceptance. No baseline or check was relaxed. Generic Svelte no-config warnings come from the existing Vite setup. Temporary partial states and failed iterations are recorded without a full-suite pass claim. Headless parent baseline proves three older initial-control failures; additional changed-tree startup failures remain unresolved, and are not labelled wholly inherited. Live AgentAdvanced preview passes. Broader headless shell readiness is a remaining evidence gap; no tests or timeouts were weakened. Narrow queue functionality remains verified.
<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->
## Excluded scope

Root aggregate pnpm spec:check/check/test/build, Rust checks/builds, full Storybook, real provider prompting, installed services, fixture resets, publication and visual-baseline comparison/updates are out of scope under the repository targeted policy. Cold isolated Chromium Storybook processes qualify this UI/lifecycle surface only. Storybook preview stays running on literal loopback port 6048.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the implemented queue slice using Jujutsu and retain the workspace and Storybook. Focused queue/session-state acceptance and type checks pass. Broader headless shell startup qualification remains open in this pending evidence record; no full-suite/provider or visual-baseline acceptance is claimed.
<!-- /evidence-field -->

<!-- evidence-field:preview_verification markdown -->
## Preview verification

Live Storybook on http://127.0.0.1:6048/: current Document and Classic preview tabs retained. Native hover confirms action opacity=1 and :hover=true. Full row opacity=0.7 includes the avatar; white card contains avatar and text, metadata outside below. Browser confirms normal and queued font 14px/23.1px, DM Sans Variable; short text/avatar centre difference=0px; card paddingTop=paddingBottom=8px and paragraph margin=0px. Long-preview story shows See More and unchanged captured full text through expansion, view switches and navigation. Screenshot is a preview, not an approved visual baseline.
<!-- /evidence-field -->

<!-- evidence-field:architecture_comparison_source markdown -->
## Architecture comparison source

import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {sourceFiles,countedLines,fileKind,limits} from '/Users/stevejuma/code/lapismd/jj-store-queued-messages/scripts/web-architecture.mjs';
const root='/Users/stevejuma/code/lapismd/jj-store-queued-messages';
const governed=new Set(sourceFiles(root));
const changed=execFileSync('jj',['diff','--name-only'],{cwd:root,encoding:'utf8'}).trim().split('\n').filter(path=&gt;governed.has(path));
const baseline=JSON.parse(readFileSync(`${root}/scripts/web-architecture-baseline.json`,'utf8'));
const regressions=[];
for(const path of changed){
 const count=countedLines(readFileSync(`${root}/${path}`,'utf8'));
 const allowed=baseline.files[path]??limits[fileKind(path)];
 let before=0;
 try { before=countedLines(execFileSync('jj',['file','show','-r','@-',path],{cwd:root,encoding:'utf8',stdio:['ignore','pipe','ignore']})); } catch {}
 if(count&gt;allowed&amp;&amp;count&gt;before) regressions.push(`${path}: ${before} -> ${count}; limit ${allowed}`);
}
if(regressions.length) throw new Error(regressions.join('\n'));
console.log(`No new or enlarged governed source oversizes in ${changed.length} changed modules. Existing oversizes remain.`);

<!-- /evidence-field -->

