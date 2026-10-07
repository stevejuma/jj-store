---
schema: 1
id: E-EXEC-017
---

# E-EXEC-017 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte -t "Pending reply"
```

**Notes**

No tests selected because generated story test names use PascalCase. Not counted as regression proof; reran the full exact story file.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Before the source-aligned fix: new pending-state regression observes entire composer opacity 0.6. Cold local-folder acceptance also reported insufficient contrast in warning/retained reply context. Restored Community interactiveDrawerWhenDisabled using the existing pinned source revision and Design Core public prop; no local styling override.
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

Expanded direct consumers because the shared composer status behavior changed: 48 stories across ten families pass, including pending recovery contrast, palette behavior, repository threads and exact reply recovery. Fresh Vite/browser process; accessibility errors enforced; baselines unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-discussions.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-runtime-preparation.test.ts web/tests/vite/local-inbox-threads.test.ts
```

**Notes**

Final 16 focused unit tests pass.
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
pnpm web:check
pnpm catalog:check
pnpm check:architecture
pnpm check:ui-ownership
pnpm check:boundaries
```

**Notes**

Final UI/catalog zero diagnostics, source budgets, Community ownership and 17-package boundary checks pass.
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
pnpm contracts:check
```

**Notes**

Generated schemas match; 300 actual HTTP/SSE responses conform, seven contract tests and TypeScript contract consistency pass. Private journal fields do not change public session DTOs.
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
node scripts/docs.mjs review configuration "Private delivery journals retain queued batches and exact started acknowledgements, with generation-scoped reservations and cancellation-safe initial continuation context."
node scripts/docs.mjs review application "Incremental feedback errors retain drafts; pending reply context and recovery status use the same readable interactive drawer setting as Community."
pnpm web:build
```

**Notes**

Services and HTTP topics also reviewed with specific collector/refusal explanations. Final production bundle and docs staging pass: nine diagrams, 141-file closure, valid links/anchors. Existing bundle-size warning remains.
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
pnpm dev:backend:build
pnpm dev:backend:restart
```

**Notes**

Final backend rebuilt and development service restarted at 7462. Vite remains on 7460; existing .local/dev-demo fixture preserved and search ready. No browser reload/draft mutation or installed executable replacement.
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
pnpm spec:first
```

**Notes**

Provisional pending E-EXEC-017 was not accepted as a completed unchanged-contract confirmation. Final governed evidence/check is recorded after acceptance.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Second run reached repository-screen reply and explicit resume; new helper call used the wrong argument order, causing Invalid URL. Corrected to its existing signature.
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
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Third run passed full native incremental browser flow. Local-folder replies exposed one contrast rule failure across pending composer warning/context nodes. Fixed via the source-aligned Community prop and reproduced in the new focused story.
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
python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Fourth cold run passed full native flow and fixed local pending-state accessibility. New local thread-screen step timed out because the existing repository thread screen is native-only. The local profile now uses its supported ordinary collaboration command API; native still exercises the real thread screen. This preserves the planned supported-screen/client boundary; no new local navigation support is claimed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The named-test validator accepts fn/def/test declarations, not Storybook Story labels. Removed the invalid named-test references; exact story regression commands/results remain recorded in Runs. Rust named regression references remain validated.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Fifth run again passed native flow; local read helper hit the expected stale_local_metadata fence between capturing a pin and reading its thread. Added at most three fresh read selections for that exact 409 code. Mutations and request identities are never retried by this helper.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
pnpm check:boundaries
pnpm check:ui-ownership
pnpm web:check
pnpm catalog:check
pnpm check:architecture
```

**Notes**

Completed boundary confirmation: generated contracts and 300 real responses, 17 package boundaries, owned Community presentation, zero web/catalog diagnostics and source budgets pass. The retained historical cold-script failures do not change these unchanged interface/ownership results.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Sixth run passed native behavior. Local accessibility exposed busy reply context without a warning/status: the parent composer still had opacity 0.6. Added a focused busy-state story and enabled the existing Design Core interactive drawer for reply/edit context as well as status. No CSS override or accessibility waiver.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte -t 'Saving reply keeps context readable without a status'
```

**Notes**

All three tests skipped because generated test names use PascalCase. This is not regression evidence; immediately reran the exact full story file.
<!-- /run -->
<!-- /evidence-field -->

