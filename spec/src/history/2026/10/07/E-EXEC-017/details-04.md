---
schema: 1
id: E-EXEC-017
---

# E-EXEC-017 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Before final fix: two passed, busy-state story failed with actual opacity 0.6 instead of 1. Editor and Send were already correctly disabled.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

After fix: all three pass, including pending recovery and busy context without a status. Send and editor remain disabled during the save.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-threads catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte
```

**Notes**

49 affected stories across ten families pass in a cold Vite/browser process, with accessibility enforcement. Existing palette, thread, reaction, recovery and execution presentation pass; no visual baselines updated.
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
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
pnpm docs:check
```

**Notes**

Zero web/catalog diagnostics, 273 catalog modules and owned Community boundary pass. Documentation closure includes 141 files and nine light/dark diagrams; links and anchors valid.
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
pnpm check:architecture
```

**Notes**

Formatter expanded the drawer condition to 301 counted composer lines (limit 300). Expressed the same boolean condition directly with Boolean(status || context); no behavior or budget relaxation.
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
pnpm exec prettier --write web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte catalog/fixtures/AppearanceComposerFixture.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte docs/maintenance/coverage.json
pnpm check:architecture
```

**Notes**

Final source fits the unchanged architecture budget; 1263 modules checked. Application documentation fingerprint reviewed for the drawer and prompt-refusal changes.
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
jj --no-pager diff --check
```

**Notes**

Optional whitespace inspection flag is unsupported by this installed JJ version. No files changed. Scoped Prettier and affected-package rustfmt checks supply formatting verification.
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
cargo fmt -p jj-store-server -p jj-store-service -p jj-store-http -- --check
```

**Notes**

Affected Rust packages are formatted; no Rust changes after focused package checks and tests.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Five changed authored documentation files needed wrapping/table formatting. Applied Prettier to exactly those files; generated evidence pages already matched.
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
pnpm exec prettier --write docs/site/guides/agent-execution.md spec/src/current/contracts/collaboration/36-interactive-agent-execution.md spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/progress/M23/history/M23/index.md spec/src/current/progress/M23/index.md
pnpm web:build
```

**Notes**

Formatted owning docs and rebuilt the final production assets. Documentation closure passes (141 files, nine diagrams). Existing chunk-size warning remains; no bundle or visual threshold relaxed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Final Boolean expression and formatted source: all three composer stories pass in a new browser process.
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
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Full native real-RustFS and registered-folder Codex/jj-acp browser flow passed, including persisted stopped-session replies and reactions with no implicit prompt; repository-thread screen (native) and supported ordinary API (local); next Send includes new revisions; later Send excludes delivered revisions; edit becomes eligible; original request retry retains exact prepared event. Native code publication refusal stays enforced. Both session histories, replies and delivery state survive service restart; explicit resume sends no prompt, and the following explicit Send does not repeat prior feedback. Accessibility enforced. Disposable fixtures cleaned; source dirty files preserved. Final rebuilt output passed again after the semantically equivalent drawer boolean simplification (logs context-cold-browser-final-accepted.log and context-cold-browser-delivery.log).
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

144 tasks and 640 evidence records; seven validators, 178 requirements; 27 protected files and three canonical owning chapters pass before final evidence completion. The same gates are rerun after recording final results.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
JS
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

After final cold completion, passing evidence and M23 history update: all changed frontend/script/document files formatted; 144 tasks, 640 evidence records, seven validators, 178 requirements and the spec-first boundary pass. M23 status remains unchanged; no unrelated aggregate lanes or baselines claimed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-008
- JST-ARCH-001
- JST-ARCH-004
- JST-DOC-002
- JST-MUT-001
- JST-CLI-004
- JST-COMPAT-001
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

Enclosing Jujutsu change kkrwxwvw. macOS ARM64; Rust 1.98.1, JJ 0.45.1, Node 24.10.0, pnpm 10.32.1, Vitest 4.1.11. Installed Codex through jj-acp control v2; explicit development executable paths.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Backward-compatible private JSON journal additions (delivered revisions, prompt context batch, original sanitized intent and generation-scoped reservations). Ordinary canonical AgentStore/ThreadStore/ReactionStore and public session schemas unchanged. Two new prompt-refusal HTTP codes; no jj-acp protocol change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Second slice of ordinary replies with incremental agent context. Before: commands.rs used all feedback replies on every Send and excluded primary thread discussion. New collector captures one authorized repository snapshot and exact immutable revision identities; durable preparation/acknowledgement prevents queued duplicate delivery and retains unknown outcomes without prompt replay.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain runner-independent repository collaboration and incremental delivery. Optional strict Clippy audits remain unqualified due to unchanged jj-store-jj and native_inventory style debt; no full lint pass is claimed. Pooling, native code reducer-3 publication and visual baseline approval remain separate work. Root aggregate check/test/build lanes are out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-MUT-001/JST-CLI-004 repository mutation/CLI semantics,
  JST-COMPAT-001 native formats, JST-ARCH-001/JST-ARCH-004 source/layer
  ownership, JST-GOV-002/JST-GOV-008 governance and JST-DOC-002 documentation
  contracts remain unchanged. Read-only service composition and private
  supervisor state do not alter canonical stores, publication, ordinary command
  receipts, ACP protocol, or packaged presentation dependencies.

```
<!-- /evidence-field -->

